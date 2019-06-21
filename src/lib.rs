//!This crate provides simple methods to cast from and into byte arrays.
//!
//!# Note
//!
//!The crates will not take care of byte order for you. Cuz lazy.
//!## Usage
//!
//!### Slice integer as bytes
//!
//!```rust
//!use lazy_bytes_cast::slice::{
//!   ByteSlice,
//!   ByteIndex
//!};
//!
//!fn main() {
//!    let some_int = 666;
//!
//!    for (idx, byte) in some_int.byte_slice().iter().enumerate() {
//!        assert_eq!(some_int.byte(idx).unwrap(), *byte);
//!        println!("bytes[{}]={}", idx, byte);
//!    }
//!}
//!```
//!
//!### Cast bytes to integer
//!
//!```rust
//!use lazy_bytes_cast::from::{
//!   bytes_cast
//!};
//!
//!fn main() {
//!    let bytes = vec![127, 150, 152, 0];
//!
//!    if let Some(int) = bytes_cast::<u32>(&bytes) {
//!        println!("bytes={}", int);
//!    }
//!    else {
//!        println!("Couldn't extract integer from bytes");
//!    }
//!}
//!```
//!
//!### Cast integer to bytes
//!
//!```rust
//!use lazy_bytes_cast::to::{
//!   ToBytesCast
//!};
//!
//!fn main() {
//!    let some_int = 666;
//!
//!    let bytes = some_int.to_bytes();
//!
//!    println!("bytes={:?}", bytes);
//!}
//!```

#![cfg_attr(not(feature = "std"), no_std)]

pub mod to;
pub mod from;
pub mod slice;
pub mod array;
