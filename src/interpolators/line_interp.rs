use crate::renders::LineInterpolator;
//use crate::renders::RendererPrimatives;

use crate::DistanceInterpolator;
use crate::FixedLike;
use crate::LineParameters;
use crate::MAX_HALF_WIDTH;
use crate::POLY_SUBPIXEL_MASK;
use crate::POLY_SUBPIXEL_SCALE;
use crate::POLY_SUBPIXEL_SHIFT;
use crate::RealLike;
use crate::SubPixel;
use crate::U8;

/// Line Interpolator AA
#[derive(Debug)]
pub(crate) struct LineInterpolatorAA {
  /// Line Parameters
  pub lp: LineParameters,
  /// Line Interpolator
  pub li: LineInterpolator,
  /// Length of Line
  pub len: i64,
  /// Current x position of line in pixels
  pub x: i64,
  /// Current y position of line in pixels
  pub y: i64,
  /// Previous x position in pixels
  pub old_x: i64,
  /// Previous y position in pixels
  pub old_y: i64,
  /// Number of pixels from start to end points
  ///  in either the `y` or `x` direction
  pub count: i64,
  /// Width of line in subpixels width
  pub width: i64,
  /// Maximum width of line in pixels
  pub max_extent: i64,

  pub step: i64,
  //pub dist: [i64; MAX_HALF_WIDTH + 1],
  pub dist: Vec<i64>,
  //pub covers: [u64; MAX_HALF_WIDTH * 2 + 4],
  pub covers: Vec<U8>,
}

impl LineInterpolatorAA {
  /// Create new Line Interpolator AA
  pub fn new(lp: LineParameters, subpixel_width: i64) -> Self {
    let len = if lp.vertical == (lp.inc > 0) { -lp.len } else { lp.len };
    let x = lp.x1 >> POLY_SUBPIXEL_SHIFT;
    let y = lp.y1 >> POLY_SUBPIXEL_SHIFT;
    let old_x = x;
    let old_y = y;
    let count = if lp.vertical {
      ((lp.y2 >> POLY_SUBPIXEL_SHIFT) - y).abs()
    } else {
      ((lp.x2 >> POLY_SUBPIXEL_SHIFT) - x).abs()
    };
    let width = subpixel_width;
    let max_extent = (width + POLY_SUBPIXEL_MASK) >> POLY_SUBPIXEL_SHIFT;
    let step = 0;
    let y1 = if lp.vertical {
      (lp.x2 - lp.x1) << POLY_SUBPIXEL_SHIFT
    } else {
      (lp.y2 - lp.y1) << POLY_SUBPIXEL_SHIFT
    };
    let n = if lp.vertical {
      (lp.y2 - lp.y1).abs()
    } else {
      (lp.x2 - lp.x1).abs() + 1
    };

    // Setup Number Interpolator from 0 .. y1 with n segments
    let m_li = LineInterpolator::new_back_adjusted_2(SubPixel::from_raw(y1), n);

    // Length of line in subpixels
    let mut dd = if lp.vertical { lp.dy } else { lp.dx };
    dd <<= POLY_SUBPIXEL_SHIFT; // to subpixels
    let mut li = LineInterpolator::new_foward_adjusted(SubPixel::ZERO, SubPixel::from_raw(dd), lp.len);

    // Get Distances along the line
    let mut dist = vec![0i64; MAX_HALF_WIDTH + 1];
    let stop = width + POLY_SUBPIXEL_SCALE * 2;
    for d in dist.iter_mut().take(MAX_HALF_WIDTH) {
      *d = li.y.into_raw();
      if li.y >= stop {
        break;
      }
      li.inc();
    }
    dist[MAX_HALF_WIDTH] = 0x7FFF_0000;
    // Setup covers to 0
    let covers = vec![U8::ZERO; MAX_HALF_WIDTH * 2 + 4];
    Self {
      lp,
      li: m_li,
      len,
      x,
      y,
      old_x,
      old_y,
      count,
      width,
      max_extent,
      step,
      dist,
      covers,
    }
  }
  /// Step the Line forward horizontally
  pub(crate) fn step_hor_base<DI>(&mut self, di: &mut DI) -> i64
  where
    DI: DistanceInterpolator,
  {
    // Increment the Interpolator
    self.li.inc();
    // Increment the x by the LineParameter increment, typically +1 or -1
    self.x += self.lp.inc;
    // Set y value to initial + new y value
    self.y = (self.lp.y1 + self.li.y.into_raw()) >> POLY_SUBPIXEL_SHIFT;
    // "Increment" the distance interpolator
    if self.lp.inc > 0 {
      di.inc_x(self.y - self.old_y);
    } else {
      di.dec_x(self.y - self.old_y);
    }
    // Save current point
    self.old_y = self.y;
    // Return some measure of distance
    di.dist() / self.len
  }
  pub(crate) fn step_ver_base<DI>(&mut self, di: &mut DI) -> i64
  where
    DI: DistanceInterpolator,
  {
    self.li.inc();
    self.y += self.lp.inc;
    self.x = (self.lp.x1 + self.li.y.into_raw()) >> POLY_SUBPIXEL_SHIFT;

    if self.lp.inc > 0 {
      di.inc_y(self.x - self.old_x);
    } else {
      di.dec_y(self.x - self.old_x);
    }

    self.old_x = self.x;
    di.dist() / self.len
  }
}

#[derive(Debug, Default)]
pub(crate) struct DrawVars {
  pub idx: usize,
  pub x1: i64,
  pub y1: i64,
  pub x2: i64,
  pub y2: i64,
  pub curr: LineParameters,
  pub next: LineParameters,
  pub lcurr: i64,
  pub lnext: i64,
  pub xb1: i64,
  pub yb1: i64,
  pub xb2: i64,
  pub yb2: i64,
  pub flags: u8,
}

impl DrawVars {
  pub fn new() -> Self {
    Self { ..Default::default() }
  }
}
