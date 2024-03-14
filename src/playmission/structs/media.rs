use std::any::Any;

use serde::{ Serialize, Deserialize };

use super::{ traits::Prerequisite, CollapsedObject, ConstructedObject, Intermediary, Object, Properties, Raw };
use crate::playmission::{
    error::{PlaymissionError as Error, Result},
    filemap::Filemap,
    structs::Value
};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename = "MEDIA", rename_all = "SCREAMING_SNAKE_CASE")]
pub struct MediaRaw {
    properties: Properties,
}

impl Raw for MediaRaw {

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

impl Intermediary for MediaRaw {

    // request media resource file
    fn files(&self) -> Result<Vec<Prerequisite>> {
        if let Value::String(filename) = self.properties.get_value("Filename")? {
            Ok(vec![Prerequisite::new(filename, false)])
        } else {
            Err(Error::WrongTypeFound("Filename".into(), "VTYPE_STRING".into()))
        }
    }

    // parses datafile and default for remaining properties
    fn construct(self: Box<Self>, files: Filemap) -> Result<ConstructedObject> {

        let Value::String(filename) = self.properties.get_value("Filename")? else {
            return Err(Error::WrongTypeFound("Filename".into(), "VTYPE_STRING".into()))
        };

        if !files.contains_key(filename) { return Err(Error::MissingFile("Filename".into())) };

        let new = Media {
            properties: self.properties,
            files
        };

        Ok(ConstructedObject::done(new))

    }

}

#[derive(Debug, PartialEq, Clone)]
struct Media {
    properties: Properties,
    files: Filemap,
}

impl Object for Media {
    
	fn into_any(self: Box<Self>) -> Box<dyn Any> {
        self as Box<dyn Any>
    }

    // iteratively collapses to raw stage and emits files to place in filemap
    fn collapse(self: Box<Self>) -> Result<CollapsedObject> {
        let raw = Box::new(MediaRaw { properties: self.properties });
        Ok(CollapsedObject::new(raw, self.files))
    }

    fn properties(self: &Self) -> &Properties {
        &self.properties
    }

}