//! Packed scanline container (scanline_p8)
//!
//! This is a closer translation of the original AGG `scanline_p8` to
//! facilitate parity/image comparison with the reference C++ code.
//! Unlike `ScanlineU8` (which stores explicit span cover vectors), this
//! structure packs cover data into a single contiguous buffer and each
//! span references a slice (by pointer in C++, by range in Rust) within
//! that buffer. Negative `len` indicates a solid span (all pixels share
//! the single cover at `covers[0]`). Positive `len` indicates an array
//! of per‑pixel covers of length `len`.

use crate::U8;

/// Span within a packed scanline.
///
/// If `len` is negative the span is solid and `covers` contains exactly one
/// element which should be replicated logically `-len` times. When `len` is
/// positive, `covers.len()` equals `len`.
#[derive(Debug, Clone)]
pub struct PackedSpan {
  pub x: i32,
  /// Length of span. Negative => solid span with single cover.
  pub len: i32,
  pub covers: SpanCovers,
}

/// Cover storage abstraction for a span.
#[derive(Debug, Clone)]
pub enum SpanCovers {
  /// Single cover value (solid span, `len` stored as negative)
  Single(U8),
  /// Slice (owned) of per-pixel covers
  Slice(Vec<U8>),
}

impl SpanCovers {
  fn first(&self) -> U8 {
    match self {
      SpanCovers::Single(v) => *v,
      SpanCovers::Slice(v) => v[0],
    }
  }
  fn push_many(&mut self, cover: U8, count: usize) {
    match self {
      SpanCovers::Single(v) => {
        // Convert to slice if we need to append multiple distinct entries
        let mut vec = Vec::with_capacity(count + 1);
        vec.push(*v);
        vec.extend(std::iter::repeat(cover).take(count));
        *self = SpanCovers::Slice(vec);
      }
      SpanCovers::Slice(v) => v.extend(std::iter::repeat(cover).take(count)),
    }
  }
  fn extend_from_slice(&mut self, slice: &[U8]) {
    match self {
      SpanCovers::Single(v) => {
        let mut vec = Vec::with_capacity(slice.len() + 1);
        vec.push(*v);
        vec.extend_from_slice(slice);
        *self = SpanCovers::Slice(vec);
      }
      SpanCovers::Slice(v) => v.extend_from_slice(slice),
    }
  }
  fn len(&self) -> usize {
    match self {
      SpanCovers::Single(_) => 1,
      SpanCovers::Slice(v) => v.len(),
    }
  }
}

/// Packed scanline container (p8 variant)
#[derive(Debug, Default, Clone)]
pub struct ScanlineP8 {
  last_x: i32,
  y: i32,
  // Packed cover buffer (equivalent to `m_covers` in C++). We append into
  // this and store slices in spans. Simpler to just clone needed ranges for
  // now (could be optimized with indices later if needed for perf).
  covers: Vec<U8>,
  // Spans collected for current scanline.
  spans: Vec<PackedSpan>,
  // Working pointer index into covers buffer (like m_cover_ptr)
  cover_ptr: usize,
}

const LAST_X_I32: i32 = 0x7FFF_FFF0u32 as i32; // mimic original sentinel

impl ScanlineP8 {
  pub fn new() -> Self {
    Self {
      last_x: LAST_X_I32,
      y: 0,
      covers: Vec::new(),
      spans: Vec::new(),
      cover_ptr: 0,
    }
  }

  /// Reset with anticipated x range (min_x, max_x). Pre-allocates buffers.
  pub fn reset(&mut self, min_x: i32, max_x: i32) {
    let max_len = (max_x - min_x + 3).max(0) as usize; // defensive
    if max_len > self.covers.len() {
      self.covers.resize(max_len, U8::new(0));
    }
    // We reuse capacity; logical length starts at zero each time.
    self.cover_ptr = 0;
    self.last_x = LAST_X_I32;
    self.spans.clear();
  }

  /// Add a single cell (x, cover)
  pub fn add_cell(&mut self, x: i32, cover: u64) {
    let cov = U8::new(cover as _);
    self.ensure_cover_capacity(1);
    self.covers[self.cover_ptr] = cov;
    if x == self.last_x + 1 && matches!(self.spans.last(), Some(s) if s.len > 0) {
      // Extend last positive span
      if let Some(last) = self.spans.last_mut() {
        if let SpanCovers::Slice(v) = &mut last.covers {
          v.push(cov);
        } else if let SpanCovers::Single(prev) = &mut last.covers {
          // Convert single to slice with previous + new
            let prev_cov = *prev;
            *last = PackedSpan {
              x: last.x,
              len: last.len + 1,
              covers: SpanCovers::Slice(vec![prev_cov, cov]),
            };
            self.cover_ptr += 1; // we logically consumed one more cover
            self.last_x = x;
            return;
        }
        last.len += 1;
      }
    } else {
      // New span
      self.spans.push(PackedSpan { x, len: 1, covers: SpanCovers::Slice(vec![cov]) });
    }
    self.cover_ptr += 1;
    self.last_x = x;
  }

  /// Add multiple cells with explicit covers slice.
  pub fn add_cells(&mut self, x: i32, len: u32, covers: &[U8]) {
    let len_i = len as i32;
    debug_assert_eq!(covers.len(), len as usize);
    self.ensure_cover_capacity(len as usize);
    // Copy covers into buffer (to mimic contiguous buffer semantics). Not strictly
    // necessary for logic here but keeps layout closer to C++ port potential.
    for (i, c) in covers.iter().enumerate() {
      self.covers[self.cover_ptr + i] = *c;
    }
    if x == self.last_x + 1 && matches!(self.spans.last(), Some(s) if s.len > 0) {
      if let Some(last) = self.spans.last_mut() {
        if let SpanCovers::Slice(v) = &mut last.covers {
          v.extend_from_slice(covers);
        }
        last.len += len_i;
      }
    } else {
      self.spans.push(PackedSpan { x, len: len_i, covers: SpanCovers::Slice(covers.to_vec()) });
    }
    self.cover_ptr += len as usize;
    self.last_x = x + len_i - 1;
  }

  /// Add a solid span (same cover repeated `len` times). Negative len stored.
  pub fn add_span(&mut self, x: i32, len: u32, cover: u64) {
    let cov = U8::new(cover as _);
    let len_i = len as i32;
    if x == self.last_x + 1 {
      if let Some(last) = self.spans.last_mut() {
        if last.len < 0 { // solid span
          if last.covers.first() == cov { // same cover, extend
            last.len -= len_i; // remember negative length
            self.last_x = x + len_i - 1;
            return;
          }
        }
      }
    }
    // New solid span
    self.spans.push(PackedSpan { x, len: -len_i, covers: SpanCovers::Single(cov) });
    self.last_x = x + len_i - 1;
  }

  pub fn finalize(&mut self, y: i32) {
    self.y = y;
  }

  pub fn reset_spans(&mut self) {
    self.last_x = LAST_X_I32;
    self.cover_ptr = 0;
    self.spans.clear();
  }

  pub fn y(&self) -> i32 { self.y }
  pub fn num_spans(&self) -> usize { self.spans.len() }
  pub fn spans(&self) -> &[PackedSpan] { &self.spans }

  fn ensure_cover_capacity(&mut self, add: usize) {
    let need = self.cover_ptr + add;
    if need > self.covers.len() {
      self.covers.resize(need, U8::new(0));
    }
  }
}
