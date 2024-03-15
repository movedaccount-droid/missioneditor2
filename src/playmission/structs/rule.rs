use serde::{ Serialize, Deserialize };

use super::{ traits::ObjectHandler, CollapsedObject, ConstructedObject, Object, Properties, Raw, Value };
use crate::playmission::{error::Result, filemap::Filemap};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename = "RULE", rename_all = "SCREAMING_SNAKE_CASE")]
pub struct RuleRaw {
    properties: Properties,
}

impl Raw for RuleRaw {

    // based on if any loading needs to happen at all,
	// returns self as either intermediary or object
	fn begin(self: Box<Self>) -> Result<ConstructedObject>  {

        let handler = Box::new(Rule);
        let new = Object::new(handler, self.properties, None, None, None);
        Ok(ConstructedObject::done(new))

    }

	// cast self to serialize
	fn as_serialize(self: Box<Self>) -> Box<dyn erased_serde::Serialize> {
        Box::new(self)
    }

}

struct Rule;

impl ObjectHandler for Rule {

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

        let raw = RuleRaw {
            properties,
        };
        let raw = Box::new(raw) as Box<dyn Raw>;

        Ok(CollapsedObject::new(raw, files))

    }

}