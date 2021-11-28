use daumdic::*;

#[tokio::test]
async fn empty_word() {
    assert!(search("").await.is_err())
}

#[tokio::test]
async fn not_found() {
    let res = search("asdfaserqfasd").await.unwrap();
    assert!(res.words.is_empty());
}

#[tokio::test]
async fn alternatives() {
    let res = search("resista").await.unwrap();
    assert!(!res.alternatives.is_empty());
    assert_eq!(res.alternatives[0], "resist");
}

#[tokio::test]
async fn korean() {
    let res = &search("독수리").await.unwrap().words[0];
    assert_eq!(res.word, "독수리");
    assert!(!res.meaning.is_empty());
    assert!(res.pronounce.is_some());
    assert_eq!(res.lang, Lang::Korean);
}

#[tokio::test]
async fn english() {
    let res = &search("resist").await.unwrap().words[0];
    assert_eq!(res.word, "resist");
    assert!(!res.meaning.is_empty());
    assert!(res.pronounce.is_some());
    assert_eq!(res.lang, Lang::English);
}

#[tokio::test]
async fn japanese() {
    let res = &search("あと").await.unwrap().words[0];
    assert_eq!(res.word, "あと");
    assert!(!res.meaning.is_empty());
    assert_eq!(res.lang, Lang::Japanese);
}

#[tokio::test]
async fn hanja() {
    let res = &search("方").await.unwrap().words[0];
    assert_eq!(res.word, "方");
    assert!(!res.meaning.is_empty());
    assert!(res.pronounce.is_some());
    assert_eq!(res.lang, Lang::Hanja);
}

#[tokio::test]
async fn other() {
    let res = &search("加油站").await.unwrap().words[0];
    assert_eq!(res.word, "加油站");
    assert!(!res.meaning.is_empty());
    assert!(res.pronounce.is_some());
    assert_eq!(res.lang, Lang::Other("중국어사전".to_owned()));
}
