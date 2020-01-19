//!This crate provides simple methods to cast from and into byte arrays.
//!
//!# Example
//!
//!```rust
//!
//! use lazy_bytes_cast::{FromByteArray, IntoByteArray, ByteSliceAsType, AsByteSlice};
//!
//! let val = 9999999u32;
//! let bytes = [127u8, 150, 152, 0];
//! assert_eq!(val.as_slice(), bytes);
//! assert_eq!(val.into_byte_array(), bytes);
//!
//! assert_eq!(*bytes.as_type::<u32>().unwrap(), val);
//! assert_eq!(u32::from_byte_array(bytes), val);
//!```

#![warn(missing_docs)]
#![cfg_attr(feature = "cargo-clippy", allow(clippy::style))]
#![no_std]

pub mod array;
pub mod slice;

pub use slice::{ByteSliceAsType, AsByteSlice};
pub use array::{FromByteArray, IntoByteArray};
