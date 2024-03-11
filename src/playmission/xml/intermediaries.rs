// structures for reading and writing from .mission files with serde
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::HashMap;

use crate::playmission::filemap::Filemap;
use crate::playmission::structs::{ MissionObject, Object, Prop };
use super::error::MissionSerdeError as Error;

// intermediary for highest-level mission container object
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct IntermediaryMission {
    #[serde(rename = "ExpandedSize")]
    pub expanded_size: u32,
    #[serde(rename = "BLANKINGPLATES")]
    pub blanking_plates: String,
    #[serde(rename = "Meta")]
    pub meta: String,
    pub properties: Properties,

    #[serde(default)]
    #[serde(rename = "OBJECT")]
    pub intermediaries: Vec<Intermediary>,
}

// intermediary for game objects
// we have to manually define all renames because the
// impl_deserialize_for_internally_tagged_enum macro doesn't handle rename_all
#[derive(Serialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Intermediary {
    ActiveProp {
        #[serde(rename = "DATAFILE")]
        datafile: String,
        #[serde(rename = "PROPERTIES")]
        properties: Properties,
        #[serde(rename = "ORIENTATION")]
        orientation: String,
    },
    Character {
        #[serde(rename = "DATAFILE")]
        datafile: String,
        #[serde(rename = "PROPERTIES")]
        properties: Properties,
        #[serde(rename = "ORIENTATION")]
        orientation: String,
    },
    Door {
        #[serde(rename = "DATAFILE")]
        datafile: String,
        #[serde(rename = "PROPERTIES")]
        properties: Properties,
        #[serde(rename = "ORIENTATION")]
        orientation: String,
    },
    Location {
        #[serde(rename = "DATAFILE")]
        datafile: String,
        #[serde(rename = "PROPERTIES")]
        properties: Properties,
        #[serde(rename = "BBOX_MIN")]
        bbox_min: String,
        #[serde(rename = "BBOX_MAX")]
        bbox_max: String,
    },
    Media {
        #[serde(rename = "PROPERTIES")]
        properties: Properties,
    },
    Pickup {
        #[serde(rename = "DATAFILE")]
        datafile: String,
        #[serde(rename = "PROPERTIES")]
        properties: Properties,
        #[serde(rename = "ORIENTATION")]
        orientation: String,
    },
    Player {
        #[serde(rename = "PROPERTIES")]
        properties: Properties,
        #[serde(rename = "ORIENTATION")]
        orientation: String,
        #[serde(rename = "START_POSITION")]
        start_position: String,
        #[serde(rename = "START_ORIENTATION")]
        start_orientation: String,
    },
    Prop {
        #[serde(rename = "DATAFILE")]
        datafile: String,
        #[serde(rename = "PROPERTIES")]
        properties: Properties,
        #[serde(rename = "ORIENTATION")]
        orientation: String,
    },
    Rule {
        #[serde(rename = "PROPERTIES")]
        properties: Properties,
    },
    SpecialEffect {
        #[serde(rename = "DATAFILE")]
        datafile: String,
        #[serde(rename = "PROPERTIES")]
        properties: Properties,
        #[serde(rename = "ORIENTATION")]
        orientation: String,
    },
    Trigger {
        #[serde(rename = "DATAFILE")]
        datafile: String,
        #[serde(rename = "PROPERTIES")]
        properties: Properties,
        #[serde(rename = "ORIENTATION")]
        orientation: String,
    },
    UserData {
        #[serde(rename = "PROPERTIES")]
        properties: Properties,
        #[serde(rename = "DATA")]
        data: String,
        #[serde(rename = "ExpandedSize")]
        expanded_size: u32,
    },
}

impl Intermediary {
    // returns name of prerequisite datafile containing object properties
    pub fn datafile(&self) -> Option<&str> {
        match self {
            Self::ActiveProp { datafile, .. }
            | Self::Character { datafile, .. }
            | Self::Door { datafile, .. }
            | Self::Location { datafile, .. }
            | Self::Pickup { datafile, .. }
            | Self::Prop { datafile, .. }
            | Self::SpecialEffect { datafile, .. }
            | Self::Trigger { datafile, .. } => Some(&datafile),
            Self::Media { .. }
            | Self::Player { .. }
            | Self::Rule { .. }
            | Self::UserData { .. } => None,
        }
    }

    // returns name of default file containing default datafile values
    pub fn default(&self) -> Option<&str> {
        match self {
            Self::ActiveProp { .. } => Some("Default.aprop"),
            Self::Character { .. } => Some("Default.character"),
            Self::Door { .. } => Some("Default.door"),
            Self::Location { .. } => Some("Default.Tile"),
            Self::Pickup { .. } => Some("Default.pickup"),
            Self::Prop { .. } => Some("Default.prop"),
            Self::SpecialEffect { .. } => Some("Default.effect"),
            Self::Trigger { .. } => Some("Default.trigger"),
            Self::Media { .. }
            | Self::Player { .. }
            | Self::Rule { .. }
            | Self::UserData { .. } => None,
        }
    }

    // get mutable properties
    // ishould haveeee fucckiing used structurs!! for tthisdss !!! whatever
    pub fn properties_mut(&mut self) -> &mut Properties {
        match self {
            Self::ActiveProp { properties, .. }
            | Self::Character { properties, .. }
            | Self::Door { properties, .. }
            | Self::Location { properties, .. }
            | Self::Pickup { properties, .. }
            | Self::Prop { properties, .. }
            | Self::SpecialEffect { properties, .. }
            | Self::Trigger { properties, .. }
            | Self::Media { properties, .. }
            | Self::Player { properties, .. }
            | Self::Rule { properties, .. }
            | Self::UserData { properties, .. } => properties,
        }
    }

    // get properties ref
    // godd\
    pub fn properties(&self) -> &Properties {
        match self {
            Self::ActiveProp { properties, .. }
            | Self::Character { properties, .. }
            | Self::Door { properties, .. }
            | Self::Location { properties, .. }
            | Self::Pickup { properties, .. }
            | Self::Prop { properties, .. }
            | Self::SpecialEffect { properties, .. }
            | Self::Trigger { properties, .. }
            | Self::Media { properties, .. }
            | Self::Player { properties, .. }
            | Self::Rule { properties, .. }
            | Self::UserData { properties, .. } => properties,
        }
    }

    // merges properties with another map of property values
    pub fn merge_properties(&mut self, new: HashMap<String, Property>) -> Result<(), Error> {
        self.properties_mut().merge(new)
    }

    // returns vec of resource files required by object
    pub fn files(&self) -> Result<Vec<String>, Error> {
        let f = |k: &str| self.properties().get(k.to_string()).ok_or(Error::NoFileEntry(k.to_string())).map(|v| v.value.to_string());
        let v = match self {
            Self::ActiveProp { .. } => vec![f("Object")?],
            Self::Character { .. } => vec![f("Head")?, f("Torso Object")?, f("Legs Object")?],
            Self::Door { .. } => vec![f("Object")?],
            Self::Location { .. } => vec![f("Blanking Plate Filename")?],
            Self::Pickup { .. } => vec![f("Object")?],
            Self::Prop { .. } => vec![f("Object")?],
            Self::Trigger { .. } => vec![f("Object")?],
            Self::SpecialEffect { .. }
            | Self::Media { .. }
            | Self::Player { .. }
            | Self::Rule { .. }
            | Self::UserData { .. } => vec![],
        };
        Ok(v)
    }

    // ohughgh my god. please let me get ouf t of here
    pub fn construct(self, files: Filemap) -> Result<Box<dyn Object>, Error> {
        let result = match self {
            Self::Prop { .. } => Prop::from_intermediary(self, files)?,
            Self::ActiveProp { .. }
            | Self::Character { .. }
            | Self::Door { .. }
            | Self::Location { .. }
            | Self::Pickup { .. }
            | Self::SpecialEffect { .. }
            | Self::Trigger { .. }
            | Self::Media { .. }
            | Self::Player { .. }
            | Self::Rule { .. }
            | Self::UserData { .. } => todo!(),
        };
        Ok(Box::new(result) as Box<dyn Object>)
    }

}

// custom serde deserializer to parse unusual internal tagged enum pattern
impl_deserialize_for_internally_tagged_enum! {
    Intermediary, "@variant",
    ("ACTIVEPROP" => ActiveProp {
        #[serde(rename = "DATAFILE")] datafile: String,
        #[serde(rename = "PROPERTIES")] properties: Properties,
        #[serde(rename = "ORIENTATION")] orientation: String
    }),
    ("CHARACTER" => Character {
        #[serde(rename = "DATAFILE")] datafile: String,
        #[serde(rename = "PROPERTIES")] properties: Properties,
        #[serde(rename = "ORIENTATION")] orientation: String
    }),
    ("DOOR" => Door {
        #[serde(rename = "DATAFILE")] datafile: String,
        #[serde(rename = "PROPERTIES")] properties: Properties,
        #[serde(rename = "ORIENTATION")] orientation: String
    }),
    ("LOCATION" => Location {
        #[serde(rename = "DATAFILE")] datafile: String,
        #[serde(rename = "PROPERTIES")] properties: Properties,
        #[serde(rename = "BBOX_MIN")] bbox_min: String,
        #[serde(rename = "BBOX_MAX")] bbox_max: String
    }),
    ("MEDIA" => Media {
        #[serde(rename = "PROPERTIES")] properties: Properties
    }),
    ("PICKUP" => Pickup {
        #[serde(rename = "DATAFILE")] datafile: String,
        #[serde(rename = "PROPERTIES")] properties: Properties,
        #[serde(rename = "ORIENTATION")] orientation: String
    }),
    ("PLAYER" => Player {
        #[serde(rename = "PROPERTIES")] properties: Properties,
        #[serde(rename = "ORIENTATION")] orientation: String,
        #[serde(rename = "START_POSITION")] start_position: String,
        #[serde(rename = "START_ORIENTATION")] start_orientation: String
    }),
    ("PROP" => Prop {
        #[serde(rename = "DATAFILE")] datafile: String,
        #[serde(rename = "PROPERTIES")] properties: Properties,
        #[serde(rename = "ORIENTATION")] orientation: String
    }),
    ("RULE" => Rule {
        #[serde(rename = "PROPERTIES")] properties: Properties
    }),
    ("SPECIALEFFECT" => SpecialEffect {
        #[serde(rename = "DATAFILE")] datafile: String,
        #[serde(rename = "PROPERTIES")] properties: Properties,
        #[serde(rename = "ORIENTATION")] orientation: String
    }),
    ("TRIGGER" => Trigger {
        #[serde(rename = "DATAFILE")] datafile: String,
        #[serde(rename = "PROPERTIES")] properties: Properties,
        #[serde(rename = "ORIENTATION")] orientation: String
    }),
    ("USERDATA" => UserData {
        #[serde(rename = "PROPERTIES")] properties: Properties,
        #[serde(rename = "DATA")] data: String,
        #[serde(rename = "ExpandedSize")] expanded_size: u32
    }),
}

// raw intermediary for properties, for parsing
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
struct PropertiesRaw {
    #[serde(rename = "PROPERTY")]
    properties: Vec<PropertyRaw>,
}

impl PropertiesRaw {
    // parses serde-compatible input from intermediary
    fn from_properties(properties: Properties) -> Self {
        let properties = (*properties).into_iter()
            .map(|(name, property)| PropertyRaw::from_property(name, property))
            .collect();
        Self { properties } 
    }
}

// serde implementations intermediary properties and contained properties
impl<'de> Deserialize<'de> for Properties {
    fn deserialize<D>(deserializer: D) -> Result<Properties, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;
        let raw = PropertiesRaw::deserialize(deserializer)?;
        Properties::from_raw(raw).map_err(Error::custom)
    }
}

impl Serialize for Properties {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let raw = PropertiesRaw::from_intermediary(self);
        raw.serialize(serializer)
    }
}

// raw intermediary for a property, for parsing
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
struct PropertyRaw {
    name: String,
    vtype: String,
    value: String,
    flags: Option<String>,
}

impl PropertyRaw {
    // parses serde-compatible input from property
    fn from_property(name: Into<String>, property: Property) -> Self {
        Self {
            name: name.into(),
            vtype: property.value.vtype(),
            value: property.value.to_string(),
            flags: property.flags,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::get_test;
    use quick_xml::de::from_str;
    use std::str::from_utf8;

    #[test]
    fn properties() {
        let mut expected = Properties::new();

        let mut ins =
            |k: &str, v: Property| expected.properties.insert(k.to_string(), v);
        ins(
            "Active",
            Property {
                value: Value::Bool(true),
                flags: None,
            },
        );
        ins(
            "Name",
            Property {
                value: Value::String("Baronial_2Door".to_string()),
                flags: Some("READONLY | HIDDEN".to_string()),
            },
        );
        ins(
            "Size",
            Property {
                value: Value::Float(1.0),
                flags: Some("READONLY | HIDDEN".to_string()),
            },
        );
        ins(
            "Initial Timer Setting",
            Property {
                value: Value::Int(5090),
                flags: None,
            },
        );

        let raw = get_test("missionserde_properties.txt");
        let s = from_utf8(&raw).unwrap();
        let found = from_str(s).unwrap();
        assert_eq!(expected, found)
    }

    #[test]
    fn from() {
        let mut mission_properties = Properties::new();
        mission_properties.properties.insert(
            "Name".to_string(),
            Property {
                value: Value::String("My G:::ame".to_string()),
                flags: Some("HIDDEN".to_string()),
            },
        );
        mission_properties.properties.insert(
            "Save TTS Audio Files".to_string(),
            Property {
                value: Value::Bool(true),
                flags: Some("HIDDEN".to_string()),
            },
        );

        let mut player_properties = Properties::new();
        player_properties.properties.insert(
            "Name".to_string(),
            Property {
                value: Value::String("Pla: yer".to_string()),
                flags: Some("HIDDEN".to_string()),
            },
        );

        let mut pickup_properties = Properties::new();
        pickup_properties.properties.insert(
            "Active".to_string(),
            Property {
                value: Value::Bool(true),
                flags: None,
            },
        );
        pickup_properties.properties.insert(
            "Name".to_string(),
            Property {
                value: Value::String("Ham".to_string()),
                flags: Some("READONLY | HIDDEN".to_string()),
            },
        );

        let mut expected = IntermediaryMission {
			expanded_size: 244,
			blanking_plates: "eNpjY2BgEC7LTC7JL8pMzItPyknMy9bLT8piYDhwkIGhwYSBYYUjkN6vt9d6MxMOlR9AKrcQo3ICUCUIFEBV7t7MzMAfnJzplhnvhFB1A6rKwXFnZjbQ3AZ7FiyqPkDdBzbLHqjShAWPWRMctwJVgGxlYQAA0gVKow".to_string(),
			meta: "bb68tcb0fu097d1v".to_string(),
			properties: mission_properties,
			intermediaries: vec![
				Intermediary::Player {
					properties: player_properties,
					orientation: "0.8447118998, -0.3739055395, 0.3501844704, -0.1550072879".to_string(),
					start_position: "-18.2835502625, 0.8828631639, 16.6819877625".to_string(),
					start_orientation: "0.1426707655, -0.0290981755, 0.9693839550, -0.1977201551".to_string(),
				},
				Intermediary::Pickup {
					datafile: "ham.pickup".to_string(),
					properties: pickup_properties,
					orientation: "1.0, 0.0, 0.0, 0.0".to_string()
				},
			]
		};

        let raw = get_test("missionserde_mission.txt");
        let parsed = from_utf8(&raw).unwrap();
        let found = from_str(parsed).unwrap();
        assert_eq!(expected, found, "left = {:#?}\nright = {:#?}", expected, found);
    }

    #[test]
    fn datafile() {
        let active_prop = Intermediary::ActiveProp {
            datafile: "foo".to_string(),
            properties: Properties::new(),
            orientation: "bar".to_string(),
        };
        assert_eq!(Some("foo"), active_prop.datafile());
        let media = Intermediary::Media {
            properties: Properties::new(),
        };
        assert_eq!(None, media.datafile());
    }
}
