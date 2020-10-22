//!This crate provides simple methods to cast from and into byte arrays.
//!
//!# Example
//!
//!```rust
//!
//! use lazy_bytes_cast::{FromBytes, IntoBytes, read_u32, read_u64, read_u128, read_u16, ReadBytes};
//!
//! let val = 9999999u32;
//! let bytes = [127u8, 150, 152, 0];
//! let mut bytes_slice = &bytes[..];
//! assert_eq!(val.into_bytes(), bytes);
//!
//! assert_eq!(u32::from_bytes(bytes), val);
//! assert_eq!(u32::from_bytes(bytes), read_u32(&bytes, 0));
//! assert_eq!(u32::from_bytes(bytes), bytes_slice.read_value::<u32>().unwrap());
//! assert_eq!(bytes_slice.len(), 0);
//! assert!(bytes_slice.read_value::<u32>().is_none());
//!
//! assert_eq!(read_u16(&u16::max_value().to_ne_bytes(), 0), u16::max_value());
//! assert_eq!(read_u64(&u64::max_value().to_ne_bytes(), 0), u64::max_value());
//! assert_eq!(read_u128(&u128::max_value().to_ne_bytes(), 0), u128::max_value());
//!```

#![warn(missing_docs)]
#![cfg_attr(feature = "cargo-clippy", allow(clippy::style))]
#![no_std]

mod array;
mod slice;

pub use array::{FromBytes, IntoBytes};
pub use slice::{ReadBytes, AsBytes};

#[inline(always)]
///Read u16 at compile time.
///
///`cursor` allows to specify position in slice, allowing avoid the need for slicing
pub const fn read_u16(input: &[u8], cursor: usize) -> u16 {
    u16::from_ne_bytes([input[cursor], input[cursor + 1]])
}

#[inline(always)]
///Read u32 at compile time.
///
///`cursor` allows to specify position in slice, allowing avoid the need for slicing
pub const fn read_u32(input: &[u8], cursor: usize) -> u32 {
    u32::from_ne_bytes([input[cursor], input[cursor + 1], input[cursor + 2], input[cursor + 3]])
}

#[inline(always)]
///Read u64 at compile time.
///
///`cursor` allows to specify position in slice, allowing avoid the need for slicing
pub const fn read_u64(input: &[u8], cursor: usize) -> u64 {
    u64::from_ne_bytes([
        input[cursor], input[cursor + 1], input[cursor + 2], input[cursor + 3],
        input[cursor + 4], input[cursor + 5], input[cursor + 6], input[cursor + 7],
    ])
}

#[inline(always)]
///Read u128 at compile time.
///
///`cursor` allows to specify position in slice, allowing avoid the need for slicing
pub const fn read_u128(input: &[u8], cursor: usize) -> u128 {
    u128::from_ne_bytes([
        input[cursor], input[cursor + 1], input[cursor + 2], input[cursor + 3],
        input[cursor + 4], input[cursor + 5], input[cursor + 6], input[cursor + 7],
        input[cursor + 8], input[cursor + 9], input[cursor + 10], input[cursor + 11],
        input[cursor + 12], input[cursor + 13], input[cursor + 14], input[cursor + 15],
    ])
}
