//! Rendering buffer

/// Rendering Buffer
///
/// Data is stored as row-major order (C-format)
#[derive(Debug, Default)]
pub struct RenderingBuffer {
  /// Pixel / Component level data of Image
  pub data: Vec<u8>,
  /// Image Width in pixels
  pub width: usize,
  /// Image Height in pixels
  pub height: usize,
  pub flip: bool,
  /// Bytes per pixel or number of color components
  pub bpp: usize,
}

impl RenderingBuffer {
  /// Create a new buffer of width, height, and bpp
  ///
  /// Data for the Image is allocated
  pub fn new(width: usize, height: usize, bpp: usize) -> Self {
    RenderingBuffer {
      width,
      height,
      bpp,
      flip: false,
      data: vec![0u8; width * height * bpp],
    }
  }
  /// Set the flip-flag, which causes the row order to be reversed
  /// This just set the flag, does not re-arrange the data
  pub fn fliped(mut self) -> Self {
    self.flip = !self.flip;
    self
  }
  /// Is the underlying data empty
  pub fn is_empty(&self) -> bool {
    self.data.is_empty()
  }
  /// Size of underlying [`data`](Self::data)
  pub fn len(&self) -> usize {
    self.data.len()
  }
  /// Stride of row in bytes, `width * bpp`
  pub fn stride(&self) -> usize {
    self.width * self.bpp
  }
  /// Underlying [`data`](Self::data) as slice
  pub fn buf(&self) -> &[u8] {
    &self.data
  }
  /// Underlying [`data`](Self::data) as mutable slice
  pub fn buf_mut(&mut self) -> &mut [u8] {
    &mut self.data
  }
  /// Get the index of the start of row `y`, `0 <= y < height`
  ///
  /// This would consider the [`flip`](Self::flip) flag
  pub fn row_index(&self, y: usize) -> usize {
    debug_assert!(y < self.height, "request {} >= {} height :: index", y, self.height);
    if self.flip {
      (self.height - 1 - y) * self.stride()
    } else {
      y * self.stride()
    }
  }
  /// Get a slice of row `y`, `0 <= y < height`
  ///
  /// The length of the slice is [`stride()`](Self::stride)
  pub fn row(&self, y: usize) -> &[u8] {
    let i = self.row_index(y);
    &self.data[i..i + self.stride()]
  }
  /// Get a mutable slice of row `y`, `0 <= y < height`
  ///
  /// The length of the slice is [`stride()`](Self::stride)
  pub fn row_mut(&mut self, y: usize) -> &mut [u8] {
    let i = self.row_index(y);
    let stride = self.stride();
    &mut self.data[i..i + stride]
  }
  /// Get the index of pixel at (`x`,`y`), `0 <= x < width`, `0 <= y < height`
  ///
  /// This would consider the [`flip`](Self::flip) flag
  pub fn offset_index(&self, x: usize, y: usize) -> usize {
    debug_assert!(x < self.width, "request {} >= {} width :: index", x, self.width);
    self.row_index(y) + x * self.bpp
  }
  /// Get a slice starting at pixel (`x`,`y`), `0 <= x < width`, `0 <= y < height`
  ///
  /// The slice goes to the end of the underlying [`data`](Self::data)
  pub fn offset(&self, x: usize, y: usize) -> &[u8] {
    let i = self.offset_index(x, y);
    &self.data[i..]
  }
  /// Get a mutable slice starting at pixel (`x`,`y`), `0 <= x < width`, `0 <= y < height`
  ///
  /// The slice goes to the end of the underlying [`data`](Self::data)
  pub fn offset_mut(&mut self, x: usize, y: usize) -> &mut [u8] {
    let i = self.offset_index(x, y);
    &mut self.data[i..]
  }
  /// Get a slice of pixel at (`x`,`y`), `0 <= x < width`, `0 <= y < height`
  ///
  /// The length of the slice is `bpp`
  pub fn get_pixel(&self, p: (usize, usize)) -> &[u8] {
    let i = self.offset_index(p.0, p.1);
    &self.data[i..i + self.bpp]
  }
  /// Get a mutable slice of pixel at (`x`,`y`), `0 <= x < width`, `0 <= y < height`
  ///
  /// The length of the slice is `bpp`
  pub fn get_pixel_mut(&mut self, p: (usize, usize)) -> &mut [u8] {
    let i = self.offset_index(p.0, p.1);
    &mut self.data[i..i + self.bpp]
  }
  /// Fill the underlying [`data`](Self::data) with value `v`
  pub fn fill(&mut self, v: u8) {
    self.data.fill(v);
  }
  /// Create a RenderingBuffer from existing data
  ///
  /// The length of `data` must be `width * height * bpp`
  pub fn from_buf(data: Vec<u8>, width: usize, height: usize, bpp: usize) -> Self {
    assert_eq!(data.len(), width * height * bpp);
    RenderingBuffer {
      width,
      height,
      bpp,
      data,
      flip: false,
    }
  }
}

use std::ops::Index;
use std::ops::IndexMut;

impl Index<(usize, usize)> for RenderingBuffer {
  type Output = [u8];
  fn index(&self, index: (usize, usize)) -> &[u8] {
    debug_assert!(
      index.0 < self.width,
      "request {} >= {} width :: index",
      index.0,
      self.width
    );
    debug_assert!(
      index.1 < self.height,
      "request {} >= {} height :: index",
      index.1,
      self.height
    );
    let i = ((index.1 * self.width) + index.0) * self.bpp;
    debug_assert!(i < self.data.len());
    &self.data[i..]
  }
}
impl IndexMut<(usize, usize)> for RenderingBuffer {
  fn index_mut(&mut self, index: (usize, usize)) -> &mut [u8] {
    debug_assert!(
      index.0 < self.width,
      "request {} >= {} width :: index_mut",
      index.0,
      self.width
    );
    debug_assert!(
      index.1 < self.height,
      "request {} >= {} height :: index_mut",
      index.1,
      self.height
    );
    let i = ((index.1 * self.width) + index.0) * self.bpp;
    debug_assert!(i < self.data.len());
    &mut self.data[i..]
  }
}
