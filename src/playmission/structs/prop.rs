// struct for static game props
use std::any::Any;

use crate::playmission::xml::intermediaries::{ Intermediary, IntermediaryProperties, IntermediaryProperty, PropertyValue };
use crate::playmission::structs::Object;
use crate::playmission::filemap::Filemap;
use super::error::{ Result, StructError as Error};

#[derive(Debug, PartialEq, Clone)]
pub struct Prop {
	properties: IntermediaryProperties,
	datafile_path: String,
	files: Filemap,
}

impl Prop {

	// creates new struct
	pub fn new(
		properties: IntermediaryProperties,
		datafile_path: String,
		files: Filemap
	) -> Self {
		Self { properties, datafile_path, files }
	}

	// creates self from intermediary
	pub fn from_intermediary(intermediary: Intermediary, files: Filemap) -> Result<Self> {
		let Intermediary::Prop { datafile, mut properties, orientation } = intermediary else {
			return Err(Error::WrongIntermediary)
		};
		properties.insert_new("orientation", PropertyValue::String(orientation), None);
		Ok(Self { properties, datafile_path: datafile, files })
	}

}

impl Object for Prop {

	fn into_any(self: Box<Self>) -> Box<dyn Any> {
		self
	}

}