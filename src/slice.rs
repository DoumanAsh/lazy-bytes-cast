//! Slice conversion module
//!
//! Provides utilities to treat arbitrary types as slice of bytes

use core::{slice, mem};

#[inline]
///Access value as slice of bytes
pub unsafe fn as_slice<T: Sized>(val: &T) -> &[u8] {
    slice::from_raw_parts(val as *const _ as *const _, mem::size_of::<T>())
}

#[inline]
///Access value as mutable slice of bytes
pub unsafe fn as_slice_mut<T: Sized>(val: &mut T) -> &mut [u8] {
    slice::from_raw_parts_mut(val as *mut _ as *mut _, mem::size_of::<T>())
}

#[inline]
///Get reference to the value from slice
pub unsafe fn as_type<T: Sized>(slice: &[u8]) -> Option<&T> {
    if mem::size_of::<T>() == slice.len() {
        Some(as_type_unchecked(slice))
    } else {
        None
    }
}

#[inline]
///Get mutable reference to the value from slice
pub unsafe fn as_type_mut<T: Sized>(slice: &mut [u8]) -> Option<&mut T> {
    if mem::size_of::<T>() == slice.len() {
        Some(as_type_mut_unchecked(slice))
    } else {
        None
    }
}

#[inline]
///Get reference to the value from slice
///
///This function is UB if `slice.len() < mem::size_of::<T>()`
pub unsafe fn as_type_unchecked<T: Sized>(slice: &[u8]) -> &T {
    &*(slice.as_ptr() as *const T)
}

#[inline]
///Get mutable reference to the value from slice
///
///This function is UB if `slice.len() < mem::size_of::<T>()`
pub unsafe fn as_type_mut_unchecked<T: Sized>(slice: &mut [u8]) -> &mut T {
    &mut *(slice.as_mut_ptr() as *mut T)
}

/// Trait which should be implemented for types that are safe to treat as byte
///
/// While it is possible to consider all types as bytes, it doesn't make sense for some (e.g.  `Vec`)
pub unsafe trait AsByteSlice: Sized {
    #[inline]
    ///Access value as slice of bytes
    fn as_slice(&self) -> &[u8] {
        unsafe {
            as_slice(self)
        }
    }

    #[inline]
    ///Access value as mutable slice of bytes
    fn as_slice_mut(&mut self) -> &mut [u8] {
        unsafe {
            as_slice_mut(self)
        }
    }
}

/// Trait, which implements accessing type as reference from byte slice.
///
/// This is safe as long as type implements `AsByteSlice` correctly.
pub trait ByteSliceAsType {
    ///Gets reference
    fn as_type<T: AsByteSlice>(&self) -> Option<&T>;
    ///Gets mutable reference
    fn as_type_mut<T: AsByteSlice>(&mut self) -> Option<&mut T>;
}

impl ByteSliceAsType for [u8] {
    #[inline]
    fn as_type<T: AsByteSlice>(&self) -> Option<&T> {
        unsafe {
            as_type(self)
        }
    }

    #[inline]
    fn as_type_mut<T: AsByteSlice>(&mut self) -> Option<&mut T> {
        unsafe {
            as_type_mut(self)
        }
    }
}

macro_rules! impl_trait {
    ($($type:ty,)+) => {
        $(
            unsafe impl AsByteSlice for $type {}
        )+
    }
}

impl_trait!(u8, i8, u16, i16, u32, i32, f32, u64, i64, f64, usize, isize, i128, u128,);
