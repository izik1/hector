//! Hector is a hex encoding library,
//! persuing the library trifecta of fast, free, and easy to use.

#![no_std]
#![warn(let_underscore_drop, noop_method_call)]
#![warn(clippy::must_use_candidate)]
#![deny(unreachable_pub)]

#[cfg(feature = "alloc")]
extern crate alloc;

// Assume we have `std` in tests.
#[cfg(any(feature = "std", test))]
extern crate std;

mod decode;
mod encode;

pub use decode::Error as DecodeError;
pub use encode::Error as EncodeError;

pub use encode::{encode_to_array, encode_to_array_upper, encode_to_slice, encode_to_slice_upper};

#[cfg(feature = "alloc")]
pub use encode::{encode, encode_upper};

#[cfg(feature = "alloc")]
pub use decode::decode;
