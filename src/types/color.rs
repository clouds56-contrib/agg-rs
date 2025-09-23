use palette::{
  bool_mask::HasBoolMask,
  num::{One, Real, Zero},
};

use crate::math::{combine_u8, lerp_u8, multiply_u8, prelerp_u8};

#[derive(Copy, Clone, Debug, PartialEq, Hash, PartialOrd, Ord, Eq)]
pub struct UFixed<T>(pub T);

impl<T> UFixed<T> {
  pub fn new(v: T) -> Self {
    Self(v)
  }
}

impl<T> std::ops::Deref for UFixed<T> {
  type Target = T;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}
impl<T> std::ops::DerefMut for UFixed<T> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}
impl<T> std::fmt::Display for UFixed<T>
where
  T: std::fmt::Display,
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0)
  }
}
impl<T> From<T> for UFixed<T> {
  fn from(v: T) -> Self {
    Self(v)
  }
}

impl<T> std::ops::Mul for UFixed<T>
where
  Self: MulOps,
{
  type Output = Self;
  fn mul(self, rhs: Self) -> Self::Output {
    <Self as MulOps>::mul(self, rhs)
  }
}

impl<T> std::ops::Div for UFixed<T>
where
  Self: MulOps,
{
  type Output = Self;
  fn div(self, rhs: Self) -> Self::Output {
    <Self as MulOps>::div(self, rhs)
  }
}

impl<T> std::ops::Add for UFixed<T>
where
  T: std::ops::Add<Output = T>,
{
  type Output = Self;
  fn add(self, rhs: Self) -> Self::Output {
    Self(self.0 + rhs.0)
  }
}

impl<T> std::ops::Sub for UFixed<T>
where
  T: std::ops::Sub<Output = T>,
{
  type Output = Self;
  fn sub(self, rhs: Self) -> Self::Output {
    Self(self.0 - rhs.0)
  }
}

impl<T> palette::num::Powf for UFixed<T>
where
  Self: MulOps,
{
  fn powf(self, n: Self) -> Self {
    <Self as MulOps>::powf(self, n.to_f64())
  }
}

impl<T> palette::num::MulAdd for UFixed<T>
where
  Self: std::ops::Mul<Output = Self> + std::ops::Add<Output = Self>,
{
  fn mul_add(self, a: Self, b: Self) -> Self {
    self * a + b
  }
}

impl<T> palette::num::MulSub for UFixed<T>
where
  Self: std::ops::Mul<Output = Self> + std::ops::Sub<Output = Self>,
{
  fn mul_sub(self, a: Self, b: Self) -> Self {
    self * a - b
  }
}

impl<T> palette::num::PartialCmp for UFixed<T>
where
  T: PartialOrd,
{
  #[inline]
  fn lt(&self, other: &Self) -> Self::Mask {
    self < other
  }

  #[inline]
  fn lt_eq(&self, other: &Self) -> Self::Mask {
    self <= other
  }

  #[inline]
  fn eq(&self, other: &Self) -> Self::Mask {
    self == other
  }

  #[inline]
  fn neq(&self, other: &Self) -> Self::Mask {
    self != other
  }

  #[inline]
  fn gt_eq(&self, other: &Self) -> Self::Mask {
    self >= other
  }

  #[inline]
  fn gt(&self, other: &Self) -> Self::Mask {
    self > other
  }
}

impl<T> palette::bool_mask::HasBoolMask for UFixed<T> {
  type Mask = bool;
}
impl<T> palette::num::IsValidDivisor for UFixed<T>
where
  Self: RealLike,
  Self: HasBoolMask<Mask = bool>,
{
  fn is_valid_divisor(&self) -> bool {
    *self != Self::ZERO
  }
}

pub type U8 = UFixed<u8>;
pub type U16 = UFixed<u16>;

macro_rules! impl_num {
  ($ty:ident) => {
    impl palette::num::Zero for UFixed<$ty> {
      fn zero() -> Self { UFixed(0) }
    }

    impl palette::num::One for UFixed<$ty> {
        fn one() -> Self { Self($ty::MAX) }
    }

    impl palette::num::Real for UFixed<$ty> {
      fn from_f64(n: f64) -> Self {
        if n <= 0.0 {
          Self::zero()
        } else if n >= 1.0 {
          Self::one()
        } else {
          Self((n * Self::one().0 as f64).round() as _)
        }
      }
    }

    impl From<UFixed<$ty>> for $ty {
      fn from(v: UFixed<$ty>) -> Self { v.0 }
    }

    impl RealLike for UFixed<$ty> {
      type Raw = $ty;
      const ZERO: Self = Self(0);
      const ONE: Self = Self($ty::MAX);
      const _MAX: f64 = Self::ONE.0 as f64;
      fn to_f64(self) -> f64 { f64::from(self.0) / Self::_MAX }
    }
  };
  ($ty:ident $($ty2:ident)+) => {
      impl_num!($ty);
      impl_num!($($ty2)+);
  };
}

macro_rules! impl_float {
  ($ty:ident) => {
    impl RealLike for $ty {
      type Raw = $ty;
      const ZERO: Self = 0.0;
      const ONE: Self = 1.0;
      const _MAX: f64 = 1.0;
      fn to_f64(self) -> f64 { f64::from(self) }
    }
  };
  ($ty:ident $($ty2:ident)+) => {
      impl_float!($ty);
      impl_float!($($ty2)+);
  };
}

pub trait RealLike:
  Real + One + Zero + Copy + PartialOrd + std::fmt::Debug + PartialEq + From<Self::Raw> + Into<Self::Raw> + 'static
{
  type Raw;
  const ZERO: Self;
  const ONE: Self;
  const _MAX: f64;
  fn byte_size() -> usize {
    std::mem::size_of::<Self>()
  }
  fn to_f64(self) -> f64;
  fn as_<T: RealLike>(self) -> T {
    if std::any::TypeId::of::<Self>() == std::any::TypeId::of::<T>() {
      // This is safe because we just checked that Self and T are the same type
      unsafe { std::mem::transmute_copy(&self) }
    } else {
      T::from_f64(self.to_f64())
    }
  }
}

impl_num!(u8 u16);
impl_float!(f32 f64);

pub trait MulOps: RealLike + std::ops::Mul<Output = Self> + std::ops::Div<Output = Self> {
  /// Multiply two color values, clamping the result to [0, 1]
  fn mul(self, rhs: Self) -> Self {
    Self::from_f64(self.to_f64() * rhs.to_f64())
  }

  fn fast_mul(self, rhs: Self) -> Self {
    <Self as MulOps>::mul(self, rhs)
  }

  fn div(self, rhs: Self) -> Self {
    Self::from_f64(self.to_f64() / rhs.to_f64())
  }

  /// TODO: this powf is different from palette::num::Powf
  /// because it takes f64 as exponent, not Self
  fn powf(self, n: f64) -> Self {
    Self::from_f64(self.to_f64().powf(n))
  }

  /// Interpolator a value between two end points pre-calculated by alpha
  ///
  /// p + q - (p*a)
  fn lerp(p: Self, q: Self, a: Self) -> Self {
    let a = a.to_f64();
    let p = p.to_f64();
    let q = q.to_f64();
    Self::from_f64(a * (q - p) + p)
  }

  /// p + q - a * p;
  fn prelerp(p: Self, q: Self, a: Self) -> Self {
    let a = a.to_f64();
    let p = p.to_f64();
    let q = q.to_f64();
    Self::from_f64(p + q - a * p)
  }
}

impl MulOps for U8 {
  fn mul(self, rhs: Self) -> Self {
    Self(multiply_u8(self.0, rhs.0))
  }
  fn fast_mul(self, rhs: Self) -> Self {
    Self(combine_u8(self.0, rhs.0))
  }
  fn lerp(p: Self, q: Self, a: Self) -> Self {
    Self(lerp_u8(p.0, q.0, a.0))
  }
  fn prelerp(p: Self, q: Self, a: Self) -> Self {
    Self(prelerp_u8(p.0, q.0, a.0))
  }
}
impl MulOps for U16 {}
impl MulOps for f32 {}
impl MulOps for f64 {}
