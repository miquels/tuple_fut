# tuple-fut

[![Apache-2.0 licensed](https://img.shields.io/badge/license-Apache2.0-blue.svg)](https://www.apache.org/licenses/LICENSE-2.0.txt)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](http://opensource.org/licenses/MIT)
[![crates.io](https://meritbadge.herokuapp.com/tuple-fut)](https://crates.io/crates/tuple-fut)
[![Released API docs](https://docs.rs/tuple-fut/badge.svg)](https://docs.rs/tuple-fut)

join and select as methods on tuples instead of macros.

## Select

Call `select()` on a tuple of futures to await the first one that completes.
They must all have the same output type.

#### Example

```rust
use tuple_fut::Select;

let result = (fut1, fut2, fut3).select().await;
```

## Join

Call `join()` on a tuple of futures to await all of them.
It returns a tuple of the output values of the resolved futures.

#### Example

```rust
use tuple_fut::Join;

let (res1, res2, res3) = (fut1, fut2, fut3).join().await;
```

## Caveats.

All futures must be `Unpin`. That means you cannot use an async
function directly as future in a tuple. You need to pin it first,
for example by using [tokio::pin](https://docs.rs/tokio/1.0/tokio/macro.pin.html)
or [futures::pin_mut](https://docs.rs/futures/0.3/futures/macro.pin_mut.html).

#### Example

```rust
use tuple_fut::Select;

async fn foo() -> u32 {
    42
}

async fn bar() -> u32 {
    23
}

async fn something() -> u32 {
    tokio::pin {
        let foo = foo();
        let bar = bar();
    }

    (foo, bar).select().await
}
```

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
