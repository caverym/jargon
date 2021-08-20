use crate::Key;
use std::fmt::{Display, Formatter};

#[derive(Debug, Eq, PartialEq)]
pub enum Error {
    MissingArg(Key),
    Other(String),
}

impl std::error::Error for Error {}

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
