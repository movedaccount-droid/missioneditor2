use std::io::{ Cursor, Read, Seek, Write };
use serde::{ Deserialize, Serialize, Deserializer };
use zip::{write::FileOptions, ZipWriter};

use super::{ active_prop::ActivePropRaw, character::CharacterRaw, ConstructedObject, door::DoorRaw, location::LocationRaw, media::MediaRaw, pickup::PickupRaw, player::PlayerRaw, prop::PropRaw, rule::RuleRaw, special_effect::SpecialEffectRaw, trigger::TriggerRaw, user_data::UserDataRaw, CollapsedObject, Object, Properties, Raw, Value };
use crate::playmission::{
    error::{PlaymissionError as Error, Result},
    filemap::Filemap,
    xmlcleaner,
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

    // create new from existing structures
    fn new(expanded_size: u32, blanking_plates: String, meta: String, properties: Properties, raws: Vec<Box<dyn Raw>>) -> Self {
        Self { expanded_size, blanking_plates, meta, properties, raws }
    }

    // join raw datastructures into trait object vector
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

    // creates self from reader over zip file
    pub fn deserialize(r: impl Read + Seek) -> Result<(Self, Vec<Box<dyn Object>>)> {

        // load all files in zip to map
        let mut filemap = Filemap::from_reader(r)?;
    
        // parse intermediary objects from base mission file
        let mission_file = filemap.take_closure(|s| s.ends_with(".mission")).ok_or(Error::MissingFile("{.mission file}".into()))?;
        let mut mission: IntermediaryMission = xmlcleaner::deserialize(&mission_file)?;
    
        // construct full objects from intermediaries
        let mut objects: Vec<Box<dyn Object>> = vec![];
        for object in mission.raws.into_iter() {
            let object = load_intermediary(object, &mut filemap)?;
            objects.push(object)
        }
    
        // move mission attributes to properties
		mission.properties.insert_new("Expanded Size", mission.expanded_size.to_string(), "VTYPE_INT", None)?;
        mission.properties.insert_new("Blanking Plates", mission.blanking_plates, "VTYPE_STRING", None)?;
        mission.properties.insert_new("Meta", mission.meta, "VTYPE_STRING", None)?;
        let mission = Self::new(mission.properties, filemap);
		Ok((mission, objects))

    }

    fn serialize(mut self, objects: Vec<Box<dyn Object>>) -> Result<Vec<u8>> {

        // regain remnants from missionobject
        let Value::Int(expanded_size) = self.properties.take_value("Expanded Size")? else {
            return Err(Error::WrongTypeFound("expanded_size".into(), "VTYPE_INT".into()))
        };
        let Value::String(blanking_plates) = self.properties.take_value("Blanking Plates")? else {
            return Err(Error::WrongTypeFound("blanking_plates".into(), "VTYPE_STRING".into()))
        };
        let Value::String(meta) = self.properties.take_value("Meta")? else {
            return Err(Error::WrongTypeFound("meta".into(), "VTYPE_STRING".into()))
        };
    
        // collapse objects
        let collapsed = objects.into_iter().map(|o| o.collapse()).collect::<Result<Vec<CollapsedObject>>>()?;
    
        // collect results
        let mut raws = vec![];
        for co in collapsed {
            self.files.merge(co.files)?;
            raws.push(co.raw);
        }

        // serialize .mission
        let intermediary_mission = IntermediaryMission::new(expanded_size, blanking_plates, meta, self.properties, raws);
        let intermediary_mission_ser = xmlcleaner::serialize(&intermediary_mission)?;
    
        // TODO: we should probably make sure that hits is aactually alwyas Test.mission,
        // or otherwise save it earlier in execution
        self.files.insert("Test.mission".into(), intermediary_mission_ser);

        // construct zip
        let buf: Vec<u8> = vec![];
        let mut zip = ZipWriter::new(Cursor::new(buf));
        let options = FileOptions::default();

        for (name, buf) in self.files.into_iter() {
            zip.start_file(name, options)?;
            zip.write_all(&buf)?;
        }

        let cursor = zip.finish()?;
        Ok(cursor.into_inner())
    
    }

}

// loads single object based on files in filemap
fn load_intermediary(raw: Box<dyn Raw>, filemap: &mut Filemap) -> Result<Box<dyn Object>> {

    macro_rules! intermediary_or_return {
        ($i:expr) => {
            match $i {
                ConstructedObject::Done(object) => return Ok(object),
                ConstructedObject::More(intermediary) => intermediary,
            }
        };
    }

    let mut intermediary = intermediary_or_return!(raw.begin()?);

    loop {

        let mut files = Filemap::new();
        for prequisite in intermediary.files()? {

            let file = if prequisite.shared {
                filemap.get(prequisite.file_name).cloned()
            } else {
                filemap.remove(prequisite.file_name)
            };
            
            let file = file.ok_or(Error::MissingFile(prequisite.file_name.into()))?;

            files.add(prequisite.file_name, file)?;
            
        }

        intermediary = intermediary_or_return!(intermediary.construct(files)?);

    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::get_test;
    use crate::pretty_assert_eq;
    use crate::playmission::structs::{ prop::Prop, Properties };
    use std::io::Cursor;

    #[test]
    fn props() {
    	let mut expected_objects = vec![];
    	let df = Some("READONLY,HIDDEN");

    	let mut p1_properties = Properties::new();
        p1_properties.insert_new("Active", "true", "VTYPE_BOOL", None).unwrap();
        p1_properties.insert_new("Name", "Barrier Bars", "VTYPE_STRING", Some("READONLY | HIDDEN")).unwrap();
        p1_properties.insert_new("Orientation", "1.0, 0.0, 0.0, 0.0", "VTYPE_STRING", None).unwrap();

        let mut p1_datafile = Properties::new();
        p1_datafile.insert_new("Active", "true", "VTYPE_BOOL", df.clone()).unwrap();
        p1_datafile.insert_new("Name", "Barrier Bars", "VTYPE_STRING", df.clone()).unwrap();
        p1_datafile.insert_new("Description", "defaulted", "VTYPE_STRING", df.clone()).unwrap();
        p1_datafile.insert_new("Object", "Barrier_Bars.obj", "VTYPE_STRING", df.clone()).unwrap();
        p1_datafile.insert_new("Categories", "Plain", "VTYPE_STRING", df.clone()).unwrap();
        p1_datafile.insert_new("Size", "1.0", "VTYPE_FLOAT", df.clone()).unwrap();

        let p1 = Prop::new(p1_properties, p1_datafile, "barrier_bars.prop".to_string());
    	expected_objects.push(p1);

        let mut p2_properties = Properties::new();
        p2_properties.insert_new("Active", "true", "VTYPE_BOOL", None).unwrap();
        p2_properties.insert_new("Name", "Bookcase", "VTYPE_STRING", Some("READONLY | HIDDEN")).unwrap();
        p2_properties.insert_new("Orientation", "1.0, 0.0, 0.0, 0.0", "VTYPE_STRING", None).unwrap();

        let mut p2_datafile = Properties::new();
        p2_datafile.insert_new("Active", "true", "VTYPE_BOOL", df.clone()).unwrap();
        p2_datafile.insert_new("Name", "Bookcase", "VTYPE_STRING", df.clone()).unwrap();
        p2_datafile.insert_new("Description", "defaulted", "VTYPE_STRING", df.clone()).unwrap();
        p2_datafile.insert_new("Object", "MG_Bookcase.obj", "VTYPE_STRING", df.clone()).unwrap();
        p2_datafile.insert_new("Categories", "Victorian", "VTYPE_STRING", df.clone()).unwrap();
        p2_datafile.insert_new("Size", "1.0", "VTYPE_FLOAT", df.clone()).unwrap();

    	let p2 = Prop::new(p2_properties, p2_datafile, "mg_bookcase.prop".to_string());
    	expected_objects.push(p2);

    	let mut mission_properties = Properties::new();
        mission_properties.insert_new("Name", "My Game", "VTYPE_STRING", Some("HIDDEN")).unwrap();
        mission_properties.insert_new("Save TTS Audio Files", "true", "VTYPE_BOOL", Some("HIDDEN")).unwrap();
        mission_properties.insert_new("Meta", "bb68tcb0fu097d1v", "VTYPE_STRING", None).unwrap();
        mission_properties.insert_new("Expanded Size", "244", "VTYPE_INT", None).unwrap();
        mission_properties.insert_new("Blanking Plates", "eNpjY2BgEC7LTC7JL8pMzItPyknMy9bLT8piYDhwkIGhwYSBYYUjkN6vt9d6MxMOlR9AKrcQo3ICUCUIFEBV7t7MzMAfnJzplhnvhFB1A6rKwXFnZjbQ3AZ7FiyqPkDdBzbLHqjShAWPWRMctwJVgGxlYQAA0gVKow", "VTYPE_STRING", None).unwrap();
        
    	let mut mission_filemap = Filemap::new();
    	mission_filemap.insert("Default.prop".to_string(), get_test("props/Default.prop"));
		mission_filemap.insert("Barrier_Bars.obj".to_string(), get_test("props/Barrier_Bars.obj"));
		mission_filemap.insert("MG_Bookcase.obj".to_string(), get_test("props/MG_Bookcase.obj"));

    	let expected_missionobject = MissionObject::new(mission_properties, mission_filemap);

    	let data = get_test("props.zip");
    	let c = Cursor::new(data);
    	let (found_missionobject, found_objects) = MissionObject::deserialize(c).unwrap();

    	// let fixed_expected = expected_objects.into_iter().map(|x| Box::new(x).into_any().downcast_ref::<Prop>().unwrap().clone()).collect::<Vec<Prop>>();
    	// let fixed_found = found_objects.into_iter().map(|x| Box::new(x).into_any().downcast_ref::<Prop>().unwrap().clone()).collect::<Vec<Prop>>();

		use std::io::Write;
		// let mut f = std::fs::File::create("expected.txt").unwrap();
		// f.write_fmt(format_args!("{:#?}", expected_missionobject)).unwrap();
		// let mut f = std::fs::File::create("found.txt").unwrap();
		// f.write_fmt(format_args!("{:#?}", found_missionobject)).unwrap();

    	// pretty_assert_eq!(fixed_expected, fixed_found);

    	// pretty_assert_eq!(expected_missionobject, found_missionobject);

        let zip = found_missionobject.serialize(found_objects).unwrap();
        let mut f = std::fs::File::create("mission.zip").unwrap();
        f.write_all(&zip).unwrap();
    }
}