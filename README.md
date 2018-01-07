# daumdic-rs

[Daum Dictionary](http://dic.daum.net) API with Rust, inspired by [daumdic-ruby](https://github.com/simnalamburt/daumdic-ruby)

```rust
extern crate daumdic;

fn main() {
    let res = daumdic::search("ironic").unwrap();
    println!("{}", res);
    // => "ironic  [airάnik]  아이러니한, 역설적인, 모순적인, 반어적인"
}
```

------

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE-MIT)

[![License: Apache-2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](LICENSE-APACHE)