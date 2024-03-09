use regex::Regex;

// shorthand to run regex replacement
fn replace(s: &str, rx: &str, rep: &str) -> String {
    let rx = Regex::new(rx).unwrap();
    rx.replace_all(s, rep).into_owned()
}

// replaces missionmaker illegal namespace syntax with xml-compliant tagging
// and elements, well-suited for quick-xml parsing
pub fn clean(mut s: String) -> String {
    // illegal colon -> legal tag
    s = replace(&s, r"<(\w+): (\w+) >", r#"<$1 variant="$2">"#);

    // attr as variant -> attr as element
    s = replace(&s, r#"<ATTR variant="(\w+)">(.+?)</ATTR>"#, r"<$1>$2</$1>");

    // property as variant -> property as element
    s = replace(
        &s,
        r#"(?s)<OBJECT variant="PROPERTY">(.+?)</OBJECT>"#,
        r"<PROPERTY>$1</PROPERTY>",
    );

    // properties as variant -> properties as element
    s = replace(
        &s,
        r#"(?s)<OBJECT variant="PROPERTIES">(.+?)</OBJECT>"#,
        r"<PROPERTIES>$1</PROPERTIES>",
    );

    // game as variant -> game as element
    replace(
        &s,
        r#"(?s)<OBJECT variant="GAME">(.+)</OBJECT>"#,
        r"<GAME>$1</GAME>",
    )
}

// replaces xml-compliant tagging with missionmaker illegal namespace syntax
pub fn dirty(mut s: String) -> String {
    // properties as element -> properties as variant
    s = replace(
        &s,
        r"(?s)<GAME>(.+?)</GAME>",
        r#"<OBJECT variant="GAME">$1</OBJECT>"#,
    );

    // properties as element -> properties as variant
    s = replace(
        &s,
        r"(?s)<PROPERTIES>(.+?)</PROPERTIES>",
        r#"<OBJECT variant="PROPERTIES">$1</OBJECT>"#,
    );

    // property as element -> property as variant
    s = replace(
        &s,
        r"(?s)<PROPERTY>(.+?)</PROPERTY>",
        r#"<OBJECT variant="PROPERTY">$1</OBJECT>"#,
    );

    // attr as element -> attr as variant
    s = replace(&s, r"<(\w+)>(.+?)</\w+>", r#"<ATTR variant="$1">$2</ATTR>"#);

    // legal tag -> illegal colon
    replace(&s, r#"<(\w+) variant="(\w+)">"#, r"<$1: $2 >")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::get_test;
    use std::str::from_utf8;

    #[test]
    fn get_clean() {
        let expected_raw = get_test("cleaner_clean.txt");
        let dirty_raw = get_test("cleaner_dirty.txt");

        let expected = from_utf8(&expected_raw).unwrap().to_owned();
        let dirty = from_utf8(&dirty_raw).unwrap().to_owned();

        let found = clean(dirty);
        assert_eq!(expected, found)
    }

    #[test]
    fn get_dirty() {
        let expected_raw = get_test("cleaner_dirty.txt");
        let clean_raw = get_test("cleaner_clean.txt");

        let expected = from_utf8(&expected_raw).unwrap().to_owned();
        let clean = from_utf8(&clean_raw).unwrap().to_owned();

        let found = dirty(clean);
        assert_eq!(expected, found)
    }
}
