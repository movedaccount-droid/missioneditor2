use serde::{ Serialize, Deserialize };

use super::{ CollapsedObject, ConstructedObject, Intermediary, Object, Properties, Property, Raw, Value };
use crate::playmission::{
    error::PlaymissionError as Error,
    error::Result,
    filemap::Filemap
};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename = "ACTIVEPROP", rename_all = "SCREAMING_SNAKE_CASE")]
pub struct PropRaw {
    properties: Properties,
    #[serde(rename = "DATAFILE")]
    datafile_name: String,
    orientation: String,
}

impl PropRaw {
    const DEFAULT: &'static str = "Default.prop";
}

impl Raw for PropRaw {

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

impl Intermediary for PropRaw {

    // request datafile and default
    fn files(&self) -> Result<Vec<&str>> {
        Ok(vec![&self.datafile_name, Self::DEFAULT.into()])
    }

    // parses datafile and default for remaining properties
    fn construct(mut self, files: Filemap) -> Result<ConstructedObject> {

        let mut files = *files;

        let datafile = files.remove(&self.datafile_name).ok_or(Error::MissingFile(self.datafile_name))?;
        let default = files.get(Self::DEFAULT).ok_or(Error::MissingFile(Self::DEFAULT.into()))?;

        let orientation_property = Property::new(Value::String(self.orientation), None);
        self.properties.add("orientation", orientation_property)?;

        let new = Prop {
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
pub struct Prop {
    properties: Properties,
    datafile: Properties,
    datafile_name: String,
}

impl Object for Prop {
    
}