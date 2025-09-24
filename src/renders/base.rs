//! Rendering Base

use crate::{Color, Pixel, Position};
use crate::{Covers, RealLike};
use std::cmp::max;
use std::cmp::min;

/// Rendering Base
#[derive(Debug)]
pub struct RenderingBase<T> {
  /// Pixel Format
  pub pixf: T,
}

impl<T> RenderingBase<T>
where
  T: Pixel,
{
  /// Create new Rendering Base from Pixel Format
  pub fn new(pixf: T) -> RenderingBase<T> {
    RenderingBase { pixf }
  }
  pub fn as_bytes(&self) -> &[u8] {
    self.pixf.as_bytes()
  }
  pub fn to_file<P: AsRef<std::path::Path>>(&self, filename: P) -> Result<(), std::io::Error> {
    self.pixf.to_file(filename)
  }
  /// Set Image to a single color
  pub fn clear(&mut self, color: T::Color) {
    self.pixf.fill(color);
  }
  /// Get Image size
  pub fn limits(&self) -> (Position, Position, Position, Position) {
    let w = self.pixf.width() as Position;
    let h = self.pixf.height() as Position;
    (0, w - 1, 0, h - 1)
  }
  /// Blend a color along y-row from x1 to x2
  pub fn blend_hline<C: Color, U: RealLike>(&mut self, x1: Position, y: Position, x2: Position, c: C, cover: U) {
    let (xmin, xmax, ymin, ymax) = self.limits();
    let (x1, x2) = if x2 > x1 { (x1, x2) } else { (x2, x1) };
    if y > ymax || y < ymin || x1 > xmax || x2 < xmin {
      return;
    }
    let x1 = max(x1, xmin);
    let x2 = min(x2, xmax);
    self.pixf.blend_hline(x1, y, x2 - x1 + 1, c, cover);
  }

  /// Blend a color from (x,y) with variable covers
  pub fn blend_solid_hspan<C: Color, U: RealLike>(&mut self, x: Position, y: Position, len: Position, c: C, covers: &[U]) {
    let (xmin, xmax, ymin, ymax) = self.limits();
    if y > ymax || y < ymin {
      return;
    }
    let (mut x, mut len, mut off) = (x, len, 0);
    if x < xmin {
      len -= xmin - x;
      if len <= 0 {
        return;
      }
      off = off + xmin - x; // Woah!!!!
      x = xmin;
    }
    if x + len > xmax {
      len = xmax - x + 1;
      if len <= 0 {
        return;
      }
    }
    let covers_win = &covers[off as usize..(off + len) as usize];
    assert!(len as usize <= covers[off as usize..].len());
    self.pixf.blend_solid_hspan(x, y, len, c, covers_win);
  }

  /// Blend a color from (x,y) with variable covers
  pub fn blend_solid_vspan<C: Color, U: RealLike>(&mut self, x: Position, y: Position, len: Position, c: C, covers: &[U]) {
    let (xmin, xmax, ymin, ymax) = self.limits();
    if x > xmax || x < xmin {
      return;
    }
    let (mut y, mut len, mut off) = (y, len, 0);
    if y < ymin {
      len -= ymin - y;
      if len <= 0 {
        return;
      }
      off = off + ymin - y; // Woah!!!!
      y = ymin;
    }
    if y + len > ymax {
      len = ymax - y + 1;
      if len <= 0 {
        return;
      }
    }
    let covers_win = &covers[off as usize..(off + len) as usize];
    assert!(len as usize <= covers[off as usize..].len());
    self.pixf.blend_solid_vspan(x, y, len, c, covers_win);
  }

  pub fn blend_color_vspan<'a, C, U, Co>(&mut self, x: Position, y: Position, len: Position, colors: &[C], covers: Co)
  where
    C: Color,
    U: RealLike,
    Co: Into<Covers<'a, U>>,
  {
    let (xmin, xmax, ymin, ymax) = self.limits();
    if x > xmax || x < xmin {
      return;
    }
    let (mut y, mut len, mut off) = (y, len, 0);
    if y < ymin {
      len -= ymin - y;
      if len <= 0 {
        return;
      }
      off = off + ymin - y; // Woah!!!!
      y = ymin;
    }
    if y + len > ymax {
      len = ymax - y + 1;
      if len <= 0 {
        return;
      }
    }
    let colors_win = &colors[off as usize..(off + len) as usize];
    let covers: Covers<'a, U> = covers.into();
    let covers_win = covers.slice(off as usize..(off + len) as usize);
    self.pixf.blend_color_vspan(x, y, len, colors_win, covers_win)
  }

  pub fn blend_color_hspan<'a, C, U, Co>(&mut self, x: i64, y: i64, len: i64, colors: &[C], covers: Co)
  where
    C: Color,
    U: RealLike,
    Co: Into<Covers<'a, U>>,
  {
    let (xmin, xmax, ymin, ymax) = self.limits();
    if y > ymax || y < ymin {
      return;
    }
    let (mut x, mut len, mut off) = (x, len, 0);
    if x < xmin {
      len -= xmin - x;
      if len <= 0 {
        return;
      }
      off = off + xmin - x; // Woah!!!!
      x = xmin;
    }
    if x + len > xmax {
      len = xmax - x + 1;
      if len <= 0 {
        return;
      }
    }
    let colors_win = &colors[off as usize..(off + len) as usize];
    let covers: Covers<'a, U> = covers.into();
    let covers_win = covers.slice(off as usize..(off + len) as usize);
    self.pixf.blend_color_hspan(x, y, len, colors_win, covers_win)
  }
}
