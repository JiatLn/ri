use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum CommonError {
    NotFound(String),
    JsonParseError(String),
    Uninitialized,
}

impl Display for CommonError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CommonError::NotFound(str) => write!(f, "NotFoundError: {}", str),
            CommonError::JsonParseError(str) => write!(f, "JsonParserError: {}", str),
            CommonError::Uninitialized => write!(f, "UninitializedError"),
        }
    }
}

impl Error for CommonError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            CommonError::NotFound(_) => None,
            CommonError::JsonParseError(_) => None,
            CommonError::Uninitialized => None,
        }
    }
}

impl From<std::io::Error> for CommonError {
    fn from(err: std::io::Error) -> Self {
        match err.kind() {
            std::io::ErrorKind::NotFound => CommonError::NotFound(err.to_string()),
            _ => CommonError::Uninitialized,
        }
    }
}

impl From<serde_json::Error> for CommonError {
    fn from(err: serde_json::Error) -> Self {
        CommonError::JsonParseError(err.to_string())
    }
}

impl From<requestty::ErrorKind> for CommonError {
    fn from(err: requestty::ErrorKind) -> Self {
        CommonError::JsonParseError(err.to_string())
    }
}
