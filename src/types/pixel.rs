use fixed::{traits::Fixed, types::{extra as fixed_extra}};

pub trait PixelLike: Sized {
  /// BITS in the underlying representation
  /// if BITS is 0, the type is floating point
  const BITS: usize = 0;
  const SHIFT: usize;
  const ZERO: Self;
  const ONE: Self;
  fn from_f64(x: f64) -> Self;
  fn to_f64(self) -> f64;

  fn ipart(self) -> i64;
  fn round(self) -> Self;
  fn frac(self) -> Self;
}

macro_rules! impl_pixel_like_float {
  ($($ty:ty)+) => {
    $(
      impl PixelLike for $ty {
        const BITS: usize = 0;
        const SHIFT: usize = usize::MAX; // not used
        const ZERO: Self = 0.0;
        const ONE: Self = 1.0;
        fn from_f64(x: f64) -> Self { x as _ }
        fn to_f64(self) -> f64 { self as f64 }

        fn ipart(self) -> i64 { self as i64 }
        fn round(self) -> Self { self.round_ties_even() }
        fn frac(self) -> Self { self.fract() }
      }
    )+
  };
}

impl_pixel_like_float!(f32 f64);

trait CheckFixedFrac<U, const SHIFT: usize> { }
impl CheckFixedFrac<fixed_extra::U8, 8> for () { }
impl CheckFixedFrac<fixed_extra::U16, 16> for () { }

#[derive(Copy, Clone, Debug, PartialEq, Hash, PartialOrd, Ord, Eq)]
pub struct IFixed<T, const SHIFT: usize>(pub T);
pub type I64F8 = IFixed<fixed::types::I56F8, 8>;
pub type I64F16 = IFixed<fixed::types::I48F16, 16>;
pub type U64F8 = IFixed<fixed::types::U56F8, 8>;
pub type U64F16 = IFixed<fixed::types::U48F16, 16>;

impl<U, const SHIFT: usize> PixelLike for IFixed<U, SHIFT>
where
  U: Fixed,
  (): CheckFixedFrac<U::Frac, SHIFT>,
{
  const BITS: usize = <U::Bits as fixed::traits::FixedBits>::BITS as usize;
  const SHIFT: usize = 8;
  const ZERO: Self = Self(fixed::traits::Fixed::ZERO);
  const ONE: Self = Self(fixed::traits::Fixed::TRY_ONE.unwrap());
  fn from_f64(x: f64) -> Self {
    Self(fixed::traits::Fixed::from_num(x))
  }
  fn to_f64(self) -> f64 { self.0.to_num() }

  fn ipart(self) -> i64 { self.0.int().to_num::<fixed::types::I64F0>().to_bits() }
  fn round(self) -> Self {
    Self(self.0.saturating_round_ties_even())
  }
  fn frac(self) -> Self {
    Self(self.0.frac())
  }
}

#[cfg(test)]
mod test {
  use super::*;
  #[test]
  fn test_fixed() {
    let a = I64F8::from_f64(1.5);
    assert_eq!(a.ipart(), 1);
    assert_eq!(a.frac().to_f64(), 0.5);
    assert_eq!(a.round().to_f64(), 2.0);

    let b = I64F16::from_f64(1.5);
    assert_eq!(b.ipart(), 1);
    assert_eq!(b.frac().to_f64(), 0.5);
    assert_eq!(b.round().to_f64(), 2.0);

    let c = U64F8::from_f64(1.5);
    assert_eq!(c.ipart(), 1);
    assert_eq!(c.frac().to_f64(), 0.5);
    assert_eq!(c.round().to_f64(), 2.0);

    let d = U64F16::from_f64(1.5);
    assert_eq!(d.ipart(), 1);
    assert_eq!(d.frac().to_f64(), 0.5);
    assert_eq!(d.round().to_f64(), 2.0);
  }

  #[test]
  fn test_float() {
    let a = 1.5f32;
    assert_eq!(a.ipart(), 1);
    assert_eq!(a.frac(), 0.5);
    assert_eq!(a.round(), 2.0);
  }
}
