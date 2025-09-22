mod base;
mod outline_aa;
mod primitives;
mod scanline;

pub use base::*;
pub use outline_aa::*;
pub use primitives::*;
pub use scanline::*;

use crate::{Color, LineParameters, RealLike};

/// Render scanlines to Image
pub trait Render {
  /// Render a single scanlines to the image
  fn render(&mut self, data: &RenderData);
  /// Set the Color of the Renderer
  fn color<C: Color>(&mut self, color: C);
  /// Prepare the Renderer
  fn prepare(&self) {}
}

pub(crate) trait RenderOutline {
  type Cover: RealLike;
  fn cover(&self, d: i64) -> Self::Cover;
  fn blend_solid_hspan(&mut self, x: i64, y: i64, len: i64, covers: &[Self::Cover]);
  fn blend_solid_vspan(&mut self, x: i64, y: i64, len: i64, covers: &[Self::Cover]);
}
/// Functions for Drawing Outlines
//pub trait DrawOutline: Lines + AccurateJoins + SetColor {}
pub trait DrawOutline {
  /// Set the current Color
  fn color<C: Color>(&mut self, color: C);
  /// If Line Joins are Accurate
  fn accurate_join_only(&self) -> bool;
  fn line0(&mut self, lp: &LineParameters);
  fn line1(&mut self, lp: &LineParameters, sx: i64, sy: i64);
  fn line2(&mut self, lp: &LineParameters, ex: i64, ey: i64);
  fn line3(&mut self, lp: &LineParameters, sx: i64, sy: i64, ex: i64, ey: i64);
  fn semidot<F>(&mut self, cmp: F, xc1: i64, yc1: i64, xc2: i64, yc2: i64)
  where
    F: Fn(i64) -> bool;
  fn pie(&mut self, xc: i64, y: i64, x1: i64, y1: i64, x2: i64, y2: i64);
}
