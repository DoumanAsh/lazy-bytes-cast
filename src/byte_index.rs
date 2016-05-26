use std::{
    mem
};

/// Returns slice of bytes from integer
pub trait ByteSlice {
    /// Returns read-only slice over integer bytes
    fn byte_slice(&self) -> &[u8];
    /// Returns mutable slice over integer bytes
    fn byte_mut_slice(&mut self) -> &mut [u8];
}

/// Provides indexing over integer bytes
pub trait ByteIndex : ByteSlice {
    /// Returns byte from integer by index.
    ///
    ///# Parameters:
    ///
    ///* `idx` - Index of byte starting from 0.
    ///
    ///# Result:
    ///
    ///* `Some` - Contains byte.
    ///* `None` - Index out of bounds.
    fn byte(&self, idx: usize) -> Option<u8>;
}

macro_rules! impl_index_trait1
{
    ($($t:ty), +) => {
        $(
            impl ByteSlice for $t {
                fn byte_slice(&self) -> &[u8] {
                    let bytes: &[u8; 1] = unsafe { mem::transmute(self) };
                    &bytes[..]
                }

                fn byte_mut_slice(&mut self) -> &mut [u8] {
                    let bytes: &mut [u8; 1] = unsafe { mem::transmute(self) };
                    &mut bytes[..]
                }
            }

            impl ByteIndex for $t {
                fn byte(&self, idx: usize) -> Option<u8> {
                    if idx >= mem::size_of::<$t>() {
                        return None;
                    }

                    Some(self.byte_slice()[idx])
                }
            }
        )+
    };
}

macro_rules! impl_index_trait2
{
    ($($t:ty), +) => {
        $(
            impl ByteSlice for $t {
                fn byte_slice(&self) -> &[u8] {
                    let bytes: &[u8; 2] = unsafe { mem::transmute(self) };
                    &bytes[..]
                }

                fn byte_mut_slice(&mut self) -> &mut [u8] {
                    let bytes: &mut [u8; 2] = unsafe { mem::transmute(self) };
                    &mut bytes[..]
                }
            }

            impl ByteIndex for $t {
                fn byte(&self, idx: usize) -> Option<u8> {
                    if idx >= mem::size_of::<$t>() {
                        return None;
                    }

                    Some(self.byte_slice()[idx])
                }
            }
        )+
    };
}

macro_rules! impl_index_trait4
{
    ($($t:ty), +) => {
        $(
            impl ByteSlice for $t {
                fn byte_slice(&self) -> &[u8] {
                    let bytes: &[u8; 4] = unsafe { mem::transmute(self) };
                    &bytes[..]
                }

                fn byte_mut_slice(&mut self) -> &mut [u8] {
                    let bytes: &mut [u8; 4] = unsafe { mem::transmute(self) };
                    &mut bytes[..]
                }
            }
            impl ByteIndex for $t {
                fn byte(&self, idx: usize) -> Option<u8> {
                    if idx >= mem::size_of::<$t>() {
                        return None;
                    }

                    Some(self.byte_slice()[idx])
                }
            }
        )+
    };
}

macro_rules! impl_index_trait8
{
    ($($t:ty), +) => {
        $(
            impl ByteSlice for $t {
                fn byte_slice(&self) -> &[u8] {
                    let bytes: &[u8; 8] = unsafe { mem::transmute(self) };
                    &bytes[..]
                }

                fn byte_mut_slice(&mut self) -> &mut [u8] {
                    let bytes: &mut [u8; 8] = unsafe { mem::transmute(self) };
                    &mut bytes[..]
                }
            }
            impl ByteIndex for $t {
                fn byte(&self, idx: usize) -> Option<u8> {
                    if idx >= mem::size_of::<$t>(){
                        return None;
                    }

                    Some(self.byte_slice()[idx])
                }
            }
        )+
    };
}

impl_index_trait1!(i8, u8);
impl_index_trait2!(i16, u16);
impl_index_trait4!(i32, u32, f32);
impl_index_trait8!(i64, u64, f64);
