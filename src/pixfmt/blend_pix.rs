use crate::{Color, ColorValue, Gray, RealLike, Rgb, Rgba, RgbaPre};

/// TODO: do memory layout optimization for this enum
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Cover<T> {
  None,
  Mask(T),
  Full,
}

pub type CoverOf<C> = Cover<<C as Color>::Component>;

impl From<u64> for Cover<u64> {
  fn from(v: u64) -> Self {
    if v == 0 {
      Cover::None
    } else if v >= Self::FULL {
      Cover::Full
    } else {
      Cover::Mask(v)
    }
  }
}

impl Cover<u64> {
  const BITS: usize = 8;
  const NONE: u64 = 0;
  const FULL: u64 = (1 << Self::BITS) - 1;

  pub fn into_<T: RealLike>(self) -> Cover<T> {
    match self {
      Cover::None => Cover::None,
      Cover::Mask(v) => Cover::Mask(T::from_f64(v as f64 / Self::FULL as f64)),
      Cover::Full => Cover::Full,
    }
  }
}

impl<T: RealLike> Cover<T> {
  pub fn new(v: T) -> Self {
    if v == T::ZERO {
      Cover::None
    } else if v == T::ONE {
      Cover::Full
    } else {
      Cover::Mask(v)
    }
  }

  pub fn from_raw(v: T::Raw) -> Self {
    Self::new(v.into())
  }

  pub fn to_f64(self) -> f64 {
    self.get().to_f64()
  }

  pub fn get(self) -> T {
    match self {
      Cover::None => T::ZERO,
      Cover::Mask(v) => v,
      Cover::Full => T::ONE,
    }
  }


  pub fn is_full(&self) -> bool {
    *self == Cover::Full
  }

  pub fn is_none(&self) -> bool {
    *self == Cover::None
  }
}

pub trait BlendPix: Color {
  fn blend_pix<C: Color>(self, c: C, cover: Cover<Self::Component>) -> Self;
}

impl<T: ColorValue> BlendPix for Rgba<T> {
  fn blend_pix<C: Color>(self, c: C, cover: Cover<Self::Component>) -> Self {
    blend_pix_on_rgba(self, c, cover.get())
  }
}

impl<T: ColorValue> BlendPix for Rgb<T> {
  fn blend_pix<C: Color>(self, c: C, cover: Cover<Self::Component>) -> Self {
    blend_pix_on_rgb(self, c, cover.get())
  }
}

impl<T: ColorValue> BlendPix for RgbaPre<T> {
  fn blend_pix<C: Color>(self, c: C, cover: Cover<Self::Component>) -> Self {
    blend_pix_on_rgba_pre(self, c, cover.get())
  }
}

impl<T: ColorValue> BlendPix for Gray<T> {
  fn blend_pix<C: Color>(self, c: C, cover: Cover<Self::Component>) -> Self {
    blend_pix_on_gray(self, c, cover.get())
  }
}

pub fn blend_pix_on_rgba<T, C>(src: Rgba<T>, dst: C, cover: T) -> Rgba<T>
where
  T: ColorValue,
  C: Color,
{
  let dst = dst.rgba();
  let beta = cover * dst.alpha;
  let red = T::lerp(src.red, dst.red, beta);
  let green = T::lerp(src.green, dst.green, beta);
  let blue = T::lerp(src.blue, dst.blue, beta);
  let alpha = T::prelerp(src.alpha, beta, beta);
  Rgba::new(red, green, blue, alpha)
}

pub fn blend_pix_on_rgb<T, C>(src: Rgb<T>, dst: C, cover: T) -> Rgb<T>
where
  T: ColorValue,
  C: Color,
{
  let dst = dst.rgba();
  let beta = cover * dst.alpha;
  let red = T::lerp(src.red, dst.red, beta);
  let green = T::lerp(src.green, dst.green, beta);
  let blue = T::lerp(src.blue, dst.blue, beta);
  Rgb::new(red, green, blue)
}

pub fn blend_pix_on_rgba_pre<T, C>(src: RgbaPre<T>, dst: C, cover: T) -> RgbaPre<T>
where
  T: ColorValue,
  C: Color,
{
  let dst = dst.rgba();
  let beta = cover * dst.alpha;
  let dst_red = cover * dst.red;
  let dst_green = cover * dst.green;
  let dst_blue = cover * dst.blue;

  let red = T::prelerp(src.red, dst_red, beta);
  let green = T::prelerp(src.green, dst_green, beta);
  let blue = T::prelerp(src.blue, dst_blue, beta);
  let alpha = T::prelerp(src.alpha, beta, beta);
  RgbaPre {
    color: Rgb::new(red, green, blue),
    alpha,
  }
}

pub fn blend_pix_on_gray<T, C>(src: Gray<T>, dst: C, cover: T) -> Gray<T>
where
  T: ColorValue,
  C: Color,
{
  let dst = dst.gray();
  let beta = cover * dst.alpha;
  let luma = T::lerp(src.luma, dst.luma, beta);
  Gray::new(luma, beta)
}
