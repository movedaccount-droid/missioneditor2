// structs for serialization/deserialization of datafiles
use std::collections::HashMap;
use regex::Regex;
use quick_xml::de::from_str;
use super::xml::intermediaries::{ IntermediaryProperties, IntermediaryProperty, PropertyValue };
use crate::playmission::xmlcleaner;
use error::Result;
use error::DatafileError as Error;


#[derive(Debug, PartialEq, Clone)]
pub struct Datafile {
	pub properties: HashMap<String, IntermediaryProperty>,
}

struct Pair {

}

impl Datafile {
	// creates blank datafile mapping
	pub fn new() -> Self {
		Self { properties: HashMap::new() }
	}

	// creates filled datafile mapping using values and types from default file 
	fn from_default(default: &str) -> Result<Self> {
		let clean = xmlcleaner::clean(default);
		let default: IntermediaryProperties = from_str(&clean)?;
		Ok(Self { properties: default.properties })
	}

	// creates filled datafile mapping from datafile, parsing all values as strings
	pub fn from_datafile(datafile: &str) -> Result<Self> {
		let mut new = Self::new();
		let split_file = Self::split(datafile)?;

		for (k, v) in split_file {
			new.properties.insert(k, IntermediaryProperty::new(
				PropertyValue::String(v),
				None
			));
		};

		Ok(new)
	}

	// creates filled datafile mapping using values and types from default file,
	// overwritten and typechecked to values from a datafile
	pub fn with_default(default: &str, datafile: &str) -> Result<Self> {
		println!("{default}\n\n\n{datafile}");
		let mut new = Self::from_default(default)?;
		let split_file = Self::split(datafile)?;

		for (k, v) in split_file {
			let (value, flags) = match new.properties.get(&k) {
				Some(default) => (PropertyValue::new(v, &default.value.vtype())?, default.flags.to_owned()),
				None => (PropertyValue::String(v), None)
			};

			new.properties.insert(k, IntermediaryProperty::new(
				value,
				flags
			));

		}

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

	// shifts a value from one key to another
	pub fn shift(&mut self, k: &str, new_k: &str) {
		let old = self.properties.remove(k);
		if let Some(v) = old {
			self.properties.insert(new_k.to_string(), v);
		}
	}

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
		put("Active", IntermediaryProperty::new(
			PropertyValue::Bool(true),
			Some("READONLY,HIDDEN".to_string()),
		));
		put("Name", IntermediaryProperty::new(
			PropertyValue::String("Baronial_2Door".to_string()),
			Some("READONLY,HIDDEN".to_string()),
		));
		put("Description", IntermediaryProperty::new(
			PropertyValue::String("".to_string()),
			Some("READONLY,HIDDEN".to_string()),
		));
		put("Float", IntermediaryProperty::new(
			PropertyValue::Float(0.7),
			Some("READONLY,HIDDEN".to_string()),
		));
		put("Size X", IntermediaryProperty::new(
			PropertyValue::Int(6),
			Some("READONLY,HIDDEN".to_string()),
		));
		let expected = Datafile { properties: map };

		let datafile = get_test_str("datafile_datafile.txt");
		let default = get_test_str("datafile_default.txt");

		let found = Datafile::with_default(&default, &datafile).unwrap();
		
		pretty_assert_eq!(expected, found);
	}

}