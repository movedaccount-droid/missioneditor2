// functions for parsing playmission zip buffer to object vec, and vice versa

use std::io::{ Read, Seek };

use crate::playmission::filemap::Filemap;
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
	let mission_file = filemap.get_closure(|s| s.ends_with(".mission")).ok_or(Error::MissingFile("{.mission file}".into()))?;
	let mission: IntermediaryMission = xmlcleaner::deserialize(mission_file)?;

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
		for file_name in intermediary.files()? {
			let file = filemap.remove(file_name).ok_or(Error::MissingFile(file_name.into()))?;
			files.add(file_name, file)?;
		}

		intermediary = intermediary_or_return!(intermediary.construct(files)?);

	}

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::get_test;
    use crate::pretty_assert_eq;
    use crate::playmission::structs::{ prop::Prop, Properties, Property, Value };
    use std::io::Cursor;

    #[test]
    fn props() {
    	let mut expected_objects = vec![];
    	type Pv = Value;
    	let df = Some("READONLY,HIDDEN");
    	let ins = |m: &mut Properties, k: &str, v, f: Option<&str>| m.insert(k.to_string(), Property::new(v, f.map(String::from)));

    	let mut p1_properties = Properties::new();
        p1_properties.insert_new("Active", "true", "VTYPE_BOOL", None);
        p1_properties.insert_new("Name", "Barrier Bars", "VTYPE_STRING", Some("READONLY | HIDDEN"));
        p1_properties.insert_new("Orientation", "1.0, 0.0, 0.0, 0.0", "VTYPE_STRING", None);

        let mut p1_datafile = Properties::new();



    	expected_objects.push(Prop {
    	    properties: p1_properties,
    		datafile_name: "barrier_bars.prop".to_string(),
    		datafile: ,
        });

    	let mut p2_properties = Properties::new();
    	ins(&mut p2_properties, "datafile_name", Pv::String("Bookcase".to_string()), df());
    	ins(&mut p2_properties, "Object", Pv::String("MG_Bookcase.obj".to_string()), df());
    	ins(&mut p2_properties, "Categories", Pv::String("Victorian".to_string()), df());
    	ins(&mut p2_properties, "Active", Pv::Bool(true), Some(""));
    	ins(&mut p2_properties, "Description", Pv::String("defaulted".to_string()), df());
    	ins(&mut p2_properties, "Size", Pv::Float(1.0), df());

    	let mut p2_filemap = Filemap::new();
    	p2_filemap.insert("MG_Bookcase.obj".to_string(), get_test("props/MG_Bookcase.obj"));

    	expected_objects.push(Prop::new(
    		p2_properties,
    		"mg_bookcase.prop".to_string(),
    		p2_filemap,
    	));

    	let mut mission_properties = Properties::new();
    	ins(&mut mission_properties, "Name", Pv::String("My Game".to_string()), Some("HIDDEN"));
    	ins(&mut mission_properties, "Save TTS Audio Files", Pv::Bool(true), Some("HIDDEN"));
    	ins(&mut mission_properties, "meta", Pv::String("bb68tcb0fu097d1v".to_string()), None);
    	ins(&mut mission_properties, "expanded_size", Pv::Int(244), None);
    	ins(&mut mission_properties, "blanking_plates", Pv::String("eNpjY2BgEC7LTC7JL8pMzItPyknMy9bLT8piYDhwkIGhwYSBYYUjkN6vt9d6MxMOlR9AKrcQo3ICUCUIFEBV7t7MzMAfnJzplhnvhFB1A6rKwXFnZjbQ3AZ7FiyqPkDdBzbLHqjShAWPWRMctwJVgGxlYQAA0gVKow".to_string()), None);

    	let mut mission_filemap = Filemap::new();
    	mission_filemap.insert("Default.prop".to_string(), get_test("props/Default.prop"));

    	let expected_missionobject = MissionObject::new(mission_properties, mission_filemap);

    	let data = get_test("props.zip");
    	let c = Cursor::new(data);
    	let (found_objects, found_missionobject) = to_objects(c).unwrap();

    	let fixed_expected = expected_objects.into_iter().map(|x| Box::new(x).into_any().downcast_ref::<Prop>().unwrap().clone()).collect::<Vec<Prop>>();
    	let fixed_found = found_objects.into_iter().map(|x| Box::new(x).into_any().downcast_ref::<Prop>().unwrap().clone()).collect::<Vec<Prop>>();

		use std::io::Write;
		let mut f = std::fs::File::create("expected.txt").unwrap();
		f.write_fmt(format_args!("{:#?}", fixed_expected));
		let mut f = std::fs::File::create("found.txt").unwrap();
		f.write_fmt(format_args!("{:#?}", fixed_found));

    	pretty_assert_eq!(fixed_expected, fixed_found);

    	pretty_assert_eq!(expected_missionobject, found_missionobject);
    }
}