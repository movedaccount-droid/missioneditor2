// structs for serialization/deserialization of datafiles
use std::collections::HashMap;
use regex::Regex;
use quick_xml::de::from_str;
use super::xml::intermediaries::{ Properties, Property, Value };
use crate::playmission::xmlcleaner;
use error::Result;
use error::DatafileError as Error;

// parse datafile to properties
fn deserialize(datafile: &str) -> Result<Properties> {
	let mut new = Self::new();
	let split_file = Self::split(datafile)?;

	for (k, v) in split_file {
		new.properties.insert(k, Property::new(
			Value::String(v),
			None
		));
	};

	Ok(new)
}


// splits datafile into string-typed properties
fn split(datafile: &str) -> Result<Vec<(String, String)>> {

	let equals = Regex::new(r" = ").unwrap();
	let mut parsed = vec![];

	for line in datafile.lines() {

		if line == "" { continue; };
		let split: Vec<&str> = equals.splitn(line, 2).collect();
		if split.len() != 2 {
			return Err(Error::MalformedLine(line.to_owned()))
		} else {
			parsed.push((split[0].to_string(), split[1].to_string()));
		}

	}

	Ok(parsed)
}

pub mod error {
	use thiserror::Error;
	use quick_xml::de::DeError;

	pub type Result<T> = std::result::Result<T, DatafileError>;

	#[derive(Debug, Error)]
	pub enum DatafileError {
		#[error("datafile default deserialization failure")]
		Des {
			#[from]
			source: quick_xml::de::DeError
		},
		#[error("intermediary object failure")]
		Mission {
			#[from]
			source: crate::playmission::xml::error::MissionSerdeError
		},
		#[error("malformed datafile line {0}")]
		MalformedLine(String),
		#[error("key already taken: {0}")]
		Taken(String),
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::pretty_assert_eq;
	use crate::utils::get_test_str;

	#[test]
	fn des_with_default() {
		let mut map = HashMap::new();
		let mut put = |s: &str, v| map.insert(s.to_string(), v);
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
		let expected = Datafile { properties: map };

		let datafile = get_test_str("datafile_datafile.txt");
		let default = get_test_str("datafile_default.txt");

		let found = Datafile::with_default(&default, &datafile).unwrap();
		
		pretty_assert_eq!(expected, found);
	}

}