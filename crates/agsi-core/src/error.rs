use thiserror::Error;

/// Result type for AGSi operations
pub type Result<T> = std::result::Result<T, Error>;

/// Errors that can occur when working with AGSi data
#[derive(Error, Debug)]
pub enum Error {
    #[error("Serialization error: {0}")]
    Serialization(String),

    #[error("Deserialization error: {0}")]
    Deserialization(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Geometry error: {0}")]
    Geometry(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Invalid schema version: expected {expected}, found {found}")]
    InvalidSchemaVersion { expected: String, found: String },

    #[error("Missing required field: {0}")]
    MissingField(String),

    #[error("Invalid material property: {0}")]
    InvalidProperty(String),

    #[error("Model not found: {0}")]
    ModelNotFound(String),

    #[error("Material not found: {0}")]
    MaterialNotFound(String),
}

impl From<validator::ValidationErrors> for Error {
    fn from(err: validator::ValidationErrors) -> Self {
        Error::Validation(err.to_string())
    }
}
