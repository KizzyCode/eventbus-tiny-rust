#![doc = include_str!("../README.md")]
#![deny(unsafe_code)]
// Clippy lints
#![warn(clippy::large_stack_arrays)]
#![warn(clippy::arithmetic_side_effects)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::indexing_slicing)]
#![warn(clippy::panic)]
#![warn(clippy::todo)]
#![warn(clippy::unimplemented)]
#![warn(clippy::unreachable)]
#![warn(clippy::allow_attributes_without_reason)]
#![warn(clippy::cognitive_complexity)]

mod bus;
mod macros;
mod sub;

// Export exposed types
pub use crate::bus::EventBus;
pub use crate::sub::Subscription;
