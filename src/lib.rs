/*!
This crate provides simple methods to cast from and into byte arrays.

# Note

The crates will not take care of byte order for you. Cuz lazy.

*/

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
extern crate core;

use core::{
    mem,
    marker,
    ptr,
    slice as std_slice
};

pub mod to;
pub mod from;
pub mod slice;
pub mod array;
