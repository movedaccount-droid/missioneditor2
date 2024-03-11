use thiserror::Error;

pub type Result<T> = std::result::Result<T, MissionSerdeError>;

#[derive(Debug, Error)]
pub enum MissionSerdeError {
    #[error("struct error")]
    Struct {
        #[from]
        source: crate::playmission::structs::StructError
    },
    #[error("failed to parse VTYPE_BOOL property value {0} as boolean")]
    FailedBool(String),
    #[error("failed to parse VTYPE_FLOAT property {0} value as float")]
    FailedFloat(String),
    #[error("failed to parse VTYPE_INT property {0} value as int")]
    FailedInt(String),
    #[error("attempted to merge properties with overlapping key {0}")]
    Overlap(String),
    #[error("object properties missing essential resource file entry {0}")]
    NoFileEntry(String),
}
