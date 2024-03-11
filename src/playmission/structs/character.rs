use crate::xmlcleaner;
use crate::datafile;
use super::properties;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename = "ACTIVEPROP", rename_all = "SCREAMING_SNAKE_CASE")]
pub struct CharacterRaw {
    properties: Properties,
    #[serde(rename = "DATAFILE")]
    datafile_name: String,
    orientation: String,
}

impl Intermediary for CharacterRaw {

    type Target = Character;
    type Raw = Self;

    const will_complete: bool = true;
    const default: &str = "Default.character"

    // request datafile and default
    pub fn files(&self) -> Result<Vec<String>, Error> {
        vec![&self.datafile_name, Self::default]
    }

    // parses datafile and default for remaining properties
    pub fn construct(self, files: Filemap) -> Result<Target, Error> {

        let files = *files

        let datafile = files.get(&self.datafile_name).ok_or(Error::MissingFile(self.datafile_name))?
        let default = files.get(Self::default).ok_or(Error::MissingFile(Self::default.into()))?

        let orientation_property = Property::new(Value::String(self.orientation), None)
        self.properties.add("orientation", orientation_property)?;

        let new = Character {
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
struct Character {
    properties: Properties,
    datafile: Properties,
    datafile_name: String,
}