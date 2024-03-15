use serde::{ Serialize, Deserialize };

use super::{ traits::ObjectHandler, CollapsedObject, ConstructedObject, Object, Properties, Raw, Value };
use crate::playmission::{
    error::{Result, PlaymissionError as Error},
    filemap::Filemap
};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename = "USER_DATA", rename_all = "SCREAMING_SNAKE_CASE")]
pub struct UserDataRaw {
    properties: Properties,
    data: String,
    expanded_size: u32,
}

impl Raw for UserDataRaw {

    // based on if any loading needs to happen at all,
	// returns self as either intermediary or object
    fn begin(mut self: Box<Self>) -> Result<ConstructedObject> {

        self.properties.insert_new("Data", self.data, "VTYPE_STRING", None)?;
        self.properties.insert_new("Expanded Size", self.expanded_size.to_string(), "VTYPE_INT", None)?;

        let handler = Box::new(UserData);

        let new = Object::new(handler, self.properties, None, None, None);

        Ok(ConstructedObject::done(new))
    }

	// cast self to serialize
	fn as_serialize(self: Box<Self>) -> Box<dyn erased_serde::Serialize> {
        Box::new(self)
    }

}

struct UserData;

impl ObjectHandler for UserData {

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

        let Value::String(data) = properties.take_value("Data")? else {
            return Err(Error::WrongTypeFound("Data".into(), "VTYPE_STRING".into()))
        };

        let Value::Int(expanded_size) = properties.take_value("Expanded Size")? else {
            return Err(Error::WrongTypeFound("Expanded Size".into(), "VTYPE_INT".into()))
        };

        let raw = UserDataRaw {
            properties,
            data,
            expanded_size,
        };
        let raw = Box::new(raw) as Box<dyn Raw>;

        Ok(CollapsedObject::new(raw, files))

    }

}