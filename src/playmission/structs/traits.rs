use erased_serde::Serialize;
use uuid::Uuid;
use crate::playmission::{error::Result, filemap::Filemap};

use super::{Properties, Value};

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

	// pass various setters through to objecthandler to ensure
	// any additional processing [three.js updates etc.] take place
	pub fn set_property(&mut self, k: impl AsRef<str>, v: impl Into<String>) -> Result<()> {
		let k = k.as_ref();
		self.properties.replace_or_add_property_value(k, v)?;
		let v = self.properties.get_value(k).unwrap();
		self.handler.view_property_update(k, v)
	}

	pub fn set_datafile(&mut self,  k: impl AsRef<str>, v: impl Into<String>) -> Result<()> {
		let k = k.as_ref();
		self.datafile.replace_or_add_property_value(k, v)?;
		let v = self.properties.get_value(k).unwrap();
		self.handler.view_property_update(k.as_ref(), v)
	}

	pub fn set_file(&mut self, k: impl Into<String> + AsRef<str>, v: Vec<u8>) -> Result<()> {
		self.handler.view_file_update(k.as_ref(), &v)?;
		self.files.insert(k.into(), v);
		Ok(())
	}

	// passthroughs to specific behaviour in handler, see ObjectHandler
	pub fn collapse(self) -> Result<CollapsedObject> {
		self.handler.collapse(self.properties, self.datafile, self.datafile_name, self.files)
	}

}

pub trait ObjectHandler {

	// handles internal state for property updates
	fn view_property_update(&self, k: &str, v: &Value) -> Result<()>;

	// sama datafile
	fn view_datafile_update(&self, k: &str, v: &Value) -> Result<()>;

	// sama file
	fn view_file_update(&self, k: &str, v: &[u8]) -> Result<()>;

	// iteratively collapses to raw stage and emits files to place in filemap
	fn collapse(&self, properties: Properties, datafile: Properties, datafile_name: Option<String>, files: Filemap) -> Result<CollapsedObject>;

}