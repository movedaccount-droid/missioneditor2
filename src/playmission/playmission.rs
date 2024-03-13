// functions for parsing playmission zip buffer to object vec, and vice versa

use std::io::{ Read, Seek };

use crate::playmission::{filemap::Filemap, structs::traits::Prerequisite};
use super::structs::{
	IntermediaryMission,
	mission::MissionObject,
	Object,
	traits::{ ConstructedObject, Raw },
};
use super::xmlcleaner;

use crate::playmission::error::{Result, PlaymissionError as Error};

// parses playmission from buffer to objet vec and missionobject
pub fn to_objects(r: impl Read + Seek) -> Result<(Vec<Box<dyn Object>>, MissionObject)> {

	// load all files in zip to map
	let mut filemap = Filemap::from_reader(r)?;

	// parse intermediary objects from base mission file
	let mission_file = filemap.take_closure(|s| s.ends_with(".mission")).ok_or(Error::MissingFile("{.mission file}".into()))?;
	let mission: IntermediaryMission = xmlcleaner::deserialize(&mission_file)?;

	// construct full objects from intermediaries
	let mut objects: Vec<Box<dyn Object>> = vec![];
	for object in mission.raws.into_iter() {
		let object = load_intermediary(object, &mut filemap)?;
		objects.push(object)
	}

	// construct missionobject from remnants
	let missionobject = MissionObject::from_remnants(
		mission.properties,
		filemap,
		mission.expanded_size,
		mission.blanking_plates,
		mission.meta
	)?;

	Ok((objects, missionobject))
}

// loads single object based on files in filemap
pub fn load_intermediary(mut raw: Box<dyn Raw>, filemap: &mut Filemap) -> Result<Box<dyn Object>> {

    macro_rules! intermediary_or_return {
        ($i:expr) => {
            match $i {
                ConstructedObject::Done(object) => return Ok(object),
                ConstructedObject::More(intermediary) => intermediary,
            };
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
        p1_properties.insert_new("Active", "true", "VTYPE_BOOL", None);
        p1_properties.insert_new("Name", "Barrier Bars", "VTYPE_STRING", Some("READONLY | HIDDEN"));
        p1_properties.insert_new("Orientation", "1.0, 0.0, 0.0, 0.0", "VTYPE_STRING", None);

        let mut p1_datafile = Properties::new();
        p1_datafile.insert_new("Active", "true", "VTYPE_BOOL", df.clone()).unwrap();
        p1_datafile.insert_new("Name", "Barrier Bars", "VTYPE_STRING", df.clone());
        p1_datafile.insert_new("Description", "defaulted", "VTYPE_STRING", df.clone());
        p1_datafile.insert_new("Object", "Barrier_Bars.obj", "VTYPE_STRING", df.clone());
        p1_datafile.insert_new("Categories", "Plain", "VTYPE_STRING", df.clone());
        p1_datafile.insert_new("Size", "1.0", "VTYPE_FLOAT", df.clone());

        let p1 = Prop::new(p1_properties, p1_datafile, "barrier_bars.prop".to_string());
    	expected_objects.push(p1);

        let mut p2_properties = Properties::new();
        p2_properties.insert_new("Active", "true", "VTYPE_BOOL", None);
        p2_properties.insert_new("Name", "Bookcase", "VTYPE_STRING", Some("READONLY | HIDDEN"));
        p2_properties.insert_new("Orientation", "1.0, 0.0, 0.0, 0.0", "VTYPE_STRING", None);

        let mut p2_datafile = Properties::new();
        p2_datafile.insert_new("Active", "true", "VTYPE_BOOL", df.clone());
        p2_datafile.insert_new("Name", "Bookcase", "VTYPE_STRING", df.clone());
        p2_datafile.insert_new("Description", "defaulted", "VTYPE_STRING", df.clone());
        p2_datafile.insert_new("Object", "MG_Bookcase.obj", "VTYPE_STRING", df.clone());
        p2_datafile.insert_new("Categories", "Victorian", "VTYPE_STRING", df.clone());
        p2_datafile.insert_new("Size", "1.0", "VTYPE_FLOAT", df.clone());

    	let p2 = Prop::new(p2_properties, p2_datafile, "mg_bookcase.prop".to_string());
    	expected_objects.push(p2);

    	let mut mission_properties = Properties::new();
        mission_properties.insert_new("Name", "My Game", "VTYPE_STRING", Some("HIDDEN"));
        mission_properties.insert_new("Save TTS Audio Files", "true", "VTYPE_BOOL", Some("HIDDEN"));
        mission_properties.insert_new("Meta", "bb68tcb0fu097d1v", "VTYPE_STRING", None);
        mission_properties.insert_new("Expanded Size", "244", "VTYPE_INT", None);
        mission_properties.insert_new("Blanking Plates", "eNpjY2BgEC7LTC7JL8pMzItPyknMy9bLT8piYDhwkIGhwYSBYYUjkN6vt9d6MxMOlR9AKrcQo3ICUCUIFEBV7t7MzMAfnJzplhnvhFB1A6rKwXFnZjbQ3AZ7FiyqPkDdBzbLHqjShAWPWRMctwJVgGxlYQAA0gVKow", "VTYPE_STRING", None);
        
    	let mut mission_filemap = Filemap::new();
    	mission_filemap.insert("Default.prop".to_string(), get_test("props/Default.prop"));
		mission_filemap.insert("Barrier_Bars.obj".to_string(), get_test("props/Barrier_Bars.obj"));
		mission_filemap.insert("MG_Bookcase.obj".to_string(), get_test("props/MG_Bookcase.obj"));

    	let expected_missionobject = MissionObject::new(mission_properties, mission_filemap);

    	let data = get_test("props.zip");
    	let c = Cursor::new(data);
    	let (found_objects, found_missionobject) = to_objects(c).unwrap();

    	let fixed_expected = expected_objects.into_iter().map(|x| Box::new(x).into_any().downcast_ref::<Prop>().unwrap().clone()).collect::<Vec<Prop>>();
    	let fixed_found = found_objects.into_iter().map(|x| Box::new(x).into_any().downcast_ref::<Prop>().unwrap().clone()).collect::<Vec<Prop>>();

		use std::io::Write;
		let mut f = std::fs::File::create("expected.txt").unwrap();
		f.write_fmt(format_args!("{:#?}", expected_missionobject));
		let mut f = std::fs::File::create("found.txt").unwrap();
		f.write_fmt(format_args!("{:#?}", found_missionobject));

    	pretty_assert_eq!(fixed_expected, fixed_found);

    	pretty_assert_eq!(expected_missionobject, found_missionobject);
    }
}