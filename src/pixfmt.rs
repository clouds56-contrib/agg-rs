//! Pixel Format

use crate::buffer::RenderingBuffer;
use crate::color::*;
use crate::math::*;

use crate::Color;
use crate::Pixel;
use crate::Source;

use std::marker::PhantomData;

use std::path::Path;

/// Pixel Format Wrapper around raw pixel component data
#[derive(Debug)]
pub struct Pixfmt<T> {
  rbuf: RenderingBuffer,
  phantom: PhantomData<T>,
}

impl<T> Pixfmt<T>
where
  Pixfmt<T>: Pixel,
{
  /// Create new Pixel Format of width * height * bpp
  ///
  /// Allocates memory of width * height * bpp
  pub fn new(width: usize, height: usize) -> Self {
    if width == 0 || height == 0 {
      panic!("Cannot create pixfmt with 0 width or height");
    }
    Self {
      rbuf: RenderingBuffer::new(width, height, Self::bpp()),
      phantom: PhantomData,
    }
  }
  // /// Fill with a color
  // pub fn fill<C: Color>(&mut self, color: C) {
  //     let (w,h) = (self.width(), self.height());
  //     for i in 0 .. w {
  //         for j in 0 .. h {
  //             self.set((i,j),color);
  //         }
  //     }
  // }

  /// Size of Rendering Buffer in bytes; width * height * bpp
  pub fn size(&self) -> usize {
    self.rbuf.len()
  }
  /// Clear the Image
  ///
  /// All color components are set to 255, including `alpha` if present
  ///
  ///     use agg::prelude::*;
  ///
  ///     // Pixfmt with Rgb8, not Alpha Component
  ///     let mut pix = Pixfmt::<Rgb8>::new(2,2);
  ///     pix.clear();
  ///     let empty = Rgba8::WHITE;
  ///     assert_eq!(pix.get((0,0)), empty);
  ///     assert_eq!(pix.get((0,1)), empty);
  ///     assert_eq!(pix.get((1,0)), empty);
  ///     assert_eq!(pix.get((1,1)), empty);
  ///
  ///     // Pixfmt with Rgba8, including Alpha Component
  ///     let mut pix = Pixfmt::<Rgb8>::new(2,2);
  ///     pix.clear();
  ///     let empty = Rgba8::WHITE;
  ///     assert_eq!(pix.get((0,0)), empty);
  ///     assert_eq!(pix.get((0,1)), empty);
  ///     assert_eq!(pix.get((1,0)), empty);
  ///     assert_eq!(pix.get((1,1)), empty);
  pub fn clear(&mut self) {
    self.rbuf.clear();
  }
  //pub fn from(rbuf: RenderingBuffer) -> Self {
  //    Self { rbuf, phantom: PhantomData }
  //}
  /// Copies the [Color] `c` to pixel at (`x`,`y`)
  ///
  /// Locations outside of the region are igorned
  ///
  ///     use agg::prelude::*;
  ///
  ///     let mut pix = Pixfmt::<Rgba8>::new(1,2);
  ///     let black = Rgba8::BLACK;
  ///     pix.copy_pixel(0,1, black);
  ///     assert_eq!(pix.get((0,0)), Rgba8::from_raw(0,0,0,0));
  ///     assert_eq!(pix.get((0,1)), black);
  ///
  ///     pix.copy_pixel(10,10, black); // Ignored, outside of range
  ///
  /// [Color]: ../trait.Color.html
  pub fn copy_pixel<C: Color>(&mut self, x: usize, y: usize, c: C) {
    if x >= self.rbuf.width || y >= self.rbuf.height {
      return;
    }
    self.set((x, y), c);
  }
  /// Copies the [Color] `c` to pixels from (`x`,`y`) to (`x+n-1`,y)
  ///
  /// Locations outside of the region are ignored
  ///
  ///     use agg::{NamedColor,Source,Pixfmt,Rgb8,Rgba8};
  ///
  ///     let mut pix = Pixfmt::<Rgb8>::new(10,1);
  ///     let black = Rgba8::BLACK;
  ///     pix.copy_hline(0,0,10, black);
  ///     assert_eq!(pix.get((0,0)), black);
  ///     assert_eq!(pix.get((1,0)), black);
  ///     assert_eq!(pix.get((9,0)), black);
  ///
  ///     pix.copy_hline(1,1,10, black); // Ignored, outside of range
  ///
  /// [Color]: ../trait.Color.html
  pub fn copy_hline<C: Color>(&mut self, x: usize, y: usize, n: usize, c: C) {
    if y >= self.rbuf.height || x >= self.rbuf.width || n == 0 {
      return;
    }
    let n = if x + n >= self.rbuf.width {
      self.rbuf.width - x
    } else {
      n
    };
    for i in 0..n {
      self.set((x + i, y), c);
    }
  }
  /// Copies the [Color] `c` to pixels from (`x`,`y`) to (`x`,`y+n-1`)
  ///
  /// Locations outside of the region are ignored
  ///
  ///     use agg::prelude::*;
  ///
  ///     let mut pix = Pixfmt::<Rgba32>::new(1,10);
  ///     let black  = Rgba32::new(0.,0.,0.,1.);
  ///     pix.copy_vline(0,0,10, black);
  ///
  ///     let black8 = black.rgba(); // pix.get() returns Rgba8
  ///     assert_eq!(pix.get((0,0)), black8);
  ///     assert_eq!(pix.get((0,1)), black8);
  ///     assert_eq!(pix.get((0,9)), black8);
  ///
  ///     pix.copy_vline(1,1,10, black); // Ignored, outside of range
  ///
  /// [Color]: ../trait.Color.html
  /// [Rgba8]: ../Color/struct.Rgba8.html
  pub fn copy_vline<C: Color>(&mut self, x: usize, y: usize, n: usize, c: C) {
    if y >= self.rbuf.height || x >= self.rbuf.width || n == 0 {
      return;
    }
    let n = if y + n >= self.rbuf.height {
      self.rbuf.height - y
    } else {
      n
    };
    for i in 0..n {
      self.set((x, y + i), c);
    }
  }

  pub fn from_file<P: AsRef<Path>>(filename: P) -> Result<Self, image::ImageError> {
    let (buf, w, h) = crate::ppm::read_file(filename)?;
    Ok(Self {
      rbuf: RenderingBuffer::from_buf(buf, w, h, 3),
      phantom: PhantomData,
    })
  }
}

impl Source for Pixfmt<Rgba8> {
  fn get(&self, id: (usize, usize)) -> Rgba8 {
    let p = &self.rbuf[id];
    Rgba8::from_slice(p)
  }
}
impl Source for Pixfmt<RgbaPre8> {
  fn get(&self, id: (usize, usize)) -> Rgba8 {
    let p = &self.rbuf[id];
    RgbaPre8::from_slice(p).rgba()
  }
}
impl Source for Pixfmt<Rgb8> {
  fn get(&self, id: (usize, usize)) -> Rgba8 {
    let p = &self.rbuf[id];
    Rgb8::from_slice(p).rgba()
  }
}
impl Source for Pixfmt<Rgba32> {
  fn get(&self, id: (usize, usize)) -> Rgba8 {
    //let n = (id.0 + id.1 * self.rbuf.width) * Pixfmt::<Rgba32>::bpp();
    let p = &self.rbuf[id];
    let red = f32::from_ne_bytes([p[0], p[1], p[2], p[3]]);
    let green = f32::from_ne_bytes([p[4], p[5], p[6], p[7]]);
    let blue = f32::from_ne_bytes([p[8], p[9], p[10], p[11]]);
    let alpha = f32::from_ne_bytes([p[12], p[13], p[14], p[15]]);

    let c = Rgba32::new(red, green, blue, alpha);
    c.rgba()
  }
}

macro_rules! impl_pixel {
  () => {
    /// Height of rendering buffer in pixels
    fn height(&self) -> usize {
      self.rbuf.height
    }
    /// Width of rendering buffer in pixels
    fn width(&self) -> usize {
      self.rbuf.width
    }
    /// Return a underlying raw pixel/component data
    fn as_bytes(&self) -> &[u8] {
      &self.rbuf.data
    }
    fn to_file<P: AsRef<std::path::Path>>(&self, filename: P) -> Result<(), std::io::Error> {
      crate::ppm::write_file(self.as_bytes(), self.width(), self.height(), filename)
    }
  };
}

impl Pixel for Pixfmt<Rgba8> {
  type Color = Rgba8;
  impl_pixel!();
  fn _set(&mut self, id: (usize, usize), n: usize, c: Self::Color) {
    let bpp = Self::bpp();
    let c = c.into_slice();
    let p = &mut self.rbuf[id][..n * bpp];
    for chunk in p.chunks_mut(bpp) {
      chunk.copy_from_slice(&c);
    }
  }
  fn bpp() -> usize {
    4
  }
  fn cover_mask() -> u64 {
    255
  }
  /// Compute **over** operator with coverage
  ///
  /// # Arguments
  ///   - id   - pixel at (`x`,`y`) - Premultiplied
  ///   - c    - Color of Overlaying pixel, not premultiplied
  ///   - cover - Coverage of overlaying pixel, percent in 0p8 format
  ///
  /// # Output
  ///   - lerp(pixel(x,y), color, cover * alpha(color))
  fn blend_pix<C: Color>(&mut self, id: (usize, usize), c: C, cover: u64) {
    let alpha = multiply_u8(c.alpha8(), cover as u8);
    let pix0 = self.get(id); // Rgba8
    let pix = self.mix_pix(pix0, c.rgba(), alpha);
    self.set(id, pix);
  }
}

impl Pixel for Pixfmt<Rgb8> {
  type Color = Rgb8;
  impl_pixel!();
  fn _set(&mut self, id: (usize, usize), n: usize, c: Self::Color) {
    let bpp = Self::bpp();
    let c = c.into_slice();
    let p = &mut self.rbuf[id][..bpp * n];
    for chunk in p.chunks_mut(bpp) {
      chunk.copy_from_slice(&c);
    }
  }
  fn bpp() -> usize {
    3
  }
  fn cover_mask() -> u64 {
    255
  }
  fn blend_pix<C: Color>(&mut self, id: (usize, usize), c: C, cover: u64) {
    let pix0 = self.raw(id);
    let pix = self.mix_pix(pix0, c.rgb(), c.alpha8(), cover);
    self.set(id, pix);
  }
}
impl Pixfmt<Gray8> {
  fn mix_pix(&mut self, id: (usize, usize), c: Gray8, alpha: u8) -> Gray8 {
    let p = Gray8::from_slice(&self.rbuf[id]);
    Gray8::from_raw(lerp_u8(p.luma.0, c.luma.0, alpha), alpha)
  }
  pub fn raw(&self, id: (usize, usize)) -> Gray8 {
    Gray8::from_slice(&self.rbuf[id])
  }
}

impl Pixfmt<Rgba8> {
  /// Computer **over** operator
  ///
  /// # Arguments
  ///   - p     - Current pixel, premultipled
  ///   - c     - Overlaying pixel, not premultipled
  ///   - alpha - Alpha Channel
  ///
  /// # Output
  ///   - lerp(p, c, alpha)
  ///
  /// **Change function name to over**
  fn mix_pix(&mut self, p: Rgba8, c: Rgba8, alpha: u8) -> Rgba8 {
    let red = lerp_u8(p.red8(), c.red8(), alpha);
    let green = lerp_u8(p.green8(), c.green8(), alpha);
    let blue = lerp_u8(p.blue8(), c.blue8(), alpha);
    let alpha = prelerp_u8(p.alpha8(), alpha, alpha);
    Rgba8::from_raw(red, green, blue, alpha)
  }
  fn _blend_pix<C: Color>(&mut self, id: (usize, usize), c: C, cover: u64) {
    let alpha = multiply_u8(c.alpha8(), cover as u8);
    let pix0 = self.get(id);
    let pix = self.mix_pix(pix0, c.rgba(), alpha);
    self.set(id, pix);
  }
}
impl Pixel for Pixfmt<RgbaPre8> {
  type Color = RgbaPre8;
  impl_pixel!();
  fn _set(&mut self, id: (usize, usize), n: usize, c: Self::Color) {
    let bpp = Self::bpp();
    let c = c.into_slice();
    let p = &mut self.rbuf[id][..n * bpp];
    for chunk in p.chunks_mut(bpp) {
      chunk.copy_from_slice(&c);
    }
  }
  fn set<C: Color>(&mut self, id: (usize, usize), c: C) {
    let c = RgbaPre8::from_raw(c.red8(), c.green8(), c.blue8(), c.alpha8());
    self._set(id, 1, c);
  }
  fn setn<C: Color>(&mut self, id: (usize, usize), n: usize, c: C) {
    let c = RgbaPre8::from_raw(c.red8(), c.green8(), c.blue8(), c.alpha8());
    self._set(id, n, c);
  }
  fn bpp() -> usize {
    4
  }
  fn cover_mask() -> u64 {
    255
  }
  fn blend_pix<C: Color>(&mut self, id: (usize, usize), c: C, cover: u64) {
    let p = self.get(id);
    let p0 = RgbaPre8::from_raw(p.red8(), p.green8(), p.blue8(), p.alpha8());
    let p = self.mix_pix(p0, c.rgba(), c.alpha8(), cover);
    self.set(id, p);
  }
}

impl Pixfmt<Rgb8> {
  pub fn raw(&self, id: (usize, usize)) -> Rgb8 {
    let p = &self.rbuf[id];
    Rgb8::from_slice(p)
  }
  /// Compute **over** operator
  ///
  /// # Arguments
  ///   - p     - Current pixel, premultipled (wow that is confusing)
  ///   - c     - Overlaying pixel, not premultiplied
  ///   - alpha - Alpha channel
  ///   - cover - Coverage
  ///
  /// # Output
  ///   - lerp( p, c, alpha * cover)
  fn mix_pix(&mut self, p: Rgb8, c: Rgb8, alpha: u8, cover: u64) -> Rgb8 {
    let alpha = multiply_u8(alpha, cover as u8);
    let red = lerp_u8(p.red8(), c.red8(), alpha);
    let green = lerp_u8(p.green8(), c.green8(), alpha);
    let blue = lerp_u8(p.blue8(), c.blue8(), alpha);
    Rgb8::from_raw(red, green, blue)
  }
}
impl Pixfmt<RgbaPre8> {
  /// Compute **over** operator
  ///
  /// # Arguments
  ///   - p     - Current pixel, premultipled
  ///   - c     - Overlaying pixel, premultiplied
  ///   - alpha - Alpha channel
  ///   - cover - Coverage
  ///
  /// # Output
  ///   - prelerp(p, c * cover, alpha * cover)
  fn mix_pix(&mut self, p: RgbaPre8, c: Rgba8, alpha: u8, cover: u64) -> RgbaPre8 {
    let alpha = multiply_u8(alpha, cover as u8);
    let red = multiply_u8(c.red8(), cover as u8);
    let green = multiply_u8(c.green8(), cover as u8);
    let blue = multiply_u8(c.blue8(), cover as u8);

    let red = prelerp_u8(p.red8(), red, alpha);
    let green = prelerp_u8(p.green8(), green, alpha);
    let blue = prelerp_u8(p.blue8(), blue, alpha);
    let alpha = prelerp_u8(p.alpha8(), alpha, alpha);
    RgbaPre8::from_raw(red, green, blue, alpha)
  }
  pub fn drop_alpha(&self) -> Pixfmt<Rgb8> {
    let buf: Vec<_> = self
      .as_bytes()
      .iter()
      .enumerate()
      .filter(|(i, _)| i % 4 < 3)
      .map(|(_, x)| *x)
      .collect();
    Pixfmt::<Rgb8> {
      rbuf: RenderingBuffer::from_buf(buf, self.width(), self.height(), 3),
      phantom: PhantomData,
    }
  }
}

impl Pixel for Pixfmt<Rgba32> {
  type Color = Rgba32;
  impl_pixel!();
  fn _set(&mut self, id: (usize, usize), n: usize, c: Self::Color) {
    for i in 0..n {
      self.set((id.0 + i, id.1), c);
    }
  }
  fn set<C: Color>(&mut self, id: (usize, usize), c: C) {
    let c = Rgba32::from_color(c);
    assert!(!self.rbuf.data.is_empty());

    self.rbuf[id][..4].copy_from_slice(&c.red.to_ne_bytes());
    self.rbuf[id][4..8].copy_from_slice(&c.green.to_ne_bytes());
    self.rbuf[id][8..12].copy_from_slice(&c.blue.to_ne_bytes());
    self.rbuf[id][12..16].copy_from_slice(&c.alpha.to_ne_bytes());

    //self.rbuf[id][ 4.. 8] = unsafe { std::mem::transmute(c.g) };
    //self.rbuf[id][ 8..12] = unsafe { std::mem::transmute(c.b) };
    //self.rbuf[id][12..16] = unsafe { std::mem::transmute(c.a) };
  }
  fn bpp() -> usize {
    4 * 4
  }
  fn cover_mask() -> u64 {
    unimplemented!("no cover mask")
  }
  fn blend_pix<C: Color>(&mut self, _id: (usize, usize), _c: C, _cover: u64) {
    unimplemented!("no blending");
    /*
    let alpha = multiply_u8(c.alpha8(), cover as u8);
    let pix0 = self.get(id); // Rgba8
    let pix  = self.mix_pix(&pix0, &Rgba8::from(c), alpha);
    self.set(id, &pix);
     */
  }
}

impl Pixel for Pixfmt<Gray8> {
  type Color = Gray8;
  impl_pixel!();
  fn _set(&mut self, id: (usize, usize), n: usize, color: Self::Color) {
    let bpp = Self::bpp();
    let c = color.into_slice();
    let p = &mut self.rbuf[id][..n * bpp];
    for chunk in p.chunks_mut(bpp) {
      chunk.copy_from_slice(&c);
    }
  }
  fn cover_mask() -> u64 {
    255
  }
  fn bpp() -> usize {
    2
  }
  fn blend_pix<C: Color>(&mut self, id: (usize, usize), c: C, cover: u64) {
    let alpha = multiply_u8(c.alpha8(), cover as u8);
    let p0 = self.mix_pix(id, Gray8::from_color(c), alpha);
    self.set(id, p0);
  }
}

use crate::base::RenderingBase;

pub struct PixfmtAlphaBlend<'a, T, C>
where
  T: Pixel,
{
  ren: &'a mut RenderingBase<T>,
  offset: usize,
  //step: usize,
  phantom: PhantomData<C>,
}

impl<'a, T, C> PixfmtAlphaBlend<'a, T, C>
where
  T: Pixel,
{
  pub fn new(ren: &'a mut RenderingBase<T>, offset: usize) -> Self {
    //let step = T::bpp();
    Self {
      ren,
      offset,
      phantom: PhantomData,
    }
  }
}
impl PixfmtAlphaBlend<'_, Pixfmt<Rgb8>, Gray8> {
  fn component(&self, c: Rgb8) -> Gray8 {
    match self.offset {
      0 => Gray8::from_raw(c.red8(), 255),
      1 => Gray8::from_raw(c.green8(), 255),
      2 => Gray8::from_raw(c.blue8(), 255),
      _ => unreachable!("incorrect offset for Rgb8"),
    }
  }
  fn mix_pix(&mut self, id: (usize, usize), c: Gray8, alpha: u8) -> Gray8 {
    let p = self.component(Rgb8::from_slice(&self.ren.pixf.rbuf[id]));
    Gray8::from_raw(lerp_u8(p.luma.0, c.luma.0, alpha), alpha)
  }
}

impl Pixel for PixfmtAlphaBlend<'_, Pixfmt<Rgb8>, Gray8> {
  type Color = Gray8;
  fn width(&self) -> usize {
    self.ren.pixf.width()
  }
  fn height(&self) -> usize {
    self.ren.pixf.height()
  }
  fn as_bytes(&self) -> &[u8] {
    self.ren.pixf.as_bytes()
  }
  fn to_file<P: AsRef<std::path::Path>>(&self, filename: P) -> Result<(), std::io::Error> {
    crate::ppm::write_file(self.as_bytes(), self.width(), self.height(), filename)
  }
  fn _set(&mut self, id: (usize, usize), n: usize, c: Self::Color) {
    let c = c.rgb8();
    for i in 0..n {
      self.ren.pixf.rbuf[(id.0 + i, id.1)][self.offset] = self.component(c).luma.0;
    }
  }
  fn cover_mask() -> u64 {
    Pixfmt::<Rgb8>::cover_mask()
  }
  fn bpp() -> usize {
    Pixfmt::<Rgb8>::bpp()
  }
  fn blend_pix<C: Color>(&mut self, id: (usize, usize), c: C, cover: u64) {
    let alpha = multiply_u8(c.alpha8(), cover as u8);

    let c = c.rgb();
    let c0 = self.component(c);
    let p0 = self.mix_pix(id, c0, alpha);
    self.set(id, p0);
  }

  fn blend_color_vspan<C: Color>(&mut self, x: i64, y: i64, len: i64, colors: &[C], covers: &[u64], cover: u64) {
    assert_eq!(len as usize, colors.len());
    let (x, y) = (x as usize, y as usize);
    if !covers.is_empty() {
      assert_eq!(colors.len(), covers.len());
      for (i, (&color, &cover)) in colors.iter().zip(covers.iter()).enumerate() {
        self.copy_or_blend_pix_with_cover((x, y + i), color, cover);
      }
    } else if cover == Self::cover_mask() {
      for (i, &color) in colors.iter().enumerate() {
        self.copy_or_blend_pix((x, y + i), color);
      }
    } else {
      for (i, &color) in colors.iter().enumerate() {
        self.copy_or_blend_pix_with_cover((x, y + i), color, cover);
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::FromRaw4;
  use crate::NamedColor;
  use crate::Pixel;
  use crate::Pixfmt;
  use crate::Rgb8;
  use crate::Rgba8;
  use crate::Rgba32;
  use crate::RgbaPre8;
  use crate::Source;
  use crate::Srgba8;
  #[test]
  fn pixfmt_test() {
    let mut p = Pixfmt::<Rgb8>::new(10, 10);
    assert_eq!(p.rbuf.data.len(), 300);

    p.copy_pixel(0, 0, Rgb8::BLACK);
    assert_eq!(p.get((0, 0)), Rgba8::BLACK);

    assert_ne!(p.get((1, 0)), Rgba8::WHITE);
    p.copy_pixel(1, 0, Rgb8::WHITE);
    assert_eq!(p.get((1, 0)), Rgba8::WHITE);

    let red = Rgba8::from_raw(255, 0, 0, 128);
    p.copy_hline(0, 1, 10, red);
    for i in 0..10 {
      assert_eq!(p.get((i, 1)), Rgba8::from_raw(255, 0, 0, 255));
    }
    let yellow = Srgba8::from_raw(128, 255, 0, 128);
    p.copy_hline(0, 2, 10, yellow);
    for i in 0..10 {
      assert_eq!(p.get((i, 2)), Rgba8::from_raw(55, 255, 0, 255));
    }
    let fuchsia = Rgba32::from_raw(0.0, 1.0, 1.0, 0.5);
    p.copy_hline(0, 3, 10, fuchsia);
    for i in 0..10 {
      assert_eq!(p.get((i, 3)), Rgba8::from_raw(0, 255, 255, 255));
    }
    p.clear();
    assert_eq!(p.get((0, 3)), Rgba8::from_raw(255, 255, 255, 255));

    let red = Rgba8::from_raw(255, 0, 0, 128);
    p.copy_vline(1, 0, 10, red);
    for i in 0..10 {
      assert_eq!(p.get((1, i)), Rgba8::from_raw(255, 0, 0, 255));
    }
    let yellow = Srgba8::from_raw(128, 255, 0, 128);
    p.copy_vline(2, 0, 10, yellow);
    for i in 0..10 {
      assert_eq!(p.get((2, i)), Rgba8::from_raw(55, 255, 0, 255));
    }
    let fuchsia = Rgba32::from_raw(0.0, 1.0, 1.0, 0.5);
    p.copy_vline(3, 0, 10, fuchsia);
    for i in 0..10 {
      assert_eq!(p.get((3, i)), Rgba8::from_raw(0, 255, 255, 255));
    }

    p.clear();
    p.copy_pixel(11, 11, Rgb8::BLACK);
    for i in 0..10 {
      for j in 0..10 {
        assert_eq!(p.get((i, j)), Rgba8::WHITE);
      }
    }
    p.copy_hline(0, 0, 20, Rgb8::BLACK);
    for i in 0..10 {
      assert_eq!(p.get((i, 0)), Rgba8::BLACK);
    }
    p.copy_hline(5, 1, 20, Rgb8::BLACK);
    for i in 5..10 {
      assert_eq!(p.get((i, 1)), Rgba8::BLACK);
    }

    p.clear();
    p.copy_vline(0, 0, 20, Rgb8::BLACK);
    for i in 0..10 {
      assert_eq!(p.get((0, i)), Rgba8::BLACK);
    }

    p.clear();
    p.copy_vline(1, 5, 20, Rgb8::BLACK);
    for i in 0..5 {
      assert_eq!(p.get((1, i)), Rgba8::WHITE, "pix({},{}): {:?}", 1, i, p.get((1, i)));
    }
    for i in 5..10 {
      assert_eq!(p.get((1, i)), Rgba8::BLACK, "pix({},{}): {:?}", 1, i, p.get((1, i)));
    }
    p.copy_vline(2, 3, 5, Rgb8::BLACK);
    for i in 0..3 {
      assert_eq!(p.get((2, i)), Rgba8::WHITE, "pix({},{}): {:?}", 2, i, p.get((2, i)));
    }
    for i in 3..8 {
      assert_eq!(p.get((2, i)), Rgba8::BLACK, "pix({},{}): {:?}", 2, i, p.get((2, i)));
    }
    for i in 8..10 {
      assert_eq!(p.get((2, i)), Rgba8::WHITE, "pix({},{}): {:?}", 2, i, p.get((2, i)));
    }
  }

  #[test]
  fn pixfmt_rgb8_test() {
    let mut pix = Pixfmt::<Rgb8>::new(1, 1);
    let black = Rgba8::BLACK;
    let white = Rgba8::WHITE;

    pix.copy_pixel(0, 0, Rgba8::from_raw(0, 0, 0, 255));
    assert_eq!(pix.get((0, 0)), black);

    let (alpha, beta, cover) = (255, 255, 255); // Copy Pixel
    pix.copy_pixel(0, 0, Rgba8::from_raw(0, 0, 0, alpha));
    pix.copy_or_blend_pix_with_cover((0, 0), Rgba8::from_raw(255, 255, 255, beta), cover);
    assert_eq!(pix.get((0, 0)), white);

    let (alpha, beta, cover) = (255, 255, 0); // Do Nothing, No Coverage
    pix.copy_pixel(0, 0, Rgba8::from_raw(0, 0, 0, alpha));
    pix.copy_or_blend_pix_with_cover((0, 0), Rgba8::from_raw(255, 255, 255, beta), cover);
    assert_eq!(pix.get((0, 0)), black);

    let (alpha, beta, cover) = (255, 0, 255); // Do Nothing, Transparent
    pix.copy_pixel(0, 0, Rgba8::from_raw(0, 0, 0, alpha));
    pix.copy_or_blend_pix_with_cover((0, 0), Rgba8::from_raw(255, 255, 255, beta), cover);
    assert_eq!(pix.get((0, 0)), black);

    let (alpha, beta, cover) = (255, 255, 128); // Partial Coverage, Blend
    pix.copy_pixel(0, 0, Rgba8::from_raw(0, 0, 0, alpha));
    pix.copy_or_blend_pix_with_cover((0, 0), Rgba8::from_raw(255, 255, 255, beta), cover);
    assert_eq!(pix.get((0, 0)), Rgba8::from_raw(128, 128, 128, 255));

    let (alpha, beta, cover) = (255, 128, 255); // Full Coverage, Alpha Color
    pix.copy_pixel(0, 0, Rgba8::from_raw(0, 0, 0, alpha));
    pix.copy_or_blend_pix_with_cover((0, 0), Rgba8::from_raw(255, 255, 255, beta), cover);
    assert_eq!(pix.get((0, 0)), Rgba8::from_raw(128, 128, 128, 255));

    let (alpha, beta, cover) = (128, 128, 255); // Partial Coverage, Blend
    pix.copy_pixel(0, 0, Rgba8::from_raw(255, 255, 255, alpha));
    assert_eq!(pix.get((0, 0)), Rgba8::from_raw(255, 255, 255, 255)); // Alpha channel is ignored
    pix.copy_or_blend_pix_with_cover((0, 0), Rgba8::from_raw(0, 0, 0, beta), cover);
    assert_eq!(pix.get((0, 0)), Rgba8::from_raw(127, 127, 127, 255));

    let (alpha, beta, cover) = (128, 128, 128); // Partial Coverage, Blend
    pix.copy_pixel(0, 0, Rgba8::from_raw(255, 255, 255, alpha));
    assert_eq!(pix.get((0, 0)), Rgba8::from_raw(255, 255, 255, 255)); // Alpha channel is ignored
    pix.copy_or_blend_pix_with_cover((0, 0), Rgba8::from_raw(0, 0, 0, beta), cover);
    assert_eq!(pix.get((0, 0)), Rgba8::from_raw(191, 191, 191, 255));
  }

  #[test]
  fn pixfmt_rgba8_test() {
    let mut pix = Pixfmt::<Rgba8>::new(1, 1);
    let black = Rgba8::BLACK;
    let white = Rgba8::WHITE;

    pix.copy_pixel(0, 0, Rgba8::from_raw(0, 0, 0, 255));
    assert_eq!(pix.get((0, 0)), black);

    let (alpha, beta, cover) = (255, 255, 255); // Copy Pixel
    pix.copy_pixel(0, 0, Rgba8::from_raw(0, 0, 0, alpha));
    pix.copy_or_blend_pix_with_cover((0, 0), Rgba8::from_raw(255, 255, 255, beta), cover);
    assert_eq!(pix.get((0, 0)), white);

    let (alpha, beta, cover) = (255, 255, 0); // Do Nothing, No Coverage
    pix.copy_pixel(0, 0, Rgba8::from_raw(0, 0, 0, alpha));
    pix.copy_or_blend_pix_with_cover((0, 0), Rgba8::from_raw(255, 255, 255, beta), cover);
    assert_eq!(pix.get((0, 0)), black);

    let (alpha, beta, cover) = (255, 0, 255); // Do Nothing, Transparent
    pix.copy_pixel(0, 0, Rgba8::from_raw(0, 0, 0, alpha));
    pix.copy_or_blend_pix_with_cover((0, 0), Rgba8::from_raw(255, 255, 255, beta), cover);
    assert_eq!(pix.get((0, 0)), black);

    let (alpha, beta, cover) = (255, 255, 128); // Partial Coverage, Blend
    pix.copy_pixel(0, 0, Rgba8::from_raw(0, 0, 0, alpha));
    pix.copy_or_blend_pix_with_cover((0, 0), Rgba8::from_raw(255, 255, 255, beta), cover);
    assert_eq!(pix.get((0, 0)), Rgba8::from_raw(128, 128, 128, 255));

    let (alpha, beta, cover) = (255, 128, 255); // Full Coverage, Alpha Color
    pix.copy_pixel(0, 0, Rgba8::from_raw(0, 0, 0, alpha));
    pix.copy_or_blend_pix_with_cover((0, 0), Rgba8::from_raw(255, 255, 255, beta), cover);
    assert_eq!(pix.get((0, 0)), Rgba8::from_raw(128, 128, 128, 255));

    let (alpha, beta, cover) = (128, 128, 255); // Partial Coverage, Blend
    pix.copy_pixel(0, 0, Rgba8::from_raw(255, 255, 255, alpha));
    assert_eq!(pix.get((0, 0)), Rgba8::from_raw(255, 255, 255, 128));
    pix.copy_or_blend_pix_with_cover((0, 0), Rgba8::from_raw(0, 0, 0, beta), cover);
    assert_eq!(pix.get((0, 0)), Rgba8::from_raw(127, 127, 127, 192));

    let (alpha, beta, cover) = (128, 128, 128); // Partial Coverage, Blend
    pix.copy_pixel(0, 0, Rgba8::from_raw(255, 255, 255, alpha));
    assert_eq!(pix.get((0, 0)), Rgba8::from_raw(255, 255, 255, 128)); // Alpha channel is ignored
    pix.copy_or_blend_pix_with_cover((0, 0), Rgba8::from_raw(0, 0, 0, beta), cover);
    assert_eq!(pix.get((0, 0)), Rgba8::from_raw(191, 191, 191, 160));
  }

  #[test]
  fn pixfmt_rgba8pre_test() {
    let mut pix = Pixfmt::<RgbaPre8>::new(1, 1);
    let black = Rgba8::BLACK;
    let white = Rgba8::WHITE;

    pix.copy_pixel(0, 0, Rgba8::from_raw(0, 0, 0, 255));
    assert_eq!(pix.get((0, 0)), black);

    let (alpha, beta, cover) = (255, 255, 255); // Copy Pixel
    pix.copy_pixel(0, 0, Rgba8::from_raw(0, 0, 0, alpha));
    pix.copy_or_blend_pix_with_cover((0, 0), Rgba8::from_raw(255, 255, 255, beta), cover);
    assert_eq!(pix.get((0, 0)), white);

    let (alpha, beta, cover) = (255, 255, 0); // Do Nothing, No Coverage
    pix.copy_pixel(0, 0, Rgba8::from_raw(0, 0, 0, alpha));
    pix.copy_or_blend_pix_with_cover((0, 0), Rgba8::from_raw(255, 255, 255, beta), cover);
    assert_eq!(pix.get((0, 0)), black);

    let (alpha, beta, cover) = (255, 0, 255); // Do Nothing, Transparent
    pix.copy_pixel(0, 0, Rgba8::from_raw(0, 0, 0, alpha));
    pix.copy_or_blend_pix_with_cover((0, 0), Rgba8::from_raw(255, 255, 255, beta), cover);
    assert_eq!(pix.get((0, 0)), black);

    let (alpha, beta, cover) = (255, 255, 128); // Partial Coverage, Blend
    pix.copy_pixel(0, 0, Rgba8::from_raw(0, 0, 0, alpha));
    pix.copy_or_blend_pix_with_cover((0, 0), Rgba8::from_raw(255, 255, 255, beta), cover);
    assert_eq!(pix.get((0, 0)), Rgba8::from_raw(128, 128, 128, 255));

    let (alpha, beta, cover) = (255, 128, 255); // Full Coverage, Alpha Color
    pix.copy_pixel(0, 0, Rgba8::from_raw(0, 0, 0, alpha));
    pix.copy_or_blend_pix_with_cover((0, 0), Rgba8::from_raw(255, 255, 255, beta), cover);
    assert_eq!(pix.get((0, 0)), Rgba8::from_raw(255, 255, 255, 255));

    let (alpha, beta, cover) = (128, 128, 255); // Partial Coverage, Blend
    pix.copy_pixel(0, 0, Rgba8::from_raw(255, 255, 255, alpha));
    assert_eq!(pix.get((0, 0)), Rgba8::from_raw(255, 255, 255, 128));
    pix.copy_or_blend_pix_with_cover((0, 0), Rgba8::from_raw(0, 0, 0, beta), cover);
    assert_eq!(pix.get((0, 0)), Rgba8::from_raw(127, 127, 127, 192));

    let (alpha, beta, cover) = (128, 128, 128); // Partial Coverage, Blend
    pix.copy_pixel(0, 0, Rgba8::from_raw(255, 255, 255, alpha));
    assert_eq!(pix.get((0, 0)), Rgba8::from_raw(255, 255, 255, 128)); // Alpha channel is ignored
    pix.copy_or_blend_pix_with_cover((0, 0), Rgba8::from_raw(0, 0, 0, beta), cover);
    assert_eq!(pix.get((0, 0)), Rgba8::from_raw(191, 191, 191, 160));
  }

  #[test]
  fn test_fill() {
    let (w, h) = (3, 5);
    let mut pix = Pixfmt::<RgbaPre8>::new(w, h);
    let black = Rgba8::BLACK;
    let white = Rgba8::WHITE;

    pix.clear();
    pix.fill(black);
    for y in 0..h {
      for x in 0..w {
        assert_eq!(pix.get((x, y)), black, "pix({},{}): {:?}", x, y, pix.get((x, y)));
      }
    }

    pix.copy_hline(0, 0, w, white);
    for x in 0..w {
      assert_eq!(pix.get((x, 0)), white, "pix({},0): {:?}", x, pix.get((x, 0)));
    }
  }
}
