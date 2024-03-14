use std::collections::HashSet;
use std::str;

use fancy_regex::{ Captures, Match, Regex, Replacer };
use lazy_static::lazy_static;
use serde::{ Deserialize, Serialize };
use quick_xml::{ se, de };

use super::error::{ PlaymissionError, Result };

lazy_static! {
    static ref OBJECTS: HashSet<&'static str> = {
        let mut m = HashSet::new();
        m.insert("ACTIVEPROP");
        m.insert("CHARACTER");
        m.insert("DOOR");
        m.insert("GAME");
        m.insert("LOCATION");
        m.insert("MEDIA");
        m.insert("PICKUP");
        m.insert("PLAYER");
        m.insert("PROP");
        m.insert("PROPERTIES");
        m.insert("PROPERTY");
        m.insert("RULE");
        m.insert("SPECIALEFFECT");
        m.insert("TRIGGER");
        m.insert("USERDATA");
        m
    };
}

// returns position of first byte of matching tag for first found opening tag
fn find_matching_tag(s: &str) -> Result<Option<Match>> {
    let mut depth = 0;
    let tag = Regex::new(r"<(.)(.*?)>").unwrap();
    let option = tag.captures_iter(s).find(|captures| {
        if let Ok(captures) = captures {
            if captures.get(1).unwrap().as_str() == "/" {
                depth -= 1;
                depth == 0
            } else {
                depth += 1;
                false
            }
        } else {
            true
        }
    });

    option.map(|result|
        result.map(|captures|
            captures.get(0).unwrap()
        ).map_err(|e|
            PlaymissionError::from(e)
        )
    ).transpose()
}

// replaces missionmaker illegal namespace syntax with xml-compliant elements,
// well-suited for quick-xml parsing
fn clean<T: Into<String>>(s: T) -> Result<String> {

    let opening_tag = Regex::new(r"<\w+: (\w+) >").unwrap();
    let mut s = s.into();
    let mut result: String = String::from("");

    while let Some(captures) = opening_tag.captures(&s)? {

        let open = captures.get(0).unwrap();
        let name = captures.get(1).unwrap().as_str();
        let new_open = &*format!("<{name}>");
        let new_close = &*format!("</{name}>");

        if let Some(end) = find_matching_tag(&s)? {
            result.push_str(&s[..open.start()]);
            result.push_str(new_open);
            result.push_str(&*clean(&s[open.end()..end.start()])?);
            result.push_str(new_close);
            s = s[end.end()..].to_string()
        } else {
            return Err(PlaymissionError::NoMatchingTag(open.as_str().into()))
        }

    };

    result.push_str(&s);

    Ok(result)

}

// replace overlapping regex, assuming the replacement pattern will not
// shift any characters before the index
fn replace_overlapping<T: Into<String>, R: Replacer>(s: T, re: &str, mut replacement: R) -> Result<String> {
    let re = Regex::new(re).unwrap();
    let mut s = s.into();
    let mut result = String::from("");
    while let Some(mtch) = re.find(s.as_ref())? {
        let i = mtch.start();
        result.push_str(&s[0..i]);
        s = re.replace(&s[i..], replacement.by_ref()).into();
    }
    result.push_str(&s);
    Ok(result)
}

// replaces xml-compliant elements with missionmaker illegal namespace syntax
fn dirty<T: Into<String>>(s: T) -> Result<String> {

    let legal_tag = r"(?s)<(\w+)>(.*?)</\1>";
    let illegal_tag = |captures: &Captures| {
        let subtype = captures.get(1).unwrap().as_str();
        let contents = captures.get(2).unwrap().as_str();
        let tag = if OBJECTS.contains(subtype) { "OBJECT" } else { "ATTR" };
        format!("<{tag}: {subtype} >{contents}</{tag}>")
    };
    replace_overlapping(s, legal_tag, illegal_tag)

}

// convenience to pipeline xml from byte buffer to finished object
pub fn deserialize<T: for<'de> Deserialize<'de>>(v: &[u8]) -> Result<T> {

    let s = str::from_utf8(v)?;
    let clean = clean(s)?;
    Ok(de::from_str(&clean)?)

}

pub fn serialize(v: &impl Serialize) -> Result<Vec<u8>> {
    let mut buf = String::new();
    let mut se = se::Serializer::new(&mut buf);
    se.indent(' ', 4);
    v.serialize(se)?;
    let dirty = dirty(buf)?;
    Ok(dirty.into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::get_test_str;

    #[test]
    fn get_clean() {
        let expected = get_test_str("cleaner_clean.txt");
        let dirty = get_test_str("cleaner_dirty.txt");

        let found = clean(dirty).unwrap();
        assert_eq!(expected, found)
    }

    #[test]
    fn get_dirty() {
        let expected = get_test_str("cleaner_dirty.txt");
        let clean = get_test_str("cleaner_clean.txt");

        let found = dirty(clean).unwrap();
        assert_eq!(expected, found)
    }
}
