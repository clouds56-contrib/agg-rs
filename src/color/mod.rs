//! Colors

pub mod color_value;
pub mod rgb;
pub mod traits;
pub use color_value::*;
pub use rgb::*;
pub use traits::*;

use crate::U8;

pub trait FromColor {
  fn from_color<C: Color>(c: C) -> Self;
}

/// Access Color properties and compoents
pub trait Color: std::fmt::Debug + Copy + 'static {
  type Component: ColorValue;
  fn bpp() -> usize;

  /// Get red value
  fn red_<T: ColorValue>(&self) -> T;
  /// Get green value
  fn green_<T: ColorValue>(&self) -> T;
  /// Get blue value
  fn blue_<T: ColorValue>(&self) -> T;
  /// Get alpha value
  fn alpha_<T: ColorValue>(&self) -> T;
  /// Get red value [0,1] as f64
  fn red64(&self) -> f64 {
    self.red_()
  }
  /// Get green value [0,1] as f64
  fn green64(&self) -> f64 {
    self.green_()
  }
  /// Get blue value [0,1] as f64
  fn blue64(&self) -> f64 {
    self.blue_()
  }
  /// Get alpha value [0,1] as f64
  fn alpha64(&self) -> f64 {
    self.alpha_()
  }
  /// Get red value [0,255] as u8
  fn red8(&self) -> u8 {
    self.red_::<U8>().0
  }
  /// Get green value [0,255] as u8
  fn green8(&self) -> u8 {
    self.green_::<U8>().0
  }
  /// Get blue value [0,255] as u8
  fn blue8(&self) -> u8 {
    self.blue_::<U8>().0
  }
  /// Get alpha value [0,255] as u8
  fn alpha8(&self) -> u8 {
    self.alpha_::<U8>().0
  }

  fn rgb<T: ColorValue>(&self) -> Rgb<T> {
    Rgb::from_color(*self)
  }
  fn rgb8(&self) -> Rgb<U8> {
    self.rgb()
  }
  fn rgb64(&self) -> Rgb<f64> {
    self.rgb()
  }
  fn rgba<T: ColorValue>(&self) -> Rgba<T> {
    Rgba::from_color(*self)
  }
  fn rgba8(&self) -> Rgba<U8> {
    self.rgba()
  }
  fn rgba64(&self) -> Rgba<f64> {
    self.rgba()
  }
  fn gray<T: ColorValue>(&self) -> Gray<T> {
    Gray::from_color(*self)
  }
  fn gray8(&self) -> Gray<U8> {
    self.gray()
  }
  fn gray64(&self) -> Gray<f64> {
    self.gray()
  }
  fn srgba<T: ColorValue>(&self) -> Srgba<T> {
    Srgba::from_color(*self)
  }
  fn srgba8(&self) -> Srgba<U8> {
    self.srgba()
  }
  fn srgba64(&self) -> Srgba<f64> {
    self.srgba()
  }

  /// Return if the color is completely transparent, alpha = 0.0
  fn is_transparent(&self) -> bool {
    self.alpha64() == 0.0
  }
  /// Return if the color is completely opaque, alpha = 1.0
  fn is_opaque(&self) -> bool {
    self.alpha64() >= 1.0
  }
  /// Return if the color has been premultiplied
  fn is_premultiplied(&self) -> bool;
}

#[cfg(test)]
mod tests {
  use crate::U16;

  use super::*;

  #[test]
  fn value_type_test() {
    for i in 0..=255u8 {
      assert_eq!(U8::new(i).as_color_::<U8>(), U8::new(i));
    }
    for i in 0..=65535u16 {
      assert_eq!(U16::new(i).as_color_::<U16>(), U16::new(i));
    }
  }

  #[test]
  fn rgb8_to_gray8_test() {
    let values = [
      [0, 0, 0, 0u8],
      [255, 255, 255, 255],
      [255, 0, 0, 54],
      [0, 255, 0, 182],
      [0, 0, 255, 18],
      [255, 255, 0, 237],
      [255, 0, 255, 73],
      [0, 255, 255, 201],
      [128, 128, 128, 128],
      [128, 0, 0, 27],
      [0, 128, 0, 92],
      [0, 0, 128, 9],
      [128, 128, 0, 119],
      [128, 0, 128, 36],
      [0, 128, 128, 101],
    ];
    for [r, g, b, z] in &values {
      let c = Rgb8::new(U8::new(*r), U8::new(*g), U8::new(*b));
      let gray = c.gray8();
      assert_eq!(gray.luma.0, *z);
    }
  }
  #[test]
  fn rgb8_test() {
    let w = Rgb8::WHITE;
    assert_eq!(w.into_raw(), (255, 255, 255));
    assert_eq!((w.red8(), w.green8(), w.blue8()), (255, 255, 255));
    let w = Rgb8::BLACK;
    assert_eq!(w.into_raw(), (0, 0, 0));
    assert_eq!((w.red8(), w.green8(), w.blue8()), (0, 0, 0));
    let w = Rgb8::gray_color(0.5);
    assert_eq!(w.into_raw(), (128, 128, 128));
    assert_eq!((w.red8(), w.green8(), w.blue8()), (128, 128, 128));
    // let w = Rgb8::from_slice(&[1, 2, 3]);
    // assert_eq!(w, Rgb8::from_raw(1, 2, 3));
    let w = Rgb8::from_raw(0, 90, 180);
    assert_eq!(w.into_raw(), (0, 90, 180));
    assert_eq!((w.red8(), w.green8(), w.blue8()), (0, 90, 180));
  }
  #[test]
  fn gray_test() {
    let g = Gray8::from_raw(34, 255);
    assert_eq!(g.into_raw(), (34, 255));
    let g = Gray8::from_raw(134, 100);
    assert_eq!(g.into_raw(), (134, 100));
    // let g = Gray8::from_slice(&[10,20]);
    // assert_eq!(g.into_raw(), (10,20));
  }
  #[test]
  fn rgba8_test() {
    let c = Rgba8::WHITE;
    assert_eq!(c.into_raw(), (255, 255, 255, 255));
    assert_eq!((c.red8(), c.green8(), c.blue8(), c.alpha8()), (255, 255, 255, 255));
    let c = Rgba8::BLACK;
    assert_eq!(c.into_raw(), (0, 0, 0, 255));
    assert_eq!((c.red8(), c.green8(), c.blue8(), c.alpha8()), (0, 0, 0, 255));
    let c = Rgba8::from_raw(255, 90, 84, 72);
    assert_eq!(c.into_raw(), (255, 90, 84, 72));
    assert_eq!((c.red8(), c.green8(), c.blue8(), c.alpha8()), (255, 90, 84, 72));
    let c = Rgba8::EMPTY;
    assert_eq!(c.into_raw(), (0, 0, 0, 0));
    assert_eq!((c.red8(), c.green8(), c.blue8(), c.alpha8()), (0, 0, 0, 0));
    let c = Rgba8::from_raw(255, 255, 255, 128);
    let p = c.premultiply();
    assert_eq!(p, RgbaPre8::from_raw(128, 128, 128, 128));
  }
  #[test]
  fn srgb_test() {
    let s = Srgba8::from_raw(50, 150, 250, 128);
    assert_eq!(s.into_raw(), (50, 150, 250, 128));
    assert_eq!((s.red8(), s.green8(), s.blue8(), s.alpha8()), (8, 78, 244, 128));
    let t = s.rgba8();
    assert_eq!(t.into_raw(), (8, 78, 244, 128));
    assert_eq!((t.red8(), t.green8(), t.blue8(), t.alpha8()), (8, 78, 244, 128));
    let s2 = t.srgba8();
    assert_eq!(s2.into_raw(), (50, 150, 250, 128));
    assert_eq!((s2.red8(), s2.green8(), s2.blue8(), s2.alpha8()), (8, 78, 244, 128));
  }

  #[test]
  fn rgba_pre_test() {
    let c = RgbaPre8::WHITE;
    assert_eq!(c.into_raw(), (255, 255, 255, 255));
    assert_eq!((c.red8(), c.green8(), c.blue8(), c.alpha8()), (255, 255, 255, 255));
    let c = RgbaPre8::BLACK;
    assert_eq!(c.into_raw(), (0, 0, 0, 255));
    assert_eq!((c.red8(), c.green8(), c.blue8(), c.alpha8()), (0, 0, 0, 255));
    let c = RgbaPre8::from_raw(255, 90, 84, 72);
    assert_eq!(c.into_raw(), (255, 90, 84, 72));
    assert_eq!((c.red8(), c.green8(), c.blue8(), c.alpha8()), (255, 90, 84, 72));
    let c = Rgba8::from_raw(255, 90, 84, 72).premultiply();
    assert_eq!(c, RgbaPre8::from_raw(72, 25, 24, 72));
    assert_eq!((c.red8(), c.green8(), c.blue8(), c.alpha8()), (72, 25, 24, 72));
  }
}
