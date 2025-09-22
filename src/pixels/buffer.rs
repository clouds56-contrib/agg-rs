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
  pub fn offset(&self, (x, y): (usize, usize)) -> &[u8] {
    let i = self.offset_index(x, y);
    &self.data[i..]
  }
  /// Get a mutable slice starting at pixel (`x`,`y`), `0 <= x < width`, `0 <= y < height`
  ///
  /// The slice goes to the end of the underlying [`data`](Self::data)
  pub fn offset_mut(&mut self, (x, y): (usize, usize)) -> &mut [u8] {
    let i = self.offset_index(x, y);
    &mut self.data[i..]
  }
  pub fn slice(&self, id: (usize, usize), len: usize) -> &[u8] {
    let start = self.offset_index(id.0, id.1);
    let end = start + len * self.bpp;
    debug_assert!(end <= self.data.len(), "slice out of bounds");
    &self.data[start..end]
  }
  pub fn slice_mut(&mut self, id: (usize, usize), len: usize) -> &mut [u8] {
    let start = self.offset_index(id.0, id.1);
    let end = start + len * self.bpp;
    debug_assert!(end <= self.data.len(), "slice out of bounds");
    &mut self.data[start..end]
  }
  /// Get a slice of pixel at (`x`,`y`), `0 <= x < width`, `0 <= y < height`
  ///
  /// The length of the slice is `bpp`
  pub fn get_pixel(&self, x: usize, y: usize) -> &[u8] {
    let i = self.offset_index(x, y);
    &self.data[i..i + self.bpp]
  }
  /// Get a mutable slice of pixel at (`x`,`y`), `0 <= x < width`, `0 <= y < height`
  ///
  /// The length of the slice is `bpp`
  pub fn get_pixel_mut(&mut self, x: usize, y: usize) -> &mut [u8] {
    let i = self.offset_index(x, y);
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

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test_rendering_buffer() {
    let mut buf = RenderingBuffer::new(3, 5, 4);
    assert_eq!(buf.width, 3);
    assert_eq!(buf.height, 5);
    assert_eq!(buf.bpp, 4);
    assert_eq!(buf.stride(), 12);
    assert!(!buf.is_empty());
    assert_eq!(buf.len(), 3 * 5 * 4);
    assert_eq!(buf.row_index(0), 0);
    assert_eq!(buf.row_index(1), 12);
    assert_eq!(buf.row_index(4), 48);
    assert_eq!(buf.offset_index(0, 0), 0);
    assert_eq!(buf.offset_index(1, 0), 4);
    assert_eq!(buf.offset_index(2, 0), 8);
    assert_eq!(buf.offset_index(0, 1), 12);
    assert_eq!(buf.offset_index(1, 1), 16);
    assert_eq!(buf.offset_index(2, 1), 20);
    assert_eq!(buf.offset_index(0, 4), 48);
    assert_eq!(buf.offset_index(1, 4), 52);
    assert_eq!(buf.offset_index(2, 4), 56);

    assert!(buf.data.iter().all(|&x| x == 0));
    buf.fill(255);
    assert!(buf.data.iter().all(|&x| x == 255));

    let p = buf.get_pixel_mut(2, 1);
    p[0] = 127;
    p[1] = 200;
    p[2] = 98;
    assert_eq!(p.len(), 4);
    assert_eq!(buf.data[20..24], [127, 200, 98, 255]);

    let p = buf.get_pixel(2, 1);
    assert_eq!(p.len(), 4);
    assert_eq!(p, [127, 200, 98, 255]);

    let row = buf.row(1);
    assert_eq!(row.len(), 12);
    assert_eq!(row[8..12], [127, 200, 98, 255]);

    let offset = buf.offset((2, 1));
    assert_eq!(offset.len(), 3 * 5 * 4 - 20);
    assert_eq!(offset[0..4], [127, 200, 98, 255]);

    let buf = buf.fliped();
    assert!(buf.flip);
    assert_eq!(buf.width, 3);
    assert_eq!(buf.height, 5);
    assert_eq!(buf.bpp, 4);
    assert_eq!(buf.stride(), 12);
    assert!(!buf.is_empty());
    assert_eq!(buf.len(), 3 * 5 * 4);
    assert_eq!(buf.row_index(0), 48);
    assert_eq!(buf.row_index(1), 36);
    assert_eq!(buf.row_index(4), 0);
    assert_eq!(buf.offset_index(0, 0), 48);
    assert_eq!(buf.offset_index(1, 0), 52);
    assert_eq!(buf.offset_index(2, 0), 56);
    assert_eq!(buf.offset_index(0, 1), 36);
    assert_eq!(buf.offset_index(1, 1), 40);
    assert_eq!(buf.offset_index(2, 1), 44);
    assert_eq!(buf.offset_index(0, 4), 0);
    assert_eq!(buf.offset_index(1, 4), 4);
    assert_eq!(buf.offset_index(2, 4), 8);

    assert_eq!(buf.data[20..24], [127, 200, 98, 255]);
    assert_eq!(buf.get_pixel(2, 3), [127, 200, 98, 255]);
    assert_eq!(buf.row(3)[8..12], [127, 200, 98, 255]);
    let offset = buf.offset((2, 3));
    assert_eq!(offset[0..4], [127, 200, 98, 255]);
    assert_eq!(offset.len(), 3 * 5 * 4 - 20);

    assert_eq!(buf.len(), 3 * 5 * 4);
  }
}
