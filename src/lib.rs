//! # daumdic
//!
//! Find word (Korean, English, Japanese, Chinese, ...) in Daum Dictionary
//! and returns its meaning and pronuciation.
//!
//! # Examples
//!
//! ## Korean
//!
//! ```
//! let res = daumdic::search("독수리").unwrap().word.unwrap();
//! assert_eq!(res.word, "독수리");
//! assert_eq!(res.lang, daumdic::Lang::Korean);
//! println!("{} {}", res.pronounce, res.meaning);
//! ```
//!
//! ## English
//!
//! ```
//! let res = daumdic::search("resist").unwrap().word.unwrap();
//! assert_eq!(res.word, "resist");
//! assert_eq!(res.lang, daumdic::Lang::English);
//! println!("{}", res);
//! ```
//!
//! ## Japanese
//!
//! ```
//! let res = daumdic::search("ざつおん").unwrap().word.unwrap();
//! assert_eq!(res.word, "ざつおん");
//! assert_eq!(res.lang, daumdic::Lang::Japanese);
//! ```
//!
//! ## Other (ex. Chinese)
//!
//! ```
//! let res = daumdic::search("加油站").unwrap().word.unwrap();
//! assert_eq!(res.word, "加油站");
//! ```

#[macro_use]
extern crate error_chain;
extern crate kuchiki;
extern crate reqwest;

use kuchiki::traits::TendrilSink;

pub mod errors;
pub use errors::{Error, ErrorKind, Result};

#[cfg(feature = "async")]
pub mod async;

/// Type of word language
#[derive(PartialEq, Clone, Debug)]
pub enum Lang {
    Korean,
    English,
    Japanese,
    Hanja,
    Other(String),
}

/// Result of `search` function
#[derive(Clone)]
pub struct Word {
    pub word: String,
    pub meaning: String,
    pub pronounce: String,
    pub lang: Lang,
}

impl std::fmt::Display for Word {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if let Lang::Other(ref d) = self.lang {
            write!(f, "({})  ", d)?;
        }
        write!(f, "{}  ", self.word)?;
        if !self.pronounce.is_empty() {
            write!(f, "{}  ", self.pronounce)?;
        }
        write!(f, "{}", self.meaning)
    }
}

/// Output of `search` function.
pub struct SearchResult {
    pub word: Option<Word>,
    pub alternatives: Vec<String>,
}

fn parse_document(content: String) -> SearchResult {
    let document = kuchiki::parse_html().one(content);

    let word = document
        .select_first(".search_box")
        .ok()
        .map(|sbox| sbox.as_node().clone())
        .map(|sbox| {
            let word = sbox.select_first(".txt_cleansch")
                .or_else(|_| sbox.select_first(".txt_searchword"))
                .or_else(|_| sbox.select_first(".txt_hanjaword"))
                .ok()
                .map(|element| element.text_contents());
            let lang = sbox.ancestors()
                .next()
                .and_then(|a| a.select_first(".tit_word").ok())
                .map(|element| element.text_contents())
                .map(|lang| {
                    if lang.starts_with("한국") {
                        Lang::Korean
                    } else if lang.starts_with("영") {
                        Lang::English
                    } else if lang.starts_with("일") {
                        Lang::Japanese
                    } else if lang.starts_with("한자") {
                        Lang::Hanja
                    } else {
                        Lang::Other(lang)
                    }
                });
            let pronounce = sbox.select_first(".sub_read")
                .ok()
                .or_else(|| sbox.select_first(".txt_pronounce").ok())
                .map(|element| element.text_contents())
                .unwrap_or_default();
            let meaning = sbox.select(".txt_search").ok().map(|select| {
                select
                    .map(|element| element.text_contents())
                    .collect::<Vec<String>>()
                    .join(", ")
            });
            (word, lang, pronounce, meaning)
        })
        .and_then(|word| match word {
            (Some(word), Some(lang), pronounce, Some(meaning)) => {
                Some((word, lang, pronounce, meaning))
            }
            _ => None,
        })
        .map(|(word, lang, pronounce, meaning)| Word {
            word,
            lang,
            pronounce,
            meaning,
        });

    let alternatives = document
        .select(".link_speller")
        .ok()
        .map(|select| select.map(|r| r.text_contents()).collect::<Vec<String>>())
        .unwrap_or_default();

    SearchResult { word, alternatives }
}

/// The main function.
///
/// # Example
///
/// ```
/// println!("{}", daumdic::search("zoo").unwrap().word.unwrap());
/// ```
///
/// # Errors
///
/// This function fails if:
///
/// - given word is empty
/// - GET request failed
pub fn search(word: &str) -> Result<SearchResult> {
    ensure!(!word.is_empty(), ErrorKind::EmptyWord);

    reqwest::get(&format!("http://dic.daum.net/search.do?q={}", word))
        .and_then(|mut resp| resp.text())
        .map(|content| parse_document(content))
        .map_err(|e| e.into())
}
