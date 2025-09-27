use crate::{LineInterpolator, Position, Transform};

pub trait GradientCalculation {
  fn calculate(&self, x: i64, y: i64, d2: i64) -> i64;
}

#[derive(Debug)]
pub struct GradientX;
impl GradientCalculation for GradientX {
  fn calculate(&self, x: i64, _: i64, _: i64) -> i64 {
    x
  }
}

#[derive(Debug)]
pub struct GradientY;
impl GradientCalculation for GradientY {
  fn calculate(&self, _: i64, y: i64, _: i64) -> i64 {
    y
  }
}

#[derive(Debug)]
struct Interpolator {
  li_x: Option<LineInterpolator>,
  li_y: Option<LineInterpolator>,
  trans: Transform,
}
impl Interpolator {
  #[inline]
  pub fn subpixel_shift(&self) -> i64 {
    8
  }
  #[inline]
  pub fn subpixel_scale(&self) -> i64 {
    1 << self.subpixel_shift()
  }
  pub fn new(trans: Transform) -> Self {
    Self {
      trans,
      li_x: None,
      li_y: None,
    }
  }
  pub fn begin(&mut self, x: f64, y: f64, len: usize) {
    let tx = x;
    let ty = y;
    let (tx, ty) = self.trans.transform(tx, ty);
    let x1 = (tx * self.subpixel_scale() as f64).round() as i64;
    let y1 = (ty * self.subpixel_scale() as f64).round() as i64;

    let tx = x + len as f64;
    let ty = y;
    let (tx, ty) = self.trans.transform(tx, ty);
    let x2 = (tx * self.subpixel_scale() as f64).round() as i64;
    let y2 = (ty * self.subpixel_scale() as f64).round() as i64;
    self.li_x = Some(LineInterpolator::new(x1, x2, len as i64));
    self.li_y = Some(LineInterpolator::new(y1, y2, len as i64));
  }
  pub fn inc(&mut self) {
    if let Some(ref mut li) = self.li_x {
      (li).inc();
    }
    if let Some(ref mut li) = self.li_y {
      (li).inc();
    }
  }
  pub fn coordinates(&self) -> (i64, i64) {
    if let (Some(x), Some(y)) = (self.li_x.as_ref(), self.li_y.as_ref()) {
      (x.y, y.y)
    } else {
      panic!("Interpolator not Initialized");
    }
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
pub struct SpanGradient<G, C> {
  /// sub-pixel index of gradient start
  ///
  /// These determine the range used when mapping the computed gradient value to the color stops.
  d1: i64,
  /// sub-pixel index of gradient end
  ///
  /// These determine the range used when mapping the computed gradient value to the color stops.
  d2: i64,
  /// generic gradient calculator implementing [`GradientCalculation`].
  gradient: G,
  /// color stop array
  color: Vec<C>,
  /// transform applied to coordinates before gradient evaluation
  trans: Transform,
}

impl<G, C: Clone> SpanGradient<G, C> {
  #[inline]
  pub fn subpixel_shift(&self) -> i64 {
    4
  }
  #[inline]
  pub fn subpixel_scale(&self) -> i64 {
    1 << self.subpixel_shift()
  }
  pub fn new(trans: Transform, gradient: G, color: &[C], d1: f64, d2: f64) -> Self {
    let mut s = Self {
      d1: 0,
      d2: 1,
      color: color.to_vec(),
      gradient,
      trans,
    };
    s.d1(d1);
    s.d2(d2);
    s
  }
  pub fn d1(&mut self, d1: f64) {
    self.d1 = (d1 * self.subpixel_scale() as f64).round() as i64;
  }
  pub fn d2(&mut self, d2: f64) {
    self.d2 = (d2 * self.subpixel_scale() as f64).round() as i64;
  }
  pub fn prepare(&mut self) {}
}

impl<G: GradientCalculation, C: Clone> Gradient for SpanGradient<G, C> {
  type Color = C;
  fn generate(&self, x: Position, y: Position, len: usize) -> Vec<Self::Color> {
    let mut interp = Interpolator::new(self.trans);

    let downscale_shift = interp.subpixel_shift() - self.subpixel_shift();

    let mut dd = self.d2 - self.d1;
    if dd < 1 {
      dd = 1;
    }
    let ncolors = self.color.len() as i64;

    interp.begin(x as f64 + 0.5, y as f64 + 0.5, len);

    (0..len)
      .map(|_| {
        let (x, y) = interp.coordinates();
        let d = self
          .gradient
          .calculate(x >> downscale_shift, y >> downscale_shift, self.d2);
        let mut d = ((d - self.d1) * ncolors) / dd;
        if d < 0 {
          d = 0;
        }
        if d >= ncolors {
          d = ncolors - 1;
        }
        interp.inc();
        self.color[d as usize].clone()
      })
      .collect()
  }
}
