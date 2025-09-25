use fixed::traits::Fixed;

pub type Position = i64;

pub trait Arithmetics:
  std::ops::Neg<Output = Self>
  + std::ops::Add<Output = Self>
  + std::ops::Sub<Output = Self>
  + std::ops::Mul<Output = Self>
  + std::ops::Div<Output = Self>
  + std::ops::Shr<usize, Output = Self>
  + std::ops::Shl<usize, Output = Self>
  + std::ops::AddAssign
  + std::ops::SubAssign
  + std::ops::MulAssign
  + std::ops::DivAssign
  + std::ops::Shr<usize, Output = Self>
  + std::ops::Shl<usize, Output = Self>
  + Copy
  + PartialOrd
  + PartialEq
  // + std::ops::Mul<Position, Output = Self>
  // + std::ops::Div<Position, Output = Self>
  + PartialEq<Position>
  + PartialOrd<Position>
  + 'static
{
}

impl<T> Arithmetics for T where
  Self:
  std::ops::Neg<Output = Self>
  + std::ops::Add<Output = Self>
  + std::ops::Sub<Output = Self>
  + std::ops::Mul<Output = Self>
  + std::ops::Div<Output = Self>
  + std::ops::Shr<usize, Output = Self>
  + std::ops::Shl<usize, Output = Self>
  + std::ops::AddAssign
  + std::ops::SubAssign
  + std::ops::MulAssign
  + std::ops::DivAssign
  + std::ops::Shr<usize, Output = Self>
  + std::ops::Shl<usize, Output = Self>
  + Copy
  + PartialOrd
  + PartialEq
  // + std::ops::Mul<Position, Output = Self>
  // + std::ops::Div<Position, Output = Self>
  + PartialEq<Position>
  + PartialOrd<Position>
  + 'static
{
}

pub trait IsSigned {}

pub trait FixedLike: Sized + PartialEq + std::fmt::Debug + Copy + 'static {
  /// BITS in the underlying representation
  /// if BITS is 0, the type is floating point
  type Raw;
  const BITS: usize = 0;
  const IS_SIGNED: bool;
  const SHIFT: usize;
  const ZERO: Self;
  const ONE: Self;
  const EPSILON: Self;
  fn from_f64(x: f64) -> Self;
  fn to_f64(self) -> f64;
  fn from_raw(x: Self::Raw) -> Self;
  fn into_raw(self) -> Self::Raw;

  fn from_fixed<P: FixedLike>(p: P) -> Self {
    Self::from_f64(p.to_f64())
  }

  fn ipart(self) -> Position {
    self.to_f64().floor() as Position
  }
  fn round(self) -> Self {
    Self::from_f64(self.to_f64().round_ties_even())
  }
  fn frac(self) -> Self {
    Self::from_f64(self.to_f64().rem_euclid(1.0))
  }
}

pub trait PixelLike: FixedLike + Arithmetics + IsSigned {
  fn scale<P: FixedLike>(self, p: P) -> Self {
    // TODO improve this
    Self::from_f64(self.to_f64() * p.to_f64())
  }
  fn div_mod_floor<P: FixedLike, const TARGET_SHIFT: usize>(self, p: P) -> (Self, Self) {
    // TODO improve this
    // a = d * scale * (p / scale) + r
    let scale = 2f64.powi(TARGET_SHIFT as i32);
    let a = self.to_f64();
    let p = p.to_f64() / scale;
    // warn!("div_mod_floor: a: {:.5}, p: {:.5} => {:.5} {:.5}", a, p, a.div_euclid(p) / scale, a.rem_euclid(p));
    (Self::from_f64(a.div_euclid(p) / scale), Self::from_f64(a.rem_euclid(p)))
  }
}

macro_rules! impl_pixel_like_float {
  ($($ty:ty)+) => {
    $(
      impl FixedLike for $ty {
        type Raw = $ty;
        const BITS: usize = 0;
        const SHIFT: usize = usize::MAX; // not used
        const IS_SIGNED: bool = true;
        const ZERO: Self = 0.0;
        const ONE: Self = 1.0;
        const EPSILON: Self = <$ty>::EPSILON;
        fn from_f64(x: f64) -> Self { x as _ }
        fn to_f64(self) -> f64 { self as f64 }
        fn from_raw(x: Self::Raw) -> Self { x }
        fn into_raw(self) -> Self::Raw { self }

        fn ipart(self) -> Position { self.floor() as _ }
        fn round(self) -> Self { self.round_ties_even() }
        fn frac(self) -> Self { self.rem_euclid(1.0) }
      }

      impl IsSigned for $ty {}
    )+
  };
}

impl_pixel_like_float!(f32 f64);

macro_rules! impl_pixel_like_fixed {
  ($($ty:ident)+) => {
    $(
      impl<U> FixedLike for fixed::$ty<U>
      where
        Self: fixed::traits::Fixed,
      {
        type Raw = <Self as fixed::traits::Fixed>::Bits;
        const BITS: usize = <<Self as fixed::traits::Fixed>::Bits as fixed::traits::FixedBits>::BITS as usize;
        const SHIFT: usize = <<Self as fixed::traits::Fixed>::Frac as fixed::types::extra::Unsigned>::USIZE;
        const IS_SIGNED: bool = <Self as fixed::traits::Fixed>::IS_SIGNED;
        const ZERO: Self = fixed::traits::Fixed::ZERO;
        const ONE: Self = fixed::traits::Fixed::TRY_ONE.unwrap();
        const EPSILON: Self = <Self as fixed::traits::Fixed>::DELTA;
        fn from_f64(x: f64) -> Self {
          <Self as fixed::traits::Fixed>::from_num(x)
        }
        fn to_f64(self) -> f64 {
          <Self as fixed::traits::Fixed>::to_num(self)
        }
        fn from_raw(x: Self::Raw) -> Self {
          <Self as fixed::traits::Fixed>::from_bits(x)
        }
        fn into_raw(self) -> Self::Raw {
          <Self as fixed::traits::Fixed>::to_bits(self)
        }

        fn ipart(self) -> Position { self.int().to_num::<fixed::types::I64F0>().to_bits() as _ }
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

impl<U> IsSigned for fixed::FixedI32<U> where Self: Fixed {}
impl<U> IsSigned for fixed::FixedI64<U> where Self: Fixed {}
impl<U> IsSigned for fixed::FixedI128<U> where Self: Fixed {}
impl<U> PixelLike for fixed::FixedI32<U> where Self: Fixed {}
impl<U> PixelLike for fixed::FixedI64<U> where Self: Fixed {}
impl<U> PixelLike for fixed::FixedI128<U> where Self: Fixed {}

#[derive(Copy, Clone, Debug, PartialEq, Hash, PartialOrd, Ord, Eq)]
pub struct IFixed<T, const SHIFT: usize>(pub T);

macro_rules! impl_pixel_like_ifixed {
  ($($ty:ty)+) => {
    $(
      impl<const SHIFT: usize> IFixed<$ty, SHIFT> {
        const FRAC_MASK: $ty = (1 << SHIFT) - 1;
      }

      impl<const SHIFT: usize> FixedLike for IFixed<$ty, SHIFT> {
        type Raw = $ty;
        const BITS: usize = <$ty>::BITS as usize;
        const SHIFT: usize = SHIFT;
        const IS_SIGNED: bool = <$ty>::MIN != 0;
        const ZERO: Self = Self(0);
        const ONE: Self = Self(1 << SHIFT);
        const EPSILON: Self = Self(1);
        fn from_f64(x: f64) -> Self {
          Self((x * Self::ONE.0 as f64) as _)
        }
        fn to_f64(self) -> f64 { self.0 as f64 / Self::ONE.0 as f64 }
        fn from_raw(x: Self::Raw) -> Self { Self(x) }
        fn into_raw(self) -> Self::Raw { self.0 }

        fn ipart(self) -> Position { (self.0 >> SHIFT) as _ }
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

      impl<const SHIFT: usize> Default for IFixed<$ty, SHIFT> {
        fn default() -> Self {
          Self::ZERO
        }
      }
    )+
  };
}

impl_pixel_like_ifixed!(u64 i64 u128 i128 u32 i32);

pub mod types {
  use super::IFixed;
  pub type U64F0 = IFixed<u64, 0>;
  pub type U56F8 = IFixed<u64, 8>;
  pub type U48F16 = IFixed<u32, 16>;
  pub type I64F0 = IFixed<i64, 0>;
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

  fn impl_test_pixel_like<T: FixedLike>(name: &str, zero: T) {
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
      T::EPSILON.to_f64()
    };
    assert!(eps < 0.01, "{} eps {}", name, eps);
    for i in test_list {
      if i < 0.0 && !T::IS_SIGNED {
        continue;
      }
      let v = T::from_f64(i);
      assert_approx_eq!(v.to_f64(), i, eps);
      let rounded = v.round().to_f64();
      assert_approx_eq!(rounded, i.round_ties_even(), eps);
      assert!((rounded - i).abs() <= 0.5 + eps, "{} round {} -> {rounded}", name, i);
      assert_eq!(rounded as i32 % 2, 0, "{} round {} -> {rounded}", name, i);

      let (ipart, frac) = (v.ipart(), v.frac().to_f64());
      assert_eq!(
        ipart,
        i.floor() as Position,
        "{} ipart {} -> ({ipart}, {frac})",
        name,
        i
      );
      assert!(ipart as f64 <= i, "{} ipart {} -> ({ipart}, {frac})", name, i);
      assert_approx_eq!(frac, i.rem_euclid(1.0), eps);
      assert!((0.0..1.0).contains(&frac), "{} frac {} -> ({ipart}, {frac})", name, i);
      assert_approx_eq!(frac + ipart as f64, i, eps);
    }
  }

  struct AssertType<T, U>(std::marker::PhantomData<(T, U)>);
  impl<T> AssertType<T, T> {
    const OK: bool = true;
  }

  #[test]
  fn test_fixed() {
    use fixed::types::*;
    test_pixel_like!(I56F8 I48F16 U56F8 U48F16);
    test_pixel_like!(I64F64);
    assert!(AssertType::<<I56F8 as FixedLike>::Raw, i64>::OK);
    assert!(AssertType::<<I48F16 as FixedLike>::Raw, i64>::OK);
    assert!(AssertType::<<U56F8 as FixedLike>::Raw, u64>::OK);
    assert!(AssertType::<<U48F16 as FixedLike>::Raw, u64>::OK);
    assert!(AssertType::<<I64F64 as FixedLike>::Raw, i128>::OK);
  }

  #[test]
  fn test_ifixed() {
    use types::*;
    test_pixel_like!(U56F8 U48F16 I56F8 I48F16 I64F64);
    assert!(AssertType::<<U56F8 as FixedLike>::Raw, u64>::OK);
    assert!(AssertType::<<U48F16 as FixedLike>::Raw, u32>::OK);
    assert!(AssertType::<<I56F8 as FixedLike>::Raw, i64>::OK);
    assert!(AssertType::<<I48F16 as FixedLike>::Raw, i32>::OK);
    assert!(AssertType::<<I64F64 as FixedLike>::Raw, i128>::OK);
  }

  #[test]
  fn test_float() {
    test_pixel_like!(f32 f64);
  }
}
