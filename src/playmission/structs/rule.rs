use crate::xmlcleaner;
use crate::datafile;
use super::properties;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename = "ACTIVEPROP", rename_all = "SCREAMING_SNAKE_CASE")]
pub struct RuleRaw {
    properties: Properties,
}

impl Intermediary for RuleRaw {

    type Target = Rule;
    type Raw = Self;

    const will_complete: bool = true;

    // rules have no dependencies
    pub fn files(&self) -> Result<Vec<String>, Error> {
        vec![]
    }

    pub fn construct(self, files: Filemap) -> Result<Target, Error> {

        let new = Rule {
            properties: self.properties,
        }

        Ok(new)

    }

    pub fn collapse(self, files: Filemap) -> Result<Raw, Error> {
        todo!()
    }

}

#[derive(Debug, PartialEq, Clone)]
struct Rule {
    properties: Properties,
}