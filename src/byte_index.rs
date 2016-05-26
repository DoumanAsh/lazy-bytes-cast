use std::{
    mem
};

/// Provides indexing over integer bytes
pub trait ByteIndex {
    fn byte(&self, idx: usize) -> Option<u8>;
}

macro_rules! impl_index_trait1
{
    ($($t:ty), +) => {
        $(
            impl ByteIndex for $t {
                fn byte(&self, idx: usize) -> Option<u8> {
                    if idx >= mem::size_of::<$t>(){
                        return None;
                    }

                    let bytes: [u8; 1] = unsafe { mem::transmute(*self) };

                    Some(bytes[idx])
                }
            }
        )+
    };
}

macro_rules! impl_index_trait2
{
    ($($t:ty), +) => {
        $(
            impl ByteIndex for $t {
                fn byte(&self, idx: usize) -> Option<u8> {
                    if idx >= mem::size_of::<$t>(){
                        return None;
                    }

                    let bytes: [u8; 2] = unsafe { mem::transmute(*self) };

                    Some(bytes[idx])
                }
            }
        )+
    };
}

macro_rules! impl_index_trait4
{
    ($($t:ty), +) => {
        $(
            impl ByteIndex for $t {
                fn byte(&self, idx: usize) -> Option<u8> {
                    if idx >= mem::size_of::<$t>(){
                        return None;
                    }

                    let bytes: [u8; 4] = unsafe { mem::transmute(*self) };

                    Some(bytes[idx])
                }
            }
        )+
    };
}

macro_rules! impl_index_trait8
{
    ($($t:ty), +) => {
        $(
            impl ByteIndex for $t {
                fn byte(&self, idx: usize) -> Option<u8> {
                    if idx >= mem::size_of::<$t>(){
                        return None;
                    }

                    let bytes: [u8; 8] = unsafe { mem::transmute(*self) };

                    Some(bytes[idx])
                }
            }
        )+
    };
}

impl_index_trait1!(i8, u8);
impl_index_trait2!(i16, u16);
impl_index_trait4!(i32, u32, f32);
impl_index_trait8!(i64, u64, f64);
