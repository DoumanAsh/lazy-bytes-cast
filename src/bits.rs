//! Bits manipulation

use core::ops::{BitAnd, Shl, BitOr, BitXor, Not};

///Bits manipulation interface.
///
///All operations panic on overflow.
///
///```
///use lazy_bytes_cast::Bits;
///
///let mut lolka = 0u128;
///
///assert!(!lolka.bit(1));
///lolka.set_bit(1, true);
///assert!(lolka.bit(1));
///
///assert!(!lolka.bit(0));
///lolka.set_bit(0, true);
///assert!(lolka.bit(0));
///
///assert!(!lolka.bit(127));
///lolka.toggle_bit(127);
///assert!(lolka.bit(127));
///lolka.toggle_bit(127);
///assert!(!lolka.bit(127));
///
///assert_ne!(lolka, 0);
///lolka.reset_bits();
///assert_eq!(lolka, 0);
///```
///
///All operations panic on overflow via math ops overflow.
///
///```should_panic
///use lazy_bytes_cast::Bits;
///
///let mut lolka = 0u8;
///
///lolka.set_bit(9, true);
///```
pub trait Bits: Copy + Sized + Not<Output=Self> + BitXor<Output=Self> + BitOr<Output=Self> + BitAnd<Output=Self> + Shl<Output=Self> + From<u8> + Eq {
    #[inline]
    ///Gets bit by index
    fn bit(&self, idx: u8) -> bool {
        self.bitand(Self::from(1u8).shl(Self::from(idx))) != Self::from(0u8)
    }

    #[inline]
    ///Gets bit by index
    fn toggle_bit(&mut self, idx: u8) {
        *self = self.bitxor(Self::from(1u8).shl(Self::from(idx)))
    }

    #[inline(always)]
    ///Sets all bits to `false`
    fn reset_bits(&mut self) {
        *self = Self::from(0u8)
    }

    #[inline]
    ///Sets bit by index
    fn set_bit(&mut self, idx: u8, value: bool) {
        //(self & !(1 << idx)) | (value << idx);

        *self = self.bitand(
            Self::from(1u8).shl(Self::from(idx)).not()
        ).bitor(
            Self::from(value as u8).shl(Self::from(idx))
        )
    }
}

macro_rules! impl_trait {
    ($($type:ty,)+) => {
        $(
            impl Bits for $type {
            }

        )+
    }
}

impl_trait!(u8, u16, u32, u64, i64, usize, u128,);
