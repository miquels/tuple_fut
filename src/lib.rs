#![doc(html_root_url = "https://docs.rs/tuple-fut/0.1.0")]
//! join and select as methods on tuples instead of macros.
//!
//! # Select
//!
//! Call `select()` on a tuple of futures to await the first one that completes.
//! They must all have the same output type.
//!
//! ### Example
//!
//! ```no_run
//! use tuple_fut::Select;
//!
//! let result = (fut1, fut2, fut3).select().await;
//! ```
//!
//! # Join
//!
//! Call `join()` on a tuple of futures to await all of them.
//! It returns a tuple of the output values of the resolved futures.
//!
//! ### Example
//!
//! ```no_run
//! use tuple_fut::Join;
//!
//! let (res1, res2, res3) = (fut1, fut2, fut3).join().await;
//! ```
//!
//! # Caveats.
//!
//! All futures must be `Unpin`. That means you cannot use an async
//! function directly as future in a tuple. You need to pin it first,
//! for example by using [tokio::pin](https://docs.rs/tokio/1.0/tokio/macro.pin.html)
//! or [futures::pin_mut](https://docs.rs/futures/0.3/futures/macro.pin_mut.html).
//!
//! ### Example
//!
//! ```no_run
//! use tuple_fut::Select;
//!
//! async fn foo() -> u32 {
//!     42
//! }
//!
//! async fn bar() -> u32 {
//!     23
//! }
//!
//! async fn something() -> u32 {
//!     tokio::pin {
//!         let foo = foo();
//!         let bar = bar();
//!     }
//!
//!     (foo, bar).select().await
//! }
//! ```
#[doc(hidden)]
pub mod join;
#[doc(hidden)]
pub mod select;

#[doc(inline)]
pub use join::Join;

#[doc(inline)]
pub use select::Select;

