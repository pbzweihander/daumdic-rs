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
//! # });
//! # std::thread::sleep(std::time::Duration::from_secs(1));
//! ```

#![deny(missing_docs)]

pub mod errors;

use scraper::Selector;
use std::sync::OnceLock;

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

struct SelectorSet {
    card: Selector,
    item: Selector,
    word: Selector,
    lang: Selector,
    pronounce: Selector,
    meaning: Selector,
    alternatives: Selector,
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
/// # });
/// # std::thread::sleep(std::time::Duration::from_secs(1));
/// ```
///
/// # Errors
///
/// This function will return an error under the following conditions:
/// - If the input search term is an empty string
/// - If the HTTP GET request fails due to network or server issues
pub async fn search(word: &str) -> errors::Result<Search> {
    if word.is_empty() {
        return Err(errors::DictionaryError::EmptyWord.into());
    }

    let client = reqwest::Client::new();
    let url = format!("https://dic.daum.net/search.do?q={}", word);

    let resp = client.get(&url).send().await?;
    let body = resp.text().await?;
    let document = scraper::Html::parse_document(&body);

    static SELECTOR_CACHE: OnceLock<SelectorSet> = OnceLock::new();
    let selector = SELECTOR_CACHE.get_or_init(|| SelectorSet {
        card: Selector::parse(".card_word").unwrap(),
        item: Selector::parse(".cleanword_type,.search_type").unwrap(),
        word: Selector::parse(".txt_cleansch,.txt_searchword,.txt_hanjaword").unwrap(),
        lang: Selector::parse(".tit_word").unwrap(),
        pronounce: Selector::parse(".sub_read,.txt_pronounce").unwrap(),
        meaning: Selector::parse(".txt_search").unwrap(),
        alternatives: Selector::parse(".link_speller").unwrap(),
    });

    let words = document
        .select(&selector.card)
        .filter_map(|card| {
            let lang = card
                .select(&selector.lang)
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

            card.select(&selector.item)
                .map(|item| {
                    let word = item
                        .select(&selector.word)
                        .map(|element| element.text().collect::<Vec<_>>().join(""))
                        .next();
                    let pronounce = item
                        .select(&selector.pronounce)
                        .map(|element| element.text().collect::<Vec<_>>().join(""))
                        .next();
                    let meaning = item
                        .select(&selector.meaning)
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
        .select(&selector.alternatives)
        .map(|element| element.text().collect::<Vec<_>>().join(""))
        .collect();

    Ok(Search {
        words,
        alternatives,
    })
}
