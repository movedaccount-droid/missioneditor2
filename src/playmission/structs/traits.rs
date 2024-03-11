// generic trait for all objects
use std::any::Any;

pub trait Object {

	// converts into any for debugging purposes
	fn into_any(self: Box<Self>) -> Box<dyn Any>;

	// // decomposes object back to intermediary state
	// fn deconstruct(self);

	// // gets refs to resource files
	// fn files(&self) -> Vec[&File];

	// // gets mutable refs to resource files
	// fn files_mut(&mut self) -> Vec[&mut File];

	// // attempts to set property to value
	// fn set(&mut self, property, value) -> Result<()>;

	// // gets property value
	// fn get(&self, property) -> Result<Option(PropertyValue)>;

}