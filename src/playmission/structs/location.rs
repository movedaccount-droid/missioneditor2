use std::any::Any;

use serde::{ Serialize, Deserialize };

use super::{ traits::Prerequisite, CollapsedObject, ConstructedObject, Intermediary, Object, Properties, Property, Raw, Value };
use crate::playmission::{
    error::{PlaymissionError as Error, Result},
    filemap::Filemap, xmlcleaner
};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename = "LOCATION", rename_all = "SCREAMING_SNAKE_CASE")]
pub struct LocationRaw {
    properties: Properties,
    #[serde(rename = "DATAFILE")]
    datafile_name: String,
    bbox_min: String,
    bbox_max: String,
}

impl LocationRaw {
    const DEFAULT: &'static str = "Default.Tile";
}

impl Raw for LocationRaw {

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

impl Intermediary for LocationRaw {

    // request datafile and default
    fn files(&self) -> Result<Vec<Prerequisite>> {
        Ok(vec![
            Prerequisite::new(&self.datafile_name, false),
            Prerequisite::new(Self::DEFAULT, true)
        ])
    }

    // parses datafile and default for remaining properties
    fn construct(mut self: Box<Self>, mut files: Filemap) -> Result<ConstructedObject> {

        let datafile = files.remove(&self.datafile_name).ok_or(Error::MissingFile(self.datafile_name.clone()))?;
        let default = files.remove(Self::DEFAULT).ok_or(Error::MissingFile(Self::DEFAULT.into()))?;

        let bbox_min_property = Property::new(Value::String(self.bbox_min), None);
        self.properties.add("Bounding Box Min", bbox_min_property)?;
        let bbox_max_property = Property::new(Value::String(self.bbox_max), None);
        self.properties.add("Bounding Box Max", bbox_max_property)?;

        let new = Location {
            properties: self.properties,
            datafile: Properties::from_datafile_default(datafile, default)?,
            datafile_name: self.datafile_name,
        };

        Ok(ConstructedObject::done(new))

    }

}

#[derive(Debug, PartialEq, Clone)]
struct Location {
    properties: Properties,
    datafile: Properties,
    datafile_name: String,
}

impl Object for Location {
    
	fn into_any(self: Box<Self>) -> Box<dyn Any> {
        self as Box<dyn Any>
    }

   // iteratively collapses to raw stage and emits files to place in filemap
   fn collapse(mut self: Box<Self>) -> Result<CollapsedObject> {
    let mut files = Filemap::new();
    files.add(&self.datafile_name, xmlcleaner::serialize(&self.datafile)?)?;

    let Value::String(bbox_min) = self.properties.take_value("Bounding Box Min")? else {
        return Err(Error::WrongTypeFound("Bounding Box Min".into(), "VTYPE_STRING".into()))
    };
    let Value::String(bbox_max) = self.properties.take_value("Bounding Box Max")? else {
        return Err(Error::WrongTypeFound("Bounding Box Max".into(), "VTYPE_STRING".into()))
    };

    let raw = LocationRaw {
        properties: self.properties,
        datafile_name: self.datafile_name,
        bbox_min,
        bbox_max
    };
    let raw = Box::new(raw) as Box<dyn Raw>;

    Ok(CollapsedObject::new(raw, files))
}

}