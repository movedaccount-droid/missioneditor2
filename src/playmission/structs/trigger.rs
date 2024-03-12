use serde::{ Serialize, Deserialize };

use super::{ CollapsedObject, ConstructedObject, Intermediary, Object, Properties, Property, Raw, Value };
use crate::playmission::{
    error::PlaymissionError as Error,
    error::Result,
    filemap::Filemap
};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename = "ACTIVEPROP", rename_all = "SCREAMING_SNAKE_CASE")]
pub struct TriggerRaw {
    properties: Properties,
    #[serde(rename = "DATAFILE")]
    datafile_name: String,
    orientation: String,
}

impl TriggerRaw {
    const DEFAULT: &'static str = "Default.trigger";
}

impl Raw for TriggerRaw {

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

impl Intermediary for TriggerRaw {

    // request datafile and default
    fn files(&self) -> Result<Vec<&str>> {
        Ok(vec![&self.datafile_name, Self::DEFAULT.into()])
    }

    // parses datafile and default for remaining properties
    fn construct(mut self, mut files: Filemap) -> Result<ConstructedObject> {

        let datafile = files.remove(&self.datafile_name).ok_or(Error::MissingFile(self.datafile_name.clone()))?;
        let default = files.get(Self::DEFAULT).ok_or(Error::MissingFile(Self::DEFAULT.into()))?;

        let orientation_property = Property::new(Value::String(self.orientation), None);
        self.properties.add("orientation", orientation_property)?;

        let new = Trigger {
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
struct Trigger {
    properties: Properties,
    datafile: Properties,
    datafile_name: String,
}

impl Object for Trigger {
    
}