// structs representing the object elements of the root game/mission object
use std::any::Any;

use crate::playmission::xml::intermediaries::{ Properties, Property, IntermediaryMission, Value };
use crate::playmission::structs::Object;
use crate::playmission::filemap::Filemap;

// intermediary for highest-level mission container object
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct IntermediaryMission {
    #[serde(rename = "ExpandedSize")]
    pub expanded_size: u32,
    #[serde(rename = "BLANKINGPLATES")]
    pub blanking_plates: String,
    #[serde(rename = "Meta")]
    pub meta: String,
    pub properties: Properties,

    #[serde(default)]
    #[serde(rename = "OBJECT")]
    pub intermediaries: Vec<Box<dyn Intermediary>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct MissionObject {
	properties: Properties,
	files: Filemap,
}

impl MissionObject {

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