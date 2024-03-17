use serde::{ Serialize, Deserialize };
use uuid::Uuid;

use super::{ traits::{ObjectHandler, Prerequisite}, CollapsedObject, ConstructedObject, Intermediary, Object, Properties, Raw };
use crate::{playmission::{
    error::{PlaymissionError as Error, Result},
    filemap::Filemap,
    structs::Value
}, three::Scene};

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
        let handler = Box::new(Media);

        let new: Object = Object::new(handler, self.properties, None, None, Some(files));

        Ok(ConstructedObject::done(new))

    }

}

pub struct Media;

impl ObjectHandler for Media {

    // renders object to canvas
	fn render(&mut self, uuid: &Uuid, properties: &Properties, datafile: &Properties, files: &Filemap, scene: &mut Scene) -> Result<()> {

        // nothing to render for this object ...
        Ok(())

	}

	// handles internal state for property updates
	fn view_property_update(&mut self, k: &str, v: &Value) -> Result<()> {
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
        let raw = Box::new(MediaRaw { properties });
        Ok(CollapsedObject::new(raw, files))
    }

    fn r#type(&self) -> &'static str {
        "MEDIA"
    }

}