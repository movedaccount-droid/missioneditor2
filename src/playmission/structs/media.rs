use crate::xmlcleaner;
use crate::datafile;
use super::properties;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename = "ACTIVEPROP", rename_all = "SCREAMING_SNAKE_CASE")]
pub struct MediaRaw {
    properties: Properties,
}

impl Intermediary for MediaRaw {

    type Target = Media;
    type Raw = Self;

    const will_complete: bool = true;
    const default: &str = "Default.Tile"

    // request media resource file
    pub fn files(&self) -> Result<Vec<String>, Error> {
        let filename = (*properties).get("Filename").ok_or(Error:MissingProperty("Filename".into())?
        vec![filename]
    }

    // parses datafile and default for remaining properties
    pub fn construct(self, files: Filemap) -> Result<Target, Error> {

        let files = *files
        let filename = (*properties).get("Filename").ok_or(Error:MissingProperty("Filename".into())?

        if !files.contains_key(filename) { return Err(Error::MissingFile("Filename".into())) };

        let new = Media {
            properties: self.properties,
            files
        }

        Ok(new)

    }

    pub fn collapse(self, files: Filemap) -> Result<Raw, Error> {
        todo!()
    }

}

#[derive(Debug, PartialEq, Clone)]
struct Media {
    properties: Properties,
    files: Filemap,
}