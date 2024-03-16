// structs representing object properties

use std::{collections::HashMap, ops::{Deref, DerefMut}};
use serde::{ Deserialize, Serialize, Deserializer, Serializer };

use crate::playmission::{
    datafile,
    error::{PlaymissionError as Error, Result},
    xmlcleaner,
};

// represents the type of a value in an Property
#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Bool(bool),
    Float(f32),
    Int(i32),
    String(String),
}

impl Value {
    // construct and convert value based on vtype string
    pub fn new<T: AsRef<str>, U: AsRef<str>>(v: T, vtype: U) -> Result<Self> {
        let vr = v.as_ref();
        let e = |s: &str| Error::WrongTypeCast(vr.into(), s.into());
        match vtype.as_ref() {
            "VTYPE_BOOL" => Ok(Self::Bool(
                vr.to_ascii_lowercase()
                    .parse()
                    .map_err(|_| e("VTYPE_BOOL"))?)
            ),
            "VTYPE_FLOAT" => {
                Ok(Self::Float(vr.parse().map_err(|_| e("VTYPE_FLOAT"))?))
            }
            "VTYPE_INT" => {
                Ok(Self::Int(vr.parse().map_err(|_| e("VTYPE_INT"))?))
            }
            _ => Ok(Self::String(vr.into()))
        }
    }

    // returns vtype string for value type
    pub fn vtype(&self) -> &str {
        match self {
            Self::Bool(_) => "VTYPE_BOOL",
            Self::Float(_) => "VTYPE_FLOAT",
            Self::Int(_) => "VTYPE_INT",
            Self::String(_) => "VTYPE_STRING",
        }
    }
}

impl ToString for Value {
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

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
struct PropertyRaw {
    name: String,
    vtype: String,
    value: String,
    flags: Option<String>,
}

impl PropertyRaw {
    // decompose property back into raw
    pub fn from_property(name: &str, property: &Property) -> Self {
        Self {
            name: name.into(),
            vtype: property.value().vtype().into(),
            value: property.value().to_string(),
            flags: property.flags.clone(),
        }
    }
}

// intermediary for a property
#[derive(Debug, PartialEq, Clone)]
pub struct Property {
    value: Value,
    flags: Option<String>,
}

impl Property {
    // creates new intermediary property
    pub fn new(value: Value, flags: Option<String>) -> Self {
        Self { value, flags: flags }
    }

    // parses new property with typed enum from raw serde output
    fn from_raw(mut raw: PropertyRaw) -> Result<(String, Self)> {
        let name = raw.name;
        // remove trailing 'f' from floats
        if raw.vtype == "VTYPE_FLOAT" && raw.value.ends_with("f") {
            raw.value.truncate(raw.value.len() - 1)
        }
        let value = Value::new(raw.value, &raw.vtype)?;
        let new = Self { value, flags: raw.flags };
        Ok((name, new))
    }

    // get ref to value
    pub fn value(&self) -> &Value {
        &self.value
    }

    // get ref to flags
    fn flags(&self) -> Option<&str> {
        self.flags.as_deref()
    }

    // consume property into value, dropping key
    fn take_value(self) -> Value {
        self.value
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename = "PROPERTIES", rename_all = "SCREAMING_SNAKE_CASE")]
struct PropertiesRaw {
    #[serde(rename = "PROPERTY")]
    properties: Vec<PropertyRaw>,
}

impl PropertiesRaw {
    // creates new raw properties
    fn from_vec(properties: Vec<PropertyRaw>) -> Self {
        Self { properties }
    }

    // decomposes properties back into raw
    fn from_properties(properties: &Properties) -> Self {
        let mut raws = vec![];
        for (k, v) in properties.iter() {
            raws.push(PropertyRaw::from_property(&k, v));
        }
        Self::from_vec(raws)
    }

}

// intermediary for properties
#[derive(Default, Debug, PartialEq, Clone)]
pub struct Properties(HashMap<String, Property>);

impl Properties {
    // creates empty mapping of properties
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    // parses new mapping from raw serde output
    fn from_raw(raw: PropertiesRaw) -> Result<Self> {
        let mut new = Self::new();
        for property in raw.properties {
            let (name, property) = Property::from_raw(property)?;
            new.insert(name, property);
        }
        Ok(new)
    }

    // parses new mapping from datafile and default buffers
    pub fn from_datafile_default(datafile: Vec<u8>, default: Vec<u8>) -> Result<Self> {
        let parsed_datafile: Properties = datafile::deserialize(&datafile)?;
        let parsed_default: Properties = xmlcleaner::deserialize(&default)?;
        parsed_default.default_for(parsed_datafile)
    }

    // add property to map, returning error if name already taken
    pub fn add<T: Into<String>>(&mut self, k: T, v: Property) -> Result<()> {
        let k = k.into();
        if self.contains_key(&k) {
            Err(Error::TakenKey(k))
        } else {
            self.insert(k, v);
            Ok(())
        }
    }

    // create new property and add to map
    pub fn insert_new<K, V, T>(&mut self, k: K, v: V, vtype: T, flags: Option<&str>) -> Result<()>
    where
        K: Into<String>,
        V: AsRef<str>,
        T: AsRef<str>,
    {
        let new = Property {
            value: Value::new(v, vtype)?,
            flags: flags.map(|s| s.into())
        };

        self.insert(k.into(), new);
        Ok(())
    }

    // get property value from map directly, returning error if missing
    pub fn get_value<T: AsRef<str>>(&self, k: T) -> Result<&Value> {
        self.get(k.as_ref())
            .map(|v| v.value())
            .ok_or(Error::MissingProperty("Filename".into()))
    }

    // take property value from map directly, returning error if missing
    pub fn take_value<T: AsRef<str>>(&mut self, k: T) -> Result<Value> {
        self.remove(k.as_ref())
            .map(|v| v.take_value())
            .ok_or(Error::MissingProperty("Filename".into()))
    }

    // merges properties over each other. self is used as a template for
    // other: the types of self are maintained, though strings are coerced.
    // keys and names are maintained. this is mainly used for default values
    pub fn default_for(mut self, other: Self) -> Result<Self> {

        for (k, v) in other.into_iter() {

            let Some(default) = self.remove(&k) else {
                self.insert(k, v);
                continue
            };

            let v_vtype = v.value().vtype().into();
            let default_vtype = default.value().vtype().into();

            let new_value = if let Value::String(s) = v.value {
                Value::new(s, default_vtype)?
            } else if v_vtype == default_vtype {
                v.value
            } else {
                return Err(Error::MergedWrongType(k.into(), v_vtype, default_vtype))
            };

            let new_flags = match v.flags {
                Some(flags) => Some(flags),
                None => default.flags
            };

            self.insert(k, Property::new(new_value, new_flags));

        }

        Ok(self)
    }

    // replaces a single property's value and returns old value,
    // or inserts as new VTYPE_STRING and returns nothing,
    // or fails
    pub fn replace_or_add_property_value(&mut self, k: impl AsRef<str>, v: impl Into<String>) -> Result<Option<Value>> {

        let k = k.as_ref();
        
        let old_property = self.remove(&k.to_owned());

        if let Some(existing) = old_property {

            let vtype = existing.value().vtype().to_owned();
            let flags = existing.flags().map(String::from);
    
            if let Err(e) = self.insert_new(k, v.into(), vtype, flags.as_deref()) {
                self.insert(k.into(), existing);
                Err(e)
            } else {
                Ok(Some(existing.take_value()))
            }

        } else {
    
            self.insert_new(k, v.into(), "VTYPE_STRING", None)?;
            Ok(None)

        }

    }

}

impl Deref for Properties {
    type Target = HashMap<String, Property>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Properties {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl IntoIterator for Properties {
    type Item = (String, Property);
    type IntoIter = std::collections::hash_map::IntoIter<String, Property>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'de> Deserialize<'de> for Properties {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Properties, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;
        let raw = PropertiesRaw::deserialize(deserializer)?;
        Properties::from_raw(raw).map_err(Error::custom)
    }
}

impl Serialize for Properties {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let raw = PropertiesRaw::from_properties(self);
        raw.serialize(serializer)
    }
}