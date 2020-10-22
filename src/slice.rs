use core::{mem, slice};

///Allow to borrow type as slice of bytes
pub unsafe trait AsBytes: Sized {
    #[inline(always)]
    ///Access self as slice of bytes
    fn as_bytes(&self) -> &[u8] {
        unsafe {
            slice::from_raw_parts(self as *const _ as *const _, mem::size_of::<Self>())
        }
    }

    #[inline(always)]
    ///Access self as mutable slice of bytes
    fn as_bytes_mut(&mut self) -> &[u8] {
        unsafe {
            slice::from_raw_parts_mut(self as *mut _ as *mut _, mem::size_of::<Self>())
        }
    }
}

unsafe impl<T: crate::IntoBytes> AsBytes for T {
}

///Utility to read values from sequence of bytes
pub trait ReadBytes {
    ///Reads bytes from self, converting it to the value.
    ///
    ///Modifies bytes sequence, offsetting cursor by the size of value.
    ///
    ///When there is not enough bytes, returns `None`
    fn read_value<T: crate::FromBytes>(&mut self) -> Option<T>;

    ///Unchecked version of `read_value`, that doesn't perform length check.
    ///
    ///Should panic if size is insufficient.
    fn read_value_unchecked<T: crate::FromBytes>(&mut self) -> T;
}

impl ReadBytes for &[u8] {
    #[inline]
    fn read_value<T: crate::FromBytes>(&mut self) -> Option<T> {
        if self.len() < mem::size_of::<T::Array>() {
            return None;
        }

        Some(Self::read_value_unchecked(self))
    }

    fn read_value_unchecked<T: crate::FromBytes>(&mut self) -> T {
        debug_assert!(self.len() >= mem::size_of::<T::Array>());
        let val_ptr = self.as_ptr() as *const T::Array;
        *self = &self[mem::size_of::<T::Array>()..];

        //This is fine because we know that slice has enough bytes.
        //We'll panic on slicing above if size is insufficient so UB cannot happen here.
        unsafe {
            T::from_bytes(*val_ptr)
        }
    }
}
