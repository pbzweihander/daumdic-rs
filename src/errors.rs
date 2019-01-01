//! Errors for `daumdic` crate using `failure`

#![allow(missing_docs)]

use failure::Fail;

pub use failure::Error;
pub type Result<T> = ::std::result::Result<T, ::failure::Error>;

/// When empty word was given to `search` function
#[derive(Debug, Fail)]
pub enum DictionaryError {
    #[fail(display = "empty word was given")]
    EmptyWord,
}
