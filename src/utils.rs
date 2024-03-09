use std::fs;
use std::str::from_utf8;

pub fn get_test(name: &str) -> Vec<u8> {
    let loc = String::from(env!("CARGO_MANIFEST_DIR")) + "/test_data/" + name;
    fs::read(loc).unwrap()
}

pub fn get_test_str(name: &str) -> String {
    let buf = get_test(name);
    from_utf8(&buf).unwrap().to_owned();
}
