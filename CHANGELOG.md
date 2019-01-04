# Changelog

## 0.6.1

- use reqwest again (better at error handling and decoding)
- `Search` struct now implements `Clone`

## 0.6.0

- use [hyper](https://hyper.rs) instead of reqwest
- `search` function returns `impl Future<Item = Search, Error = failure::Error>`
- updated dependendies
- with Rust 2018 Edition!

## 0.5.1

- better search result

## 0.5.0

- now parsing html using [scraper](https://docs.rs/scraper) instead of kuchiki
- error chaining with [failure](https://docs.rs/failure) instead of error-chain
- renamed `SearchResult` to `Search` and changed its structure
    - before
        ```rust
        struct SearchResult {
            word: Option<Word>,
            alternatives: Vec<String>,
        }
        ```
    - after
        ```rust
        struct Search {
            words: Vec<Word>,
            alternatives: Vec<String>,
        }
        ```
- changed `Word` structure
    - before
        ```rust
        struct Word {
            word: String,
            meaning: String,
            pronounce: String,
            lang: Lang,
        }
        ```
    - after
        ```rust
        struct Word {
            word: String,
            meaning: Vec<String>,
            pronounce: Option<String>,
            lang: Lang,
        }
        ```
- refactored massive scraps of codes
