//! Alphamask Adapator

//use crate::math::blend_pix;
use crate::color::Gray8;
use crate::pixels::Pixfmt;

use crate::Color;
use crate::FromRaw4;
use crate::Pixel;
use crate::RealLike;
use crate::Source;
use crate::color::Rgba8;
use crate::math::lerp_u8;
use crate::math::multiply_u8;
use crate::U8;

/// Alpha Mask Adaptor
pub struct PixfmtAlphaMask<T> {
  pub rgb: Pixfmt<T>,
  pub alpha: Pixfmt<Gray8>,
}

impl<T> PixfmtAlphaMask<T> {
  /// Create a new Alpha Mask Adapator from a two PixelFormats
  pub fn new(rgb: Pixfmt<T>, alpha: Pixfmt<Gray8>) -> Self {
    Self { rgb, alpha }
  }

  pub fn into_rendering_base(self) -> crate::RenderingBase<Self>
  where
    Self: Pixel,
  {
    crate::RenderingBase::new(self)
  }
}

impl<T> Pixel for PixfmtAlphaMask<T>
where
  Pixfmt<T>: Pixel + Source,
{
  type Color = <Pixfmt<T> as Pixel>::Color;

  fn width(&self) -> usize {
    self.rgb.width()
  }
  fn height(&self) -> usize {
    self.rgb.height()
  }
  fn as_bytes(&self) -> &[u8] {
    self.rgb.as_bytes()
  }
  fn to_file<P: AsRef<std::path::Path>>(&self, filename: P) -> Result<(), std::io::Error> {
    self.rgb.to_file(filename)
  }
  fn _set(&mut self, id: (usize, usize), n: usize, c: Self::Color) {
    self.rgb._set(id, n, c);
  }
  fn is_cover_full<U: RealLike>(_cover: &U) -> bool {
    // since we use the alpha channel from the gray scale image
    // we can not assume that the cover is full
    false
  }

  /// Blend a set of colors starting at (x,y) with a length
  ///
  /// Background color is from the rgb image and
  /// alpha form the gray scale
  ///
  /// Calls blend_pix
  //
  // From https://stackoverflow.com/a/746937 :
  // out = alpha * new + (1 - alpha) * old
  //   p[j]  = out
  //   alpha = alpha
  //   new   = c
  //   old   = p[j]
  fn blend_pix<C: Color, U: RealLike>(&mut self, id: (usize, usize), c: C, _cover: U) {
    let (x, y) = id;
    let pix = &mut self.rgb.get((x, y));
    let cover = self.alpha.get((x, y)).luma;
    let pix = blend_pix(pix, &c, cover);
    self.rgb.set((x, y), pix);
  }
}
/// Blend foreground and background pixels with an cover value
///
/// Color components are computed by:
///
/// out = (alpha * cover) * (c - p)
///
/// Computations are conducted using fixed point math
///
/// see [Alpha Compositing](https://en.wikipedia.org/wiki/Alpha_compositing)
fn blend_pix<C1: Color, C2: Color, T: RealLike>(p: &C1, c: &C2, cover: T) -> Rgba8 {
  assert!(c.alpha64() >= 0.0);
  assert!(c.alpha64() <= 1.0);

  let alpha = multiply_u8(c.alpha8(), cover.as_::<U8>().0);

  let red = lerp_u8(p.red8(), c.red8(), alpha);
  let green = lerp_u8(p.green8(), c.green8(), alpha);
  let blue = lerp_u8(p.blue8(), c.blue8(), alpha);
  let alpha = lerp_u8(p.alpha8(), c.alpha8(), alpha);

  Rgba8::from_raw(red, green, blue, alpha)
}
