trait Raw {

	// returns key of datafile needed to construct object
	pub fn datafile(&self) -> Option<&str>;

	// returns key of default needed to construct object
	pub fn default(&self) -> Option<&str>;

	// parses files, applies properties and returns IntermediaryWithProperties
	pub fn apply_datafile<T>(&mut self, datafile: AsRef<str>, default: Option<AsRef<str>>) -> Result<T>;

}

trait Intermediary {

	// returns vec of resource files required by object
	pub fn files(&self) -> Result<Vec<String>, Error>;

	// checks if object is renderable, for reboxing later
	const is_render: bool;

	// checks if this is the final intermediary stage
	const will_complete: -> bool;

	// performs construction to main object
	pub fn construct<T>(self, files: Filemap) -> Result<T, Error>;

}

trait Object {

	// ...

}

trait Render {

	// appends self to three.js canvas
	pub fn render(&self);

}