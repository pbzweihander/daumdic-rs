#![cfg(feature = "async")]

extern crate daumdic;
extern crate futures;
extern crate tokio_core;

use daumdic::Lang;
use daumdic::async::search;
use tokio_core::reactor::Core;

#[test]
fn async() {
    let mut core = Core::new().unwrap();

    let future = search(&core.handle(), "resist");

    let res = core.run(future).unwrap().word.unwrap();
    assert!(!res.meaning.is_empty());
    assert!(!res.pronounce.is_empty());
    assert_eq!(res.lang, Lang::English);
}
