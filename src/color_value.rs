use std::u8;

use crate::{
    luminance,
    math::{lerp_u8, multiply_u8, prelerp_u8},
};

use palette::{
    bool_mask::HasBoolMask,
    num::{IsValidDivisor, MulAdd, MulSub, One, PartialCmp, Powf, Real, Zero},
};

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

    impl std::ops::Mul for UFixed<$ty> {
      type Output = Self;
      fn mul(self, rhs: Self) -> Self::Output {
        <Self as MulOps>::mul(self, rhs)
      }
    }

    impl std::ops::Div for UFixed<$ty> {
      type Output = Self;
      fn div(self, rhs: Self) -> Self::Output {
        <Self as MulOps>::div(self, rhs)
      }
    }

    impl palette::num::Powf for UFixed<$ty> {
      fn powf(self, n: Self) -> Self {
        <Self as MulOps>::powf(self, n.to_f64())
      }
    }

    impl palette::num::MulAdd for UFixed<$ty> {
      fn mul_add(self, a: Self, b: Self) -> Self {
        <Self as MulOps>::mul_add(self, a, b)
      }
    }

    impl palette::num::MulSub for UFixed<$ty> {
      fn mul_sub(self, a: Self, b: Self) -> Self {
        <Self as MulOps>::mul_sub(self, a, b)
      }
    }

    impl palette::num::PartialCmp for UFixed<$ty> {
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

    impl HasBoolMask for UFixed<$ty> {
      type Mask = bool;
    }
    impl IsValidDivisor for UFixed<$ty> {
      fn is_valid_divisor(&self) -> bool {
        *self != Self::ZERO
      }
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
    Real
    + One
    + Zero
    + Copy
    + std::fmt::Debug
    + PartialEq<Self>
    + From<Self::Raw>
    + Into<Self::Raw>
    + 'static
{
    type Raw;
    const ZERO: Self;
    const ONE: Self;
    const _MAX: f64;
    fn to_f64(self) -> f64;
}

impl_num!(u8 u16);
impl_float!(f32 f64);

pub trait MulOps:
    RealLike
    + std::ops::Mul<Output = Self>
    + std::ops::Div<Output = Self>
    + Powf
    + MulAdd
    + MulSub
    + PartialCmp
    + IsValidDivisor
    + HasBoolMask<Mask = bool>
{
    /// Multiply two color values, clamping the result to [0, 1]
    fn mul(self, rhs: Self) -> Self {
        Self::from_f64(self.to_f64() * rhs.to_f64())
    }

    fn div(self, rhs: Self) -> Self {
        Self::from_f64(self.to_f64() / rhs.to_f64())
    }

    /// TODO: this powf is different from palette::num::Powf
    /// because it takes f64 as exponent, not Self
    fn powf(self, n: f64) -> Self {
        Self::from_f64(self.to_f64().powf(n))
    }

    fn mul_add(self, a: Self, b: Self) -> Self {
        Self::from_f64(self.to_f64() * a.to_f64() + b.to_f64())
    }

    fn mul_sub(self, a: Self, b: Self) -> Self {
        Self::from_f64(self.to_f64() * a.to_f64() - b.to_f64())
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

pub trait ColorValue: RealLike + MulOps + Copy + 'static {
    fn as_color_f64(self) -> f64 {
        self.to_f64()
    }
    fn as_color_u8(self) -> U8 {
        U8::from_f64(self.to_f64())
    }
    fn from_color_f64(v: f64) -> Self {
        Self::from_f64(v)
    }
    fn from_color_u8(v: U8) -> Self {
        Self::from_f64(v.to_f64())
    }
    fn as_color_<T: ColorValue>(self) -> T {
        if std::any::TypeId::of::<Self>() == std::any::TypeId::of::<T>() {
            // This is safe because we just checked that Self and T are the same type
            unsafe { std::mem::transmute_copy(&self) }
        } else {
            T::from_color_f64(self.as_color_f64())
        }
    }

    fn luminance_f64(r: Self, g: Self, b: Self) -> f64 {
        luminance(r.as_color_f64(), g.as_color_f64(), b.as_color_f64())
    }
    fn luminance_<T: ColorValue>(r: Self, g: Self, b: Self) -> T {
        Self::luminance_f64(r, g, b).as_color_()
    }
    fn luminance(r: Self, g: Self, b: Self) -> Self {
        Self::luminance_(r, g, b)
    }

    fn to_srgb_f64(self) -> f64 {
        rgb_to_srgb(self.as_color_f64())
    }
    fn to_srgb_<T: ColorValue>(self) -> T {
        self.to_srgb_f64().as_color_()
    }
    fn to_srgb(self) -> Self {
        self.to_srgb_()
    }
    fn from_srgb_f64(self) -> f64 {
        srgb_to_rgb(self.as_color_f64())
    }
    fn from_srgb_<T: ColorValue>(self) -> T {
        self.from_srgb_f64().as_color_()
    }
    fn from_srgb(self) -> Self {
        self.from_srgb_()
    }
}
impl ColorValue for U8 {
    fn as_color_u8(self) -> U8 {
        self
    }
    fn from_color_u8(v: U8) -> Self {
        v
    }
}
impl ColorValue for U16 {}
impl ColorValue for f32 {}
impl ColorValue for f64 {}

/// Convert from sRGB to RGB for a single component
fn srgb_to_rgb(x: f64) -> f64 {
    if x <= 0.04045 {
        x / 12.92
    } else {
        ((x + 0.055) / 1.055).powf(2.4)
    }
}
/// Convert from RGB to sRGB for a single component
fn rgb_to_srgb(x: f64) -> f64 {
    if x <= 0.003_130_8 {
        x * 12.92
    } else {
        1.055 * x.powf(1.0 / 2.4) - 0.055
    }
}
