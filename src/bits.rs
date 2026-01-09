use core::{mem, fmt, ops, hash};

#[repr(transparent)]
#[derive(Clone, Copy, Default)]
///Wrapper to provide access to bit fields
///
///```
///use lazy_bytes_cast::Bits;
///
///let mut bits = Bits(u32::max_value());
///assert!(!bits.empty());
///
///assert!(bits.get(1));
///
///bits = bits.set(1);
///assert!(bits.get(1));
///
///bits = bits.toggle(1);
///assert!(!bits.get(1));
///
///bits = bits.unset(1);
///assert!(!bits.get(1));
///
///bits = bits.toggle(1);
///assert!(bits.get(1));
///
///bits = bits.reset();
///assert_eq!(bits, 0);
///assert!(bits.empty());
///
///assert!(!bits.get(1));
///
///assert_eq!(bits.len(), core::mem::size_of_val(&bits) * 8);
///
///bits = Bits(u32::min_value());
///assert!(bits.empty());
///assert!(!bits.get(1));
///
///bits = bits.set(1);
///assert!(bits.get(1));
///
///bits = bits.unset(1);
///assert!(!bits.get(1));
///
///bits = bits.set(26);
///assert!(bits.get(26));
///assert!(bits.get(90)); //90 % 32 == 26
///bits = bits.set(91);
///assert!(bits.get(26));
///
///bits = bits.flip();
///assert!(!bits.get(26));
///assert!(bits.get(5));
///```
pub struct Bits<T>(pub T);

macro_rules! impl_bits {
    ($($ty:ident),*) => {$(
        impl Bits<$ty> {
            #[inline(always)]
            ///Get bit by index.
            pub const fn get(&self, idx: u32) -> bool {
                self.0.wrapping_shr(idx) & 1 != 0
            }

            #[must_use]
            #[inline(always)]
            ///Set bit by index and return updated value.
            pub const fn set(self, idx: u32) -> Self {
                const ONE: $ty = 1;

                Self(self.0 | ONE.wrapping_shl(idx))
            }

            #[must_use]
            #[inline(always)]
            ///Unset bit by index and return updated value.
            pub const fn unset(self, idx: u32) -> Self {
                Self(self.0 & !(1 << idx))
            }

            #[must_use]
            #[inline(always)]
            ///Unset bit by index and return updated value.
            pub const fn toggle(self, idx: u32) -> Self {
                Self(self.0 ^ (1 << idx))
            }

            #[inline(always)]
            ///Returns whether all bits are unset
            pub const fn empty(&self) -> bool {
                self.0 == 0
            }

            #[inline(always)]
            ///Unset all bits and returns updated value
            pub const fn reset(&self) -> Self {
                Self(0)
            }

            #[must_use]
            #[inline(always)]
            ///Flip all bits and returns updated value
            pub const fn flip(&self) -> Self {
                Self(self.0.reverse_bits())
            }

            #[inline(always)]
            ///Returns number of bits inside.
            pub const fn len(&self) -> usize {
                Self::size()
            }

            #[inline(always)]
            ///Returns number of bits inside.
            pub const fn size() -> usize {
                mem::size_of::<$ty>() * 8
            }
        }

        impl PartialEq<Bits<$ty>> for $ty {
            #[inline(always)]
            fn eq(&self, other: &Bits<$ty>) -> bool {
                PartialEq::eq(self, &other.0)
            }
        }

        impl fmt::Debug for Bits<$ty> {
            #[inline(always)]
            fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt::Debug::fmt(&self.0, fmt)
            }
        }

        impl fmt::Display for Bits<$ty> {
            #[inline(always)]
            fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt::Display::fmt(&self.0, fmt)
            }
        }

        impl fmt::Binary for Bits<$ty> {
            #[inline(always)]
            fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt::Binary::fmt(&self.0, fmt)
            }
        }

        impl PartialEq<$ty> for Bits<$ty> {
            #[inline(always)]
            fn eq(&self, other: &$ty) -> bool {
                PartialEq::eq(&self.0, other)
            }
        }

        impl ops::Deref for Bits<$ty> {
            type Target = $ty;
            #[inline(always)]
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl ops::DerefMut for Bits<$ty> {
            #[inline(always)]
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }

        impl hash::Hash for Bits<$ty> {
            #[inline(always)]
            fn hash<H: hash::Hasher>(&self, state: &mut H) {
                hash::Hash::hash(&self.0, state)
            }
        }

        impl From<$ty> for Bits<$ty> {
            #[inline(always)]
            fn from(val: $ty) -> Self {
                Self(val)
            }
        }

        impl Into<$ty> for Bits<$ty> {
            #[inline(always)]
            fn into(self) -> $ty {
                self.0
            }
        }
    )*};
}

impl_bits!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);
