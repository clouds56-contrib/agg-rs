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
  pub fn dec(&mut self) {
    self.li_x.dec();
    self.li_y.dec();
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
    if dd < P4::EPSILON {
      dd = P4::EPSILON;
    }
    let ncolors = self.color.len();

    let interp = Interpolator::<SubPixel>::new(self.trans, x as f64 + 0.5, y as f64 + 0.5, len);

    interp.take(len)
      .map(|(x, y)| {
        let d = self
          .gradient
          .calculate(P4::from_fixed(x), P4::from_fixed(y), self.d2);
        let d = ((d - self.d1) * P4::from_f64_nearest(ncolors as f64)) / dd;
        let d = (d >> P4::SHIFT).to_sub_pixel().clamp(0, ncolors as i64 - 1);
        self.color[d as usize].clone()
      })
      .collect()
  }
}

#[cfg(test)]
mod tests {
  use assert_approx_eq::assert_approx_eq;

  use crate::FixedLike;

  use super::*;

  #[test]
  fn test_interpolator() {
    let mut interp = Interpolator::<SubPixel>::new(Transform::new().then_scale(1.6, 1.0), 0.0, 0.0, 10);
    let coords = interp.by_ref().take(11).collect::<Vec<_>>();
    let expected = vec![
      (0.0, 0.0),
      (1.6, 0.0),
      (3.2, 0.0),
      (4.8, 0.0),
      (6.4, 0.0),
      (8.0, 0.0),
      (9.6, 0.0),
      (11.2, 0.0),
      (12.8, 0.0),
      (14.4, 0.0),
      (16.0, 0.0),
    ];
    assert_eq!(coords, expected.iter().map(|(x, y)| (SubPixel::from_f64_nearest(*x), SubPixel::from_f64_nearest(*y))).collect::<Vec<_>>());

    let mut interp = Interpolator::<SubPixel>::new(Transform::new().then_scale(1.0 / 6.0, 1.0), 0.0, 0.0, 7);
    let coords = interp.by_ref().take(8).collect::<Vec<_>>();
    let expected = vec![
      (0.0, 0.0),
      (1.0 / 6.0, 0.0),
      (2.0 / 6.0, 0.0),
      (3.0 / 6.0, 0.0), // 0.33333333 in [0.33203125, 0.3359375)
      (4.0 / 6.0, 0.0),
      (5.0 / 6.0, 0.0),
      (6.0 / 6.0, 0.0),
      (7.0 / 6.0, 0.0),
    ];
    assert_eq!(coords, expected.iter().map(|(x, y)| (SubPixel::from_f64_ceiled(*x), SubPixel::from_f64_ceiled(*y))).collect::<Vec<_>>());
  }

  #[test]
  fn test_span_gradient() {
    let colors = vec![
      (255, 0, 0),
      (255, 255, 0),
      (0, 255, 0),
      (0, 255, 255),
      (0, 0, 255),
      (255, 0, 255),
      (255, 0, 0),
    ];
    let grad = SpanGradient::<_, _>::new(Transform::new(), GradientX, &colors, 0.0, 7.0);
    let result = grad.generate(0, 0, 7);
    assert_eq!(result, colors);

    let colors = vec![1,2,3,4,5,6,7];
    let grad = SpanGradient::<_, _>::new(Transform::new().then_scale(1.0 / 6.0, 1.0), GradientX, &colors, 0.0, 6.0);
    let result = grad.generate(0, 0, 7);
    assert_eq!(result, vec![1, 1, 1, 1, 1, 2, 2]);
  }

  #[test]
  fn test_span_gradient_test_aa() {
    fn calc_linear_gradient_transform(x1: f64, y1: f64, x2: f64, y2: f64) -> Transform {
      let gradient_d2 = 100.0;
      let dx = x2 - x1;
      let dy = y2 - y1;
      let s = (dx * dx + dy * dy).sqrt() / gradient_d2;

      Transform::new()
        .then_scale(s, s)
        .then_rotate(dy.atan2(dx))
        .then_translate(x1 + 0.5, y1 + 0.5)
        .then_invert()
    }
    for kk in 0..=1 {
      let k = kk as f64;
      let x1 = 20.0 + k * (k + 1.0);
      let y1 = 40.5;
      let x2 = 20.0 + k * (k + 1.0) + ((k - 1.0) * 4.0);
      let y2 = 100.5;
      // println!("{k}: ({x1},{y1}) -> ({x2},{y2})");
      let gradient_mtx = calc_linear_gradient_transform(x1, y1, x2, y2);
      assert_approx_eq!(gradient_mtx.transform(x1 + 0.5, y1 + 0.5).0, 0.0);
      assert_approx_eq!(gradient_mtx.transform(x1 + 0.5, y1 + 0.5).1, 0.0);
      assert_approx_eq!(gradient_mtx.transform(x2 + 0.5, y2 + 0.5).0, 100.0);
      assert_approx_eq!(gradient_mtx.transform(x2 + 0.5, y2 + 0.5).1, 0.0);

      assert_approx_eq!(gradient_mtx.then_invert().transform(0.0, 0.0).0, x1 + 0.5);
      assert_approx_eq!(gradient_mtx.then_invert().transform(0.0, 0.0).1, y1 + 0.5);
      assert_approx_eq!(gradient_mtx.then_invert().transform(100.0, 0.0).0, x2 + 0.5);
      assert_approx_eq!(gradient_mtx.then_invert().transform(100.0, 0.0).1, y2 + 0.5);
      let gradient_colors = (0..256u16).collect::<Vec<_>>();
      let span = SpanGradient::<_, _>::new(gradient_mtx, GradientX, &gradient_colors, 0.0, 100.0);

      let line = Interpolator::<SubPixel>::new(
        gradient_mtx.then_invert(),
        -10., 0., 120,
      );
      let colors = line.take(121).map(|(x, y)| {
        let g = span.generate(x.ipart() as Position, y.ipart() as Position, 1);
        g[0]
      }).collect::<Vec<_>>();
      if kk == 0 {
        assert_eq!(colors, vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 6, 6, 10, 14, 14, 19, 19, 23, 27, 27, 31, 32, 36, 40, 40, 44, 44, 49, 53, 53, 57, 57, 61, 66, 66, 70, 70, 74, 78, 78, 83, 83, 87, 91, 91, 95, 96, 100, 104, 104, 108, 108, 113, 117, 117, 121, 121, 125, 130, 130, 134, 134, 138, 142, 142, 147, 147, 151, 155, 155, 159, 160, 164, 168, 168, 172, 172, 177, 181, 181, 185, 185, 189, 194, 194, 198, 198, 202, 206, 206, 211, 211, 215, 219, 219, 223, 224, 228, 232, 232, 236, 236, 241, 245, 245, 249, 249, 253, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255]);
      } else
      if kk == 1 {
        assert_eq!(colors, vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 6, 6, 10, 14, 14, 19, 19, 23, 27, 27, 32, 32, 36, 40, 40, 44, 44, 48, 53, 53, 57, 57, 61, 66, 66, 70, 70, 74, 78, 78, 83, 83, 87, 91, 91, 96, 96, 100, 104, 104, 108, 108, 112, 117, 117, 121, 121, 125, 130, 130, 134, 134, 138, 142, 142, 147, 147, 151, 155, 155, 160, 160, 164, 168, 168, 172, 172, 176, 181, 181, 185, 185, 189, 194, 194, 198, 198, 202, 206, 206, 211, 211, 215, 219, 219, 224, 224, 228, 232, 232, 236, 236, 240, 245, 245, 249, 249, 253, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255]);
      }
    }
  }
}
