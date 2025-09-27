//! Rendering Outline, not Anti-Aliased
//!
//! ```
//! use agg::prelude::*;
//! let pix = Pixfmt::<Rgb8>::create(100, 100);
//! let mut ren_base = agg::RenderingBase::new(pix);
//! ren_base.clear(Rgb8::WHITE);
//!
//! let mut ren = RendererPrimatives::new_black(&mut ren_base).with_line_color(agg::Rgba8::BLUE);
//!
//! let mut path = agg::Path::new();
//! path.move_to(10.0, 10.0);
//! path.line_to(50.0, 90.0);
//! path.line_to(90.0, 10.0);
//!
//! let mut ras = RasterizerOutline::with_primative(&mut ren);
//! ras.add_path(&path);
//! ren_base.to_file("primative.png").unwrap();
//! ```
//!
//! The above code produces:
//!
//! ![Output](https://raw.githubusercontent.com/savage13/agg/master/images/primative.png)

use crate::POLY_SUBPIXEL_SHIFT;
use crate::Pixel;
use crate::RendererPrimatives;
use crate::VertexSource;
use crate::paths::PathCommand;

#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub(crate) struct Subpixel(pub i64);

impl Subpixel {
  pub fn value(self) -> i64 {
    self.0
  }
}

impl From<i64> for Subpixel {
  fn from(v: i64) -> Self {
    Subpixel(v)
  }
}
impl From<Subpixel> for i64 {
  fn from(v: Subpixel) -> Self {
    v.0 >> POLY_SUBPIXEL_SHIFT
  }
}

/// Rasterizer for Outlined Shapes
///
/// The rendering is directly attached and drawing is done immediately.
pub struct RasterizerOutline<'a, T>
where
  T: Pixel,
{
  ren: &'a mut RendererPrimatives<'a, T>,
  start_x: Subpixel,
  start_y: Subpixel,
  vertices: usize,
}
impl<'a, T> RasterizerOutline<'a, T>
where
  T: Pixel,
{
  /// Create a new RasterizerOutline with a Renderer
  pub fn with_primative(ren: &'a mut RendererPrimatives<'a, T>) -> Self {
    Self {
      start_x: Subpixel::from(0),
      start_y: Subpixel::from(0),
      vertices: 0,
      ren,
    }
  }
  /// Add a path and render
  pub fn add_path<VS: VertexSource>(&mut self, path: &VS) {
    for v in path.xconvert().iter() {
      match v.cmd {
        PathCommand::MoveTo => self.move_to_d(v.x, v.y),
        PathCommand::LineTo => self.line_to_d(v.x, v.y),
        PathCommand::Close => self.close(),
        PathCommand::Stop => unimplemented!("stop encountered"),
      }
    }
  }
  /// Close the current path
  pub fn close(&mut self) {
    if self.vertices > 2 {
      let (x, y) = (self.start_x, self.start_y);
      self.line_to(x, y);
    }
    self.vertices = 0;
  }
  /// Move to position (`x`,`y`)
  pub fn move_to_d(&mut self, x: f64, y: f64) {
    let x = self.ren.coord(x);
    let y = self.ren.coord(y);
    self.move_to(x, y);
  }
  /// Draw a line from the current position to position (`x`,`y`)
  pub fn line_to_d(&mut self, x: f64, y: f64) {
    let x = self.ren.coord(x);
    let y = self.ren.coord(y);
    self.line_to(x, y);
  }
  /// Move the current position to (`x`,`y`)
  fn move_to(&mut self, x: Subpixel, y: Subpixel) {
    self.vertices = 1;
    self.start_x = x;
    self.start_y = y;
    self.ren.move_to(x, y);
  }
  fn line_to(&mut self, x: Subpixel, y: Subpixel) {
    self.vertices += 1;
    self.ren.line_to(x, y);
  }
}
