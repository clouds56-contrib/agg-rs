use std::marker::PhantomData;

use crate::{buffer::RenderingBuffer, math::{lerp_u8, multiply_u8, prelerp_u8}, Color, FromRaw2, FromRaw3, FromRaw4, Gray8, Pixel, Rgb8, Rgba32, Rgba8, RgbaPre8};


/// Pixel Format Wrapper around raw pixel component data
#[derive(Debug)]
pub struct Pixfmt<T> {
  pub(super) rbuf: RenderingBuffer,
  phantom: PhantomData<T>,
}

impl<T> Pixfmt<T>
where
  Pixfmt<T>: Pixel,
{
  pub(crate) fn new(rbuf: RenderingBuffer) -> Self {
    // if rbuf.width == 0 || rbuf.height == 0 || rbuf.len() == 0 || rbuf.len() % Self::bpp() != 0 {
    //   panic!("Cannot create pixfmt with 0 width or height");
    // }
    Self {
      rbuf,
      phantom: PhantomData,
    }
  }

  /// Create new Pixel Format of width * height * bpp
  ///
  /// Allocates memory of width * height * bpp
  pub fn create(width: usize, height: usize) -> Self {
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
  ///     let mut pix = Pixfmt::<Rgb8>::create(2,2);
  ///     pix.clear();
  ///     let empty = Rgba8::WHITE;
  ///     assert_eq!(pix.get((0,0)), empty);
  ///     assert_eq!(pix.get((0,1)), empty);
  ///     assert_eq!(pix.get((1,0)), empty);
  ///     assert_eq!(pix.get((1,1)), empty);
  ///
  ///     // Pixfmt with Rgba8, including Alpha Component
  ///     let mut pix = Pixfmt::<Rgb8>::create(2,2);
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
  ///     let mut pix = Pixfmt::<Rgba8>::create(1,2);
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
  ///     let mut pix = Pixfmt::<Rgb8>::create(10,1);
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
  ///     let mut pix = Pixfmt::<Rgba32>::create(1,10);
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

  pub fn from_file<P: AsRef<std::path::Path>>(filename: P) -> Result<Self, image::ImageError> {
    let (buf, w, h) = crate::ppm::read_file(filename)?;
    Ok(Self {
      rbuf: RenderingBuffer::from_buf(buf, w, h, 3),
      phantom: PhantomData,
    })
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
  pub fn mix_pix(&mut self, p: Rgba8, c: Rgba8, alpha: u8) -> Rgba8 {
    let red = lerp_u8(p.red8(), c.red8(), alpha);
    let green = lerp_u8(p.green8(), c.green8(), alpha);
    let blue = lerp_u8(p.blue8(), c.blue8(), alpha);
    let alpha = prelerp_u8(p.alpha8(), alpha, alpha);
    Rgba8::from_raw(red, green, blue, alpha)
  }
  pub fn _blend_pix<C: Color>(&mut self, id: (usize, usize), c: C, cover: u64) {
    let alpha = multiply_u8(c.alpha8(), cover as u8);
    let pix0 = self.get(id);
    let pix = self.mix_pix(pix0, c.rgba(), alpha);
    self.set(id, pix);
  }
}


impl Pixfmt<Gray8> {
  pub fn mix_pix(&mut self, id: (usize, usize), c: Gray8, alpha: u8) -> Gray8 {
    let p = Gray8::from_slice(&self.rbuf[id]);
    Gray8::from_raw(lerp_u8(p.luma.0, c.luma.0, alpha), alpha)
  }
  pub fn raw(&self, id: (usize, usize)) -> Gray8 {
    Gray8::from_slice(&self.rbuf[id])
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
  pub fn mix_pix(&mut self, p: Rgb8, c: Rgb8, alpha: u8, cover: u64) -> Rgb8 {
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
  pub fn mix_pix(&mut self, p: RgbaPre8, c: Rgba8, alpha: u8, cover: u64) -> RgbaPre8 {
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
    Pixfmt::<Rgb8>::new(RenderingBuffer::from_buf(buf, self.width(), self.height(), 3))
  }
}

/// Access Pixel source color
pub trait Source {
  fn get(&self, id: (usize, usize)) -> Rgba8;
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
