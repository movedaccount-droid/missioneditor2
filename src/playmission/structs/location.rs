use serde::{ Serialize, Deserialize };
use uuid::Uuid;

use super::{ traits::{render_default_orb, ObjectHandler, Prerequisite}, CollapsedObject, ConstructedObject, Intermediary, Object, Properties, Property, Raw, Value };
use crate::{playmission::{
    error::{PlaymissionError as Error, Result},
    filemap::Filemap, xmlcleaner
}, three::{Mesh, Scene}};

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
    const DEFAULT: &'static str = "Default.tile";
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

        let bbox_min = Property::new(Value::String(self.bbox_min), None);
        self.properties.add("Bounding Box Min", bbox_min)?;
        let bbox_max = Property::new(Value::String(self.bbox_max), None);
        self.properties.add("Bounding Box Max", bbox_max)?;

        let datafile = Properties::from_datafile_default(datafile, default)?;
        let handler = Box::new(Location::new());

        let new = Object::new(handler, self.properties, Some(datafile), Some(self.datafile_name), None);

        Ok(ConstructedObject::done(new))

    }

}

pub struct Location {
    mesh: Option<Mesh>
}

impl Location {

    pub fn new() -> Location {
        Location { mesh: None }
    }

}

impl ObjectHandler for Location {

    // renders object to canvas
	fn render(&mut self, uuid: &Uuid, properties: &Properties, datafile: &Properties, files: &Filemap, scene: &mut Scene) -> Result<()> {

        self.mesh = Some(render_default_orb(uuid, properties, scene)?);
        Ok(())

	}

	// handles internal state for property updates
	fn view_property_update(&mut self, k: &str, v: &Value) -> Result<()> {

        let Some(ref mut mesh) = self.mesh else { return Ok(()) };
        let Value::Float(f) = v else { return Ok(()) };

        match k {
            "Position X" => { mesh.position().set_x(*f); }
            "Position Y" => { mesh.position().set_y(*f); }
            "Position Z" => { mesh.position().set_z(*f); }
            _ => {},
        };

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

        let Value::String(bbox_min) = properties.take_value("Bounding Box Min")? else {
            return Err(Error::WrongTypeFound("Bounding Box Min".into(), "VTYPE_STRING".into()))
        };
        let Value::String(bbox_max) = properties.take_value("Bounding Box Max")? else {
            return Err(Error::WrongTypeFound("Bounding Box Max".into(), "VTYPE_STRING".into()))
        };

        let raw = LocationRaw {
            properties: properties,
            datafile_name: datafile_name,
            bbox_min,
            bbox_max,
        };
        let raw = Box::new(raw) as Box<dyn Raw>;

        Ok(CollapsedObject::new(raw, files))
    }

    fn r#type(&self) -> &'static str {
        "LOCATION"
    }

}