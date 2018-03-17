# daumdic-rs

[![circleci](https://circleci.com/gh/pbzweihander/daumdic-rs.svg?style=shield)](https://circleci.com/gh/pbzweihander/daumdic-rs)
[![crate.io](https://img.shields.io/crates/v/daumdic.svg)](https://crates.io/crates/daumdic)
[![docs.rs](https://docs.rs/daumdic/badge.svg)](https://docs.rs/daumdic)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE-MIT)
[![License: Apache-2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](LICENSE-APACHE)

[Daum Dictionary](http://dic.daum.net) API wrapper written in Rust, inspired by [daumdic-ruby](https://github.com/simnalamburt/daumdic-ruby)

```rust
extern crate daumdic;

fn main() {
    let res = daumdic::search("ironic").unwrap();
    println!("{}", res);
    // => "ironic  [airάnik]  아이러니한, 역설적인, 모순적인, 반어적인"
}
```
