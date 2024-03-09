use thiserror::Error;

pub type Result<T> = std::result::Result<T, MissionSerdeError>;

#[derive(Debug, Error)]
pub enum MissionSerdeError {
    #[error("failed to parse VTYPE_BOOL property value {0} as boolean")]
    FailedBool(String),
    #[error("failed to parse VTYPE_FLOAT property {0} value as float")]
    FailedFloat(String),
    #[error("failed to parse VTYPE_INT property {0} value as int")]
    FailedInt(String),
}
