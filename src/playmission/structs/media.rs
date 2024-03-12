use serde::{ Serialize, Deserialize };

use super::{ CollapsedObject, ConstructedObject, Intermediary, Object, Properties, Raw };
use crate::playmission::{
    error::{PlaymissionError as Error, Result},
    filemap::Filemap,
    structs::Value
};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename = "MEDIA", rename_all = "SCREAMING_SNAKE_CASE")]
pub struct MediaRaw {
    properties: Properties,
}

impl Raw for MediaRaw {

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

impl Intermediary for MediaRaw {

    // request media resource file
    fn files(&self) -> Result<Vec<&str>> {
        if let Value::String(filename) = self.properties.get_value("Filename")? {
            Ok(vec![filename])
        } else {
            Err(Error::WrongTypeFound("Filename".into(), "VTYPE_STRING".into()))
        }
    }

    // parses datafile and default for remaining properties
    fn construct(mut self, files: Filemap) -> Result<ConstructedObject> {

        let Value::String(filename) = self.properties.get_value("Filename")? else {
            return Err(Error::WrongTypeFound("Filename".into(), "VTYPE_STRING".into()))
        };

        if !files.contains_key(filename) { return Err(Error::MissingFile("Filename".into())) };

        let new = Media {
            properties: self.properties,
            files
        };

        Ok(ConstructedObject::done(new))

    }

    fn collapse(mut self, files: Filemap) -> Result<CollapsedObject> {
        todo!()
    }

}

#[derive(Debug, PartialEq, Clone)]
struct Media {
    properties: Properties,
    files: Filemap,
}

impl Object for Media {
    
}