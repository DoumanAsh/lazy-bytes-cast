# lazy-bytes-cast

[![Build](https://github.com/DoumanAsh/lazy-bytes-cast/workflows/Rust/badge.svg)](https://github.com/DoumanAsh/lazy-bytes-cast/actions?query=workflow%3ARust)
[![Crates.io](https://img.shields.io/crates/v/lazy-bytes-cast.svg)](https://crates.io/crates/lazy-bytes-cast)
[![Docs.rs](https://docs.rs/lazy-bytes-cast/badge.svg)](https://docs.rs/crate/lazy-bytes-cast/)

This crate provides simple methods to cast from and into byte arrays.

# Example

```rust
use lazy_bytes_cast::{FromBytes, IntoBytes, read_u32, read_u64, read_u128, read_u16, ReadBytes};

let val = 9999999u32;
let bytes = [127u8, 150, 152, 0];
let mut bytes_slice = &bytes[..];
assert_eq!(val.into_bytes(), bytes);

assert_eq!(u32::from_bytes(bytes), val);
assert_eq!(u32::from_bytes(bytes), read_u32(&bytes, 0));
assert_eq!(u32::from_bytes(bytes), bytes_slice.read_value::<u32>().unwrap());
assert_eq!(bytes_slice.len(), 0);
assert!(bytes_slice.read_value::<u32>().is_none());

assert_eq!(read_u16(&u16::max_value().to_ne_bytes(), 0), u16::max_value());
assert_eq!(read_u64(&u64::max_value().to_ne_bytes(), 0), u64::max_value());
assert_eq!(read_u128(&u128::max_value().to_ne_bytes(), 0), u128::max_value());
```
