use thiserror::Error;

#[derive(Debug, Error)]
pub enum JastorError {
    #[error("unable to read file {0}")]
    FileReadError(String),

    #[error("a parsing error has occurred {0}")]
    ParseError(String),

    #[error("an error occurred {0}")]
    GenericError(String),
}
