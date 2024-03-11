// structs representing the object elements of the root game/mission object
use std::any::Any;

use crate::playmission::xml::intermediaries::{ Properties, Property, IntermediaryMission, Value };
use crate::playmission::structs::Object;
use crate::playmission::filemap::Filemap;

#[derive(Debug, PartialEq, Clone)]
pub struct MissionObject {
	properties: Properties,
	files: Filemap,
}

impl MissionObject {

	// creates new struct
	pub fn new(
		properties: Properties,
		files: Filemap
	) -> Self {
		Self { properties, files }
	}

	// creates self from intermediarymission
	pub fn from_remnants(
		mut properties: Properties,
		files: Filemap,
		expanded_size: u32,
		blanking_plates: String,
		meta: String
	) -> Self {
		properties.insert_new("expanded_size", Value::Int(expanded_size), None);
		properties.insert_new("blanking_plates", Value::String(blanking_plates), None);
		properties.insert_new("meta", Value::String(meta), None);
		Self { properties, files }
	}

}

impl Object for MissionObject {

	fn into_any(self: Box<Self>) -> Box<dyn Any> {
		self
	}

}