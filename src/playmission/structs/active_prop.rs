use std::any::Any;

use serde::{ Deserialize, Serialize };

use super::{ traits::Prerequisite, CollapsedObject, ConstructedObject, Intermediary, Object, Properties, Property, Raw, Value };
use crate::playmission::{
    error::{PlaymissionError as Error, Result},
    filemap::Filemap,
    xmlcleaner
};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename = "ACTIVE_PROP", rename_all = "SCREAMING_SNAKE_CASE")]
pub struct ActivePropRaw {
    properties: Properties,
    #[serde(rename = "DATAFILE")]
    datafile_name: String,
    orientation: String,
}

impl ActivePropRaw {
    const DEFAULT: &'static str = "Default.aprop";
}

impl Raw for ActivePropRaw {

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

impl Intermediary for ActivePropRaw {

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

        let orientation_property = Property::new(Value::String(self.orientation), None);
        self.properties.add("Orientation", orientation_property)?;

        let new = ActiveProp {
            properties: self.properties,
            datafile: Properties::from_datafile_default(datafile, default)?,
            datafile_name: self.datafile_name,
        };

        Ok(ConstructedObject::done(new))

    }

}

#[derive(Debug, PartialEq, Clone)]
struct ActiveProp {
    properties: Properties,
    datafile: Properties,
    datafile_name: String,
}

impl Object for ActiveProp {

	fn into_any(self: Box<Self>) -> Box<dyn Any> {
        self as Box<dyn Any>
    }

    fn collapse(mut self: Box<Self>) -> Result<CollapsedObject> {
        
        let mut files = Filemap::new();
        files.add(&self.datafile_name, xmlcleaner::serialize(&self.datafile)?)?;

        let Value::String(orientation) = self.properties.take_value("Orientation")? else {
            return Err(Error::WrongTypeFound("Orientation".into(), "VTYPE_STRING".into()))
        };

        let raw = ActivePropRaw {
            properties: self.properties,
            datafile_name: self.datafile_name,
            orientation
        };
        let raw = Box::new(raw) as Box<dyn Raw>;

        Ok(CollapsedObject::new(raw, files))

    }

}