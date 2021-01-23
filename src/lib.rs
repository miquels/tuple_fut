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
//! ```rust
//! use tuple_fut::Select;
//!
//! async fn ret11() -> u32 {
//!   11
//! }
//!
//! async fn ret23() -> u32 {
//!   23
//! }
//!
//! async fn ret42() -> u32 {
//!   42
//! }
//!
//! # fn main() {
//! #     futures::executor::block_on(async {
//! let result = (ret11(), ret23(), ret42()).select().await;
//! println!("{}", result);
//! assert!(result == 11 || result == 23 || result == 42);
//! #     });
//! # }
//! ```
//!
//! # Join
//!
//! Call `join()` on a tuple of futures to await all of them.
//! It returns a tuple of the output values of the resolved futures.
//!
//! ### Example
//!
//! ```rust
//! use tuple_fut::Join;
//!
//! async fn ret_u32() -> u32 {
//!   23u32
//! }
//!
//! async fn ret_f64() -> f64 {
//!   42f64
//! }
//!
//! async fn ret_string() -> String {
//!   String::from("hello")
//! }
//!
//! # fn main() {
//! #     futures::executor::block_on(async {
//! let result = (ret_u32(), ret_f64(), ret_string()).join().await;
//! println!("{:?}", result);
//! assert!(result == (23u32, 42f64, String::from("hello")));
//! #     });
//! # }
//! ```
//!
//! # Caveats.
//!
//! Due to a restriction in Rust's type system, these traits are only implemented on
//! tuples of arity 12 or less. 
#[doc(hidden)]
pub mod join;
#[doc(hidden)]
pub mod select;

#[doc(inline)]
pub use join::Join;

#[doc(inline)]
pub use select::Select;

