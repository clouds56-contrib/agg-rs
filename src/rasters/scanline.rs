//! Rasterizer

use fixed::types::I24F8;
use fixed::types::I48F16;
use fixed::types::I56F8;

use crate::FixedLike;
use crate::PixelLike;
use crate::Position;
//use crate::POLY_SUBPIXEL_MASK;

use crate::cell::RasterizerCell;
use crate::clip::Clip;
use crate::paths::PathCommand;
use crate::paths::Vertex;
use crate::scanlines::ScanlineU8;

//use crate::Rasterize;
use crate::VertexSource;

struct RasConvInt {}
impl RasConvInt {
  pub fn upscale<P: PixelLike>(v: f64) -> P {
    P::from_f64_rounded(v)
  }
  //pub fn downscale(v: i64) -> i64 {
  //    v
  //}
}

/// Winding / Filling Rule
///
/// See (Non-Zero Filling Rule)[https://en.wikipedia.org/wiki/Nonzero-rule] and
/// (Even-Odd Filling)[https://en.wikipedia.org/wiki/Even%E2%80%93odd_rule]
#[derive(Debug, PartialEq, Copy, Clone, Default)]
pub enum FillingRule {
  #[default]
  NonZero,
  EvenOdd,
}

/// Path Status
#[derive(Debug, PartialEq, Copy, Clone, Default)]
pub enum PathStatus {
  #[default]
  Initial,
  Closed,
  MoveTo,
  LineTo,
}

/// Rasterizer Anti-Alias using Scanline
#[derive(Debug)]
pub struct RasterizerScanline<P = I56F8, Area = I48F16> {
  /// Clipping Region
  pub(crate) clipper: Clip<P>,
  /// Collection of Rasterizing Cells
  outline: RasterizerCell<Area>,
  /// Status of Path
  pub(crate) status: PathStatus,
  /// Current x position
  pub(crate) x0: P,
  /// Current y position
  pub(crate) y0: P,
  /// Current y row being worked on, for output
  scan_y: Position,
  /// Filling Rule for Polygons
  filling_rule: FillingRule,
  /// Gamma Corection Values
  gamma: Vec<u64>,
}

impl<P: PixelLike, Area> Default for RasterizerScanline<P, Area> {
  fn default() -> Self {
    Self::new()
  }
}

impl<P: PixelLike, Area> RasterizerScanline<P, Area> {
  /// Create a new RasterizerScanline
  pub fn new() -> Self {
    Self {
      clipper: Clip::new(),
      status: PathStatus::Initial,
      outline: RasterizerCell::new(),
      x0: P::ZERO,
      y0: P::ZERO,
      scan_y: 0,
      filling_rule: FillingRule::NonZero,
      gamma: (0..256).collect(),
    }
  }
}

impl<P: PixelLike, Area: PixelLike> RasterizerScanline<P, Area> {
  /// Reset Rasterizer
  ///
  /// Reset the RasterizerCell and set PathStatus to Initial
  pub fn reset(&mut self) {
    self.outline.reset();
    self.status = PathStatus::Initial;
  }
  /// Add a Path
  ///
  /// Walks the path from the VertexSource and rasterizes it
  pub fn add_path<VS: VertexSource>(&mut self, path: &VS) {
    //path.rewind();
    if !self.outline.sorted_y.is_empty() {
      self.reset();
    }
    for seg in path.xconvert() {
      match seg.cmd {
        PathCommand::LineTo => self.line_to(seg.x, seg.y),
        PathCommand::MoveTo => self.move_to(seg.x, seg.y),
        PathCommand::Close => self.close_polygon(),
        PathCommand::Stop => unimplemented!("stop encountered"),
      }
    }
  }

  /// Rewind the Scanline
  ///
  /// Close active polygon, sort the Rasterizer Cells, set the
  /// scan_y value to the minimum y value and return if any cells
  /// are present
  pub(crate) fn rewind_scanlines(&mut self) -> bool {
    self.close_polygon();
    self.outline.sort_cells();
    if self.outline.total_cells() == 0 {
      false
    } else {
      self.scan_y = self.outline.min_y;
      true
    }
  }

  /// Sweep the Scanline
  ///
  /// For individual y rows adding any to the input Scanline
  ///
  /// Returns true if data exists in the input Scanline
  pub(crate) fn sweep_scanline(&mut self, sl: &mut ScanlineU8) -> bool {
    loop {
      if self.scan_y < 0 {
        self.scan_y += 1;
        continue;
      }
      if self.scan_y > self.outline.max_y {
        return false;
      }
      sl.reset_spans();
      let cells = self.outline.scanline_cells(self.scan_y);
      let mut num_cells = cells.len();

      // debug!("y={} cells: [{num_cells}]{:?}", self.scan_y, cells.iter().map(|c| (c.x, c.area.to_f64() * 65536., c.cover.to_f64() * 256.)).collect::<Vec<_>>());

      let mut cover = Area::ZERO;

      let mut iter = cells.iter();

      if let Some(mut cur_cell) = iter.next() {
        while num_cells > 0 {
          let mut x = cur_cell.x;
          let mut area = cur_cell.area;

          cover += cur_cell.cover;
          num_cells -= 1;
          //accumulate all cells with the same X
          while num_cells > 0 {
            cur_cell = iter.next().unwrap();
            if cur_cell.x != x {
              break;
            }
            area += cur_cell.area;
            cover += cur_cell.cover;
            num_cells -= 1;
          }
          if area != 0 {
            let alpha = self.calculate_alpha((cover << 1) - area);
            if alpha > 0 {
              sl.add_cell(x, alpha);
            }
            x += 1;
          }
          if num_cells > 0 && cur_cell.x > x {
            let alpha = self.calculate_alpha(cover << 1);
            if alpha > 0 {
              sl.add_span(x, cur_cell.x - x, alpha);
            }
          }
        }
      }
      if sl.num_spans() != 0 {
        break;
      }
      self.scan_y += 1;
    }
    sl.finalize(self.scan_y);
    self.scan_y += 1;
    true
  }
  /// Return minimum x value from the RasterizerCell
  pub fn min_x(&self) -> Position {
    self.outline.min_x
  }
  /// Return maximum x value from the RasterizerCell
  pub fn max_x(&self) -> Position {
    self.outline.max_x
  }

  /// Set the gamma function
  ///
  /// Values are set as:
  ///```ignore
  ///      gamma = gfunc( v / mask ) * mask
  /// ```
  /// where v = 0 to 255
  pub fn gamma<F>(&mut self, gfunc: F)
  where
    F: Fn(f64) -> f64,
  {
    let aa_shift = 8;
    let aa_scale = 1 << aa_shift;
    let aa_mask = f64::from(aa_scale - 1);

    self.gamma = (0..256)
      .map(|i| gfunc(f64::from(i) / aa_mask))
      .map(|v| (v * aa_mask).round() as u64)
      .collect();
  }
  /// Create a new RasterizerScanline with a gamma function
  ///
  /// See gamma() function for description
  pub fn new_with_gamma<F>(gfunc: F) -> Self
  where
    F: Fn(f64) -> f64,
  {
    let mut new = Self::new();
    new.gamma(gfunc);
    new
  }
  /// Set Clip Box
  pub fn clip_box(&mut self, x1: f64, y1: f64, x2: f64, y2: f64) {
    self.clipper.clip_box(
      RasConvInt::upscale(x1),
      RasConvInt::upscale(y1),
      RasConvInt::upscale(x2),
      RasConvInt::upscale(y2),
    );
  }
  /// Move to point (x,y)
  ///
  /// Sets point as the initial point
  pub fn move_to(&mut self, x: f64, y: f64) {
    self.x0 = RasConvInt::upscale(x);
    self.y0 = RasConvInt::upscale(y);
    self.clipper.move_to(self.x0, self.y0);
    self.status = PathStatus::MoveTo;
  }
  /// Draw line from previous point to point (x,y)
  pub fn line_to(&mut self, x: f64, y: f64) {
    let x = RasConvInt::upscale(x);
    let y = RasConvInt::upscale(y);
    self.clipper.line_to(&mut self.outline, x, y);
    self.status = PathStatus::LineTo;
  }
  /// Close the current polygon
  ///
  /// Draw a line from current point to initial "move to" point
  pub fn close_polygon(&mut self) {
    if self.status == PathStatus::LineTo {
      self.clipper.line_to(&mut self.outline, self.x0, self.y0);
      self.status = PathStatus::Closed;
    }
  }
  /// Calculate alpha term based on area
  fn calculate_alpha(&self, area: Area) -> u64 {
    let aa_shift = 8;
    let aa_scale = 1 << aa_shift;
    let aa_scale2 = aa_scale * 2;
    let aa_mask = aa_scale - 1;
    let aa_mask2 = aa_scale2 - 1;
    let mut cover = I24F8::from_fixed(area >> 1).abs().to_bits() as u64;
    if self.filling_rule == FillingRule::EvenOdd {
      cover &= aa_mask2;
      if cover > aa_scale {
        cover = aa_scale2 - cover;
      }
    }
    cover = cover.clamp(0, aa_mask);
    self.gamma[cover as usize]
  }
}

pub(crate) fn len_i64(a: &Vertex<i64>, b: &Vertex<i64>) -> i64 {
  len_i64_xy(a.x, a.y, b.x, b.y)
}
pub(crate) fn len_i64_xy(x1: i64, y1: i64, x2: i64, y2: i64) -> i64 {
  let dx = x1 as f64 - x2 as f64;
  let dy = y1 as f64 - y2 as f64;
  (dx * dx + dy * dy).sqrt().round() as i64
}

// #[derive(Debug,PartialEq,Copy,Clone)]
// pub enum LineJoin {
//     Round,
//     None,
//     Miter,
//     MiterAccurate,
// }

#[cfg(test)]
mod tests {
  use super::*;

  use fixed::types::I48F16 as Area;
  use fixed::types::I56F8 as P;

  #[test]
  fn test_calculate_alpha() {
    let ras = RasterizerScanline::<P, Area>::new();
    let area = Area::from_f64_nearest(-0.2750244140625);
    let p = I24F8::from_fixed(area);
    assert_eq!(area.to_bits(), -18024);
    assert_eq!(area.to_f64() * 256., -70.40625);
    assert_eq!(<I24F8 as fixed::traits::Fixed>::from_num(area).to_bits(), -71);
    assert_eq!(p.to_bits(), -71);
    assert_eq!(p.to_f64(), -0.27734375);
    assert_eq!(ras.calculate_alpha(Area::from_f64_nearest(0.2750244140625)), 35);
    assert_eq!(ras.calculate_alpha(Area::from_f64_nearest(-0.2750244140625)), 36);
  }
}
