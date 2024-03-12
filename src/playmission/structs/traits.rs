use erased_serde::Serialize;
use crate::playmission::{error::Result, filemap::Filemap};

pub trait Raw: Serialize {

	// based on if any loading needs to happen at all,
	// returns self as either intermediary or object
	fn begin(self) -> Result<ConstructedObject>;

	// cast self to serialize
	fn as_serialize(self) -> Box<dyn Serialize>;
	
}

erased_serde::serialize_trait_object!(Raw);

pub trait Intermediary {

	// returns vec of resource files required by object at this stage
	fn files(&self) -> Result<Vec<&str>>;

	// constructs using files to next stage
	fn construct(self, files: Filemap) -> Result<ConstructedObject>;

	// iteratively collapses to raw stage and emits files to place in filemap
	fn collapse(self, files: Filemap) -> Result<CollapsedObject>;

}

pub enum ConstructedObject {
	Done(Box<dyn Object>),
	More(Box<dyn Intermediary>),
}

impl ConstructedObject {
	pub fn done(o: impl Object) -> Self {
		Self::Done(Box::new(o))
	}

	pub fn more(o: impl Intermediary) -> Self {
		Self::More(Box::new(o))
	}
}

pub struct CollapsedObject {
	raw: Box<dyn Raw>,
	files: Filemap,
}

pub trait Object {

	// ...

}