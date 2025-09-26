const BASE_SHIFT: u8 = u8::BITS as u8;
const BASE_MSB: u8 = 1 << (BASE_SHIFT - 1);

/// Interpolate a value between two end points using fixed point math
///
/// See agg_color_rgba.h:454 of agg version 2.4
pub fn lerp_u8(p: u8, q: u8, a: u8) -> u8 {
  let v = if p > q { 1 } else { 0 };
  let (q, p, a) = (i32::from(q), i32::from(p), i32::from(a));
  let t0: i32 = (q - p) * a + BASE_MSB as i32 - v; // Signed multiplication
  let t1: i32 = ((t0 >> BASE_SHIFT) + t0) >> BASE_SHIFT;
  (p + t1) as u8
}

/// Interpolator a value between two end points pre-calculated by alpha
///
/// p + q - (p*a)
pub fn prelerp_u8(p: u8, q: u8, a: u8) -> u8 {
  ((p as i32) + (q as i32) - multiply_u8(p, a) as i32).clamp(0, u8::MAX as i32) as u8
}

/// Multiply two u8 values using fixed point math
///
/// See agg_color_rgba.h:395
/// https://sestevenson.wordpress.com/2009/08/19/rounding-in-fixed-point-number-conversions/
/// https://stackoverflow.com/questions/10067510/fixed-point-arithmetic-in-c-programming
/// http://x86asm.net/articles/fixed-point-arithmetic-and-tricks/
/// Still not sure where the value is added and shifted multiple times
pub fn multiply_u8(a: u8, b: u8) -> u8 {
  let t = a as u32 * b as u32 + BASE_MSB as u32;
  let tt = ((t >> BASE_SHIFT) + t) >> BASE_SHIFT;
  tt as u8
}

pub fn combine_u8(a: u8, b: u8) -> u8 {
  let t = u8::MAX as u16 + (a as u16 * b as u16);
  (t >> 8).clamp(0, u8::MAX as u16) as u8
}

#[cfg(test)]
mod tests {
  use super::*;

  fn mu864(i: u8, j: u8) -> u8 {
    let i = i as f64 / 255.0;
    let j = j as f64 / 255.0;
    let c = i * j;
    (c * 255.0).round() as u8
  }
  fn lerp_u8_f64(p: u8, q: u8, a: u8) -> u8 {
    let p = p as f64 / 255.0;
    let q = q as f64 / 255.0;
    let a = a as f64 / 255.0;
    let v = a * (q - p) + p;
    (v * 255.0).round() as u8
  }

  fn prelerp_u8_f64(p: u8, q: u8, a: u8) -> u8 {
    let p = p as f64 / 255.0;
    let q = q as f64 / 255.0;
    let a = a as f64 / 255.0;
    let v = p + q - a * p;
    (v * 255.0).round() as u8
  }

  #[test]
  fn lerp_u8_test() {
    for p in 0..=255 {
      for q in 0..=255 {
        for a in 0..=255 {
          let (p, q, a) = (p as u8, q as u8, a as u8);
          let v = lerp_u8_f64(p, q, a);
          assert_eq!(lerp_u8(p, q, a), v, "lerp({p},{q},{a}) = {v}");
        }
      }
    }
  }

  #[test]
  fn perlerp_u8_test() {
    for p in 0..=255 {
      for q in 0..=255 {
        for a in 0..=255 {
          let (p, q, a) = (p as u8, q as u8, a as u8);
          let v = prelerp_u8_f64(p, q, a);
          assert_eq!(prelerp_u8(p, q, a), v, "prelerp({p},{q},{a}) = {v}");
        }
      }
    }
  }
  #[test]
  fn multiply_u8_test() {
    let mut diff = std::collections::BTreeMap::new();
    for i in 0..=255 {
      for j in 0..=255 {
        let v = mu864(i, j);
        assert_eq!(multiply_u8(i, j), v, "{i} * {j} = {v}");
        let v2 = combine_u8(i, j);
        let d = v2 as i32 - v as i32;
        assert!(d.abs() <= 1, "{i} * {j} = {v2}");
        *diff.entry(d).or_insert(0) += 1;
      }
    }
    assert_eq!(diff.len(), 3);
    assert_eq!(diff[&0], 46776);
    assert_eq!(diff[&1], 17284);
    assert_eq!(diff[&-1], 1476);
  }
}
