use crate::Pod;

use core::{mem, marker};

//To avoid unaligned read, wait until `ptr::read_unaligned` is stable to remove it
#[repr(packed(1))]
struct Out<T>(pub T);

#[derive(Copy, Clone)]
///Byte slice reader.
///
///## Usage
///
///```
///use lazy_bytes_cast::Read;
///
///const DATA: &[u8] = &[0u8, 255, 255, 255, 1];
///const OUT: u32 = match Read::<u32>::new(DATA).read() {
///   Some(out) => out,
///   None => unreachable!()
///};
///const OUT2: u32 = match Read::<u32>::new(DATA).advance(4).read() {
///   Some(_) => unreachable!(),
///   None => 0,
///};
///
///assert_eq!(OUT, u32::from_ne_bytes([0u8, 255, 255, 255]));
///assert_eq!(OUT2, 0);
///```
pub struct Read<'a, OUT> {
    bytes: &'a [u8],
    cursor: usize,
    _out: marker::PhantomData<OUT>
}

impl<'a, OUT: Pod> Read<'a, OUT> {
    ///Creates new instance
    pub const fn new(bytes: &'a [u8]) -> Self {
        Self {
            bytes,
            cursor: 0,
            _out: marker::PhantomData,
        }
    }

    const fn internal_read_unchecked(&self) -> &Out<OUT> {
        unsafe {
            let ptr = self.bytes.as_ptr().add(self.cursor) as *const _;
            &*ptr
        }
    }

    #[inline(always)]
    ///Read from current position without bounds checks
    pub const unsafe fn read_unchecked(&self) -> OUT {
        self.internal_read_unchecked().0
    }

    #[inline(always)]
    ///Read from current position without bounds checks
    pub const fn read(&self) -> Option<OUT> {
        if self.bytes.len().saturating_sub(self.cursor) >= mem::size_of::<OUT>() {
            unsafe {
                Some(self.read_unchecked())
            }
        } else {
            None
        }
    }

    #[inline(always)]
    ///Moves cursor position forward.
    pub const fn advance(mut self, add: usize) -> Self {
        self.cursor = self.cursor.saturating_add(add);
        self
    }

    #[inline(always)]
    ///Returns number of elements available for `read`
    pub const fn remaining(&self) -> usize {
        let remain = self.bytes.len().saturating_sub(self.cursor);
        remain / mem::size_of::<OUT>()
    }
}

impl<'a, OUT: Pod> Iterator for Read<'a, OUT> {
    type Item = OUT;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        match self.read() {
            Some(res) => {
                *self = self.advance(mem::size_of::<OUT>());
                Some(res)
            },
            None => None
        }
    }

    #[inline(always)]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.remaining();
        (len, Some(len))
    }
}
