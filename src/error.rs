//! Errors for `daumdic` crate using `error_chain`

#![allow(missing_docs)]

error_chain! {
    foreign_links {
        Request(::reqwest::Error);
    }
    errors {
        /// When empty word was given to `search` function
        EmptyWord {
            description("Empty word was given")
            display("Empty word")
        }
        /// Failed to find given word
        WordNotFound {
            description("Cannot find given word")
            display("Word not found")
        }
        /// No matching word but found relative search result
        RelativeResultFound(words: Vec<String>) {
            description("Cannot find given word but got relative recommendation")
            display("Did you mean: {}", words.join(", "))
        }
        ParsingFailed {
            description("Parsing failed")
            display("Parsing failed")
        }
    }
}
