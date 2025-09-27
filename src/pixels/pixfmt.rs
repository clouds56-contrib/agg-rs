use std::marker::PhantomData;

use crate::{
  Color, FromRaw2, FromRaw3, FromRaw4, Gray8, NamedColor, Pixel, Position, RenderingBuffer, Rgb8, Rgba32, Rgba8, RgbaPre8
};

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
  pub fn new(rbuf: RenderingBuffer) -> Self {
    // if rbuf.width == 0 || rbuf.height == 0 || rbuf.len() == 0 || rbuf.len() % Self::bpp() != 0 {
    //   panic!("Cannot create pixfmt with 0 width or height");
    // }
    Self {
      rbuf,
      phantom: PhantomData,
    }
  }

  /// Create new Pixel Format of `width` x `height``
  ///
  /// Allocates memory of `width * height * bpp`
  pub fn create(width: Position, height: Position) -> Self {
    let (width, height) = (width as usize, height as usize);
    if width == 0 || height == 0 {
      panic!("Cannot create pixfmt with 0 width or height");
    }
    Self {
      rbuf: RenderingBuffer::new(width, height, Self::bpp()),
      phantom: PhantomData,
    }
  }

  /// Create new Pixel Format of `width` x `height``
  ///
  /// Allocates memory of `width * height * bpp`
  pub fn create_flipped(width: Position, height: Position) -> Self {
    let (width, height) = (width as usize, height as usize);
    if width == 0 || height == 0 {
      panic!("Cannot create pixfmt with 0 width or height");
    }
    Self {
      rbuf: RenderingBuffer::new(width, height, Self::bpp()).flipped(),
      phantom: PhantomData,
    }
  }

  pub fn into_rendering_base(self) -> crate::RenderingBase<Self>
  where
    Self: Pixel,
  {
    crate::RenderingBase::new(self)
  }

  /// Size of Rendering Buffer in bytes; width * height * bpp
  pub fn size(&self) -> usize {
    self.rbuf.len()
  }
  /// Clear the Image
  ///
  /// All color components are set to 255, including `alpha` if present
  ///
  /// ```
  /// use agg::prelude::*;
  ///
  /// // Pixfmt with Rgb8, not Alpha Component
  /// let mut pix = Pixfmt::<Rgb8>::create(2, 2);
  /// pix.clear();
  /// let empty = Rgb8::WHITE;
  /// assert_eq!(pix.get((0, 0)), empty);
  /// assert_eq!(pix.get((0, 1)), empty);
  /// assert_eq!(pix.get((1, 0)), empty);
  /// assert_eq!(pix.get((1, 1)), empty);
  ///
  /// // Pixfmt with Rgba8, including Alpha Component
  /// let mut pix = Pixfmt::<Rgba8>::create(2, 2);
  /// pix.clear();
  /// let empty = Rgba8::WHITE;
  /// assert_eq!(pix.get((0, 0)), empty);
  /// assert_eq!(pix.get((0, 1)), empty);
  /// assert_eq!(pix.get((1, 0)), empty);
  /// assert_eq!(pix.get((1, 1)), empty);
  ///
  /// let mut pix = Pixfmt::<Rgba32>::create(2, 2);
  /// pix.clear();
  /// let empty = Rgba32::WHITE;
  /// assert_eq!(pix.get((0, 0)), empty);
  /// assert_eq!(pix.get((0, 1)), empty);
  /// assert_eq!(pix.get((1, 0)), empty);
  /// assert_eq!(pix.get((1, 1)), empty);
  /// ```
  pub fn clear(&mut self)
  where
    <Self as Pixel>::Color: NamedColor,
  {
    self.fill(<Self as Pixel>::Color::WHITE);
  }
  //pub fn from(rbuf: RenderingBuffer) -> Self {
  //    Self { rbuf, phantom: PhantomData }
  //}
  /// Copies the [Color] `c` to pixel at (`x`,`y`)
  ///
  /// Locations outside of the region are igorned
  ///
  /// ```
  /// use agg::prelude::*;
  ///
  /// let mut pix = Pixfmt::<Rgba8>::create(1, 2);
  /// let black = Rgba8::BLACK;
  /// pix.copy_pixel(0, 1, black);
  /// assert_eq!(pix.get((0, 0)), Rgba8::from_raw(0, 0, 0, 0));
  /// assert_eq!(pix.get((0, 1)), black);
  ///
  /// pix.copy_pixel(10, 10, black); // Ignored, outside of range
  /// ```
  ///
  /// [Color]: ../trait.Color.html
  pub fn copy_pixel<C: Color>(&mut self, x: Position, y: Position, c: C) {
    if x >= self.width() || y >= self.height() {
      return;
    }
    self.set((x, y), c);
  }
  /// Copies the [Color] `c` to pixels from (`x`,`y`) to (`x+n-1`,y)
  ///
  /// Locations outside of the region are ignored
  ///
  /// ```
  /// use agg::{NamedColor, Pixfmt, Rgb8, Rgba8, Source};
  ///
  /// let mut pix = Pixfmt::<Rgb8>::create(10, 1);
  /// let black = Rgb8::BLACK;
  /// pix.copy_hline(0, 0, 10, black);
  /// assert_eq!(pix.get((0, 0)), black);
  /// assert_eq!(pix.get((1, 0)), black);
  /// assert_eq!(pix.get((9, 0)), black);
  ///
  /// pix.copy_hline(1, 1, 10, black); // Ignored, outside of range
  /// ```
  ///
  /// [Color]: ../trait.Color.html
  pub fn copy_hline<C: Color>(&mut self, x: Position, y: Position, n: Position, c: C) {
    let (width, height) = (self.width(), self.height());
    if y >= height || x >= width || n == 0 {
      return;
    }
    let n = if x + n >= width {
      width - x
    } else {
      n
    };
    for i in 0..n {
      self.set((x + i, y), c);
    }
  }
  /// Copies the [`c: Color`](Color) to pixels from `(x, y)` to `(x, y+n-1)`
  ///
  /// Locations outside of the region are ignored
  ///
  /// ```
  /// use agg::prelude::*;
  ///
  /// let mut pix = Pixfmt::<Rgba32>::create(1, 10);
  /// let black = Rgba32::new(0., 0., 0., 1.);
  /// pix.copy_vline(0, 0, 10, black);
  ///
  /// let black8 = black.rgba(); // pix.get() returns Rgba8
  /// assert_eq!(pix.get((0, 0)), black8);
  /// assert_eq!(pix.get((0, 1)), black8);
  /// assert_eq!(pix.get((0, 9)), black8);
  ///
  /// pix.copy_vline(1, 1, 10, black); // Ignored, outside of range
  /// ```
  ///
  /// [Color]: ../trait.Color.html
  /// [Rgba8]: ../Color/struct.Rgba8.html
  pub fn copy_vline<C: Color>(&mut self, x: Position, y: Position, n: Position, c: C) {
    let (width, height) = (self.width(), self.height());
    if y >= height || x >= width || n == 0 {
      return;
    }
    let n = if y + n >= height {
      height - y
    } else {
      n
    };
    for i in 0..n {
      self.set((x, y + i), c);
    }
  }

  pub fn from_file<P: AsRef<std::path::Path>>(filename: P) -> Result<Self, image::ImageError> {
    let (buf, w, h) = crate::utils::read_file(filename)?;
    Ok(Self {
      rbuf: RenderingBuffer::from_buf(buf, w, h, 3),
      phantom: PhantomData,
    })
  }
}

impl Pixfmt<RgbaPre8> {
  pub fn drop_alpha(&self) -> Pixfmt<Rgb8> {
    let buf: Vec<_> = self
      .as_bytes()
      .iter()
      .enumerate()
      .filter(|(i, _)| i % 4 < 3)
      .map(|(_, x)| *x)
      .collect();
    Pixfmt::<Rgb8>::new(RenderingBuffer::from_buf(buf, self.width() as usize, self.height() as usize, 3))
  }
}

/// Access Pixel source color
pub trait Source {
  type Color: Color;
  fn get(&self, id: (Position, Position)) -> Self::Color;
}

impl Source for Pixfmt<Rgba8> {
  type Color = Rgba8;
  fn get(&self, (x, y): (Position, Position)) -> Self::Color {
    Rgba8::from_slice(self.rbuf.get_pixel(x as usize, y as usize))
  }
}
impl Source for Pixfmt<RgbaPre8> {
  type Color = Rgba8;
  fn get(&self, (x, y): (Position, Position)) -> Self::Color {
    RgbaPre8::from_slice(self.rbuf.get_pixel(x as usize, y as usize)).rgba()
  }
}
impl Source for Pixfmt<Rgb8> {
  type Color = Rgb8;
  fn get(&self, (x, y): (Position, Position)) -> Self::Color {
    Rgb8::from_slice(self.rbuf.get_pixel(x as usize, y as usize))
  }
}
impl Source for Pixfmt<Gray8> {
  type Color = Gray8;
  fn get(&self, (x, y): (Position, Position)) -> Self::Color {
    Gray8::from_slice(self.rbuf.get_pixel(x as usize, y as usize))
  }
}
impl Source for Pixfmt<Rgba32> {
  type Color = Rgba32;
  fn get(&self, (x, y): (Position, Position)) -> Self::Color {
    //let n = (id.0 + id.1 * self.rbuf.width) * Pixfmt::<Rgba32>::bpp();
    let p = self.rbuf.get_pixel(x as usize, y as usize);
    let red = f32::from_ne_bytes([p[0], p[1], p[2], p[3]]);
    let green = f32::from_ne_bytes([p[4], p[5], p[6], p[7]]);
    let blue = f32::from_ne_bytes([p[8], p[9], p[10], p[11]]);
    let alpha = f32::from_ne_bytes([p[12], p[13], p[14], p[15]]);

    Rgba32::new(red, green, blue, alpha)
  }
}
