use std::{
    marker,
    mem,
    ptr
};

///Converts data to a byte array.
///
///# Note:
///
///This function limits its usage to data that implements `marker::Copy`
///
///But it does not guarantee that all types with such trait will be correctly converted.
///
///# Parameters:
///
///* `data` - Arbitrary data that can be `memcpy`
///
///# Result:
///
///* `Vec` - Allocated with size equal to `size_of::<data>`.
pub fn to_bytes<T: marker::Copy>(data: T) -> Vec<u8> {
    let len = mem::size_of::<T>();
    let mut result: Vec<u8> = vec![0; len];

    unsafe {
        ptr::copy_nonoverlapping(&data as *const _ as *const u8, result.as_mut_ptr(), len);
    }

    result
}

///Converts data to a byte array by writing it into mutable slice
///
///# Note:
///
///This function limits its usage to data that implements `marker::Copy`
///
///But it does not guarantee that all types with such trait will be correctly converted.
///
///# Parameters:
///
///* `data` - Arbitrary data that can be `memcpy`
///* `to` - Byte slice where to copy
///
///# Result:
///
///* `Ok` - Success.
///* `Err` - Insufficient slice size.
pub fn copy_to_bytes<T: marker::Copy>(data: T, to: &mut [u8]) -> Result<(), ()> {
    let len = mem::size_of::<T>();

    if to.len() < len {
        return Err(());
    }

    unsafe {
        ptr::copy_nonoverlapping(&data as *const _ as *const u8, to.as_mut_ptr(), len);
    }

    Ok(())
}

///Trait to provide `to_bytes` method for a arbitrary data.
///
///This trait is implemented for a basic integer that can be safely converted.
pub unsafe trait ToBytesCast : marker::Copy {
    fn to_bytes(&self) -> Vec<u8>;
    fn copy_to_bytes(&self, to: &mut [u8]) -> Result<(), ()>;
    fn get_byte(&self, index: usize) -> Option<u8>;
}

macro_rules! impl_to_traits
{
    ($($t:ty), +) => {
        $(
            unsafe impl ToBytesCast for $t {
                #[inline]
                fn to_bytes(&self) -> Vec<u8> {
                    to_bytes(*self)
                }

                #[inline]
                fn copy_to_bytes(&self, to: &mut [u8]) -> Result<(), ()> {
                    copy_to_bytes(*self, to)
                }

                fn get_byte(&self, index: usize) -> Option<u8> {
                    let integer = *self as u64;

                    if index >= mem::size_of::<$t>(){
                        return None;
                    }

                    Some( (integer >> (index * 8) & 0xff) as u8 )
                }
            }
        )+
    };
}

impl_to_traits!(u64, u32, u16, u8, usize, i64, i32, i16, i8, isize, f32, f64);
