use serde::{ Deserialize, Serialize };

use super::{ active_prop::ActivePropRaw, character::CharacterRaw, door::DoorRaw, location::LocationRaw, media::MediaRaw, pickup::PickupRaw, player::PlayerRaw, prop::PropRaw, rule::RuleRaw, special_effect::SpecialEffectRaw, trigger::TriggerRaw, user_data::UserDataRaw, Properties, Property, Raw, Value };
use crate::playmission::{
    filemap::Filemap,
    error::Result,
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

#[derive(Debug, PartialEq, Clone)]
pub struct MissionObject {
	properties: Properties,
	files: Filemap,
}

impl MissionObject {

	// creates self from intermediarymission
	pub fn from_remnants(
		mut properties: Properties,
		files: Filemap,
		expanded_size: u32,
		blanking_plates: String,
		meta: String
	) -> Result<Self> {
        let p = |v| Property::new(v, None);
		properties.add("expanded_size", p(Value::Int(expanded_size)))?;
		properties.add("blanking_plates", p(Value::String(blanking_plates)))?;
		properties.add("meta", p(Value::String(meta)))?;
		Ok(Self { properties, files })
	}

}