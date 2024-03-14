use std::any::Any;

use serde::{ Serialize, Deserialize };

use super::{ CollapsedObject, ConstructedObject, Object, Properties, Property, Raw, Value };
use crate::playmission::{
    error::{Result, PlaymissionError as Error},
    filemap::Filemap
};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename = "ACTIVEPROP", rename_all = "SCREAMING_SNAKE_CASE")]
pub struct UserDataRaw {
    properties: Properties,
    data: String,
    expanded_size: u32,
}

impl Raw for UserDataRaw {

    // based on if any loading needs to happen at all,
	// returns self as either intermediary or object
	fn begin(mut self: Box<Self>) -> Result<ConstructedObject> {

        let orientation_property = Property::new(Value::String(self.data), None);
        self.properties.add("Data", orientation_property)?;
        let orientation_property = Property::new(Value::Int(self.expanded_size), None);
        self.properties.add("Expanded Size", orientation_property)?;

        let new = UserData {
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
struct UserData {
    properties: Properties,
}

impl Object for UserData {
    
	fn into_any(self: Box<Self>) -> Box<dyn Any> {
        self as Box<dyn Any>
    }

    fn collapse(mut self: Box<Self>) -> Result<CollapsedObject> {
        
        let files = Filemap::new();

        let Value::String(data) = self.properties.take_value("Data")? else {
            return Err(Error::WrongTypeFound("Data".into(), "VTYPE_STRING".into()))
        };
        let Value::Int(expanded_size) = self.properties.take_value("Expanded Size")? else {
            return Err(Error::WrongTypeFound("Expanded Size".into(), "VTYPE_INT".into()))
        };

        let raw = UserDataRaw {
            properties: self.properties,
            data,
            expanded_size,
        };
        let raw = Box::new(raw) as Box<dyn Raw>;

        Ok(CollapsedObject::new(raw, files))

    }

}