use thiserror::Error;

pub type Result<T> = std::result::Result<T, StructError>;

#[derive(Debug, Error)]
pub enum StructError {
    #[error("called constructor with incorrect intermediary")]
    WrongIntermediary,
}
