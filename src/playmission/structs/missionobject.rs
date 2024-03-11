// structs representing the object elements of the root game/mission object
use std::any::Any;

use crate::playmission::xml::intermediaries::{ IntermediaryProperties, IntermediaryProperty, IntermediaryMission, PropertyValue };
use crate::playmission::structs::Object;
use crate::playmission::filemap::Filemap;

#[derive(Debug, PartialEq, Clone)]
pub struct MissionObject {
	properties: IntermediaryProperties,
	files: Filemap,
}

impl MissionObject {

	// creates new struct
	pub fn new(
		properties: IntermediaryProperties,
		files: Filemap
	) -> Self {
		Self { properties, files }
	}

	// creates self from intermediarymission
	pub fn from_remnants(
		mut properties: IntermediaryProperties,
		files: Filemap,
		expanded_size: u32,
		blanking_plates: String,
		meta: String
	) -> Self {
		properties.insert_new("expanded_size", PropertyValue::Int(expanded_size), None);
		properties.insert_new("blanking_plates", PropertyValue::String(blanking_plates), None);
		properties.insert_new("meta", PropertyValue::String(meta), None);
		Self { properties, files }
	}

}

impl Object for MissionObject {

	fn into_any(self: Box<Self>) -> Box<dyn Any> {
		self
	}

}