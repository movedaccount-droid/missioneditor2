use crate::xmlcleaner;
use crate::datafile;
use super::properties;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename = "ACTIVEPROP", rename_all = "SCREAMING_SNAKE_CASE")]
pub struct ActivePropRaw {
    properties: Properties,
    #[serde(rename = "DATAFILE")]
    datafile_name: String,
    orientation: String,
}

impl Raw for ActivePropRaw {

    // returns key of datafile needed to construct object
    pub fn datafile(&self) -> Option<&str> {
        Some(&self.datafile)
    }

    // returns key of default needed to construct object
    pub fn default(&self) ->  Option<&str> {
        Some("Default.aprop")
    }

    // parses files, applies properties and returns IntermediaryWithProperties
    pub fn load(self, datafile: Option<Vec<u8>>, default: Option<Vec<u8>>) -> Result<Box<dyn IntermediaryWithProperties>> {

        let datafile = datafile.ok_or(Error::MissingDatafile)?
        let default = default.ok_or(Error::MissingDefault)?

        let orientation_property = Property::new(Value::String(self.orientation), None)
        self.properties.add("orientation", orientation_property)?;

        let new = ActivePropWithProperties {
            properties:self.properties,
            datafile: Properties::from_datafile_default(datafile, default),
            datafile_name: self.datafile_name,
        }

        Ok(Box::new(new))

    }

}

#[derive(Debug, PartialEq, Clone)]
struct ActivePropWithProperties {
    properties: Properties,
    datafile: Properties,
    datafile_name: String,
}

impl Intermediary for ActivePropWithProperties {

    const is_render: bool = true;
    const will_complete: bool = true;

    // returns vec of resource files required by object
    pub fn files(&self) -> Result<Vec<String>, Error> {
        vec![datafile.get("Object")?]
    }

    // performs construction to main dyn Object
    pub fn construct(self, files: HashMap<String, Vec<u8>>) -> Result<Self, Error> {
        // currently cannot load models :[ so we just continue

    }

}

#[derive(Debug, PartialEq, Clone)]
struct ActiveProp {
    properties: Properties,
    datafile: Properties,
    datafile_name: String,
    files: HashMap<String, File>
}

impl ActiveProp {

    // create blank activeprop, used by viewport gui [ostensibly]
    pub fn blank() -> Self {
        ActiveProp {
            properties: Properties::new(),
            datafile: Properties::new(),
            datafile_name: String::from(""),
            files: HashMap::new(),
        }
    }

}