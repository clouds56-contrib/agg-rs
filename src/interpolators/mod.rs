pub mod gradient;
pub mod line_interp;

pub use gradient::*;
pub use line_interp::*;

pub(crate) trait LineInterp {
  fn init(&mut self);
  fn step_hor(&mut self);
  fn step_ver(&mut self);
}

pub(crate) trait DistanceInterpolator {
  fn dist(&self) -> i64;
  fn inc_x(&mut self, dy: i64);
  fn inc_y(&mut self, dx: i64);
  fn dec_x(&mut self, dy: i64);
  fn dec_y(&mut self, dx: i64);
}
