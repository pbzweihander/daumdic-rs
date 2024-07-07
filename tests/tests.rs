use daumdic::{search, Lang};
use tokio::time::{interval, Duration};

#[tokio::test]
async fn test() {
    // Limit RPS to 1 req/sec
    let mut interval = interval(Duration::from_secs(1));

    // empty_word
    assert!(search("").await.is_err());
    interval.tick().await;

    // not_found
    let res = search("asdfaserqfasd").await.unwrap();
    assert!(res.words.is_empty());
    interval.tick().await;

    // alternatives
    let res = search("resista").await.unwrap();
    assert!(!res.alternatives.is_empty());
    assert_eq!(res.alternatives[0], "resist");
    interval.tick().await;

    // korean
    let res = &search("독수리").await.unwrap().words[0];
    assert_eq!(res.word, "독수리");
    assert!(!res.meaning.is_empty());
    assert!(res.pronounce.is_some());
    assert_eq!(res.lang, Lang::Korean);
    interval.tick().await;

    // english
    let res = &search("resist").await.unwrap().words[0];
    assert_eq!(res.word, "resist");
    assert!(!res.meaning.is_empty());
    assert!(res.pronounce.is_some());
    assert_eq!(res.lang, Lang::English);
    interval.tick().await;

    // japanese
    let res = &search("あと").await.unwrap().words[0];
    assert_eq!(res.word, "あと");
    assert!(!res.meaning.is_empty());
    assert_eq!(res.lang, Lang::Japanese);
    interval.tick().await;

    // hanja
    let res = &search("方").await.unwrap().words[0];
    assert_eq!(res.word, "方");
    assert!(!res.meaning.is_empty());
    assert!(res.pronounce.is_some());
    assert_eq!(res.lang, Lang::Hanja);
    interval.tick().await;

    // other
    let res = &search("加油站").await.unwrap().words[0];
    assert_eq!(res.word, "加油站");
    assert!(!res.meaning.is_empty());
    assert!(res.pronounce.is_some());
    assert_eq!(res.lang, Lang::Other("중국어사전".to_owned()));
}
