use thiserror::Error;

pub type Result<T> = std::result::Result<T, PlaymissionError>;

#[derive(Debug, Error)]
pub enum PlaymissionError {
    #[error("found malformed line ('{0}' not in form 'foo = bar') when parsing datafile")]
    MalformedDatafileLine(String),
    #[error("attempted to write value {0} of type {1} into properties when {0} was already specified as {2}")]
    MergedWrongType(String, String, String),
    #[error("missing required file {0} in filemap")]
    MissingFile(String),
    #[error("missing required property {0} in properties")]
    MissingProperty(String),
    #[error("attempted to add already-claimed key {0} to properties")]
    TakenKey(String),
    #[error("attempted to add already-claimed name {0} to filemap")]
    TakenFileName(String),
    #[error("failed to cast value {0} to type {1}")]
    WrongTypeCast(String, String),
    #[error("found wrong type for value {0} when {1} was expected")]
    WrongTypeFound(String, String),

    #[error("failed to read loaded buffer to string")]
    Utf8 {
        #[from]
        source: std::str::Utf8Error, 
    },
    #[error("failed handling playmission as zip")]
    Zip {
        #[from]
        source: zip::result::ZipError,
    },
    #[error("reader/writer failure")]
    Io {
        #[from]
        source: std::io::Error,
    },
    #[error("failed deserializing cleaned xml")]
    De {
        #[from]
        source: quick_xml::DeError,
    }
}