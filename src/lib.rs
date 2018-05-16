//! Non-leased object-pooling in Rust.
//!
//! Non-leased as in: you cannot hold onto objects given from the Pool.
//! This, unfortunately, is not something I could get enforced by the compiler
//! without making the API hard to work with.
//!
//!
//! # Getting started
//!
//! Add the following to your `Cargo.toml` file:
//!
//! ```toml
//! [dependencies]
//! fpool = "0.3"
//! ```
//!
//! Next, add this to your crate:
//!
//! ```no_run
//! extern crate fpool;
//! ```
//!
//! # Examples
//!
//! A trivial use-case for a round-robin pool:
//!
//! ```rust
//! use fpool::RoundRobinPool;
//!
//! let mut pool = RoundRobinPool::builder(5, || -> Result<_, ()> {
//!     Ok(Vec::new())
//! }).build().expect("No constructor failure case");
//!
//! for index in 0..10 {
//!     let list = pool.get().expect("No constructor failure case");
//!     list.push(index);
//! }
//!
//! // The pool now has 5 lists with 2 items each
//! for _ in 0..5 {
//!     let list = pool.get().expect("No constructor failure case");
//!     assert_eq!(list.len(), 2);
//! }
//! ```
//!
//! But a more useful and realistic example is a thread-pool, see
//! [examples/thread_pool.rs](https://github.com/DarrenTsung/fpool/blob/master/examples/thread_pool.rs).
mod pool;

pub use pool::*;
