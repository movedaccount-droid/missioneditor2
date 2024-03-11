use std::collections::HashMap;
use std::fs;
use std::io::{Cursor, Read, Seek};

use error::FilemapError as Error;
use error::Result;

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
    pub fn add(&mut self, name: Into<String>, buf: Vec<u8>) -> Result<()> {
        return match self.get(&name) {
            Some(_) => Err(Error::Taken(name)),
            None => {
                self.insert(name.into(), buf);
                Ok(())
            }
        };
    }

    // get a file from the filemap by running a closure on its name
    pub fn get_closure(&self, closure: impl Fn(AsRef<str>) -> bool) -> Option<&Vec<u8>> {
        self.0.iter().find(|(k, _)| closure(k.as_ref())).map(|(_, v)| v)
    }
}

impl Deref for Filemap {
    type Target = HashMap<String, Vec<u8>>;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

pub mod error {
    use thiserror::Error;

    pub type Result<T> = std::result::Result<T, FilemapError>;

    #[derive(Debug, Error)]
    pub enum FilemapError {
        #[error("zip failure")]
        Zip {
            #[from]
            source: zip::result::ZipError,
        },
        #[error("io failure")]
        Io {
            #[from]
            source: std::io::Error,
        },
        #[error("key already taken: {0}")]
        Taken(String),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::get_test;

    fn from(name: &str) -> Filemap {
        let mut raw = get_test(name);
        let mut cursor = Cursor::new(raw);
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
        let found = filemap.get_closure(|s: &str| s.ends_with("oo"));
        assert_eq!(expected, found.cloned());
    }

    #[test]
    fn get_closure_without_match() {
        let mut filemap = from("filemap.zip");
        let expected = None;
        let found = filemap.get_closure(|s: &str| s.ends_with("ooo"));
        assert_eq!(expected, found.cloned());
    }
}
