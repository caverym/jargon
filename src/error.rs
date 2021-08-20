use crate::Key;
use std::fmt::{Display, Formatter};

#[derive(Debug, Eq, PartialEq)]
pub enum Error {
    MissingArg(Key),
    Other(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,
        "{}",
        match self {
            Error::MissingArg(k) => format!("Missing argument: '{}'", k),
            Error::Other(e) => e.to_string(),
        })
    }
}

impl<E: std::error::Error + ToString> From<E> for Error {
    fn from(e: E) -> Self {
        Self::Other(e.to_string())
    }
}
