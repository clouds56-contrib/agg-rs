//! Anti Grain Geometry - Rust implementation
//!
//! Originally derived from version 2.4 of [AGG](http://antigrain.com)
//!
//! This crate implments the drawing / painting 2D algorithms developed in the Anti Grain Geometry C++ library. Quoting from the author in the documentation:
//!
//! > **Anti-Grain Geometry** is not a solid graphic library and it's not very easy to use. I consider **AGG** as a **"tool to create other tools"**. It means that there's no **"Graphics"** object or something like that, instead, **AGG** consists of a number of loosely coupled algorithms that can be used together or separately. All of them have well defined interfaces and absolute minimum of implicit or explicit dependencies.
//!
//!
//! # Anti-Aliasing and Subpixel Accuracy
//!
//! One primary strenght of AGG are the combination of drawing with subpixel accuracy with anti-aliasing effects.  There are many examples within the documentation and reproduced here.
//!
//! # Drawing
//!
//! There are multiple ways to put / draw pixels including:
//!
//!   - Scanline Renderers
//!     - Antialiased or Aliased (Binary)
//!   - Outline Renderer, possibly with Images
//!   - Raw Pixel Manipulation
//!
//! # Scanline Renderer
//!
//!  The multitude of renderers here include [`render_scanlines`],
//!    [`render_all_paths`], [`render_scanlines_aa_solid`] and
//!    [`render_scanlines_bin_solid`]
//!
//! ```
//! use agg::{NamedColor, Render};
//!
//! // Create a blank image 10x10 pixels
//! let pix = agg::Pixfmt::<agg::Rgb8>::create(100, 100);
//! let mut ren_base = agg::RenderingBase::new(pix);
//! ren_base.clear(agg::Rgb8::WHITE);
//!
//! // Draw a polygon from (10,10) - (50,90) - (90,10)
//! let mut ras = agg::RasterizerScanline::new();
//! ras.move_to(10.0, 10.0);
//! ras.line_to(50.0, 90.0);
//! ras.line_to(90.0, 10.0);
//!
//! // Render the line to the image
//! let mut ren = agg::RenderingScanlineAASolid::new_black(&mut ren_base);
//! agg::render_scanlines(&mut ras, &mut ren);
//!
//! // Save the image to a file
//! ren_base.to_file("little_black_triangle.png").unwrap();
//! ```
//!
//!
//! # Outline AntiAlias Renderer
//!
//! ```
//! use agg::prelude::*;
//! let pix = Pixfmt::<Rgb8>::create(100, 100);
//! let mut ren_base = agg::RenderingBase::new(pix);
//! ren_base.clear(Rgb8::WHITE);
//!
//! let mut ren =
//!   RendererOutlineAA::new(&mut ren_base, Rgba8::from_raw(102, 77, 26, 255)).with_width(3.0);
//!
//! let mut path = agg::Path::new();
//! path.move_to(10.0, 10.0);
//! path.line_to(50.0, 90.0);
//! path.line_to(90.0, 10.0);
//!
//! let mut ras = RasterizerOutlineAA::with_renderer(&mut ren);
//! ras.add_path(&path);
//! ren_base.to_file("outline_aa.png").unwrap();
//! ```
//!
//! # Primative Renderer
//!
//! Render for primative shapes: lines, rectangles, and ellipses; filled or
//!    outlined.
//!
//! ```
//! use agg::prelude::*;
//!
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
//!
//! # Raw Pixel Manipulation
//!
//!   **Note:** Functions here are a somewhat low level interface and probably not what
//!     you want to use.
//!
//!   Functions to set pixel color through [`Pixfmt`] are [`clear`], [`set`], [`copy_pixel`],
//!     [`copy_hline`], [`copy_vline`], [`fill`]
//!
//!   Functions to blend colors with existing pixels through [`Pixfmt`] are [`copy_or_blend_pix`], [`copy_or_blend_pix_with_cover`], [`blend_hline`], [`blend_vline`], [`blend_solid_hspan`], [`blend_solid_vspan`], [`blend_color_hspan`], [`blend_color_vspan`]
//!
//!
//! [`Pixfmt`]: pixfmt/struct.Pixfmt.html
//! [`clear`]: pixfmt/struct.Pixfmt.html#method.clear
//! [`set`]: pixfmt/struct.Pixfmt.html#method.set
//! [`copy_pixel`]: pixfmt/struct.Pixfmt.html#method.copy_pixel
//! [`copy_hline`]: pixfmt/struct.Pixfmt.html#method.copy_hline
//! [`copy_vline`]: pixfmt/struct.Pixfmt.html#method.copy_vline
//! [`fill`]: pixfmt/trait.PixelDraw.html#method.fill
//! [`copy_or_blend_pix`]: pixfmt/trait.PixelDraw.html#method.copy_or_blend_pix
//! [`copy_or_blend_pix_with_cover`]: pixfmt/trait.PixelDraw.html#method.copy_or_blend_pix_with_cover
//! [`blend_hline`]: pixfmt/trait.PixelDraw.html#method.blend_hline
//! [`blend_vline`]: pixfmt/trait.PixelDraw.html#method.blend_vline
//! [`blend_solid_hspan`]: pixfmt/trait.PixelDraw.html#method.blend_solid_hspan
//! [`blend_solid_vspan`]: pixfmt/trait.PixelDraw.html#method.blend_solid_vspan
//! [`blend_color_hspan`]: pixfmt/trait.PixelDraw.html#method.blend_color_hspan
//! [`blend_color_vspan`]: pixfmt/trait.PixelDraw.html#method.blend_color_vspan
//! [`render_scanlines`]: render/fn.render_scanlines.html
//! [`render_all_paths`]: render/fn.render_all_paths.html
//! [`render_scanlines_aa_solid`]: render/fn.render_scanlines_aa_solid.html
//! [`render_scanlines_bin_solid`]: render/fn.render_scanlines_bin_solid.html

#![allow(clippy::too_many_arguments, dead_code)]

#[macro_use]
extern crate log;

#[doc(hidden)]
pub use freetype as ft;

pub mod alphamask;
pub mod base;
pub mod clip;
pub mod color;
pub mod line_interp;
pub mod outline;
pub mod outline_aa;
pub mod paths;
pub mod pixfmt;
pub mod ppm;
pub mod raster;
pub mod render;
pub mod stroke;
pub mod text;
pub mod transform;

pub(crate) mod cell;
pub mod math;
pub(crate) mod scan;

pub mod gallery;

#[doc(hidden)]
pub use crate::alphamask::*;
#[doc(hidden)]
pub use crate::base::*;
#[doc(hidden)]
pub use crate::clip::*;
#[doc(hidden)]
pub use crate::color::*;
#[doc(hidden)]
pub use crate::line_interp::*;
#[doc(hidden)]
pub use crate::outline::*;
#[doc(hidden)]
pub use crate::outline_aa::*;
#[doc(hidden)]
pub use crate::paths::*;
#[doc(hidden)]
pub use crate::pixfmt::*;
#[doc(hidden)]
pub use crate::raster::*;
#[doc(hidden)]
pub use crate::render::*;
#[doc(hidden)]
pub use crate::stroke::*;
#[doc(hidden)]
pub use crate::text::*;
#[doc(hidden)]
pub use crate::transform::*;

const POLY_SUBPIXEL_SHIFT: i64 = 8;
const POLY_SUBPIXEL_SCALE: i64 = 1 << POLY_SUBPIXEL_SHIFT;
const POLY_SUBPIXEL_MASK: i64 = POLY_SUBPIXEL_SCALE - 1;
const POLY_MR_SUBPIXEL_SHIFT: i64 = 4;
const MAX_HALF_WIDTH: usize = 64;

/// Source of vertex points
pub trait VertexSource {
  /// Rewind the vertex source (unused)
  fn rewind(&self) {}
  /// Get values from the source
  ///
  /// This could be turned into an iterator
  fn xconvert(&self) -> Vec<Vertex<f64>>;
}

/// Render scanlines to Image
pub trait Render {
  /// Render a single scanlines to the image
  fn render(&mut self, data: &RenderData);
  /// Set the Color of the Renderer
  fn color<C: Color>(&mut self, color: C);
  /// Prepare the Renderer
  fn prepare(&self) {}
}
/*
/// Rasterize lines, path, and other things to scanlines
pub trait Rasterize {
    /// Setup Rasterizer, returns if data is available
    fn rewind_scanlines(&mut self) -> bool;
    /// Sweeps cells in a scanline for data, returns if data is available
    fn sweep_scanline(&mut self, sl: &mut ScanlineU8) -> bool;
    /// Return maximum x value of rasterizer
    fn min_x(&self) -> i64;
    /// Return maximum x value of rasterizer
    fn max_x(&self) -> i64;
    /// Resets the rasterizer, clearing content
    fn reset(&mut self);
    /// Rasterize a path
    fn add_path<VS: VertexSource>(&mut self, path: &VS);
}
*/

pub(crate) trait LineInterp {
  fn init(&mut self);
  fn step_hor(&mut self);
  fn step_ver(&mut self);
}

pub(crate) trait RenderOutline {
  fn cover(&self, d: i64) -> u64;
  fn blend_solid_hspan(&mut self, x: i64, y: i64, len: i64, covers: &[u64]);
  fn blend_solid_vspan(&mut self, x: i64, y: i64, len: i64, covers: &[u64]);
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

pub(crate) trait DistanceInterpolator {
  fn dist(&self) -> i64;
  fn inc_x(&mut self, dy: i64);
  fn inc_y(&mut self, dx: i64);
  fn dec_x(&mut self, dy: i64);
  fn dec_y(&mut self, dx: i64);
}

pub mod prelude {
  pub use crate::{
    Color, FromColor, FromRaw2 as _, FromRaw3 as _, FromRaw4 as _, IntoRaw2 as _, IntoRaw3 as _, IntoRaw4 as _,
    NamedColor as _, Pixel, Render as _, Source as _, VertexSource as _,
  };

  pub use crate::{DrawOutline, Pixfmt, PixfmtAlphaBlend, RenderingBase};
  pub use crate::{
    Gray8, Gray16, Gray32, Gray64, Rgb8, Rgb16, Rgb32, Rgb64, Rgba8, Rgba16, Rgba32, Rgba64, Srgba8, Srgba16, Srgba32,
    Srgba64,
  };
  pub use crate::{RasterizerOutline, RendererPrimatives};
  pub use crate::{RasterizerOutlineAA, RendererOutlineAA};
}
