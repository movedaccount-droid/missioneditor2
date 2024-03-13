use serde::{ Serialize, Deserialize };

use super::{ ConstructedObject, Object, Properties, Raw };
use crate::playmission::error::Result;

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
    properties: Properties,
}

impl Object for Rule {
    
}