mod base;
mod outline_aa;
mod primitives;
mod scanline;

pub mod gradient;
pub mod line_interp;

pub use base::*;
pub use gradient::*;
pub use line_interp::*;
pub use outline_aa::*;
pub use primitives::*;
pub use scanline::*;

use crate::Color;

/// Render scanlines to Image
pub trait Render {
  /// Render a single scanlines to the image
  fn render(&mut self, data: &RenderData);
  /// Set the Color of the Renderer
  fn color<C: Color>(&mut self, color: C);
  /// Prepare the Renderer
  fn prepare(&self) {}
}
