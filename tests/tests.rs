use {daumdic::*, futures::prelude::*, tokio::runtime::current_thread::Runtime};

fn run<T, E>(fut: impl Future<Item = T, Error = E>) -> Result<T, E> {
    let mut rt = Runtime::new().unwrap();
    rt.block_on(fut)
}

#[test]
fn empty_word() {
    assert!(run(search("")).is_err())
}

#[test]
fn not_found() {
    let res = run(search("asdfaserqfasd")).unwrap();
    assert!(res.words.is_empty());
}

#[test]
fn alternatives() {
    let res = run(search("resista")).unwrap();
    assert!(!res.alternatives.is_empty());
    assert_eq!(res.alternatives[0], "resist");
}

#[test]
fn korean() {
    let res = &run(search("독수리")).unwrap().words[0];
    assert_eq!(res.word, "독수리");
    assert!(!res.meaning.is_empty());
    assert!(res.pronounce.is_some());
    assert_eq!(res.lang, Lang::Korean);
}

#[test]
fn english() {
    let res = &run(search("resist")).unwrap().words[0];
    assert_eq!(res.word, "resist");
    assert!(!res.meaning.is_empty());
    assert!(res.pronounce.is_some());
    assert_eq!(res.lang, Lang::English);
}

#[test]
fn japanese() {
    let res = &run(search("あと")).unwrap().words[0];
    assert_eq!(res.word, "あと");
    assert!(!res.meaning.is_empty());
    assert_eq!(res.lang, Lang::Japanese);
}

#[test]
fn hanja() {
    let res = &run(search("방")).unwrap().words[0];
    assert_eq!(res.word, "方");
    assert!(!res.meaning.is_empty());
    assert!(res.pronounce.is_some());
    assert_eq!(res.lang, Lang::Hanja);
}

#[test]
fn other() {
    let res = &run(search("加油站")).unwrap().words[0];
    assert_eq!(res.word, "加油站");
    assert!(!res.meaning.is_empty());
    assert!(res.pronounce.is_some());
    assert_eq!(res.lang, Lang::Other("중국어사전".to_owned()));
}
