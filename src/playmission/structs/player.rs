use crate::xmlcleaner;
use crate::datafile;
use super::properties;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename = "ACTIVEPROP", rename_all = "SCREAMING_SNAKE_CASE")]
pub struct PlayerRaw {
    properties: Properties,
    orientation: String,
    start_position: String,
    start_orientation: String,
}

impl Intermediary for PlayerRaw {

    type Target = Player;
    type Raw = Self;

    // nothing needed, move straight to target
    pub fn files(&self) -> Result<Vec<String>, Error> {
        vec![]
    }

    // parses datafile and default for remaining properties
    pub fn construct(self, files: Filemap) -> Result<Target, Error> {

        let new = Player {
            properties: self.properties,
            orientation: self.orientation,
            start_position: self.start_position,
            start_orientation: self.start_orientation,
        }

        Ok(new)

    }

    pub fn collapse(self, files: Filemap) -> Result<Raw, Error> {
        todo!()
    }

}

#[derive(Debug, PartialEq, Clone)]
struct Player {
    properties: Properties,
    orientation: String,
    start_position: String,
    start_orientation: String,
}