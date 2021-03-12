<h1 align="center">Async Destruction</h1>
<div align="center">
 <strong> A smart pointer which executes drop asynchronously in tokio.  </strong>
</div>
<br />
<div align="center">
  <!-- Crates version -->
  <a href="https://crates.io/crates/async_destruction">
<img src="https://img.shields.io/crates/v/async_destruction.svg?style=flat-square"
alt="Crates.io version" />
  </a>
  <!-- Downloads -->
  <a href="https://crates.io/crates/async_destruction">
<img src="https://img.shields.io/crates/d/async_destruction.svg?style=flat-square"
  alt="Download" />
  </a>
  <!-- docs.rs docs -->
  <a href="https://docs.rs/async_destruction">
<img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square"
  alt="docs.rs docs" />
  </a>
  <!-- ci -->
  <a href="https://docs.rs/async_destruction">
<img src="https://github.com/liangyongrui/async_destruction/workflows/Rust/badge.svg"
  alt="ci" />
  </a>
</div>

<br/>

## Basic usage

dependencies

```toml
async_destruction = "0.1"
tokio = { version = '1', features = ["full"] }
# Only used in the current example
chrono = "0.4"
```

demo

```rust
use async_destruction::AsyncDestruction;
use chrono::Utc;
use std::{thread::sleep, time::Duration};

#[derive(Clone)]
struct S;
impl Drop for S {
    fn drop(&mut self) {
        sleep(Duration::from_millis(1));
        println!("drop!");
    }
}

#[test]
fn it_works() {
    let a = vec![S; 10];
    let t1 = Utc::now().timestamp_millis();
    drop(a);
    let t2 = Utc::now().timestamp_millis();
    // will print 'drop cost time: 12ms'
    println!("drop cost time: {}ms", t2 - t1);
}

#[tokio::test]
async fn async_works() {
    let a = AsyncDestruction::new(vec![S; 10]);
    let t1 = Utc::now().timestamp_millis();
    drop(a);
    let t2 = Utc::now().timestamp_millis();
    // will print 'drop cost time: 0ms'
    println!("drop cost time: {}ms", t2 - t1);
}
```

## License

Licensed under either of

- [Apache License, Version 2.0](LICENSE-APACHE)
- [MIT license](LICENSE-MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions
