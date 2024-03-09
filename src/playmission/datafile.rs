// structs for serialization/deserialization of datafiles
use regex::Regex;
use quick_xml::de::from_str;
use xml::{ IntermediaryProperties, PropertyValue };
use error::{ Result, DatafileError };

#[derive(Debug, PartialEq, Clone)]
struct Datafile {
	properties: HashMap<String, String>,
}

struct Pair {

}

impl Datafile {
	// creates blank datafile mapping
	fn new() -> Self {
		Self { properties: HashMap::new() }
	}

	// creates filled datafile mapping using values and types from default file 
	fn from_default(default: &str) -> Result<Self> {
		let default: IntermediaryProperties = from_str(default)?;
		Self { properties: default.properties }
	}

	// creates filled datafile mapping using values and types from default file,
	// overwritten and typechecked to values from a datafile
	fn with_default(default: &str, datafile: &str) -> Result<Self> {
		let new = Self::from_default(default)?;
		let split_file = Self::deserialize(datafile)?;

		for (k, v) in split_file {
			let Some(default) = new.properties.get(k) else {
				new.properties.insert(k, IntermediaryProperty {
					value: PropertyValue::String(k),
					flags: None
				});
				continue;
			}

			let converted = PropertyValue::new(v, default.vtype)
			new.properties.insert(k, IntermediaryProperty {
				value: converted,
				flags: v.flags,
			});
		}

		Ok(new)
	}

	// splits datafile into string-typed properties
	fn split(datafile: &str) -> Result<Vec<(String, String)>> {

		let equals = Regex::new(r" = ").unwrap();
		let parsed = vec![]

		for line in datafile.split("\n") {

			let split = re.splitn(line, 2)
			let get = |i| split.get(i).map_err(|| Error::MalformedLine(line.to_owned()))?;
			parsed.properties.push((get(0), get(1)))

		}

		Ok(parsed)
	}

	// moves a value from one key to another. always Ok unless
	// destination key already exists
	fn move(k: &str, new_k: &str) {
		todo!()
	}
}

pub mod error {
	use thiserror::Error;
	use quick_xml::de::DeError;

	pub type Result<T> = std::result::Result<T, DatafileError>;

	#[derive(Debug, Error)]
	pub enum DatafileError {
		#[error("datafile deserialization failure")]
		Des {
			#[from]
			source: quick_xml::de::DeError
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
	use crate::utils::get_test;

	#[test]
	fn des_with_default() {
		let mut expected = HashMap::new();
		let put = |s, v| expected.insert(s.to_string(), v.to_string());
		put("Name", "Baronial_2Door");
		put("Filename", "Baronial_2Door.til");
		put("Categories", "TwoDoor, Baronial");
		put("Size X", "6");
		put("Blanking Plate Filename", "victorian_blank.obj");

		let raw = get_test_str("datafile.txt")

	}

}