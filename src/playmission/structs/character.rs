use std::any::Any;

use serde::{ Serialize, Deserialize };

use super::{ traits::Prerequisite, CollapsedObject, ConstructedObject, Intermediary, Object, Properties, Property, Raw, Value };
use crate::playmission::{
    error::PlaymissionError as Error,
    error::Result,
    filemap::Filemap
};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename = "CHARACTER", rename_all = "SCREAMING_SNAKE_CASE")]
pub struct CharacterRaw {
    properties: Properties,
    #[serde(rename = "DATAFILE")]
    datafile_name: String,
    orientation: String,
}

impl CharacterRaw {
    const DEFAULT: &'static str = "Default.character";
}

impl Raw for CharacterRaw {

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

impl Intermediary for CharacterRaw {

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

        let new = Character {
            properties: self.properties,
            datafile: Properties::from_datafile_default(datafile, default)?,
            datafile_name: self.datafile_name,
        };

        Ok(ConstructedObject::done(new))

    }

    fn collapse(self: Box<Self>, _files: Filemap) -> Result<CollapsedObject> {
        todo!()
    }

}

#[derive(Debug, PartialEq, Clone)]
struct Character {
    properties: Properties,
    datafile: Properties,
    datafile_name: String,
}

impl Object for Character {
    
	fn into_any(self: Box<Self>) -> Box<dyn Any> {
        self as Box<dyn Any>
    }

}