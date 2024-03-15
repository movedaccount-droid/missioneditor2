use std::any::Any;

use serde::{ Serialize, Deserialize };
use uuid::Uuid;

use super::{ CollapsedObject, ConstructedObject, Object, Properties, Raw, Value };
use crate::playmission::{
    error::{PlaymissionError as Error, Result},
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
	fn begin(mut self: Box<Self>) -> Result<ConstructedObject> {

        self.properties.insert_new("Orientation", self.orientation, "VTYPE_STRING", None)?;
        self.properties.insert_new("Start Position", self.start_position, "VTYPE_STRING", None)?;
        self.properties.insert_new("Start Orientation", self.start_orientation, "VTYPE_STRING", None)?;

        let new = Player {
            uuid: Uuid::new_v4(),
            properties: self.properties,
        };

        Ok(ConstructedObject::done(new))
    }

	// cast self to serialize
	fn as_serialize(self: Box<Self>) -> Box<dyn erased_serde::Serialize> {
        Box::new(self)
    }

}

#[derive(Debug, PartialEq, Clone)]
struct Player {
    uuid: Uuid,
    properties: Properties,
}

impl Object for Player {
    
	fn into_any(self: Box<Self>) -> Box<dyn Any> {
        self as Box<dyn Any>
    }

    // iteratively collapses to raw stage and emits files to place in filemap
    fn collapse(mut self: Box<Self>) -> Result<CollapsedObject> {

        let Value::String(orientation) = self.properties.take_value("Orientation")? else {
            return Err(Error::WrongTypeFound("Orientation".into(), "VTYPE_STRING".into()))
        };

        let Value::String(start_position) = self.properties.take_value("Start Position")? else {
            return Err(Error::WrongTypeFound("Start Position".into(), "VTYPE_STRING".into()))
        };

        let Value::String(start_orientation) = self.properties.take_value("Start Orientation")? else {
            return Err(Error::WrongTypeFound("Start Orientation".into(), "VTYPE_STRING".into()))
        };

        let object = PlayerRaw {
            properties: self.properties,
            orientation,
            start_position,
            start_orientation,
        };

        let files = Filemap::new();

        Ok(CollapsedObject { raw: Box::new(object), files })

    }

    fn properties(self: &Self) -> &Properties {
        &self.properties
    }

    fn uuid(&self) -> &Uuid {
        &self.uuid
    }

}