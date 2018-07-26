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
//! let res = &daumdic::search("독수리").unwrap().words[0];
//! assert_eq!(res.word, "독수리");
//! assert_eq!(res.lang, daumdic::Lang::Korean);
//! println!("{:?} {}", res.pronounce, res.meaning.join(", "));
//! ```
//!
//! ## English
//!
//! ```
//! let res = &daumdic::search("resist").unwrap().words[0];
//! assert_eq!(res.word, "resist");
//! assert_eq!(res.lang, daumdic::Lang::English);
//! println!("{}", res);
//! ```
//!
//! ## Japanese
//!
//! ```
//! let res = &daumdic::search("あと").unwrap().words[0];
//! assert_eq!(res.word, "あと");
//! assert_eq!(res.lang, daumdic::Lang::Japanese);
//! ```
//!
//! ## Other (ex. Chinese)
//!
//! ```
//! let res = &daumdic::search("加油站").unwrap().words[0];
//! assert_eq!(res.word, "加油站");
//! ```

#[macro_use]
extern crate failure;
extern crate reqwest;
extern crate scraper;
#[macro_use]
extern crate lazy_static;

pub mod errors;

use errors::Result;
use scraper::{Html, Selector};

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
#[derive(Clone, Debug)]
pub struct Word {
    pub word: String,
    pub meaning: Vec<String>,
    pub pronounce: Option<String>,
    pub lang: Lang,
}

impl std::fmt::Display for Word {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if let Lang::Other(ref d) = self.lang {
            write!(f, "({})  ", d)?;
        }
        write!(f, "{}  ", self.word)?;
        if let Some(ref pronounce) = self.pronounce {
            write!(f, "{}  ", pronounce)?;
        }
        write!(f, "{}", self.meaning.join(", "))
    }
}

/// Output of `search` function.
#[derive(Debug)]
pub struct Search {
    pub words: Vec<Word>,
    pub alternatives: Vec<String>,
}

fn parse_document(document: Html) -> Result<Search> {
    lazy_static! {
        static ref SELECTOR_BOX: Selector = Selector::parse(".search_box").unwrap();
        static ref SELECTOR_WORD: Selector =
            Selector::parse(".txt_cleansch,.txt_searchword,.txt_hanjaword").unwrap();
        static ref SELECTOR_LANG: Selector = Selector::parse(".tit_word").unwrap();
        static ref SELECTOR_PRONOUNCE: Selector =
            Selector::parse(".sub_read,.txt_pronounce").unwrap();
        static ref SELECTOR_MEANING: Selector = Selector::parse(".txt_search").unwrap();
        static ref SELECTOR_ALTERNATIVES: Selector = Selector::parse(".link_speller").unwrap();
    }

    let words = document
        .select(&SELECTOR_BOX)
        .map(|element| {
            let word = element
                .select(&SELECTOR_WORD)
                .flat_map(|element| element.text())
                .map(|s| s.to_string())
                .next();
            let lang = element
                .parent()
                .and_then(|node| scraper::ElementRef::wrap(node))
                .and_then(|element| {
                    element
                        .select(&SELECTOR_LANG)
                        .flat_map(|element| element.text())
                        .next()
                })
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
                        Lang::Other(lang.to_string())
                    }
                });
            let pronounce = element
                .select(&SELECTOR_PRONOUNCE)
                .map(|element| element.text().collect::<Vec<_>>().join(""))
                .next();
            let meaning = element
                .select(&SELECTOR_MEANING)
                .map(|element| element.text().collect::<Vec<_>>().join(""))
                .collect::<Vec<_>>();
            (word, lang, pronounce, meaning)
        })
        .filter_map(|t| match t {
            (Some(word), Some(lang), pronounce, meaning) => Some((word, lang, pronounce, meaning)),
            _ => None,
        })
        .map(|(word, lang, pronounce, meaning)| Word {
            word,
            lang,
            pronounce,
            meaning,
        })
        .collect();
    let alternatives = document
        .select(&SELECTOR_ALTERNATIVES)
        .map(|element| element.text().collect::<Vec<_>>().join(""))
        .collect();

    Ok(Search {
        words,
        alternatives,
    })
}

/// The main function.
///
/// # Example
///
/// ```
/// for word in daumdic::search("zoo").unwrap().words {
///     println!("{}", word);
/// }
/// ```
///
/// # Errors
///
/// This function fails if:
///
/// - given word is empty
/// - GET request failed
pub fn search(word: &str) -> Result<Search> {
    ensure!(!word.is_empty(), errors::DictionaryError::EmptyWord);

    let mut resp = reqwest::get(&format!("http://dic.daum.net/search.do?q={}", word))?;
    let document = Html::parse_document(&resp.text()?);
    parse_document(document)
}
