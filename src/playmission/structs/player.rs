use serde::{ Serialize, Deserialize };

use super::{ traits::ObjectHandler, CollapsedObject, ConstructedObject, Object, Properties, Raw, Value };
use crate::playmission::{
    error::{PlaymissionError as Error, Result},
    filemap::Filemap
};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename = "PLAYER", rename_all = "SCREAMING_SNAKE_CASE")]
pub struct PlayerRaw {
    properties: Properties,
    orientation: String,
    start_position: String,
    start_orientation: String,
}

impl Raw for PlayerRaw {

    // based on if any loading needs to happen at all,
	// returns self as either intermediary or object
	fn begin(mut self: Box<Self>) -> Result<ConstructedObject> {

        self.properties.insert_new("Orientation", self.orientation, "VTYPE_STRING", None)?;
        self.properties.insert_new("Start Position", self.start_position, "VTYPE_STRING", None)?;
        self.properties.insert_new("Start Orientation", self.start_orientation, "VTYPE_STRING", None)?;

        let handler = Box::new(Player);

        let new = Object::new(handler, self.properties, None, None, None);

        Ok(ConstructedObject::done(new))
    }

	// cast self to serialize
	fn as_serialize(self: Box<Self>) -> Box<dyn erased_serde::Serialize> {
        Box::new(self)
    }

}

struct Player;

impl ObjectHandler for Player {

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

        let Value::String(orientation) = properties.take_value("Orientation")? else {
            return Err(Error::WrongTypeFound("Orientation".into(), "VTYPE_STRING".into()))
        };

        let Value::String(start_position) = properties.take_value("Start Position")? else {
            return Err(Error::WrongTypeFound("Start Position".into(), "VTYPE_STRING".into()))
        };

        let Value::String(start_orientation) = properties.take_value("Start Orientation")? else {
            return Err(Error::WrongTypeFound("Start Orientation".into(), "VTYPE_STRING".into()))
        };

        let raw = PlayerRaw {
            properties,
            orientation,
            start_position,
            start_orientation,
        };
        let raw = Box::new(raw) as Box<dyn Raw>;

        Ok(CollapsedObject::new(raw, files))

    }

}