//! A Rust library that searches for words (Korean, English, Japanese, Chinese, ...) in the [Daum
//! dictionary] and returns their meanings and pronunciations.
//!
//! [Daum Dictionary]: https://dic.daum.net/
//!
//! ```
//! # tokio::runtime::Runtime::new().unwrap().block_on(async {
//! let res = &daumdic::search("독수리").await.unwrap().words[0];
//! assert_eq!(res.word, "독수리");
//! assert_eq!(res.lang, daumdic::Lang::Korean);
//! println!("{:?} {}", res.pronounce, res.meaning.join(", "));
//! # })
//! ```

#![deny(missing_docs)]

pub mod errors;

use {
    crate::errors::Result,
    lazy_static::lazy_static,
    reqwest::Client,
    scraper::{Html, Selector},
};

/// A type indicating the language of a word.
#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Lang {
    /// Korean
    Korean,
    /// English
    English,
    /// Japanese
    Japanese,
    /// Hanzi, also known as Chinese characters. They may appear in search results for Korean and
    /// Japanese, not just Chinese.
    Hanja,
    /// Other languages
    Other(String),
}

/// A type that contains the meaning, pronunciation, and language of each word returned by the
/// [`search`] function.
///
/// [`search`]: crate::search
#[derive(Clone, Debug)]
pub struct Word {
    /// The word returned as a search result
    pub word: String,
    /// The meaning of the word, primarily written in Korean
    pub meaning: Vec<String>,
    /// The pronunciation of the word, primarily using [IPA]
    ///
    /// [IPA]: https://en.wikipedia.org/wiki/International_Phonetic_Alphabet
    pub pronounce: Option<String>,
    /// The language of the word
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

/// A type that contains the search results for words and alternative search terms returned by the
/// [`search`] function.
///
/// [`search`]: crate::search
#[derive(Debug, Clone)]
pub struct Search {
    /// The words returned as search results
    pub words: Vec<Word>,
    /// Alternative search terms suggested by the Daum dictionary
    pub alternatives: Vec<String>,
}

fn parse_document(document: &Html) -> Result<Search> {
    lazy_static! {
        static ref SELECTOR_CARD: Selector = Selector::parse(".card_word").unwrap();
        static ref SELECTOR_ITEM: Selector =
            Selector::parse(".cleanword_type,.search_type").unwrap();
        static ref SELECTOR_WORD: Selector =
            Selector::parse(".txt_cleansch,.txt_searchword,.txt_hanjaword").unwrap();
        static ref SELECTOR_LANG: Selector = Selector::parse(".tit_word").unwrap();
        static ref SELECTOR_PRONOUNCE: Selector =
            Selector::parse(".sub_read,.txt_pronounce").unwrap();
        static ref SELECTOR_MEANING: Selector = Selector::parse(".txt_search").unwrap();
        static ref SELECTOR_ALTERNATIVES: Selector = Selector::parse(".link_speller").unwrap();
    }

    let words = document
        .select(&SELECTOR_CARD)
        .filter_map(|card| {
            let lang = card
                .select(&SELECTOR_LANG)
                .map(|element| element.text().collect::<Vec<_>>().join(""))
                .map(|lang| {
                    if lang.starts_with("한국") {
                        Lang::Korean
                    } else if lang.starts_with('영') {
                        Lang::English
                    } else if lang.starts_with('일') {
                        Lang::Japanese
                    } else if lang.starts_with("한자") {
                        Lang::Hanja
                    } else {
                        Lang::Other(lang.to_string())
                    }
                })
                .next();

            card.select(&SELECTOR_ITEM)
                .map(|item| {
                    let word = item
                        .select(&SELECTOR_WORD)
                        .map(|element| element.text().collect::<Vec<_>>().join(""))
                        .next();
                    let pronounce = item
                        .select(&SELECTOR_PRONOUNCE)
                        .map(|element| element.text().collect::<Vec<_>>().join(""))
                        .next();
                    let meaning = item
                        .select(&SELECTOR_MEANING)
                        .map(|element| element.text().collect::<Vec<_>>().join(""))
                        .collect::<Vec<_>>();

                    (word, lang.clone(), pronounce, meaning)
                })
                .filter_map(|t| match t {
                    (Some(word), Some(lang), pronounce, meaning) => {
                        Some((word, lang, pronounce, meaning))
                    }
                    _ => None,
                })
                .map(|(word, lang, pronounce, meaning)| Word {
                    word,
                    lang,
                    pronounce,
                    meaning,
                })
                .next()
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

/// A function that sends an HTTP GET request to the [Daum dictionary] to search for a word.
///
/// [Daum Dictionary]: https://dic.daum.net/
///
/// ```
/// # tokio::runtime::Runtime::new().unwrap().block_on(async {
/// let res = daumdic::search("zoo").await.unwrap();
/// for word in res.words {
///     println!("{}", word);
/// }
/// # })
/// ```
///
/// # Errors
///
/// This function will return an error under the following conditions:
/// - If the input search term is an empty string
/// - If the HTTP GET request fails due to network or server issues
pub async fn search(word: &str) -> Result<Search> {
    if word.is_empty() {
        Err(errors::DictionaryError::EmptyWord.into())
    } else {
        let client = Client::new();
        let url = format!("https://dic.daum.net/search.do?q={}", word);

        let resp = client.get(&url).send().await?;
        let body = resp.text().await?;
        let document = Html::parse_document(&body);
        parse_document(&document)
    }
}
