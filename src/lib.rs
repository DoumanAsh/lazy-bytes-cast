//!Lazy utilities to work with bits and bytes.

#![warn(missing_docs)]
#![cfg_attr(feature = "cargo-clippy", allow(clippy::style))]
#![no_std]

mod bits;
pub use bits::Bits;
mod convert;
pub use convert::*;
mod read;
pub use read::*;
