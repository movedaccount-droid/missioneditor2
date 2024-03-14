use serde::{ Deserialize, Serialize, Deserializer };

use super::{ active_prop::ActivePropRaw, character::CharacterRaw, door::DoorRaw, location::LocationRaw, media::MediaRaw, pickup::PickupRaw, player::PlayerRaw, prop::PropRaw, rule::RuleRaw, special_effect::SpecialEffectRaw, trigger::TriggerRaw, user_data::UserDataRaw, Properties, Property, Raw, Value };
use crate::playmission::{
    error::{PlaymissionError as Error, Result},
    filemap::Filemap
};

#[derive(Deserialize, Debug, PartialEq, Clone)]
#[serde(rename = "GAME", rename_all = "SCREAMING_SNAKE_CASE")]
pub struct IntermediaryMissionRaw {
    #[serde(rename = "ExpandedSize")]
    pub expanded_size: u32,
    #[serde(rename = "BLANKINGPLATES")]
    pub blanking_plates: String,
    #[serde(rename = "Meta")]
    pub meta: String,
    pub properties: Properties,

    #[serde(default, rename = "ACTIVE_PROP")]
    pub active_props: Vec<ActivePropRaw>,
    #[serde(default, rename = "CHARACTER")]
    pub characters: Vec<CharacterRaw>,
    #[serde(default, rename = "DOOR")]
    pub doors: Vec<DoorRaw>,
    #[serde(default, rename = "LOCATION")]
    pub locations: Vec<LocationRaw>,
    #[serde(default, rename = "MEDIA")]
    pub medias: Vec<MediaRaw>,
    #[serde(default, rename = "PICKUP")]
    pub pickups: Vec<PickupRaw>,
    #[serde(default, rename = "PROP")]
    pub props: Vec<PropRaw>,
    #[serde(default, rename = "PLAYER")]
    pub players: Vec<PlayerRaw>,
    #[serde(default, rename = "RULE")]
    pub rules: Vec<RuleRaw>,
    #[serde(default, rename = "SPECIAL_EFFECT")]
    pub special_effects: Vec<SpecialEffectRaw>,
    #[serde(default, rename = "TRIGGER")]
    pub triggers: Vec<TriggerRaw>,
    #[serde(default, rename = "USER_DATA")]
    pub user_datas: Vec<UserDataRaw>,
}

#[derive(Serialize)]
#[serde(rename = "GAME", rename_all = "SCREAMING_SNAKE_CASE")]
pub struct IntermediaryMission {
    #[serde(rename = "ExpandedSize")]
    pub expanded_size: u32,
    #[serde(rename = "BLANKINGPLATES")]
    pub blanking_plates: String,
    #[serde(rename = "Meta")]
    pub meta: String,
    pub properties: Properties,

    pub raws: Vec<Box<dyn Raw>>,
}

impl<'de> Deserialize<'de> for IntermediaryMission {
    fn deserialize<D>(deserializer: D) -> std::result::Result<IntermediaryMission, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = IntermediaryMissionRaw::deserialize(deserializer)?;
        Ok(Self::from_raw(raw))
    }
}

impl IntermediaryMission {
    fn from_raw(raw: IntermediaryMissionRaw) -> Self {
        
        macro_rules! chain_as_raw {
            ($i:expr,$($j:expr),+) => {
                $i.into_iter().map(|o| Box::new(o) as Box<dyn Raw>)
                $(.chain($j.into_iter().map(|o| Box::new(o) as Box<dyn Raw>)))+
            };
        }
        
        let raws = chain_as_raw!(raw.active_props, raw.characters,
            raw.doors, raw.locations, raw.medias, raw.pickups, raw.props, raw.players,
            raw.rules, raw.special_effects, raw.triggers, raw.user_datas).collect();

        Self {
            expanded_size: raw.expanded_size,
            blanking_plates: raw.blanking_plates,
            meta: raw.meta,
            properties: raw.properties,
            raws,
        }
    }

}

#[derive(Debug, PartialEq, Clone)]
pub struct MissionObject {
	properties: Properties,
	files: Filemap,
}

impl MissionObject {

    // creates new self
    pub fn new(properties: Properties, files: Filemap) -> Self {
        Self { properties, files }
    }

	// creates self from intermediarymission
	pub fn from_remnants(
		mut properties: Properties,
		files: Filemap,
		expanded_size: u32,
		blanking_plates: String,
		meta: String
	) -> Result<Self> {
        let p = |v| Property::new(v, None);
		properties.add("Expanded Size", p(Value::Int(expanded_size)))?;
		properties.add("Blanking Plates", p(Value::String(blanking_plates)))?;
		properties.add("Meta", p(Value::String(meta)))?;
		Ok(Self { properties, files })
	}

    // decompose into elements for intermediarymission
    pub fn into_remnants(self) -> Result<(Properties, Filemap, u32, String, String)> {
        let Value::Int(expanded_size) = self.properties.take_value("Expanded Size")? else {
            return Err(Error::WrongTypeFound("expanded_size".into(), "VTYPE_INT".into()))
        };
        let Value::String(blanking_plates) = self.properties.take_value("Blanking Plates")? else {
            return Err(Error::WrongTypeFound("blanking_plates".into(), "VTYPE_STRING".into()))
        };
        let Value::String(meta) = self.properties.take_value("Meta")? else {
            return Err(Error::WrongTypeFound("meta".into(), "VTYPE_STRING".into()))
        };
        Ok((self.properties,
            self.files,
            expanded_size,
            blanking_plates,
            meta))
    }

}