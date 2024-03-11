use crate::xmlcleaner;
use crate::datafile;
use super::properties;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename = "ACTIVEPROP", rename_all = "SCREAMING_SNAKE_CASE")]
pub struct UserDataRaw {
    properties: Properties,
    data: String,
    expanded_size: u32,
}

impl Intermediary for UserDataRaw {

    type Target = UserData;
    type Raw = Self;

    const will_complete: bool = true;

    // request datafile and default
    pub fn files(&self) -> Result<Vec<String>, Error> {
        vec![]
    }

    // parses datafile and default for remaining properties
    pub fn construct(self, files: Filemap) -> Result<Target, Error> {

        let files = *files

        let datafile = files.get(&self.datafile_name).ok_or(Error::MissingFile(self.datafile_name))?
        let default = files.get(Self::default).ok_or(Error::MissingFile(Self::default.into()))?

        let orientation_property = Property::new(Value::String(self.data), None)
        self.properties.add("data", orientation_property)?;
        let orientation_property = Property::new(Value::Int(self.expanded_size), None)
        self.properties.add("expanded_size", orientation_property)?;

        let new = UserData {
            properties: self.properties,
        }

        Ok(new)

    }

    pub fn collapse(self, files: Filemap) -> Result<Raw, Error> {
        todo!()
    }

}

#[derive(Debug, PartialEq, Clone)]
struct UserData {
    properties: Properties,
}