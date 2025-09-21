use crate::math::multiply_u8;
use crate::{Color, FromColor, FromRaw4, Gray8, IntoRaw2, IntoRaw3, IntoRaw4, Pixfmt, PixfmtAlphaBlend, Rgb8, Rgba32, Rgba8, RgbaPre8, Source};

/// Drawing and pixel related routines
pub trait Pixel {
  type Color: Color + FromColor;
  fn cover_mask() -> u64;
  fn bpp() -> usize;
  fn as_bytes(&self) -> &[u8];
  fn to_file<P: AsRef<std::path::Path>>(&self, filename: P) -> Result<(), std::io::Error>;
  fn width(&self) -> usize;
  fn height(&self) -> usize;
  fn _set(&mut self, id: (usize, usize), n: usize, c: Self::Color);
  fn set<C: Color>(&mut self, id: (usize, usize), c: C) {
    self._set(id, 1, Self::Color::from_color(c));
  }
  fn setn<C: Color>(&mut self, id: (usize, usize), n: usize, c: C) {
    self._set(id, n, Self::Color::from_color(c));
  }
  /// Fill the data with the specified `color`
  fn fill<C: Color>(&mut self, color: C) {
    let (w, h) = (self.width(), self.height());
    for i in 0..h {
      self.setn((0, i), w, color);
    }
  }
  fn blend_pix<C: Color>(&mut self, id: (usize, usize), c: C, cover: u64);
  /// Copy or blend a pixel at `id` with `color`
  ///
  /// If `color` [`is_opaque`], the color is copied directly to the pixel,
  ///   otherwise the color is blended with the pixel at `id`
  ///
  /// If `color` [`is_transparent`] nothing is done
  ///
  /// [`is_opaque`]: ../trait.Color.html#method.is_opaque
  /// [`is_transparent`]: ../trait.Color.html#method.is_transparent
  fn copy_or_blend_pix<C: Color>(&mut self, id: (usize, usize), color: C) {
    if !color.is_transparent() {
      if color.is_opaque() {
        self.set(id, color);
      } else {
        self.blend_pix(id, color, 255);
      }
    }
  }
  /// Copy or blend a pixel at `id` with `color` and a `cover`
  ///
  /// If `color` [`is_opaque`] *and* `cover` equals [`cover_mask`] then
  ///   the color is copied to the pixel at `id`, otherwise the `color`
  ///   is blended with the pixel at `id` considering the amount of `cover`
  ///
  /// If `color` [`is_transparent`] nothing is done
  ///
  ///     use agg::prelude::*;
  ///
  ///     let mut pix = Pixfmt::<Rgb8>::create(1,1);
  ///     let black  = Rgba8::BLACK;
  ///     let white  = Rgba8::WHITE;
  ///     pix.copy_pixel(0,0,black);
  ///     assert_eq!(pix.get((0,0)), black);
  ///
  ///     let (alpha, cover) = (255, 255); // Copy Pixel
  ///     let color = Rgba8::from_raw(255,255,255,alpha);
  ///     pix.copy_or_blend_pix_with_cover((0,0), color, cover);
  ///     assert_eq!(pix.get((0,0)), white);
  ///
  ///     let (alpha, cover) = (255, 128); // Partial Coverage, Blend
  ///     let color = Rgba8::from_raw(255,255,255,alpha);
  ///     pix.copy_pixel(0,0,black);
  ///     pix.copy_or_blend_pix_with_cover((0,0), color, cover);
  ///     assert_eq!(pix.get((0,0)), Rgba8::from_raw(128,128,128,255));
  ///
  ///     let (alpha, cover) = (128, 255); // Partial Coverage, Blend
  ///     let color = Rgba8::from_raw(255,255,255,alpha);
  ///     pix.copy_pixel(0,0,black);
  ///     pix.copy_or_blend_pix_with_cover((0,0), color, cover);
  ///     assert_eq!(pix.get((0,0)), Rgba8::from_raw(128,128,128,255));
  ///
  /// [`is_opaque`]: ../trait.Color.html#method.is_opaque
  /// [`is_transparent`]: ../trait.Color.html#method.is_transparent
  /// [`cover_mask`]: ../trait.Pixel.html#method.cover_mask
  fn copy_or_blend_pix_with_cover<C: Color>(&mut self, id: (usize, usize), color: C, cover: u64) {
    if !color.is_transparent() {
      if color.is_opaque() && cover == Self::cover_mask() {
        self.set(id, color);
      } else {
        self.blend_pix(id, color, cover);
      }
    }
  }
  /// Copy or Blend a single `color` from (`x`,`y`) to (`x+len-1`,`y`)
  ///    with `cover`
  fn blend_hline<C: Color>(&mut self, x: i64, y: i64, len: i64, color: C, cover: u64) {
    if color.is_transparent() {
      return;
    }
    let (x, y, len) = (x as usize, y as usize, len as usize);
    if color.is_opaque() && cover == Self::cover_mask() {
      self.setn((x, y), len, color);
    } else {
      for i in 0..len {
        self.blend_pix((x + i, y), color, cover);
      }
    }
  }
  /// Blend a single `color` from (`x`,`y`) to (`x+len-1`,`y`) with collection
  ///   of `covers`
  fn blend_solid_hspan<C: Color>(&mut self, x: i64, y: i64, len: i64, color: C, covers: &[u64]) {
    assert_eq!(len as usize, covers.len());
    for (i, &cover) in covers.iter().enumerate() {
      self.blend_hline(x + i as i64, y, 1, color, cover);
    }
  }
  /// Copy or Blend a single `color` from (`x`,`y`) to (`x`,`y+len-1`)
  ///    with `cover`
  fn blend_vline<C: Color>(&mut self, x: i64, y: i64, len: i64, color: C, cover: u64) {
    if color.is_transparent() {
      return;
    }
    let (x, y, len) = (x as usize, y as usize, len as usize);
    if color.is_opaque() && cover == Self::cover_mask() {
      for i in 0..len {
        self.set((x, y + i), color);
      }
    } else {
      for i in 0..len {
        self.blend_pix((x, y + i), color, cover);
      }
    }
  }
  /// Blend a single `color` from (`x`,`y`) to (`x`,`y+len-1`) with collection
  ///   of `covers`
  fn blend_solid_vspan<C: Color>(&mut self, x: i64, y: i64, len: i64, color: C, covers: &[u64]) {
    assert_eq!(len as usize, covers.len());
    for (i, &cover) in covers.iter().enumerate() {
      self.blend_vline(x, y + i as i64, 1, color, cover);
    }
  }
  /// Blend a collection of `colors` from (`x`,`y`) to (`x+len-1`,`y`) with
  ///   either a collection of `covers` or a single `cover`
  ///
  /// A collection of `covers` takes precedance over a single `cover`
  fn blend_color_hspan<C: Color>(&mut self, x: i64, y: i64, len: i64, colors: &[C], covers: &[u64], cover: u64) {
    assert_eq!(len as usize, colors.len());
    let (x, y) = (x as usize, y as usize);
    if !covers.is_empty() {
      assert_eq!(colors.len(), covers.len());
      for (i, (&color, &cover)) in colors.iter().zip(covers.iter()).enumerate() {
        self.copy_or_blend_pix_with_cover((x + i, y), color, cover);
      }
    } else if cover == 255 {
      for (i, &color) in colors.iter().enumerate() {
        self.copy_or_blend_pix((x + i, y), color);
      }
    } else {
      for (i, &color) in colors.iter().enumerate() {
        self.copy_or_blend_pix_with_cover((x + i, y), color, cover);
      }
    }
  }
  /// Blend a collection of `colors` from (`x`,`y`) to (`x`,`y+len-1`) with
  ///   either a collection of `covers` or a single `cover`
  ///
  /// A collection of `covers` takes precedance over a single `cover`
  fn blend_color_vspan<C: Color>(&mut self, x: i64, y: i64, len: i64, colors: &[C], covers: &[u64], cover: u64) {
    assert_eq!(len as usize, colors.len());
    let (x, y) = (x as usize, y as usize);
    if !covers.is_empty() {
      assert_eq!(colors.len(), covers.len());
      for (i, (&color, &cover)) in colors.iter().zip(covers.iter()).enumerate() {
        self.copy_or_blend_pix_with_cover((x, y + i), color, cover);
      }
    } else if cover == 255 {
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
