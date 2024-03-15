use std::io::Read;
// structs for serialization/deserialization of datafiles
use std::str;
use gloo_console::log;

use crate::playmission::structs::{ Properties, Property, Value };
use crate::playmission::error::{Result, PlaymissionError as Error};

// parse datafile to properties
pub fn deserialize(datafile: &[u8]) -> Result<Properties> {

	let mut new = Properties::new();
	let datafile = str::from_utf8(&datafile)?;
	let split_file = split(datafile)?;

	for (k, v) in split_file {
		let property = Property::new(Value::String(v), None);
		new.add(k, property)?;
	};

	Ok(new)
}

// splits datafile into string-typed properties
fn split(datafile: &str) -> Result<Vec<(String, String)>> {

	let mut parsed = vec![];

	for line in datafile.lines().filter(|l| *l != "") {

		let split: Vec<&str> = line.splitn(2, " = ").collect();
		if split.len() != 2 {
			return Err(Error::MalformedDatafileLine(line.into()))
		} else {
			parsed.push((split[0].into(), split[1].into()));
		}

	}

	Ok(parsed)
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::pretty_assert_eq;
	use crate::utils::get_test;

	#[test]
	fn des_with_default() {
		let mut expected = Properties::new();
		let mut put = |s: &str, v| (*expected).insert(s.to_string(), v);
		put("Active", Property::new(
			Value::Bool(true),
			Some("READONLY,HIDDEN".to_string()),
		));
		put("Name", Property::new(
			Value::String("Baronial_2Door".to_string()),
			Some("READONLY,HIDDEN".to_string()),
		));
		put("Description", Property::new(
			Value::String("".to_string()),
			Some("READONLY,HIDDEN".to_string()),
		));
		put("Float", Property::new(
			Value::Float(0.7),
			Some("READONLY,HIDDEN".to_string()),
		));
		put("Size X", Property::new(
			Value::Int(6),
			Some("READONLY,HIDDEN".to_string()),
		));

		let datafile = get_test("datafile_datafile.txt");
		let default = get_test("datafile_default.txt");

		let found = Properties::from_datafile_default(datafile, default).unwrap();
		
		pretty_assert_eq!(expected, found);
	}

}