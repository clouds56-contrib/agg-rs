//! Transformations

use crate::paths::Path;
use crate::paths::Vertex;

use crate::VertexSource;

use std::ops::Mul;
use std::ops::MulAssign;

/// Transformation
#[derive(Debug, Default, Copy, Clone, PartialEq)]
/// double sx, shy, shx, sy, tx, ty;
pub struct Transform {
  pub sx: f64,
  pub sy: f64,
  pub shx: f64,
  pub shy: f64,
  pub tx: f64,
  pub ty: f64,
}

impl Transform {
  pub fn affine(sx: f64, shy: f64, shx: f64, sy: f64, tx: f64, ty: f64) -> Self {
    Self {
      sx,
      sy,
      shx,
      shy,
      tx,
      ty,
    }
  }
  /// Creates a new Transform
  pub fn new() -> Self {
    Self::identity()
  }
  pub fn identity() -> Self {
    Self::affine(1.0, 0.0, 0.0, 1.0, 0.0, 0.0)
  }
  pub fn rotation(angle: f64) -> Self {
    let ca = angle.cos();
    let sa = angle.sin();
    Self::affine(ca, sa, -sa, ca, 0.0, 0.0)
  }
  pub fn scaling(sx: f64, sy: f64) -> Self {
    Self::affine(sx, 0.0, 0.0, sy, 0.0, 0.0)
  }
  pub fn translation(dx: f64, dy: f64) -> Self {
    Self::affine(1.0, 0.0, 0.0, 1.0, dx, dy)
  }
  pub fn skewing(x: f64, y: f64) -> Self {
    Self::affine(1.0, y.tan(), x.tan(), 1.0, 0.0, 0.0)
  }
  /// Add a translation to the transform
  #[must_use]
  pub fn then_translate(mut self, dx: f64, dy: f64) -> Self {
    self.tx += dx;
    self.ty += dy;
    self
  }
  /// Add a scaling to the transform
  #[must_use]
  pub fn then_scale(mut self, sx: f64, sy: f64) -> Self {
    self.sx *= sx;
    self.shx *= sx;
    self.tx *= sx;
    self.sy *= sy;
    self.shy *= sy;
    self.ty *= sy;
    self
  }
  /// Add a rotation to the transform
  ///
  /// angle is in radians
  #[must_use]
  pub fn then_rotate(self, angle: f64) -> Self {
    let ca = angle.cos();
    let sa = angle.sin();
    let sx = self.sx * ca - self.shy * sa;
    let shx = self.shx * ca - self.sy * sa;
    let tx = self.tx * ca - self.ty * sa;
    let shy = self.sx * sa + self.shy * ca;
    let sy = self.shx * sa + self.sy * ca;
    let ty = self.tx * sa + self.ty * ca;
    Self::affine(sx, shy, shx, sy, tx, ty)
  }

  /// Perform the transform
  pub fn transform(&self, x: f64, y: f64) -> (f64, f64) {
    (
      x * self.sx + y * self.shx + self.tx,
      x * self.shy + y * self.sy + self.ty,
    )
  }
  fn determinant(&self) -> f64 {
    self.sx * self.sy - self.shy * self.shx
  }
  #[must_use]
  pub fn then_invert(mut self) -> Self {
    let d = 1.0 / self.determinant();
    let sx = self.sy * d;
    self.sy = self.sx * d;
    self.shy = -self.shy * d;
    self.shx = -self.shx * d;
    let tx = -self.tx * sx - self.ty * self.shx;
    self.ty = -self.tx * self.shy - self.ty * self.sy;

    self.sx = sx;
    self.tx = tx;
    self
  }
  #[must_use]
  pub fn then(self, m: Self) -> Self {
    let t0 = self.sx * m.sx + self.shy * m.shx;
    let t2 = self.shx * m.sx + self.sy * m.shx;
    let t4 = self.tx * m.sx + self.ty * m.shx + m.tx;
    let shy = self.sx * m.shy + self.shy * m.sy;
    let sy = self.shx * m.shy + self.sy * m.sy;
    let ty = self.tx * m.shy + self.ty * m.sy + m.ty;
    let sx = t0;
    let shx = t2;
    let tx = t4;
    Self::affine(sx, shy, shx, sy, tx, ty)
  }
}

impl Mul<Transform> for Transform {
  type Output = Transform;
  fn mul(self, rhs: Transform) -> Self {
    self.then(rhs)
  }
}

impl MulAssign<Transform> for Transform {
  fn mul_assign(&mut self, rhs: Transform) {
    *self = *self * rhs;
  }
}

/// Path Transform
#[derive(Debug, Default)]
pub struct ConvTransform {
  /// Source Path to Transform
  pub source: Path,
  /// Transform to apply
  pub trans: Transform,
}

impl VertexSource for ConvTransform {
  /// Apply the Transform
  fn xconvert(&self) -> Vec<Vertex<f64>> {
    self.transform()
  }
}

impl ConvTransform {
  /// Create a new Path Transform
  pub fn new(source: Path, trans: Transform) -> Self {
    Self { source, trans }
  }
  /// Transform the Path
  pub fn transform(&self) -> Vec<Vertex<f64>> {
    let mut out = vec![];
    for v in &self.source.xconvert() {
      let (x, y) = self.trans.transform(v.x, v.y);
      out.push(Vertex::new(x, y, v.cmd));
    }
    out
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test_transform() {
    let m = Transform::new()
      .then_translate(10.0, 20.0)
      .then_scale(2.0, 3.0)
      .then_rotate(std::f64::consts::FRAC_PI_2);
    let m2 = Transform::new()
      * Transform::translation(10.0, 20.0)
      * Transform::scaling(2.0, 3.0)
      * Transform::rotation(std::f64::consts::FRAC_PI_2);
    assert!(m == m2);
    let mut m2 = Transform::new();
    m2 *= Transform::translation(10.0, 20.0);
    m2 *= Transform::scaling(2.0, 3.0);
    m2 *= Transform::rotation(std::f64::consts::FRAC_PI_2);
    assert!(m == m2);

    let (x, y) = m.transform(1.0, 1.0);
    assert!((x + 63.0).abs() < 1e-10);
    assert!((y - 22.0).abs() < 1e-10);
    let mi = m.then_invert();
    let (x2, y2) = mi.transform(x, y);
    assert!((x2 - 1.0).abs() < 1e-10);
    assert!((y2 - 1.0).abs() < 1e-10);
  }
}
