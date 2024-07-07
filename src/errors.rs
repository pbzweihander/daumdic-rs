//! Errors for [`daumdic`] crate using [`failure`]
//!
//! [`daumdic`]: crate
//! [`failure`]: failure

use failure::Fail;

pub use failure::Error;

/// A specialized [`Result`] type for [`daumdic`] crate.
///
/// [`Result`]: std::result::Result
/// [`daumdic`]: crate
pub type Result<T> = ::std::result::Result<T, ::failure::Error>;

/// When empty word was given to `search` function
#[derive(Debug, Fail)]
pub enum DictionaryError {
    /// Error when trying to search for an empty string
    #[fail(display = "empty word was given")]
    EmptyWord,
}
