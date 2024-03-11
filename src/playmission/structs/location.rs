use crate::xmlcleaner;
use crate::datafile;
use super::properties;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename = "ACTIVEPROP", rename_all = "SCREAMING_SNAKE_CASE")]
pub struct LocationRaw {
    properties: Properties,
    #[serde(rename = "DATAFILE")]
    datafile_name: String,
    bbox_min: String,
    bbox_max: String,
}

impl Intermediary for LocationRaw {

    type Target = Location;
    type Raw = Self;

    const will_complete: bool = true;
    const default: &str = "Default.Tile"

    // request datafile and default
    pub fn files(&self) -> Result<Vec<String>, Error> {
        vec![&self.datafile_name, Self::default]
    }

    // parses datafile and default for remaining properties
    pub fn construct(self, files: Filemap) -> Result<Target, Error> {

        let files = *files

        let datafile = files.get(&self.datafile_name).ok_or(Error::MissingFile(self.datafile_name))?
        let default = files.get(Self::default).ok_or(Error::MissingFile(Self::default.into()))?

        let bbox_min_property = Property::new(Value::String(self.bbox_min), None)
        self.properties.add("bbox_min", orientation_property)?;
        let bbox_max_property = Property::new(Value::String(self.bbox_max), None)
        self.properties.add("bbox_max", orientation_property)?;

        let new = Location {
            properties: self.properties,
            datafile: Properties::from_datafile_default(datafile, default)?,
            datafile_name: self.datafile_name,
        }

        Ok(new)

    }

    pub fn collapse(self, files: Filemap) -> Result<Raw, Error> {
        todo!()
    }

}

#[derive(Debug, PartialEq, Clone)]
struct Location {
    properties: Properties,
    datafile: Properties,
    datafile_name: String,
}