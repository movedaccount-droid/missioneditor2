use std::any::Any;

use serde::{ Serialize, Deserialize };
use uuid::Uuid;

use super::{ CollapsedObject, ConstructedObject, Object, Properties, Raw };
use crate::playmission::{error::Result, filemap::Filemap};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename = "ACTIVEPROP", rename_all = "SCREAMING_SNAKE_CASE")]
pub struct RuleRaw {
    properties: Properties,
}

impl Raw for RuleRaw {

    // based on if any loading needs to happen at all,
	// returns self as either intermediary or object
	fn begin(self: Box<Self>) -> Result<ConstructedObject>  {

        let new = Rule {
            uuid: Uuid::new_v4(),
            properties: self.properties,
        };

        Ok(ConstructedObject::done(new))
    }

	// cast self to serialize
	fn as_serialize(self: Box<Self>) -> Box<dyn erased_serde::Serialize> {
        Box::new(self)
    }

}

#[derive(Debug, PartialEq, Clone)]
struct Rule {
    uuid: Uuid,
    properties: Properties,
}

impl Object for Rule {
    
	fn into_any(self: Box<Self>) -> Box<dyn Any> {
        self as Box<dyn Any>
    }

    // iteratively collapses to raw stage and emits files to place in filemap
    fn collapse(self: Box<Self>) -> Result<CollapsedObject> {
        let raw = Box::new(RuleRaw { properties: self.properties });
        Ok(CollapsedObject::new(raw, Filemap::new()))
    }

    fn properties(self: &Self) -> &Properties {
        &self.properties
    }

    fn uuid(&self) -> &Uuid {
        &self.uuid
    }

}