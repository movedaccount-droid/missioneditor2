use serde::{ Serialize, Deserialize };

use super::{ CollapsedObject, ConstructedObject, Intermediary, Object, Properties, Property, Raw, Value };
use crate::playmission::{
    error::PlaymissionError as Error,
    error::Result,
    filemap::Filemap
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
	fn begin(self) -> Result<ConstructedObject> {
        Ok(ConstructedObject::more(self))
    }

	// cast self to serialize
	fn as_serialize(self) -> Box<dyn erased_serde::Serialize> {
        Box::new(self)
    }

}

impl Intermediary for LocationRaw {

    // request datafile and default
    fn files(&self) -> Result<Vec<&str>> {
        Ok(vec![&self.datafile_name, Self::DEFAULT])
    }

    // parses datafile and default for remaining properties
    fn construct(mut self, files: Filemap) -> Result<ConstructedObject> {

        let mut files = *files;

        let datafile = files.remove(&self.datafile_name).ok_or(Error::MissingFile(self.datafile_name))?;
        let default = files.get(Self::DEFAULT).ok_or(Error::MissingFile(Self::DEFAULT.into()))?;

        let bbox_min_property = Property::new(Value::String(self.bbox_min), None);
        self.properties.add("bbox_min", bbox_min_property)?;
        let bbox_max_property = Property::new(Value::String(self.bbox_max), None);
        self.properties.add("bbox_max", bbox_max_property)?;

        let new = Location {
            properties: self.properties,
            datafile: Properties::from_datafile_default(datafile, default.clone())?,
            datafile_name: self.datafile_name,
        };

        Ok(ConstructedObject::done(new))

    }

    fn collapse(mut self, files: Filemap) -> Result<CollapsedObject> {
        todo!()
    }

}

#[derive(Debug, PartialEq, Clone)]
struct Location {
    properties: Properties,
    datafile: Properties,
    datafile_name: String,
}

impl Object for Location {
    
}