extern crate daumdic;

use daumdic::*;

#[test]
fn empty_word() {
    assert!(search("").is_err())
}

#[test]
fn not_found() {
    let res = search("asdfaserqfasd").unwrap();
    assert!(res.word.is_none());
}

#[test]
fn alternatives() {
    let res = search("resista").unwrap();
    assert!(!res.alternatives.is_empty());
    assert_eq!(res.alternatives[0], "resist");
}

#[test]
fn korean() {
    let res = search("독수리").unwrap().word.unwrap();
    assert_eq!(res.word, "독수리");
    assert!(!res.meaning.is_empty());
    assert!(!res.pronounce.is_empty());
    assert_eq!(res.lang, Lang::Korean);
}

#[test]
fn english() {
    let res = search("resist").unwrap().word.unwrap();
    assert_eq!(res.word, "resist");
    assert!(!res.meaning.is_empty());
    assert!(!res.pronounce.is_empty());
    assert_eq!(res.lang, Lang::English);
}

#[test]
fn japanese() {
    let res = search("ざつおん").unwrap().word.unwrap();
    assert_eq!(res.word, "ざつおん");
    assert!(!res.meaning.is_empty());
    assert!(res.pronounce.is_empty());
    assert_eq!(res.lang, Lang::Japanese);
}

#[test]
fn hanja() {
    let res = search("방").unwrap().word.unwrap();
    assert_eq!(res.word, "方");
    assert!(!res.meaning.is_empty());
    assert!(!res.pronounce.is_empty());
    assert_eq!(res.lang, Lang::Hanja);
}

#[test]
fn other() {
    let res = search("加油站").unwrap().word.unwrap();
    assert_eq!(res.word, "加油站");
    assert!(!res.meaning.is_empty());
    assert!(!res.pronounce.is_empty());
    assert_eq!(res.lang, Lang::Other("중국어사전".to_owned()));
}
