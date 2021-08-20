use crate::Key;
use std::fmt::{Display, Formatter};

/// # Error
///
/// This Enum currently contains two variants, MissingArgs and Other.
///
/// ## MissingArgs(Key)
///
/// This variant is used internally by Jargon to warn when a required argument
/// (from the `res_*` functions) does not exist.
///
/// ## Other(String)
///
/// This variant is used internally by Jargon when converting other types of errors to itself.
/// *STD ERROR TYPES ARE NOT YET IMPLEMENTED IN TRAITS*
#[derive(Debug, Eq, PartialEq)]
pub enum Error {
    /// # MissingArgs(Key)
    ///
    /// This variant is used internally by Jargon to warn when a required argument
    /// (from the `res_*` functions) does not exist.
    MissingArg(Key),

    /// # Other(String)
    ///
    /// This variant is used internally by Jargon when converting other types of errors to itself.
    /// *STD ERROR TYPES ARE NOT YET IMPLEMENTED IN TRAITS*
    Other(String),
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Error::MissingArg(k) => format!("Missing argument: '{}'", k),
                Error::Other(e) => e.to_string(),
            }
        )
    }
}
