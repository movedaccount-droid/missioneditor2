// structures for reading and writing from .mission files with serde
use super::error::MissionSerdeError as Error;
use quick_xml::impl_deserialize_for_internally_tagged_enum;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::HashMap;

// intermediary for highest-level mission container object
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct IntermediaryMission {
    #[serde(rename = "ExpandedSize")]
    expanded_size: u32,
    #[serde(rename = "BLANKINGPLATES")]
    blanking_plates: String,
    #[serde(rename = "Meta")]
    meta: String,
    properties: IntermediaryProperties,

    #[serde(default)]
    #[serde(rename = "OBJECT")]
    intermediaries: Vec<Intermediary>,
}

// intermediary for game objects
// we have to manually define all renames because the
// impl_deserialize_for_internally_tagged_enum macro doesn't handle rename_all
#[derive(Serialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
enum Intermediary {
    ActiveProp {
        #[serde(rename = "DATAFILE")]
        datafile: String,
        #[serde(rename = "PROPERTIES")]
        properties: IntermediaryProperties,
        #[serde(rename = "ORIENTATION")]
        orientation: String,
    },
    Character {
        #[serde(rename = "DATAFILE")]
        datafile: String,
        #[serde(rename = "PROPERTIES")]
        properties: IntermediaryProperties,
        #[serde(rename = "ORIENTATION")]
        orientation: String,
    },
    Door {
        #[serde(rename = "DATAFILE")]
        datafile: String,
        #[serde(rename = "PROPERTIES")]
        properties: IntermediaryProperties,
        #[serde(rename = "ORIENTATION")]
        orientation: String,
    },
    Location {
        #[serde(rename = "DATAFILE")]
        datafile: String,
        #[serde(rename = "PROPERTIES")]
        properties: IntermediaryProperties,
        #[serde(rename = "BBOX_MIN")]
        bbox_min: String,
        #[serde(rename = "BBOX_MAX")]
        bbox_max: String,
    },
    Media {
        #[serde(rename = "PROPERTIES")]
        properties: IntermediaryProperties,
    },
    Pickup {
        #[serde(rename = "DATAFILE")]
        datafile: String,
        #[serde(rename = "PROPERTIES")]
        properties: IntermediaryProperties,
        #[serde(rename = "ORIENTATION")]
        orientation: String,
    },
    Player {
        #[serde(rename = "PROPERTIES")]
        properties: IntermediaryProperties,
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
        properties: IntermediaryProperties,
        #[serde(rename = "ORIENTATION")]
        orientation: String,
    },
    Rule {
        #[serde(rename = "PROPERTIES")]
        properties: IntermediaryProperties,
    },
    SpecialEffect {
        #[serde(rename = "DATAFILE")]
        datafile: String,
        #[serde(rename = "PROPERTIES")]
        properties: IntermediaryProperties,
        #[serde(rename = "ORIENTATION")]
        orientation: String,
    },
    Trigger {
        #[serde(rename = "DATAFILE")]
        datafile: String,
        #[serde(rename = "PROPERTIES")]
        properties: IntermediaryProperties,
        #[serde(rename = "ORIENTATION")]
        orientation: String,
    },
    UserData {
        #[serde(rename = "PROPERTIES")]
        properties: IntermediaryProperties,
        #[serde(rename = "DATA")]
        data: String,
        #[serde(rename = "ExpandedSize")]
        expanded_size: u32,
    },
}

impl Intermediary {
    // returns name of prerequisite datafile containing object properties
    fn datafile(&self) -> Option<&str> {
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
}

// custom serde deserializer to parse unusual internal tagged enum pattern
impl_deserialize_for_internally_tagged_enum! {
    Intermediary, "@variant",
    ("ACTIVEPROP" => ActiveProp {
        #[serde(rename = "DATAFILE")] datafile: String,
        #[serde(rename = "PROPERTIES")] properties: IntermediaryProperties,
        #[serde(rename = "ORIENTATION")] orientation: String
    }),
    ("CHARACTER" => Character {
        #[serde(rename = "DATAFILE")] datafile: String,
        #[serde(rename = "PROPERTIES")] properties: IntermediaryProperties,
        #[serde(rename = "ORIENTATION")] orientation: String
    }),
    ("DOOR" => Door {
        #[serde(rename = "DATAFILE")] datafile: String,
        #[serde(rename = "PROPERTIES")] properties: IntermediaryProperties,
        #[serde(rename = "ORIENTATION")] orientation: String
    }),
    ("LOCATION" => Location {
        #[serde(rename = "DATAFILE")] datafile: String,
        #[serde(rename = "PROPERTIES")] properties: IntermediaryProperties,
        #[serde(rename = "BBOX_MIN")] bbox_min: String,
        #[serde(rename = "BBOX_MAX")] bbox_max: String
    }),
    ("MEDIA" => Media {
        #[serde(rename = "PROPERTIES")] properties: IntermediaryProperties
    }),
    ("PICKUP" => Pickup {
        #[serde(rename = "DATAFILE")] datafile: String,
        #[serde(rename = "PROPERTIES")] properties: IntermediaryProperties,
        #[serde(rename = "ORIENTATION")] orientation: String
    }),
    ("PLAYER" => Player {
        #[serde(rename = "PROPERTIES")] properties: IntermediaryProperties,
        #[serde(rename = "ORIENTATION")] orientation: String,
        #[serde(rename = "START_POSITION")] start_position: String,
        #[serde(rename = "START_ORIENTATION")] start_orientation: String
    }),
    ("PROP" => Prop {
        #[serde(rename = "DATAFILE")] datafile: String,
        #[serde(rename = "PROPERTIES")] properties: IntermediaryProperties,
        #[serde(rename = "ORIENTATION")] orientation: String
    }),
    ("RULE" => Rule {
        #[serde(rename = "PROPERTIES")] properties: IntermediaryProperties
    }),
    ("SPECIALEFFECT" => SpecialEffect {
        #[serde(rename = "DATAFILE")] datafile: String,
        #[serde(rename = "PROPERTIES")] properties: IntermediaryProperties,
        #[serde(rename = "ORIENTATION")] orientation: String
    }),
    ("TRIGGER" => Trigger {
        #[serde(rename = "DATAFILE")] datafile: String,
        #[serde(rename = "PROPERTIES")] properties: IntermediaryProperties,
        #[serde(rename = "ORIENTATION")] orientation: String
    }),
    ("USERDATA" => UserData {
        #[serde(rename = "PROPERTIES")] properties: IntermediaryProperties,
        #[serde(rename = "DATA")] data: String,
        #[serde(rename = "ExpandedSize")] expanded_size: u32
    }),
}

// intermediary for properties
#[derive(Debug, PartialEq, Clone)]
pub struct IntermediaryProperties {
    pub properties: HashMap<String, IntermediaryProperty>,
}

impl IntermediaryProperties {
    // creates empty mapping of properties
    fn new() -> Self {
        Self {
            properties: HashMap::new(),
        }
    }

    // parses new mapping from raw serde output
    fn from_raw(raw: IntermediaryPropertiesRaw) -> Result<Self, Error> {
        let mut new = Self::new();
        for property in raw.properties {
            let (name, property) = IntermediaryProperty::from_raw(property)?;
            new.properties.insert(name, property);
        }
        Ok(new)
    }
}

impl<'de> Deserialize<'de> for IntermediaryProperties {
    // custom serde deserializer for intermediary properties and contained properties
    fn deserialize<D>(deserializer: D) -> Result<IntermediaryProperties, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;
        let raw = IntermediaryPropertiesRaw::deserialize(deserializer)?;
        IntermediaryProperties::from_raw(raw).map_err(Error::custom)
    }
}

impl Serialize for IntermediaryProperties {
    // custom serde serializer for intermediary properties and contained properties
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let raw = IntermediaryPropertiesRaw::from_intermediary(self);
        raw.serialize(serializer)
    }
}

// raw intermediary for properties, for parsing
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
struct IntermediaryPropertiesRaw {
    #[serde(rename = "PROPERTY")]
    properties: Vec<IntermediaryPropertyRaw>,
}

impl IntermediaryPropertiesRaw {
    // creates empty list of rawproperties
    fn new() -> Self {
        Self { properties: vec![] }
    }

    // parses serde-compatible input from intermediary
    fn from_intermediary(intermediary: &IntermediaryProperties) -> Self {
        let mut raw = Self::new();
        for (name, property) in &intermediary.properties {
            let raw_prop = IntermediaryPropertyRaw::from_intermediary(property, name);
            raw.properties.push(raw_prop);
        }
        raw
    }
}

// intermediary for a property
#[derive(Debug, PartialEq, Clone)]
struct IntermediaryProperty {
    value: PropertyValue,
    flags: Option<String>,
}

impl IntermediaryProperty {
    // parses new property with typed enum from raw serde output
    fn from_raw(raw: IntermediaryPropertyRaw) -> Result<(String, Self), Error> {
        let name = raw.name;
        let value = PropertyValue::new(raw.value, raw.vtype);
        Ok((
            name,
            Self {
                value,
                flags: raw.flags,
            },
        ))
    }
}

// represents the type of a value in an intermediaryproperty
#[derive(Debug, PartialEq, Clone)]
pub enum PropertyValue {
    Bool(bool),
    Float(f32),
    Int(u32),
    String(String),
}

impl PropertyValue {
    // construct and convert value based on vtype string
    pub fn new(value: &str, vtype: &str) -> Result<Self> {
        match vtype {
            "VTYPE_BOOL" => PropertyValue::Bool(
                value
                    .to_ascii_lowercase()
                    .parse()
                    .map_err(|_| Error::FailedBool(value))?,
            ),
            "VTYPE_FLOAT" => {
                PropertyValue::Float(value.parse().map_err(|_| Error::FailedFloat(value))?)
            }
            "VTYPE_INT" => PropertyValue::Int(value.parse().map_err(|_| Error::FailedInt(value))?),
            _ => PropertyValue::String(raw.value),
        }
    }

    // returns vtype string for value type
    pub fn get_vtype(&self) -> String {
        match self {
            Self::Bool(_) => String::from("VTYPE_BOOL"),
            Self::Float(_) => String::from("VTYPE_FLOAT"),
            Self::Int(_) => String::from("VTYPE_INT"),
            Self::String(_) => String::from("VTYPE_STRING"),
        }
    }
}

impl ToString for PropertyValue {
    // returns parsed string from value type
    fn to_string(&self) -> String {
        match self {
            Self::Bool(v) => v.to_string(),
            Self::Float(v) => v.to_string(),
            Self::Int(v) => v.to_string(),
            Self::String(v) => v.clone(),
        }
    }
}

// raw intermediary for a property, for parsing
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
struct IntermediaryPropertyRaw {
    name: String,
    vtype: String,
    value: String,
    flags: Option<String>,
}

impl IntermediaryPropertyRaw {
    // parses serde-compatible input from intermediary
    fn from_intermediary(intermediary: &IntermediaryProperty, name: &str) -> Self {
        Self {
            name: name.to_owned(),
            vtype: intermediary.value.get_vtype(),
            value: intermediary.value.to_string(),
            flags: intermediary.flags.clone(),
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
        let mut expected = IntermediaryProperties::new();

        let mut ins =
            |k: &str, v: IntermediaryProperty| expected.properties.insert(k.to_string(), v);
        ins(
            "Active",
            IntermediaryProperty {
                value: PropertyValue::Bool(true),
                flags: None,
            },
        );
        ins(
            "Name",
            IntermediaryProperty {
                value: PropertyValue::String("Baronial_2Door".to_string()),
                flags: Some("READONLY | HIDDEN".to_string()),
            },
        );
        ins(
            "Size",
            IntermediaryProperty {
                value: PropertyValue::Float(1.0),
                flags: Some("READONLY | HIDDEN".to_string()),
            },
        );
        ins(
            "Initial Timer Setting",
            IntermediaryProperty {
                value: PropertyValue::Int(5090),
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
        let mut mission_properties = IntermediaryProperties::new();
        mission_properties.properties.insert(
            "Name".to_string(),
            IntermediaryProperty {
                value: PropertyValue::String("My G:::ame".to_string()),
                flags: Some("HIDDEN".to_string()),
            },
        );
        mission_properties.properties.insert(
            "Save TTS Audio Files".to_string(),
            IntermediaryProperty {
                value: PropertyValue::Bool(true),
                flags: Some("HIDDEN".to_string()),
            },
        );

        let mut player_properties = IntermediaryProperties::new();
        player_properties.properties.insert(
            "Name".to_string(),
            IntermediaryProperty {
                value: PropertyValue::String("Pla: yer".to_string()),
                flags: Some("HIDDEN".to_string()),
            },
        );

        let mut pickup_properties = IntermediaryProperties::new();
        pickup_properties.properties.insert(
            "Active".to_string(),
            IntermediaryProperty {
                value: PropertyValue::Bool(true),
                flags: None,
            },
        );
        pickup_properties.properties.insert(
            "Name".to_string(),
            IntermediaryProperty {
                value: PropertyValue::String("Ham".to_string()),
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
        let found = from_str(from_utf8(&raw).unwrap()).unwrap();
        assert_eq!(expected, found);
    }

    #[test]
    fn datafile() {
        let active_prop = Intermediary::ActiveProp {
            datafile: "foo".to_string(),
            properties: IntermediaryProperties::new(),
            orientation: "bar".to_string(),
        };
        assert_eq!(Some("foo"), active_prop.datafile());
        let media = Intermediary::Media {
            properties: IntermediaryProperties::new(),
        };
        assert_eq!(None, media.datafile());
    }
}
