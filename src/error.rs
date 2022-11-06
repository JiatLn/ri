use std::{
    error::Error,
    fmt::{Debug, Display},
};

#[derive(Debug)]
pub enum CommonError {
    NotFound(String),
    JsonParseError(String),
    Termination(String),
    Uninitialized,
}

impl Display for CommonError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotFound(str) => write!(f, "NotFoundError: {}", str),
            Self::JsonParseError(str) => write!(f, "JsonParserError: {}", str),
            Self::Termination(str) => write!(f, "Termination: {}", str),
            Self::Uninitialized => write!(f, "Uninitialized Error"),
        }
    }
}

impl Error for CommonError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::NotFound(_) => None,
            Self::JsonParseError(_) => None,
            Self::Termination(_) => None,
            Self::Uninitialized => None,
        }
    }
}

impl From<std::io::Error> for CommonError {
    fn from(err: std::io::Error) -> Self {
        match err.kind() {
            std::io::ErrorKind::NotFound => Self::NotFound(err.to_string()),
            err => {
                dbg!(err);
                Self::Uninitialized
            }
        }
    }
}

impl From<serde_json::Error> for CommonError {
    fn from(err: serde_json::Error) -> Self {
        Self::JsonParseError(err.to_string())
    }
}

impl From<requestty::ErrorKind> for CommonError {
    fn from(err: requestty::ErrorKind) -> Self {
        match err {
            requestty::ErrorKind::IoError(_) => Self::Uninitialized,
            requestty::ErrorKind::Interrupted => Self::Termination("Ctrl+C".to_string()),
            requestty::ErrorKind::Eof => Self::Uninitialized,
            requestty::ErrorKind::Aborted => Self::Termination("Esc".to_string()),
        }
    }
}
