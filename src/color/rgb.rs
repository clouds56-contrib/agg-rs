use palette::{
  blend::Premultiply,
  encoding::{FromLinear, IntoLinear},
};

use crate::{
  Color, ColorValue, FromColor, FromRaw2, FromRaw3, FromRaw4, IntoRaw2, IntoRaw3, IntoRaw4, NamedColor, U8, U16,
};

pub type Rgba<T> = palette::rgb::LinSrgba<T>;
pub type Rgba8 = Rgba<U8>;
pub type Rgba16 = Rgba<U16>;
pub type Rgba32 = Rgba<f32>;
pub type Rgba64 = Rgba<f64>;

impl<T: ColorValue> FromRaw4 for Rgba<T> {
  type Raw = T::Raw;
  fn from_raw(red: T::Raw, green: T::Raw, blue: T::Raw, alpha: T::Raw) -> Self {
    Self::new(red.into(), green.into(), blue.into(), alpha.into())
  }
}
impl<T: ColorValue> IntoRaw4 for Rgba<T> {
  type Raw = T::Raw;
  fn into_raw(self) -> (Self::Raw, Self::Raw, Self::Raw, Self::Raw) {
    (self.red.into(), self.green.into(), self.blue.into(), self.alpha.into())
  }
}
impl<T: ColorValue> FromColor for Rgba<T> {
  fn from_color<C: Color>(c: C) -> Self {
    Self::new(c.red_(), c.green_(), c.blue_(), c.alpha_())
  }
}

impl<T: ColorValue> NamedColor for Rgba<T> {
  const EMPTY: Self = Self::new(T::ZERO, T::ZERO, T::ZERO, T::ZERO);
  const WHITE: Self = Self::new(T::ONE, T::ONE, T::ONE, T::ONE);
  const BLACK: Self = Self::new(T::ZERO, T::ZERO, T::ZERO, T::ONE);
  const RED: Self = Self::new(T::ONE, T::ZERO, T::ZERO, T::ONE);
  const GREEN: Self = Self::new(T::ZERO, T::ONE, T::ZERO, T::ONE);
  const BLUE: Self = Self::new(T::ZERO, T::ZERO, T::ONE, T::ONE);
  fn gray_color<T2: ColorValue>(value: T2) -> Self {
    let value = value.as_color_();
    Self::new(value, value, value, T::ONE)
  }
}

// impl Rgba8 {
//     /// Crate new color from a wavelength and gamma
//     pub fn from_wavelength_gamma(w: f64, gamma: f64) -> Self {
//         let c = Rgb8::from_wavelength_gamma(w, gamma);
//         Self::from_color(c)
//     }
// }

pub type Gray<T> = palette::Alpha<palette::luma::LinLuma<palette::white_point::D65, T>, T>;
pub type Gray8 = Gray<U8>;
pub type Gray16 = Gray<U16>;
pub type Gray32 = Gray<f32>;
pub type Gray64 = Gray<f64>;

impl<T: ColorValue> FromRaw2 for Gray<T> {
  type Raw = T::Raw;
  fn from_raw(luma: T::Raw, alpha: T::Raw) -> Self {
    Self::new(luma.into(), alpha.into())
  }
}
impl<T: ColorValue> IntoRaw2 for Gray<T> {
  type Raw = T::Raw;
  fn into_raw(self) -> (Self::Raw, Self::Raw) {
    (self.luma.into(), self.alpha.into())
  }
}
impl<T: ColorValue> FromColor for Gray<T> {
  fn from_color<C: Color>(c: C) -> Self {
    let l = T::luminance(c.red_(), c.green_(), c.blue_());
    Self::new(l, c.alpha_())
  }
}

// impl<T: ColorValue> NamedColor for Gray<T> {
//     const BLACK: Self = Self::new(T::ZERO, T::ONE);
//     const WHITE: Self = Self::new(T::ONE, T::ONE);
//     const EMPTY: Self = Self::new(T::ZERO, T::ZERO);
//     const RED:   Self = Self::new(T::luminance(T::ONE, T::ZERO, T::ZERO), T::ONE);
//     const GREEN: Self = Self::new(T::luminance(T::ZERO, T::ONE, T::ZERO), T::ONE);
//     const BLUE:  Self = Self::new(T::luminance(T::ZERO, T::ZERO, T::ONE), T::ONE);
// }

pub const fn luminance(red: f64, green: f64, blue: f64) -> f64 {
  0.2126 * red + 0.7152 * green + 0.0722 * blue
}

/// Lightness (max(R, G, B) + min(R, G, B)) / 2
pub fn lightness(red: f64, green: f64, blue: f64) -> f64 {
  let mut cmax = red;
  let mut cmin = red;
  if green > cmax {
    cmax = green;
  }
  if blue > cmax {
    cmax = blue;
  }
  if green < cmin {
    cmin = green;
  }
  if blue < cmin {
    cmin = blue;
  }

  (cmax + cmin) / 2.0
}
/// Average
pub fn average(red: f64, green: f64, blue: f64) -> f64 {
  (red + green + blue) / 3.0
}

pub type Rgb<T> = palette::rgb::LinSrgb<T>;
pub type Rgb8 = Rgb<U8>;
pub type Rgb16 = Rgb<U16>;
pub type Rgb32 = Rgb<f32>;
pub type Rgb64 = Rgb<f64>;

impl<T: ColorValue> FromRaw3 for Rgb<T> {
  type Raw = T::Raw;
  fn from_raw(red: T::Raw, green: T::Raw, blue: T::Raw) -> Self {
    Self::new(red.into(), green.into(), blue.into())
  }
}
impl<T: ColorValue> IntoRaw3 for Rgb<T> {
  type Raw = T::Raw;
  fn into_raw(self) -> (Self::Raw, Self::Raw, Self::Raw) {
    (self.red.into(), self.green.into(), self.blue.into())
  }
}
impl<T: ColorValue> FromColor for Rgb<T> {
  fn from_color<C: Color>(c: C) -> Self {
    Self::new(c.red_(), c.green_(), c.blue_())
  }
}

impl<T: ColorValue> NamedColor for Rgb<T> {
  const EMPTY: Self = Self::new(T::ZERO, T::ZERO, T::ZERO);
  const WHITE: Self = Self::new(T::ONE, T::ONE, T::ONE);
  const BLACK: Self = Self::new(T::ZERO, T::ZERO, T::ZERO);
  const RED: Self = Self::new(T::ONE, T::ZERO, T::ZERO);
  const GREEN: Self = Self::new(T::ZERO, T::ONE, T::ZERO);
  const BLUE: Self = Self::new(T::ZERO, T::ZERO, T::ONE);
  fn gray_color<T2: ColorValue>(value: T2) -> Self {
    let value = value.as_color_();
    Self::new(value, value, value)
  }
}

pub fn rgb8_from_wavelength_gamma(w: f64, gamma: f64) -> Rgb8 {
  let (r, g, b) = if (380.0..=440.0).contains(&w) {
    (-(w - 440.0) / (440.0 - 380.0), 0.0, 1.0)
  } else if (440.0..=490.0).contains(&w) {
    (0.0, (w - 440.0) / (490.0 - 440.0), 1.0)
  } else if (490.0..=510.0).contains(&w) {
    (0.0, 1.0, -(w - 510.0) / (510.0 - 490.0))
  } else if (510.0..=580.0).contains(&w) {
    ((w - 510.0) / (580.0 - 510.0), 1.0, 0.0)
  } else if (580.0..=645.0).contains(&w) {
    (1.0, -(w - 645.0) / (645.0 - 580.0), 0.0)
  } else if (645.0..=780.0).contains(&w) {
    (1.0, 0.0, 0.0)
  } else {
    (0., 0., 0.)
  };
  let scale = if w > 700.0 {
    0.3 + 0.7 * (780.0 - w) / (780.0 - 700.0)
  } else if w < 420.0 {
    0.3 + 0.7 * (w - 380.0) / (420.0 - 380.0)
  } else {
    1.0
  };
  let r = (r * scale).powf(gamma) * 255.0;
  let g = (g * scale).powf(gamma) * 255.0;
  let b = (b * scale).powf(gamma) * 255.0;
  Rgb::from_raw(r as u8, g as u8, b as u8)
}

pub type RgbaPre<T> = palette::blend::PreAlpha<Rgb<T>>;
pub type RgbaPre8 = RgbaPre<U8>;
pub type RgbaPre16 = RgbaPre<U16>;
pub type RgbaPre32 = RgbaPre<f32>;
pub type RgbaPre64 = RgbaPre<f64>;

impl<T: ColorValue> FromRaw4 for RgbaPre<T> {
  type Raw = T::Raw;
  fn from_raw(red: T::Raw, green: T::Raw, blue: T::Raw, alpha: T::Raw) -> Self {
    Self {
      color: Rgb::new(red.into(), green.into(), blue.into()),
      alpha: alpha.into(),
    }
  }
}
impl<T: ColorValue> IntoRaw4 for RgbaPre<T> {
  type Raw = T::Raw;
  fn into_raw(self) -> (Self::Raw, Self::Raw, Self::Raw, Self::Raw) {
    (self.red.into(), self.green.into(), self.blue.into(), self.alpha.into())
  }
}
impl<T: ColorValue> FromColor for RgbaPre<T> {
  fn from_color<C: Color>(c: C) -> Self {
    Self::new(Rgb::new(c.red_(), c.green_(), c.blue_()), c.alpha_())
  }
}

impl<T: ColorValue> NamedColor for RgbaPre<T> {
  const EMPTY: Self = Self {
    color: Rgb::new(T::ZERO, T::ZERO, T::ZERO),
    alpha: T::ZERO,
  };
  const BLACK: Self = Self {
    color: Rgb::new(T::ZERO, T::ZERO, T::ZERO),
    alpha: T::ONE,
  };
  const WHITE: Self = Self {
    color: Rgb::new(T::ONE, T::ONE, T::ONE),
    alpha: T::ONE,
  };
  const RED: Self = Self {
    color: Rgb::new(T::ONE, T::ZERO, T::ZERO),
    alpha: T::ONE,
  };
  const GREEN: Self = Self {
    color: Rgb::new(T::ZERO, T::ONE, T::ZERO),
    alpha: T::ONE,
  };
  const BLUE: Self = Self {
    color: Rgb::new(T::ZERO, T::ZERO, T::ONE),
    alpha: T::ONE,
  };

  fn gray_color<T2: ColorValue>(value: T2) -> Self {
    Rgba::gray_color(value).premultiply()
  }
}

pub type Srgba<T> = palette::rgb::Srgba<T>;
pub type Srgba8 = Srgba<U8>;
pub type Srgba16 = Srgba<U16>;
pub type Srgba32 = Srgba<f32>;
pub type Srgba64 = Srgba<f64>;

impl<T: ColorValue> FromRaw4 for Srgba<T> {
  type Raw = T::Raw;
  fn from_raw(red: T::Raw, green: T::Raw, blue: T::Raw, alpha: T::Raw) -> Self {
    Self::new(red.into(), green.into(), blue.into(), alpha.into())
  }
}
impl<T: ColorValue> IntoRaw4 for Srgba<T> {
  type Raw = T::Raw;
  fn into_raw(self) -> (Self::Raw, Self::Raw, Self::Raw, Self::Raw) {
    (self.red.into(), self.green.into(), self.blue.into(), self.alpha.into())
  }
}
impl<T: ColorValue> FromColor for Srgba<T> {
  fn from_color<C: Color>(c: C) -> Self {
    if std::any::TypeId::of::<C>() == std::any::TypeId::of::<Srgba<T>>() {
      // This is safe because we just checked that C and Srgba<T> are the same type
      return unsafe { std::mem::transmute_copy(&c) };
    }
    let red: f64 = palette::encoding::Srgb::from_linear(c.red64());
    let green: f64 = palette::encoding::Srgb::from_linear(c.green64());
    let blue: f64 = palette::encoding::Srgb::from_linear(c.blue64());
    Self::new(red.as_color_(), green.as_color_(), blue.as_color_(), c.alpha_())
  }
}

impl<T: ColorValue> NamedColor for Srgba<T> {
  const EMPTY: Self = Self::new(T::ZERO, T::ZERO, T::ZERO, T::ZERO);
  const WHITE: Self = Self::new(T::ONE, T::ONE, T::ONE, T::ONE);
  const BLACK: Self = Self::new(T::ZERO, T::ZERO, T::ZERO, T::ONE);
  const RED: Self = Self::new(T::ONE, T::ZERO, T::ZERO, T::ONE);
  const GREEN: Self = Self::new(T::ZERO, T::ONE, T::ZERO, T::ONE);
  const BLUE: Self = Self::new(T::ZERO, T::ZERO, T::ONE, T::ONE);

  fn gray_color<T2: ColorValue>(value: T2) -> Self {
    let value = value.as_color_();
    Self::new(value, value, value, T::ONE)
  }
}

impl<T: ColorValue> Color for Rgba<T> {
  fn red_<T2: ColorValue>(&self) -> T2 {
    self.red.as_color_()
  }
  fn green_<T2: ColorValue>(&self) -> T2 {
    self.green.as_color_()
  }
  fn blue_<T2: ColorValue>(&self) -> T2 {
    self.blue.as_color_()
  }
  fn alpha_<T2: ColorValue>(&self) -> T2 {
    self.alpha.as_color_()
  }
  fn is_premultiplied(&self) -> bool {
    false
  }
}
impl<T: ColorValue> Color for Rgb<T> {
  fn red_<T2: ColorValue>(&self) -> T2 {
    self.red.as_color_()
  }
  fn green_<T2: ColorValue>(&self) -> T2 {
    self.green.as_color_()
  }
  fn blue_<T2: ColorValue>(&self) -> T2 {
    self.blue.as_color_()
  }
  fn alpha_<T2: ColorValue>(&self) -> T2 {
    T2::ONE
  }
  fn is_premultiplied(&self) -> bool {
    false
  }
}
impl<T: ColorValue> Color for RgbaPre<T>
where
  Rgb<T>: Premultiply<Scalar = T>,
{
  fn red_<T2: ColorValue>(&self) -> T2 {
    self.red.as_color_()
  }
  fn green_<T2: ColorValue>(&self) -> T2 {
    self.green.as_color_()
  }
  fn blue_<T2: ColorValue>(&self) -> T2 {
    self.blue.as_color_()
  }
  fn alpha_<T2: ColorValue>(&self) -> T2 {
    self.alpha.as_color_()
  }
  fn is_premultiplied(&self) -> bool {
    true
  }
}
impl<T: ColorValue> Color for Srgba<T> {
  fn red_<T2: ColorValue>(&self) -> T2 {
    palette::encoding::Srgb::into_linear(self.red.to_f64()).as_color_()
  }
  fn green_<T2: ColorValue>(&self) -> T2 {
    palette::encoding::Srgb::into_linear(self.green.to_f64()).as_color_()
  }
  fn blue_<T2: ColorValue>(&self) -> T2 {
    palette::encoding::Srgb::into_linear(self.blue.to_f64()).as_color_()
  }
  fn alpha_<T2: ColorValue>(&self) -> T2 {
    self.alpha.as_color_()
  }
  fn is_premultiplied(&self) -> bool {
    false
  }
}
impl<T: ColorValue> Color for Gray<T> {
  fn red_<T2: ColorValue>(&self) -> T2 {
    self.luma.as_color_()
  }
  fn green_<T2: ColorValue>(&self) -> T2 {
    self.luma.as_color_()
  }
  fn blue_<T2: ColorValue>(&self) -> T2 {
    self.luma.as_color_()
  }
  fn alpha_<T2: ColorValue>(&self) -> T2 {
    self.alpha.as_color_()
  }
  fn is_premultiplied(&self) -> bool {
    false
  }
}
