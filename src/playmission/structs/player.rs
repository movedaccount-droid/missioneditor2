use std::any::Any;

use serde::{ Serialize, Deserialize };

use super::{ traits::Prerequisite, CollapsedObject, ConstructedObject, Intermediary, Object, Properties, Raw };
use crate::playmission::{
    error::Result,
    filemap::Filemap
};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename = "ACTIVEPROP", rename_all = "SCREAMING_SNAKE_CASE")]
pub struct PlayerRaw {
    properties: Properties,
    orientation: String,
    start_position: String,
    start_orientation: String,
}

impl Raw for PlayerRaw {

    // based on if any loading needs to happen at all,
	// returns self as either intermediary or object
	fn begin(self: Box<Self>) -> Result<ConstructedObject> {
        Ok(ConstructedObject::more(*self))
    }

	// cast self to serialize
	fn as_serialize(self: Box<Self>) -> Box<dyn erased_serde::Serialize> {
        Box::new(self)
    }

}

impl Intermediary for PlayerRaw {

    // nothing needed, move straight to target
    // TODO: we shgould use raw for this
    fn files(&self) -> Result<Vec<Prerequisite>> {
        Ok(vec![])
    }

    // parses datafile and default for remaining properties
    fn construct(self: Box<Self>, _files: Filemap) -> Result<ConstructedObject> {

        let new = Player {
            properties: self.properties,
            orientation: self.orientation,
            start_position: self.start_position,
            start_orientation: self.start_orientation,
        };

        Ok(ConstructedObject::done(new))

    }

    fn collapse(self: Box<Self>, _files: Filemap) -> Result<CollapsedObject> {
        todo!()
    }

}

#[derive(Debug, PartialEq, Clone)]
struct Player {
    properties: Properties,
    orientation: String,
    start_position: String,
    start_orientation: String,
}

impl Object for Player {
    
	fn into_any(self: Box<Self>) -> Box<dyn Any> {
        self as Box<dyn Any>
    }

}