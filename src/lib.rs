//!Lazy utilities to work with bits and bytes.

#![warn(missing_docs)]
#![allow(clippy::style)]
#![no_std]

mod bits;
pub use bits::Bits;
mod convert;
pub use convert::*;
mod read;
pub use read::*;
