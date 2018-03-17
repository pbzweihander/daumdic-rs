extern crate futures;
extern crate reqwest;
extern crate tokio_core;

use reqwest::unstable::async as request;
use self::futures::prelude::*;
use self::futures::stream::iter_ok;
use self::tokio_core::reactor::Handle;
use super::{parse_document, Error, SearchResult};

/// Async search function. Requires handle.
pub fn search(handle: &Handle, word: &str) -> Box<Future<Item = SearchResult, Error = Error>> {
    Box::new(
        request::Client::new(handle)
            .get(&format!("http://dic.daum.net/search.do?q={}", word))
            .send()
            .map_err::<_, Error>(|e| e.into())
            .and_then(|resp| {
                resp.into_body()
                    .map(|chunk| iter_ok::<_, Error>(chunk.into_iter()))
                    .flatten()
                    .collect()
            })
            .and_then(|v| String::from_utf8(v).map_err(|e| e.into()).into_future())
            .map(|content| parse_document(content)),
    )
}
