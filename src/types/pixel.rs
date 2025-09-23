use fixed::traits::Fixed;

pub trait PixelLike: Sized + PartialEq + std::fmt::Debug + Copy + 'static {
  /// BITS in the underlying representation
  /// if BITS is 0, the type is floating point
  const BITS: usize = 0;
  const IS_SIGNED: bool;
  const SHIFT: usize;
  const ZERO: Self;
  const ONE: Self;
  fn from_f64(x: f64) -> Self;
  fn to_f64(self) -> f64;

  fn ipart(self) -> i64 {
    self.to_f64().floor() as i64
  }
  fn round(self) -> Self {
    Self::from_f64(self.to_f64().round_ties_even())
  }
  fn frac(self) -> Self {
    Self::from_f64(self.to_f64().rem_euclid(1.0))
  }
}

macro_rules! impl_pixel_like_float {
  ($($ty:ty)+) => {
    $(
      impl PixelLike for $ty {
        const BITS: usize = 0;
        const SHIFT: usize = usize::MAX; // not used
        const IS_SIGNED: bool = true;
        const ZERO: Self = 0.0;
        const ONE: Self = 1.0;
        fn from_f64(x: f64) -> Self { x as _ }
        fn to_f64(self) -> f64 { self as f64 }

        fn ipart(self) -> i64 { self.floor() as i64 }
        fn round(self) -> Self { self.round_ties_even() }
        fn frac(self) -> Self { self.rem_euclid(1.0) }
      }
    )+
  };
}

impl_pixel_like_float!(f32 f64);

macro_rules! impl_pixel_like_fixed {
  ($($ty:ident)+) => {
    $(
      impl<U> PixelLike for fixed::$ty<U>
      where
        Self: Fixed,
      {
        const BITS: usize = <<Self as fixed::traits::Fixed>::Bits as fixed::traits::FixedBits>::BITS as usize;
        const SHIFT: usize = <<Self as fixed::traits::Fixed>::Frac as fixed::types::extra::Unsigned>::USIZE;
        const IS_SIGNED: bool = <Self as fixed::traits::Fixed>::IS_SIGNED;
        const ZERO: Self = fixed::traits::Fixed::ZERO;
        const ONE: Self = fixed::traits::Fixed::TRY_ONE.unwrap();
        fn from_f64(x: f64) -> Self {
          <Self as fixed::traits::Fixed>::from_num(x)
        }
        fn to_f64(self) -> f64 { self.to_num() }

        fn ipart(self) -> i64 { self.int().to_num::<fixed::types::I64F0>().to_bits() }
        fn round(self) -> Self {
          <Self as fixed::traits::Fixed>::saturating_round_ties_even(self)
        }
        fn frac(self) -> Self {
          <Self as fixed::traits::Fixed>::frac(self)
        }
      }
    )+
  };
}

impl_pixel_like_fixed!(FixedI64 FixedU64 FixedI128 FixedU128 FixedI32 FixedU32);

#[derive(Copy, Clone, Debug, PartialEq, Hash, PartialOrd, Ord, Eq)]
pub struct IFixed<T, const SHIFT: usize>(pub T);
pub type I64F8 = IFixed<fixed::types::I56F8, 8>;
pub type I64F16 = IFixed<fixed::types::I48F16, 16>;
pub type U64F8 = IFixed<fixed::types::U56F8, 8>;
pub type U64F16 = IFixed<fixed::types::U48F16, 16>;

macro_rules! impl_pixel_like_ifixed {
  ($($ty:ty)+) => {
    $(
      impl<const SHIFT: usize> IFixed<$ty, SHIFT> {
        const FRAC_MASK: $ty = (1 << SHIFT) - 1;
      }

      impl<const SHIFT: usize> PixelLike for IFixed<$ty, SHIFT> {
        const BITS: usize = <$ty>::BITS as usize;
        const SHIFT: usize = SHIFT;
        const IS_SIGNED: bool = <$ty>::MIN != 0;
        const ZERO: Self = Self(0);
        const ONE: Self = Self(1 << SHIFT);
        fn from_f64(x: f64) -> Self {
          Self((x * Self::ONE.0 as f64) as _)
        }
        fn to_f64(self) -> f64 { self.0 as f64 / Self::ONE.0 as f64 }

        fn ipart(self) -> i64 { (self.0 >> SHIFT) as _ }
        fn round(self) -> Self {
          let half = 1 << (SHIFT - 1);
          if (self.0 & Self::FRAC_MASK) == half {
            return Self((self.0 + (self.0 & (1 << SHIFT))) & !(Self::FRAC_MASK))
          }
          Self((self.0 + half) & !Self::FRAC_MASK)
        }
        fn frac(self) -> Self {
          Self(self.0 & Self::FRAC_MASK)
        }
      }
    )+
  };
}

impl_pixel_like_ifixed!(u64 i64 u128 i128 u32 i32);

pub mod types {
  use super::IFixed;
  pub type U56F8 = IFixed<u64, 8>;
  pub type U48F16 = IFixed<u32, 16>;
  pub type I56F8 = IFixed<i64, 8>;
  pub type I48F16 = IFixed<i32, 16>;
  pub type I64F64 = IFixed<i128, 64>;
}


#[cfg(test)]
mod test {
  use assert_approx_eq::assert_approx_eq;

  use super::*;

  macro_rules! test_pixel_like {
    ($($ty:ty)+) => {
      $(
        impl_test_pixel_like(stringify!($ty), <$ty>::ZERO);
      )+
    };
  }

  fn impl_test_pixel_like<T: PixelLike>(name: &str, zero: T) {
    let test_list = [
      1.5, 2.5, 3.5, 4.5, 2.2, 2.3, 2.4, -1.5, -2.5, -3.5, -4.5, -2.2, -2.3, -2.4,
    ];
    let one = T::ONE;
    assert_eq!(zero.to_f64(), 0.0, "{} to_f64 zero", name);
    assert_eq!(one.to_f64(), 1.0, "{} to_f64 one", name);
    assert_eq!(T::from_f64(0.0), zero, "{} from_f64 zero", name);
    assert_eq!(T::from_f64(1.0), one, "{} from_f64 one", name);
    let eps = if T::BITS == 0 {
      1e-6
    } else {
      1.0 / 2f64.powi(T::SHIFT as _)
    };
    assert!(eps < 0.01, "{} eps {}", name, eps);
    for i in test_list {
      if i < 0.0 && !T::IS_SIGNED { continue; }
      let v = T::from_f64(i);
      assert_approx_eq!(v.to_f64(), i, eps);
      let rounded = v.round().to_f64();
      assert_approx_eq!(rounded, i.round_ties_even(), eps);
      assert!((rounded - i).abs() <= 0.5 + eps, "{} round {} -> {rounded}", name, i);
      assert_eq!(rounded as i32 % 2, 0, "{} round {} -> {rounded}", name, i);

      let (ipart, frac) = (v.ipart(), v.frac().to_f64());
      assert_eq!(ipart, i.floor() as i64, "{} ipart {} -> ({ipart}, {frac})", name, i);
      assert!(ipart as f64 <= i, "{} ipart {} -> ({ipart}, {frac})", name, i);
      assert_approx_eq!(frac, i.rem_euclid(1.0), eps);
      assert!(frac >= 0.0 && frac < 1.0, "{} frac {} -> ({ipart}, {frac})", name, i);
      assert_approx_eq!(frac + ipart as f64, i, eps);
    }
  }

  #[test]
  fn test_fixed() {
    use fixed::types::*;
    test_pixel_like!(I56F8 I48F16 U56F8 U48F16);
    test_pixel_like!(I64F64);
  }

  #[test]
  fn test_ifixed() {
    use types::*;
    test_pixel_like!(U56F8 U48F16 I56F8 I48F16 I64F64);
  }

  #[test]
  fn test_float() {
    test_pixel_like!(f32 f64);
  }
}
