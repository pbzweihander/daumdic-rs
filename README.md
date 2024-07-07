daumdic-rs [![docs.rs]](https://crates.io/crates/daumdic)
========
[Daum Dictionary] API wrapper written in Rust, inspired by [daumdic-ruby].

```rust
use daumdic::search;

fn main() {
    let res = search("ironic").unwrap();
    println!("{}", res);
    // => "ironic  [airάnik]  아이러니한, 역설적인, 모순적인, 반어적인"
}
```

&nbsp;

--------
*daumdic-rs* is primarily distributed under the terms of both the [Apache
License (Version 2.0)] and the [MIT license]. See [COPYRIGHT] for details.

[docs.rs]: https://badgen.net/crates/v/daumdic
[Daum Dictionary]: https://dic.daum.net
[daumdic-ruby]: https://github.com/simnalamburt/daumdic-ruby
[Apache License (Version 2.0)]: LICENSE-APACHE
[MIT license]: LICENSE-MIT
[COPYRIGHT]: COPYRIGHT
