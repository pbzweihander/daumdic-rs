//! Errors for `daumdic` crate using `error_chain`

#![allow(missing_docs)]

error_chain! {
    foreign_links {
        Request(::reqwest::Error);
        FromUtf8(::std::string::FromUtf8Error);
    }
    errors {
        /// When empty word was given to `search` function
        EmptyWord {
            description("Empty word was given")
            display("Empty word")
        }
    }
}
