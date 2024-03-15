use std::{any::Any, borrow::Borrow, cell::RefCell, ops::Deref, rc::{Rc, Weak}};

use erased_serde::Serialize;
use uuid::Uuid;
use crate::playmission::{error::Result, filemap::Filemap};

use super::Properties;

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
	Done(Box<dyn Object>),
	More(Box<dyn Intermediary>),
}

impl ConstructedObject {
	pub fn done<'a>(o: impl Object + 'static) -> Self {
		Self::Done(Box::new(o))
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

pub struct UpdateObject {
	object: Weak<RefCell<Box<dyn Object>>>,
	updated: bool,
}

impl UpdateObject {

	pub fn get(&self) -> std::option::Option<Box<dyn Object>> {
		self.object.upgrade().map(|rc| *rc.as_ref().borrow())
	}

	pub fn update(&mut self) {
		self.updated = !self.updated;
	}

}

impl PartialEq for UpdateObject {

	fn eq(&self, other: &Self) -> bool {
		self.updated == other.updated
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

pub trait Object {

	// converts into any. for test case use only!!
	fn into_any(self: Box<Self>) -> Box<dyn Any>;

	// iteratively collapses to raw stage and emits files to place in filemap
	fn collapse(self: Box<Self>) -> Result<CollapsedObject>;

	// get ref to properties
	fn properties(&self) -> &Properties;

	// get ref to uuid
	fn uuid(&self) -> &Uuid;

}