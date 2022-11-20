#![no_std]
#![warn(clippy::must_use_candidate)]

#[cfg(feature = "alloc")]
extern crate alloc;

// assume we have `std` in tests.
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