use palette::{
  bool_mask::HasBoolMask,
  num::{IsValidDivisor, MulAdd, MulSub, PartialCmp, Powf, Real},
};

use crate::{MulOps, RealLike, U8, U16, luminance};

pub trait PremultiplyNeeds: Powf + MulAdd + MulSub + PartialCmp + IsValidDivisor + HasBoolMask<Mask = bool> {}

impl<T> PremultiplyNeeds for T where T: Powf + MulAdd + MulSub + PartialCmp + IsValidDivisor + HasBoolMask<Mask = bool> {}

pub trait ColorValue: RealLike + MulOps + PremultiplyNeeds + Copy + 'static {
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
