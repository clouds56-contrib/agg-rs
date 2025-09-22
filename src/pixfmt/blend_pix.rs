

use crate::{Color, ColorValue, Gray, RealLike, Rgb, Rgba, RgbaPre};


/// TODO: do memory layout optimization for this enum
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Cover<T> {
  None,
  Mask(T),
  Full,
}

impl<T: RealLike> Cover<T> {
  pub fn to_f64(self) -> f64 {
    match self {
      Cover::None => 0.0,
      Cover::Mask(v) => v.to_f64(),
      Cover::Full => 1.0,
    }
  }

  pub fn new(v: T) -> Self {
    if v == T::ZERO {
      Cover::None
    } else if v == T::ONE {
      Cover::Full
    } else {
      Cover::Mask(v)
    }
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


pub fn blend_mix_on_rgb<T, C>(src: Rgb<T>, dst: C, cover: T) -> Rgb<T>
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


pub fn blend_mix_on_rgba_pre<T, C>(src: RgbaPre<T>, dst: C, cover: T) -> RgbaPre<T>
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

pub fn blend_mix_on_gray<T, C>(src: Gray<T>, dst: C, cover: T) -> Gray<T>
where
  T: ColorValue,
  C: Color,
{
  let dst = dst.gray();
  let beta = cover * dst.alpha;
  let luma = T::lerp(src.luma, dst.luma, beta);
  Gray::new(luma, beta)
}
