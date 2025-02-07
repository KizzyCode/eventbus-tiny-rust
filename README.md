[![License BSD-2-Clause](https://img.shields.io/badge/License-BSD--2--Clause-blue.svg)](https://opensource.org/licenses/BSD-2-Clause)
[![License MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![AppVeyor CI](https://ci.appveyor.com/api/projects/status/github/KizzyCode/eventbus-tiny-rust?svg=true)](https://ci.appveyor.com/project/KizzyCode/eventbus-tiny-rust)
[![docs.rs](https://docs.rs/eventbus-tiny/badge.svg)](https://docs.rs/eventbus-tiny)
[![crates.io](https://img.shields.io/crates/v/eventbus-tiny.svg)](https://crates.io/crates/eventbus-tiny)
[![Download numbers](https://img.shields.io/crates/d/eventbus-tiny.svg)](https://crates.io/crates/eventbus-tiny)
[![dependency status](https://deps.rs/crate/eventbus-tiny/latest/status.svg)](https://deps.rs/crate/eventbus-tiny)


# `eventbus-tiny`
Welcome to `eventbus-tiny` 🎉

`eventbus-tiny` is a small, dependency-free, no-`unsafe` crate that provides a multi-producer broadcast-consumer event
bus for arbitrary event types (as long as they are `Send`).


## Implementation and Locking
The implementation nearly lock-free, only requiring a write-lock if the registry changes (i.e. if a new subscriber is
added, or `shrink_to_fit` is called) - everything else uses either a cooperative read-lock, or is completely lockfree
via the underlying [`std::sync::mpsc`]-channels.

This approach provides a reasonable compromise between ease-of-implementation (no dependencies, no `unsafe` code,
well-known semantics of built-in types), and performance with lock-free operation in all critical hot-zones like event
publication and awaiting/receiving events.


## Example
```rust
use eventbus_tiny::EventBus;
use eventbus_tiny::Subscription;

// Create bus and subscribers
let bus = EventBus::new();
let subscriber_a: Subscription<usize> = bus.subscribe(64);
let subscriber_b: Subscription<usize> = bus.subscribe(64);

// Publish some events and ensure they get delivered to all two subscribers
assert_eq!(bus.publish(1usize), 2);
assert_eq!(bus.publish(4usize), 2);
assert_eq!(bus.publish(7usize), 2);

// Receive events
assert_eq!(subscriber_a.recv(), Ok(1usize));
assert_eq!(subscriber_a.recv(), Ok(4usize));
assert_eq!(subscriber_a.recv(), Ok(7usize));

assert_eq!(subscriber_b.recv(), Ok(1usize));
assert_eq!(subscriber_b.recv(), Ok(4usize));
assert_eq!(subscriber_b.recv(), Ok(7usize));

// Drop bus and assert the subscribers are dead
drop(bus);
assert!(subscriber_a.recv().is_err());
assert!(subscriber_b.recv().is_err());
```
