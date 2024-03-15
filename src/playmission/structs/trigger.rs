use serde::{ Serialize, Deserialize };

use super::{ traits::{ObjectHandler, Prerequisite}, CollapsedObject, ConstructedObject, Intermediary, Object, Properties, Property, Raw, Value };
use crate::playmission::{
    error::{PlaymissionError as Error, Result},
    filemap::Filemap, xmlcleaner
};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename = "TRIGGER", rename_all = "SCREAMING_SNAKE_CASE")]
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
	fn begin(self: Box<Self>) -> Result<ConstructedObject> {
        Ok(ConstructedObject::more(*self))
    }

	// cast self to serialize
	fn as_serialize(self: Box<Self>) -> Box<dyn erased_serde::Serialize> {
        Box::new(self)
    }

}

impl Intermediary for TriggerRaw {

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

        let datafile = Properties::from_datafile_default(datafile, default)?;
        let handler = Box::new(Trigger);

        let new = Object::new(handler, self.properties, Some(datafile), Some(self.datafile_name), None);

        Ok(ConstructedObject::done(new))

    }

}

struct Trigger;

impl ObjectHandler for Trigger {

	// handles internal state for property updates
	fn view_property_update(&self, k: &str, v: &Value) -> Result<()> {
        Ok(())
    }

	// sama datafile
	fn view_datafile_update(&self, k: &str, v: &Value) -> Result<()> {
        Ok(())
    }

	// sama file
	fn view_file_update(&self, k: &str, v: &[u8]) -> Result<()> {
        Ok(())
    }

	// iteratively collapses to raw stage and emits files to place in filemap
	fn collapse(&self, mut properties: Properties, datafile: Properties, datafile_name: Option<String>, mut files: Filemap) -> Result<CollapsedObject> {

        let datafile_name = datafile_name.ok_or(Error::NoDatafileName)?;
        files.add(datafile_name.clone(), xmlcleaner::serialize(&datafile)?)?;

        let Value::String(orientation) = properties.take_value("Orientation")? else {
            return Err(Error::WrongTypeFound("Orientation".into(), "VTYPE_STRING".into()))
        };

        let raw = TriggerRaw {
            properties: properties,
            datafile_name: datafile_name,
            orientation
        };
        let raw = Box::new(raw) as Box<dyn Raw>;

        Ok(CollapsedObject::new(raw, files))
    }

}