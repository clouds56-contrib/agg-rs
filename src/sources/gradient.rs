use fixed::types::I28F4;

use crate::{LineInterpolator, PixelLike, Position, SubPixel, Transform};

pub trait GradientCalculation {
  fn calculate<P: PixelLike>(&self, x: P, y: P, d2: P) -> P;
}

#[derive(Debug)]
pub struct GradientX;
impl GradientCalculation for GradientX {
  fn calculate<P: PixelLike>(&self, x: P, _: P, _: P) -> P {
    x
  }
}

#[derive(Debug)]
pub struct GradientY;
impl GradientCalculation for GradientY {
  fn calculate<P: PixelLike>(&self, _: P, y: P, _: P) -> P {
    y
  }
}

#[derive(Debug)]
struct Interpolator<P> {
  li_x: LineInterpolator<P>,
  li_y: LineInterpolator<P>,
  trans: Transform,
}
impl<P: PixelLike> Interpolator<P> {
  pub fn new(trans: Transform, x: f64, y: f64, len: usize) -> Self {
    let (tx, ty) = trans.transform(x, y);
    let x1 = P::from_f64_rounded(tx);
    let y1 = P::from_f64_rounded(ty);

    let (tx, ty) = trans.transform(x + len as f64, y);
    let x2 = P::from_f64_rounded(tx);
    let y2 = P::from_f64_rounded(ty);

    Self {
      trans,
      li_x: LineInterpolator::new(x1, x2, len as i64),
      li_y: LineInterpolator::new(y1, y2, len as i64),
    }
  }
  pub fn inc(&mut self) {
    self.li_x.inc();
    self.li_y.inc();
  }
  pub fn coordinates(&self) -> (P, P) {
    (self.li_x.y, self.li_y.y)
  }
}

impl<P: PixelLike> Iterator for Interpolator<P> {
  type Item = (P, P);
  fn next(&mut self) -> Option<Self::Item> {
    let result = self.coordinates();
    self.inc();
    Some(result)
  }
}

pub trait Gradient {
  type Color;
  fn generate(&self, x: Position, y: Position, len: usize) -> Vec<Self::Color>;
}

/// SpanGradient
///
/// A small helper that generates a horizontal span of colors for a gradient.
///
/// Encapsulates gradient parameters and a color stop array and produces a
/// vector of colors for a horizontal span (used by scanline renderers).
///
/// Usage:
/// 1. Construct with [`SpanGradient::new(trans, gradient, &colors, d1, d2)`](Self::new).
/// 3. Call [`generate(x, y, len)`](Self::generate) to obtain a `Vec<C>` with `len` color entries
///    corresponding to the gradient along the horizontal span starting at
///    `(x, y)`.
///
/// Notes:
/// - `generate` requires [`G: GradientCalculation`](GradientCalculation) and uses an internal
///   [`Interpolator`] to apply `trans` and subpixel precision. The implementation
///   guards against a zero-length color range (`d2 - d1`) by treating it as at
///   least 1, so callers do not need to perform that check.
/// - [`Self::color`] should contain at least one entry; otherwise indexing will panic.
///   The subpixel shift used by this struct is `4` (see [`Self::subpixel_shift`]).
#[derive(Debug)]
pub struct SpanGradient<G, C, P4 = I28F4> {
  /// sub-pixel index of gradient start
  ///
  /// These determine the range used when mapping the computed gradient value to the color stops.
  d1: P4,
  /// sub-pixel index of gradient end
  ///
  /// These determine the range used when mapping the computed gradient value to the color stops.
  d2: P4,
  /// generic gradient calculator implementing [`GradientCalculation`].
  gradient: G,
  /// color stop array
  color: Vec<C>,
  /// transform applied to coordinates before gradient evaluation
  trans: Transform,
}

impl<G, C: Clone, P4: PixelLike> SpanGradient<G, C, P4> {
  #[inline]
  pub fn subpixel_shift(&self) -> i64 {
    4
  }
  #[inline]
  pub fn subpixel_scale(&self) -> i64 {
    1 << self.subpixel_shift()
  }
  pub fn new(trans: Transform, gradient: G, color: &[C], d1: f64, d2: f64) -> Self {
    Self {
      d1: P4::from_f64_rounded(d1),
      d2: P4::from_f64_rounded(d2),
      color: color.to_vec(),
      gradient,
      trans,
    }
  }
  pub fn d1(&mut self, d1: f64) {
    self.d1 = P4::from_f64_rounded(d1);
  }
  pub fn d2(&mut self, d2: f64) {
    self.d2 = P4::from_f64_rounded(d2);
  }
  pub fn prepare(&mut self) {}
}

impl<G: GradientCalculation, C: Clone, P4: PixelLike> Gradient for SpanGradient<G, C, P4> {
  type Color = C;
  fn generate(&self, x: Position, y: Position, len: usize) -> Vec<Self::Color> {

    // let downscale_shift = interp.subpixel_shift() - self.subpixel_shift();

    let mut dd = self.d2 - self.d1;
    if dd < 1 {
      dd = P4::ONE;
    }
    let ncolors = self.color.len();

    let interp = Interpolator::<SubPixel>::new(self.trans, x as f64 + 0.5, y as f64 + 0.5, len);

    interp.take(len)
      .map(|(x, y)| {
        let d = self
          .gradient
          .calculate(P4::from_fixed(x), P4::from_fixed(y), self.d2);
        let d = ((d - self.d1) * P4::from_f64_nearest(ncolors as f64)) / dd;
        let d = d.to_sub_pixel() as usize;
        self.color[d.clamp(0, ncolors - 1)].clone()
      })
      .collect()
  }
}
