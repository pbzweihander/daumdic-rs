use super::*;

#[test]
fn empty_word() {
    match search("") {
        Ok(o) => panic!("Should panic but got Ok: {}", o),
        Err(e) => match e.downcast::<SearchError>() {
            Ok(e) => match e {
                SearchError::EmptyWordError => {}
                e => panic!("Should EmptyWordError but got {}", e),
            },
            Err(e) => panic!("Should SearchError but got {}", e),
        },
    };
}

#[test]
fn not_found() {
    match search("asdfaserqfasd") {
        Ok(o) => panic!("Should panic but got Ok: {}", o),
        Err(e) => match e.downcast::<SearchError>() {
            Ok(e) => match e {
                SearchError::WordNotFoundError => {}
                e => panic!("Should WordNotFoundError but got {}", e),
            },
            Err(e) => panic!("Should SearchError but got {}", e),
        },
    }
}

#[test]
fn relative() {
    match search("resista") {
        Ok(o) => panic!("Should panic but got Ok: {}", o),
        Err(e) => match e.downcast::<SearchError>() {
            Ok(e) => match e {
                SearchError::RelativeResultFoundError(v) => match v[0].as_str() {
                    "resist" => {}
                    o => panic!("Should 'resist' but got '{}'", o),
                },
                e => panic!("Should RelativeResultFoundError but got {}", e),
            },
            Err(e) => panic!("Should SearchError but got {}", e),
        },
    }
}

#[test]
fn korean() {
    let res = match search("독수리") {
        Ok(o) => o,
        Err(e) => panic!("Should Ok but got error: {}", e),
    };
    assert_eq!(res.word, "독수리");
    assert_eq!(res.meaning, "수릿과에 속한 큰 새");
    assert_eq!(res.pronounce, "[-쑤-]");
    assert_eq!(res.lang, Lang::Korean);
}

#[test]
fn english() {
    let res = match search("resist") {
        Ok(o) => o,
        Err(e) => panic!("Should Ok but got error: {}", e),
    };
    assert_eq!(res.word, "resist");
    assert_eq!(
        res.meaning,
        "저항하다, 반대하다, 참다, 저지하다"
    );
    assert_eq!(res.pronounce, "[rizíst]");
    assert_eq!(res.lang, Lang::English);
}

#[test]
fn japanese() {
    let res = match search("ざつおん") {
        Ok(o) => o,
        Err(e) => panic!("Should Ok but got error: {}", e),
    };
    assert_eq!(res.word, "ざつおん");
    assert_eq!(
        res.meaning,
        "잡음, 소음, (라디오 등의) 잡음, <속어> 뜬소문, <속어> 말참견, 잡음, 시끄러운 소리, (비유적으로) 관계자 이외로부터 나오는 무책임한 발언‧의견, 전화‧라디오 등의 청취를 방해하는 소리, 불쾌한 느낌을 주는 소리"
    );
    assert_eq!(res.pronounce, "");
    assert_eq!(res.lang, Lang::Japanese);
}

#[test]
fn other() {
    let res = match search("加油站") {
        Ok(o) => o,
        Err(e) => panic!("Should Ok but got error: {}", e),
    };
    assert_eq!(res.word, "加油站");
    assert_eq!(res.meaning, "주유소");
    assert_eq!(res.pronounce, "[jiāyóuzhàn]");
    assert_eq!(res.lang, Lang::Other("중국어사전".to_owned()));
}

#[test]
fn display() {
    let res = match search("ironic") {
        Ok(o) => o,
        Err(e) => panic!("Should Ok but got error: {}", e),
    };
    assert_eq!(
        format!("{}", res),
        "ironic  [airάnik]  아이러니한, 역설적인, 모순적인, 반어적인"
    );
}
