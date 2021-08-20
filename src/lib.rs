#[forbid(unsafe_code)]
#[warn(missing_docs)]
mod error;
mod jargons;
mod keys;

pub use error::*;
pub use jargons::*;
pub use keys::*;

#[cfg(test)]
mod tests;
