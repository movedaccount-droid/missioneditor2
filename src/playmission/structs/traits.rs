trait Intermediary {

	// next type in line
	type Target;
	// origin raw type
	type Raw;

	// returns vec of resource files required by object at this stage
	pub fn files(&self) -> Result<Vec<String>, Error>;

	// checks if this is the final intermediary stage
	const will_complete: -> bool;

	// constructs using files to next stage
	pub fn construct(self, files: Filemap) -> Result<Target, Error>;

	// iteratively collapses to raw stage and emits files to place in filemap
	pub fn collapse(self, files: Filemap) -> Result<Raw, Error>;

}

trait Object {

	// ...

}