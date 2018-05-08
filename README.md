# fpool
Non-leased object-pooling in Rust.

Non-leased as in: objects are not passed out from the Pool, but can
be acted upon synchronously. This means that the logic for re-creating
and choosing the next item can be done by the pool.

# Getting started

Add the following to your `Cargo.toml` file:

```toml
[dependencies]
fpool = "0.2"
```

Next, add this to your crate:

```no_run
extern crate fpool;
```

# Examples

A trivial use-case for a round-robin pool:

```rust
use fpool::RoundRobinPool;

let mut pool = RoundRobinPool::builder(5, || {
   Ok(Vec::new())
}).build().expect("No constructor failure case");

for index in 0..10 {
   pool.act(|list| {
       list.push(index);
       true // action succeeded and item is still valid
   }).expect("No constructor failure case");
}

// The pool now has 5 lists with 2 items each
for _ in 0..5 {
   pool.act(|list| {
       assert_eq!(list.len(), 2);
       true // action succeeded and item is still valid
   }).expect("No constructor failure case");
}
```

But a more useful and realistic example is a thread-pool, see
[./examples/thread_pool.rs](examples/thread_pool.rs).
