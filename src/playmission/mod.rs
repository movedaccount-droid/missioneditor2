mod filemap;
mod xmlcleaner;
mod datafile;
mod structs;
pub mod error;

pub use structs::mission::MissionObject;
pub use structs::traits::Object;
pub use structs::properties::Value;
pub use error::Result;