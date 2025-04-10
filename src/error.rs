use thiserror::Error;

#[derive(Debug, Error)]
pub enum JastorError {
    #[error("unable to read file {0}")]
    FileReadError(String),

    #[error("a parsing error has occurred {0}")]
    ParseError(String),

    #[error("encountered an unknown event {0}")]
    UnknownEvent(String),

    #[error("unknown value encountered {0}")]
    UnknownValue(String),

    #[error("an error occurred {0}")]
    GenericError(String),
}
