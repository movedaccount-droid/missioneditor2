use erased_serde::Serialize;
use gloo_console::log;
use uuid::Uuid;
use wasm_bindgen::JsCast;
use crate::{
	playmission::{
		error::{PlaymissionError as Error, Result},
		filemap::Filemap,
		structs::player::Player
	},
	three::{
		BoxGeometry,
		Mesh,
		MeshBasicMaterial,
		Object3D,
		Scene
	}
};

use super::{active_prop::ActiveProp, character::Character, door::Door, location::Location, media::Media, pickup::Pickup, prop::Prop, rule::Rule, special_effect::{SpecialEffect, SpecialEffectRaw}, trigger::Trigger, user_data::UserData, Properties, Value};

pub trait Raw: Serialize {

	// based on if any loading needs to happen at all,
	// returns self as either intermediary or object
	fn begin(self: Box<Self>) -> Result<ConstructedObject>;

	// cast self to serialize
	fn as_serialize(self: Box<Self>) -> Box<dyn Serialize>;
	
}

erased_serde::serialize_trait_object!(Raw);

pub trait Intermediary {

	// returns vec of resource files required by object at this stage
	fn files(&self) -> Result<Vec<Prerequisite>>;

	// constructs using files to next stage
	fn construct(self: Box<Self>, files: Filemap) -> Result<ConstructedObject>;

}

pub enum ConstructedObject {
	Done(Object),
	More(Box<dyn Intermediary>),
}

impl ConstructedObject {
	pub fn done<'a>(o: Object) -> Self {
		Self::Done(o)
	}

	pub fn more(o: impl Intermediary + 'static) -> Self {
		Self::More(Box::new(o))
	}
}

pub struct CollapsedObject {
	pub raw: Box<dyn Raw>,
	pub files: Filemap,
}

impl CollapsedObject {

	// contruct new
	pub fn new(raw: Box<dyn Raw>, files: Filemap) -> Self {
		Self{ raw, files }
	}

}

pub struct Prerequisite<'a> {
	pub file_name: &'a str,
	pub shared: bool,
}

impl<'a> Prerequisite<'a> {

	// create new
	pub fn new(file_name: &'a str, shared: bool) -> Self {
		Self { file_name, shared }
	}

}

pub struct Object {
	uuid: Uuid,
	handler: Box<dyn ObjectHandler>,
	properties: Properties,
	datafile: Properties,
	datafile_name: Option<String>,
	files: Filemap,
}

impl Object {

	// new with defaults
	pub fn new(handler: Box<dyn ObjectHandler>, properties: Properties, datafile: Option<Properties>, datafile_name: Option<String>, files: Option<Filemap>) -> Self {
		Self {
			uuid: Uuid::new_v4(),
			handler,
			properties,
			datafile: datafile.unwrap_or_default(),
			datafile_name,
			files: files.unwrap_or_default()
		}
	}

	// various getters
	// get ref to uuid
	pub fn uuid(&self) -> &Uuid {
		&self.uuid
	}

	// get ref to properties
	pub fn properties(&self) -> &Properties {
		&self.properties
	}

	// get ref to datafile
	pub fn datafile(&self) -> &Properties {
		&self.datafile
	}

	// get ref to filemap
	pub fn files(&self) -> &Filemap {
		&self.files
	}

	// get name
	pub fn name(&self) -> Option<String> {
		self.properties.get_value("Name").ok().map(|n| n.to_string())
	}

	// pass various setters through to objecthandler to ensure
	// any additional processing [three.js updates etc.] take place
	// returns old value
	pub fn set_property(&mut self, k: impl AsRef<str>, v: impl Into<String>) -> Result<Option<Value>> {
		let k = k.as_ref();
		let old = self.properties.replace_or_add_property_value(k, v)?;
		let v = self.properties.get_value(k).unwrap();
		self.handler.view_property_update(k, v)?;
		Ok(old)
	}

	pub fn set_datafile(&mut self,  k: impl AsRef<str>, v: impl Into<String>) -> Result<Option<Value>> {
		let k = k.as_ref();
		let old = self.datafile.replace_or_add_property_value(k, v)?;
		let v = self.datafile.get_value(k).unwrap();
		self.handler.view_property_update(k.as_ref(), v)?;
		Ok(old)
	}

	pub fn set_file(&mut self, k: impl Into<String> + AsRef<str>, v: Vec<u8>) -> Result<Option<Vec<u8>>> {
		self.handler.view_file_update(k.as_ref(), &v)?;
		let old = self.files.insert(k.into(), v);
		Ok(old)
	}

	// passthroughs to specific behaviour in handler, see ObjectHandler
	pub fn collapse(self) -> Result<CollapsedObject> {
		self.handler.collapse(self.properties, self.datafile, self.datafile_name, self.files)
	}

	pub fn render(&mut self, scene: &mut Scene) -> Result<()> {
		self.handler.render(&self.uuid, &self.properties, &self.datafile, &self.files, scene)
	}

}

// hacky shit.....  .... .
impl Clone for Object {
	fn clone(&self) -> Self {
		log!(self.handler.r#type());
		let handler: Box<dyn ObjectHandler> = match self.handler.r#type() {
			"ACTIVE_PROP" => Box::new(ActiveProp::new()),
			"CHARACTER" => Box::new(Character::new()),
			"DOOR" => Box::new(Door::new()),
			"LOCATION" => Box::new(Location::new()),
			"MEDIA" => Box::new(Media),
			"PICKUP" => Box::new(Pickup::new()),
			"PLAYER" => Box::new(Player::new()),
			"PROP" => Box::new(Prop::new()),
			"RULE" => Box::new(Rule),
			"SPECIAL_EFFECT" => Box::new(SpecialEffect::new()),
			"TRIGGER" => Box::new(Trigger::new()),
			"USER_DATA" => Box::new(UserData),
			_ => panic!("handler case unimplemented")
		};

		Self {
			uuid: self.uuid,
			handler,
			properties: self.properties.clone(),
			datafile: self.datafile.clone(),
			datafile_name: self.datafile_name.clone(),
			files: self.files.clone(),
		}
	}
}

impl PartialEq for Object {
	fn eq(&self, other: &Self) -> bool {
		(self.uuid == other.uuid) &&
		(self.handler.r#type() == other.handler.r#type()) &&
		(self.properties == other.properties) &&
		(self.datafile == other.datafile) &&
		(self.datafile_name == other.datafile_name) &&
		(self.files == other.files)
	}
}

pub trait ObjectHandler {

	// renders object to canvas
	fn render(&mut self, uuid: &Uuid, properties: &Properties, datafile: &Properties, files: &Filemap, scene: &mut Scene) -> Result<()>;

	// handles internal state for property updates
	fn view_property_update(&mut self, k: &str, v: &Value) -> Result<()>;

	// sama datafile
	fn view_datafile_update(&self, k: &str, v: &Value) -> Result<()>;

	// sama file
	fn view_file_update(&self, k: &str, v: &[u8]) -> Result<()>;

	// iteratively collapses to raw stage and emits files to place in filemap
	fn collapse(&self, properties: Properties, datafile: Properties, datafile_name: Option<String>, files: Filemap) -> Result<CollapsedObject>;

	// returns type. handlers should almost certainly be enums in a sane system ....
	fn r#type(&self) -> &'static str;

}

pub fn render_default_orb(uuid: &Uuid, properties: &Properties, scene: &mut Scene) -> Result<Mesh> {

	let geo = BoxGeometry::new(1.0, 1.0, 1.0);
	let mat = MeshBasicMaterial::new();
	mat.color().set_rgb(1.0, 0.0, 0.0);
	let cube = Mesh::new(&geo, &mat);

	let pos_x = properties.get_float("Position X")?;
	let pos_y = properties.get_float("Position Y")?;
	let pos_z = properties.get_float("Position Z")?;

	cube.position().set(pos_x, pos_y, pos_z);

	cube.dyn_ref::<Object3D>()
		.unwrap()
		.set_name(uuid.to_string());

	scene.add(&cube);
	Ok(cube)

}