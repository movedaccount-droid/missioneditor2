use std::collections::HashMap;
use std::collections::hash_map::IntoIter;
use std::io::{ Read, Seek };
use std::ops::{Deref, DerefMut};

use crate::playmission::error::{Result, PlaymissionError as Error};

// manages access to a set of loaded files
#[derive(Clone, Debug, PartialEq)]
pub struct Filemap(HashMap<String, Vec<u8>>);

impl Filemap {
    // creates empty filemap
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    // reads zip from buffer and empties into new filemap
    pub fn from_reader(r: impl Read + Seek) -> Result<Self> {
        let mut zip = zip::ZipArchive::new(r)?;
        let mut new = Self::new();
        let mut i = 0;
        while let Ok(mut file) = zip.by_index(i) {
            let mut buf = vec![];
            file.read_to_end(&mut buf)?;
            new.insert(file.name().to_string(), buf);
            i += 1
        }
        Ok(new)
    }

    // add a file to the filemap, returning an error if the name is
    // already taken
    pub fn add<T: AsRef<str> + Into<String>>(&mut self, name: T, buf: Vec<u8>) -> Result<()> {
        return match self.get(name.as_ref()) {
            Some(_) => Err(Error::TakenFileName(name.into())),
            None => {
                self.insert(name.into(), buf);
                Ok(())
            }
        };
    }

    // take a file from the filemap by running a closure on its name
    pub fn take_closure(&mut self, closure: impl Fn(&str) -> bool) -> Option<Vec<u8>> {
        match self.0.keys().find(|k| closure(&**k)) {
            Some(k) => self.0.remove(&k.clone()),
            None => None
        }
    }

    // merge two filemaps, failing on any overlap
    pub fn merge(&mut self, other: Self) -> Result<()> {
        for k in other.keys() {
            if self.contains_key(k) {
                return Err(Error::TakenFileName(k.into()))
            }
        }

        for (k, v) in other.into_iter() {
            self.insert(k, v);
        }

        Ok(())
    }

}

impl Deref for Filemap {
    type Target = HashMap<String, Vec<u8>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Filemap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl IntoIterator for Filemap {
    type Item = (String, Vec<u8>);
    type IntoIter = IntoIter<String, Vec<u8>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::utils::get_test;
    use std::io::Cursor;

    fn from(name: &str) -> Filemap {
        let raw = get_test(name);
        let cursor = Cursor::new(raw);
        Filemap::from_reader(cursor).unwrap()
    }

    #[test]
    fn loads() {
        let mut expected = HashMap::new();
        expected.insert("foo".to_string(), "oof".as_bytes().to_vec());
        expected.insert("bar".to_string(), "rab".as_bytes().to_vec());
        let expected = Filemap(expected);

        let found = from("filemap.zip");

        assert_eq!(expected, found);
    }

    #[test]
    fn add() {
        let mut filemap = from("filemap.zip");
        if let Err(_) = filemap.add("new".to_string(), "new".as_bytes().to_vec()) {
            panic!("errored when okay");
        }
    }

    #[test]
    fn add_with_existing() {
        let mut filemap = from("filemap.zip");
        if let Ok(_) = filemap.add("foo".to_string(), "new".as_bytes().to_vec()) {
            panic!("inserted when illegal");
        }
    }

    #[test]
    fn get_closure() {
        let mut filemap = from("filemap.zip");
        let expected = Some("oof".as_bytes().to_vec());
        let found = filemap.take_closure(|s: &str| s.ends_with("oo"));
        assert_eq!(expected, found);
    }

    #[test]
    fn get_closure_without_match() {
        let mut filemap = from("filemap.zip");
        let expected = None;
        let found = filemap.take_closure(|s: &str| s.ends_with("ooo"));
        assert_eq!(expected, found);
    }
}
