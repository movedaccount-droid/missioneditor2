// structs representing object properties

// intermediary for properties
#[derive(Debug, PartialEq, Clone)]
pub struct Properties(HashMap<String, Property),
}

impl Properties {
    // creates empty mapping of properties
    pub fn new() -> Self {
        Self {
            properties: HashMap::new(),
        }
    }

    // parses new mapping from raw serde output
    fn from_raw(raw: PropertiesRaw) -> Result<Self, Error> {
        let mut new = Self::new();
        for property in raw.properties {
            let (name, property) = Property::from_raw(property)?;
            new.insert(name, property);
        }
        Ok(new)
    }

    // parses new mapping from datafile buffer
    fn from_datafile(datafile: Vec<u8>) -> Result<Self, Error> {
        xmlcleaner::deserialize(default)?
    }

    // parses new mapping from datafile and default buffers
    fn from_datafile_default(datafile: Vec<u8>, default: Vec<u8>) -> Result<Self, Error> {
        let parsed_datafile: Properties = datafile::deserialize(dat)?;
        let parsed_default: Properties = xmlcleaner::deserialize(default)?;
        parsed_default.merge_over(parsed_datafile)
    }

    // add property to map, returning error if name already taken
    pub fn add(&mut self, k: String, v: Property) -> Result<(), Error> {
        return match self.properties.contains_key(&k) {
            Some(_) => Err(Error::TakenKey(k)),
            None => {
                self.insert(k, v);
                Ok(())
            }
        };
    }

    // merges properties together, failing on any intersection
    pub fn merge(&mut self, other: Self) -> Result<(), Error> {
        for (k, v) in other.properties {
            self.add(k, v)?
        }
        Ok(())
    }

    // merges properties over each other. self is used as a template for
    // other: the types of self are maintained, though strings are coerced.
    // keys and names are maintained. this is mainly used for default values
    pub fn merge_over(&mut self, other: Self) -> Result<(), Error> {

        for (k, v) in other.properties {

            let Some(default) = self.properties.get(k) else {
                self.insert(k, v);
            }

            let new_value = if let Value::String(s) = v.value {
                Value::new(s, default.value.vtype)?
            } else if v.value.vtype == default.value.vtype {
                v.value
            } else {
                return Err(Error::MergedWrongType(k.into(), v.value.vtype.into(), default.value.vtype.into()))
            }

            let new_flags = match v.flags {
                Some(flags) => v.flags,
                None => default.flags
            }

            self.insert(k, Property::new(new_value, new_flags))

        }

        Ok(())
    }

    // shifts a value from one key to another. errors if key already taken
    pub fn shift(&mut self, k: Into<String>, new_k: Into<String>) {
        let old = self.properties.remove(k.into());
        if let Some(v) = old {
            self.add(new_k.into(), v);
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
        Self { value, flags }
    }

    // parses new property with typed enum from raw serde output
    fn from_raw(raw: PropertyRaw) -> Result<(String, Self), Error> {
        let name = raw.name;
        let value = Value::new(raw.value, &raw.vtype)?;
        let new = Self { value, flags: raw.flags };
        Ok((name, new))
    }
}

// represents the type of a value in an Property
#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Bool(bool),
    Float(f32),
    Int(u32),
    String(String),
}

impl Value {
    // construct and convert value based on vtype string
    pub fn new(v: AsRef<str>, vtype: &str) -> Result<Self, Error> {
        v = v.as_ref();
        match vtype {
            "VTYPE_BOOL" => Ok(Self::Bool(
                v.to_ascii_lowercase()
                    .parse()
                    .map_err(|_| Error::WrongType(value, "VTYPE_BOOL"))?)
            ),
            "VTYPE_FLOAT" => {
                Ok(Self::Float(v.parse().map_err(|_| Error::WrongType(value, "VTYPE_FLOAT"))?))
            }
            "VTYPE_INT" => {
                Ok(Self::Int(v.parse().map_err(|_| Error::WrongType(value, "VTYPE_INT"))?))
            }
            _ => Ok(Self::String(v.into()))
        }
    }

    // returns vtype string for value type
    pub fn vtype(&self) -> String {
        match self {
            Self::Bool(_) => String::from("VTYPE_BOOL"),
            Self::Float(_) => String::from("VTYPE_FLOAT"),
            Self::Int(_) => String::from("VTYPE_INT"),
            Self::String(_) => String::from("VTYPE_STRING"),
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