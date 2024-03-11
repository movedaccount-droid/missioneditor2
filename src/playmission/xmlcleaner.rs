use std::collections::HashSet;
use std::str;

use fancy_regex::Regex;
use lazy_static::lazy_static;

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
    }
}

// replace overlapping regex, assuming the replacement pattern will not
// shift any characters before the index
fn replace_overlapping(mut s: AsRef<str>, re: &str, replacement: &str) -> String {
    let re = Regex::new(re).unwrap();
    let mut i = 0;
    let mut result = String::from("");
    while Some(mtch) = let re.find(s.as_ref(), i) {
        i = mtch.start();
        result.push_str(s[0..i]);
        let s = re.replace(s[i..], replacement);
    }
    result.push_str(s);
    result
}

// replaces missionmaker illegal namespace syntax with xml-compliant elements,
// well-suited for quick-xml parsing
fn clean(s: AsRef<str>) -> String {

    let illegal_tag = r"<\w+: (\w+) >(.*?)</\1>"
    let legal_tag = r"<$1>$2<$1>"
    replace_overlapping(s, illegal_tag, legal_tag)

}

// replaces xml-compliant elements with missionmaker illegal namespace syntax
fn dirty(s: &str) -> String {

    let legal_tag = r"<(\w+)>(.*?)<\1>"
    let illegal_tag = |match| {
        let subtype = match.get(1);
        let contents = match.get(2);
        let tag = if OBJECTS.contains(subtype) { "OBJECT" } else { "ATTR" };
        format!("<{tag} {subtype} >{contents}</{tag}>")
    }
    replace_overlapping(s, illegal_tag, legal_tag)

}

// convenience to pipeline xml from byte buffer to finished object
fn deserialize<T>(v: Vec<u8>) -> T {

    let s = str::from_utf8(v);
    let clean = clean(s);
    quick_xml::de::from_str(clean)

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::get_test_str;
    use std::str::from_utf8;

    #[test]
    fn get_clean() {
        let expected = get_test_str("cleaner_clean.txt");
        let dirty = get_test_str("cleaner_dirty.txt");

        let found = clean(dirty);
        assert_eq!(expected, found)
    }

    #[test]
    fn get_dirty() {
        let expected = get_test_str("cleaner_dirty.txt");
        let clean = get_test_str("cleaner_clean.txt");

        let found = dirty(clean);
        assert_eq!(expected, found)
    }
}
