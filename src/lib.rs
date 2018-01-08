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
//! let res = daumdic::search("독수리").unwrap();
//! assert_eq!(res.word, "독수리");
//! assert_eq!(res.meaning, "수릿과에 속한 큰 새");
//! assert_eq!(res.pronounce, "[-쑤-]");
//! assert_eq!(res.lang, daumdic::Lang::Korean);
//! ```
//!
//! ## English
//!
//! ```
//! let res = daumdic::search("resist").unwrap();
//! assert_eq!(res.word, "resist");
//! assert_eq!(
//!     res.meaning,
//!     "저항하다, 반대하다, 참다, 저지하다"
//! );
//! assert_eq!(res.pronounce, "[rizíst]");
//! assert_eq!(res.lang, daumdic::Lang::English);
//! ```
//!
//! ## Japanese
//!
//! ```
//! let res = daumdic::search("ざつおん").unwrap();
//! assert_eq!(res.word, "ざつおん");
//! assert_eq!(
//!     res.meaning,
//!     "잡음, 소음, (라디오 등의) 잡음, <속어> 뜬소문, <속어> 말참견, 잡음, 시끄러운 소리, (비유적으로) 관계자 이외로부터 나오는 무책임한 발언‧의견, 전화‧라디오 등의 청취를 방해하는 소리, 불쾌한 느낌을 주는 소리"
//! );
//! assert_eq!(res.pronounce, "");
//! assert_eq!(res.lang, daumdic::Lang::Japanese);
//! ```
//!
//! ## Other (ex. Chinese)
//!
//! ```
//! let res = daumdic::search("加油站").unwrap();
//! assert_eq!(res.word, "加油站");
//! assert_eq!(res.meaning, "주유소");
//! assert_eq!(res.pronounce, "[jiāyóuzhàn]");
//! assert_eq!(res.lang, daumdic::Lang::Other("중국어사전".to_owned()));
//! ```

#[macro_use]
extern crate error_chain;
extern crate kuchiki;
extern crate reqwest;

use kuchiki::traits::TendrilSink;

#[cfg(test)]
mod tests;

pub mod error;
pub use error::{Error, ErrorKind, Result};

/// Type of word language
#[derive(PartialEq, Debug)]
pub enum Lang {
    Korean,
    English,
    Japanese,
    Hanja,
    Other(String),
}

/// Result of `search` function
pub struct Word {
    pub word: String,
    pub meaning: String,
    pub pronounce: String,
    pub lang: Lang,
}

/// Quick way to make clean output
///
/// # Example
///
/// ```
/// assert_eq!(format!("{}", daumdic::search("ironic").unwrap()), "ironic  [airάnik]  아이러니한, 역설적인, 모순적인, 반어적인");
/// ```
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

/// The main function.
///
/// # Example
///
/// ```
/// println!("{}", daumdic::search("zoo").unwrap().meaning);
/// ```
///
/// # Errors
///
/// This function fails if:
///
/// - given word is empty
/// - cannot find given word
/// - GET request failed
pub fn search(word: &str) -> Result<Word> {
    ensure!(!word.is_empty(), ErrorKind::EmptyWord);

    let mut addr = String::from("http://dic.daum.net/search.do?q=");
    addr.push_str(word);
    let mut resp = reqwest::get(&addr)?;
    let document = kuchiki::parse_html().one(resp.text()?);

    let rel = document
        .select(".link_speller")
        .unwrap()
        .map(|r| r.text_contents())
        .collect::<Vec<String>>();
    ensure!(rel.is_empty(), ErrorKind::RelativeResultFound(rel));

    let sbox = match document.select_first(".search_box") {
        Ok(b) => b,
        Err(_) => bail!(ErrorKind::WordNotFound),
    };
    let sbox = sbox.as_node();

    let word = sbox.select_first(".txt_cleansch")
        .or(sbox.select_first(".txt_searchword"))
        .or(sbox.select_first(".txt_hanjaword"))
        .map_err::<Error, _>(|_| ErrorKind::ParsingFailed.into())?
        .text_contents();
    let lang: Lang = {
        let lang = sbox.ancestors()
            .next()
            .and_then(|a| a.select_first(".tit_word").ok())
            .ok_or::<Error>(ErrorKind::ParsingFailed.into())?
            .text_contents();
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
    };
    let meaning = sbox.select(".txt_search")
        .map_err::<Error, _>(|_| ErrorKind::ParsingFailed.into())?
        .map(|m| m.text_contents())
        .collect::<Vec<String>>()
        .join(", ");
    let pronounce = match lang {
        Lang::Hanja => sbox.select_first(".sub_read"),
        _ => sbox.select_first(".txt_pronounce"),
    }.map(|p| p.text_contents())
        .unwrap_or(String::new());

    Ok(Word {
        word,
        meaning,
        pronounce,
        lang,
    })
}
