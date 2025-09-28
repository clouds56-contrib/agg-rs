use crate::{
  DistanceInterpolator, DistanceInterpolator1, DistanceInterpolator2, DistanceInterpolator3, FixedLike,
  LineInterpolatorAA, LineParameters, MAX_HALF_WIDTH, POLY_SUBPIXEL_MASK, POLY_SUBPIXEL_SHIFT, RealLike, RenderOutline,
  U8,
};

/// Line Interpolator0
#[derive(Debug)]
pub(crate) struct AA0 {
  /// Distance Interpolator v1
  pub di: DistanceInterpolator1,
  /// Line Interpolator AA-version
  pub li: LineInterpolatorAA,
}
impl AA0 {
  /// Create a new Line Interpolator-0
  pub fn new(lp: LineParameters, subpixel_width: i64) -> Self {
    let mut li = LineInterpolatorAA::new(lp, subpixel_width);
    li.li.adjust_forward();
    Self {
      li,
      di: DistanceInterpolator1::new(
        lp.x1,
        lp.y1,
        lp.x2,
        lp.y2,
        lp.x1 & !POLY_SUBPIXEL_MASK,
        lp.y1 & !POLY_SUBPIXEL_MASK,
      ),
    }
  }
  /// Size of the Interpolation
  pub fn count(&self) -> i64 {
    self.li.count
  }
  /// Return if the line is more Vertical than horizontal
  pub fn vertical(&self) -> bool {
    self.li.lp.vertical
  }
  /// Conduct a horizontal step, used for "horizontal lines"
  pub fn step_hor<R>(&mut self, ren: &mut R) -> bool
  where
    R: RenderOutline<Cover = U8>,
  {
    // Step the Interpolator horizontally and get the width
    //   projected onto the vertical
    let s1 = self.li.step_hor_base(&mut self.di);
    let mut p0 = MAX_HALF_WIDTH + 2;
    let mut p1 = p0;

    // Get the coverage at the center for value of s1
    self.li.covers[p1] = ren.cover(s1);

    p1 += 1;
    //Generate covers for "one" side of the line
    let mut dy = 1;
    let mut dist = self.li.dist[dy] - s1;
    while dist <= self.li.width {
      self.li.covers[p1] = ren.cover(dist);
      p1 += 1;
      dy += 1;
      dist = self.li.dist[dy] - s1;
    }
    //Generate covers for the "other" side of the line
    let mut dy = 1;
    dist = self.li.dist[dy] + s1;
    while dist <= self.li.width {
      p0 -= 1;
      self.li.covers[p0] = ren.cover(dist);
      dy += 1;
      dist = self.li.dist[dy] + s1;
    }
    // Draw Line using coverages
    ren.blend_solid_vspan(
      self.li.x,
      self.li.y - dy as i64 + 1,
      (p1 - p0) as i64,
      &self.li.covers[p0..],
    );
    // Step the Line Interpolator AA
    self.li.step += 1;
    self.li.step < self.li.count
  }
  /// Conduct a vertical step, used for "vertical lines"
  pub fn step_ver<R: RenderOutline<Cover = U8>>(&mut self, ren: &mut R) -> bool {
    let s1 = self.li.step_ver_base(&mut self.di);
    let mut p0 = MAX_HALF_WIDTH + 2;
    let mut p1 = p0;
    self.li.covers[p1] = ren.cover(s1);
    p1 += 1;
    let mut dx = 1;
    let mut dist = self.li.dist[dx] - s1;
    while dist <= self.li.width {
      self.li.covers[p1] = ren.cover(dist);
      p1 += 1;
      dx += 1;
      dist = self.li.dist[dx] - s1;
    }

    dx = 1;
    dist = self.li.dist[dx] + s1;
    while dist <= self.li.width {
      p0 -= 1;
      self.li.covers[p0] = ren.cover(dist);
      dx += 1;
      dist = self.li.dist[dx] + s1;
    }
    ren.blend_solid_hspan(
      self.li.x - dx as i64 + 1,
      self.li.y,
      (p1 - p0) as i64,
      &self.li.covers[p0..],
    );
    self.li.step += 1;
    self.li.step < self.li.count
  }
}
#[derive(Debug)]
pub(crate) struct AA1 {
  pub di: DistanceInterpolator2,
  pub li: LineInterpolatorAA,
}
impl AA1 {
  pub fn new(lp: LineParameters, sx: i64, sy: i64, subpixel_width: i64) -> Self {
    let mut li = LineInterpolatorAA::new(lp, subpixel_width);
    let mut di = DistanceInterpolator2::new(
      lp.x1,
      lp.y1,
      lp.x2,
      lp.y2,
      sx,
      sy,
      lp.x1 & !POLY_SUBPIXEL_MASK,
      lp.y1 & !POLY_SUBPIXEL_MASK,
      true,
    );
    let mut npix = 1;
    if lp.vertical {
      loop {
        li.li.dec();
        li.y -= lp.inc;
        li.x = (li.lp.x1 + li.li.y.into_raw()) >> POLY_SUBPIXEL_SHIFT;

        if lp.inc > 0 {
          di.dec_y(li.x - li.old_x);
        } else {
          di.inc_y(li.x - li.old_x);
        }
        li.old_x = li.x;

        let mut dist1_start = di.dist_start;
        let mut dist2_start = di.dist_start;

        let mut dx = 0;
        if dist1_start < 0 {
          npix += 1;
        }
        loop {
          dist1_start += di.dy_start;
          dist2_start -= di.dy_start;
          if dist1_start < 0 {
            npix += 1;
          }
          if dist2_start < 0 {
            npix += 1
          }
          dx += 1;
          if li.dist[dx] > li.width {
            break;
          }
        }
        li.step -= 1;
        if npix == 0 {
          break;
        }
        npix = 0;
        if li.step < -li.max_extent {
          break;
        }
      }
    } else {
      loop {
        li.li.dec();
        li.x -= lp.inc;
        li.y = (li.lp.y1 + li.li.y.into_raw()) >> POLY_SUBPIXEL_SHIFT;
        if lp.inc > 0 {
          di.dec_x(li.y - li.old_y);
        } else {
          di.inc_x(li.y - li.old_y);
        }
        li.old_y = li.y;

        let mut dist1_start = di.dist_start;
        let mut dist2_start = di.dist_start;

        let mut dy = 0;
        if dist1_start < 0 {
          npix += 1;
        }
        loop {
          dist1_start -= di.dx_start;
          dist2_start += di.dx_start;
          if dist1_start < 0 {
            npix += 1;
          }
          if dist2_start < 0 {
            npix += 1;
          }
          dy += 1;
          if li.dist[dy] > li.width {
            break;
          }
        }
        li.step -= 1;
        if npix == 0 {
          break;
        }
        npix = 0;
        if li.step < -li.max_extent {
          break;
        }
      }
    }
    li.li.adjust_forward();
    Self { li, di }
  }
  //pub fn count(&self) -> i64 {        self.li.count    }
  pub fn vertical(&self) -> bool {
    self.li.lp.vertical
  }
  pub fn step_hor<R: RenderOutline<Cover = U8>>(&mut self, ren: &mut R) -> bool {
    let s1 = self.li.step_hor_base(&mut self.di);

    let mut dist_start = self.di.dist_start;
    let mut p0 = MAX_HALF_WIDTH + 2;
    let mut p1 = p0;
    self.li.covers[p1] = RealLike::ZERO;
    if dist_start <= 0 {
      self.li.covers[p1] = ren.cover(s1);
    }
    p1 += 1;
    let mut dy = 1;
    let mut dist = self.li.dist[dy] - s1;
    while dist <= self.li.width {
      dist_start -= self.di.dx_start;
      self.li.covers[p1] = RealLike::ZERO;
      if dist_start <= 0 {
        self.li.covers[p1] = ren.cover(dist);
      }
      p1 += 1;
      dy += 1;
      dist = self.li.dist[dy] - s1;
    }

    dy = 1;
    dist_start = self.di.dist_start;
    dist = self.li.dist[dy] + s1;
    while dist <= self.li.width {
      dist_start += self.di.dx_start;
      p0 -= 1;
      self.li.covers[p0] = RealLike::ZERO;
      if dist_start <= 0 {
        self.li.covers[p0] = ren.cover(dist);
      }
      dy += 1;
      dist = self.li.dist[dy] + s1;
    }
    ren.blend_solid_vspan(
      self.li.x,
      self.li.y - dy as i64 + 1,
      (p1 - p0) as i64,
      &self.li.covers[p0..],
    );
    self.li.step += 1;
    self.li.step < self.li.count
  }
  pub fn step_ver<R: RenderOutline<Cover = U8>>(&mut self, ren: &mut R) -> bool {
    let s1 = self.li.step_ver_base(&mut self.di);
    let mut p0 = MAX_HALF_WIDTH + 2;
    let mut p1 = p0;

    let mut dist_start = self.di.dist_start;
    self.li.covers[p1] = RealLike::ZERO;
    if dist_start <= 0 {
      self.li.covers[p1] = ren.cover(s1);
    }
    p1 += 1;
    let mut dx = 1;
    let mut dist = self.li.dist[dx] - s1;
    while dist <= self.li.width {
      dist_start += self.di.dy_start;
      self.li.covers[p1] = RealLike::ZERO;
      if dist_start <= 0 {
        self.li.covers[p1] = ren.cover(dist);
      }
      p1 += 1;
      dx += 1;
      dist = self.li.dist[dx] - s1;
    }
    dx = 1;
    dist_start = self.di.dist_start;
    dist = self.li.dist[dx] + s1;
    while dist <= self.li.width {
      dist_start -= self.di.dy_start;
      p0 -= 1;
      self.li.covers[p0] = RealLike::ZERO;
      if dist_start <= 0 {
        self.li.covers[p0] = ren.cover(dist);
      }
      dx += 1;
      dist = self.li.dist[dx] + s1;
    }
    ren.blend_solid_hspan(
      self.li.x - dx as i64 + 1,
      self.li.y,
      (p1 - p0) as i64,
      &self.li.covers[p0..],
    );
    self.li.step += 1;
    self.li.step < self.li.count
  }
}
#[derive(Debug)]
pub(crate) struct AA2 {
  di: DistanceInterpolator2,
  li: LineInterpolatorAA,
}
impl AA2 {
  pub fn new(lp: LineParameters, ex: i64, ey: i64, subpixel_width: i64) -> Self {
    let mut li = LineInterpolatorAA::new(lp, subpixel_width);
    let di = DistanceInterpolator2::new(
      lp.x1,
      lp.y1,
      lp.x2,
      lp.y2,
      ex,
      ey,
      lp.x1 & !POLY_SUBPIXEL_MASK,
      lp.y1 & !POLY_SUBPIXEL_MASK,
      false,
    );
    li.li.adjust_forward();
    li.step -= li.max_extent;
    Self { li, di }
  }
  //pub fn count(&self) -> i64 {        self.li.count    }
  pub fn vertical(&self) -> bool {
    self.li.lp.vertical
  }
  pub fn step_hor<R: RenderOutline<Cover = U8>>(&mut self, ren: &mut R) -> bool {
    let s1 = self.li.step_hor_base(&mut self.di);
    let mut p0 = MAX_HALF_WIDTH + 2;
    let mut p1 = p0;

    let mut dist_end = self.di.dist_start;

    let mut npix = 0;
    self.li.covers[p1] = RealLike::ZERO;
    if dist_end > 0 {
      self.li.covers[p1] = ren.cover(s1);
      npix += 1;
    }
    p1 += 1;

    let mut dy = 1;
    let mut dist = self.li.dist[dy] - s1;
    while dist <= self.li.width {
      dist_end -= self.di.dx_start;
      self.li.covers[p1] = RealLike::ZERO;
      if dist_end > 0 {
        self.li.covers[p1] = ren.cover(dist);
        npix += 1;
      }
      p1 += 1;
      dy += 1;
      dist = self.li.dist[dy] - s1;
    }

    dy = 1;
    dist_end = self.di.dist_start;
    dist = self.li.dist[dy] + s1;
    while dist <= self.li.width {
      dist_end += self.di.dx_start;
      p0 -= 1;
      self.li.covers[p0] = RealLike::ZERO;
      if dist_end > 0 {
        self.li.covers[p0] = ren.cover(dist);
        npix += 1;
      }
      dy += 1;
      dist = self.li.dist[dy] + s1;
    }
    ren.blend_solid_vspan(
      self.li.x,
      self.li.y - dy as i64 + 1,
      (p1 - p0) as i64,
      &self.li.covers[p0..],
    );
    self.li.step += 1;
    npix != 0 && self.li.step < self.li.count
  }
  pub fn step_ver<R: RenderOutline<Cover = U8>>(&mut self, ren: &mut R) -> bool {
    let s1 = self.li.step_ver_base(&mut self.di);
    let mut p0 = MAX_HALF_WIDTH + 2;
    let mut p1 = p0;

    let mut dist_end = self.di.dist_start; // Really dist_end

    let mut npix = 0;
    self.li.covers[p1] = RealLike::ZERO;
    if dist_end > 0 {
      self.li.covers[p1] = ren.cover(s1);
      npix += 1;
    }
    p1 += 1;

    let mut dx = 1;
    let mut dist = self.li.dist[dx] - s1;
    while dist <= self.li.width {
      dist_end += self.di.dy_start;
      self.li.covers[p1] = RealLike::ZERO;
      if dist_end > 0 {
        self.li.covers[p1] = ren.cover(dist);
        npix += 1;
      }
      p1 += 1;
      dx += 1;
      dist = self.li.dist[dx] - s1;
    }

    dx = 1;
    dist_end = self.di.dist_start;
    dist = self.li.dist[dx] + s1;
    while dist <= self.li.width {
      dist_end -= self.di.dy_start;
      p0 -= 1;
      self.li.covers[p0] = RealLike::ZERO;
      if dist_end > 0 {
        self.li.covers[p0] = ren.cover(dist);
        npix += 1;
      }
      dx += 1;
      dist = self.li.dist[dx] + s1;
    }
    ren.blend_solid_hspan(
      self.li.x - dx as i64 + 1,
      self.li.y,
      (p1 - p0) as i64,
      &self.li.covers[p0..],
    );
    self.li.step += 1;
    npix != 0 && self.li.step < self.li.count
  }
}
#[derive(Debug)]
pub(crate) struct AA3 {
  pub di: DistanceInterpolator3,
  pub li: LineInterpolatorAA,
}
impl AA3 {
  pub fn new(lp: LineParameters, sx: i64, sy: i64, ex: i64, ey: i64, subpixel_width: i64) -> Self {
    let mut li = LineInterpolatorAA::new(lp, subpixel_width);
    let mut di = DistanceInterpolator3::new(
      lp.x1,
      lp.y1,
      lp.x2,
      lp.y2,
      sx,
      sy,
      ex,
      ey,
      lp.x1 & !POLY_SUBPIXEL_MASK,
      lp.y1 & !POLY_SUBPIXEL_MASK,
    );
    let mut npix = 1;
    if lp.vertical {
      loop {
        li.li.dec();
        li.y -= lp.inc;
        li.x = (li.lp.x1 + li.li.y.into_raw()) >> POLY_SUBPIXEL_SHIFT;

        if lp.inc > 0 {
          di.dec_y(li.x - li.old_x);
        } else {
          di.inc_y(li.x - li.old_x);
        }

        li.old_x = li.x;

        let mut dist1_start = di.dist_start;
        let mut dist2_start = di.dist_start;

        let mut dx = 0;
        if dist1_start < 0 {
          npix += 1;
        }
        loop {
          dist1_start += di.dy_start;
          dist2_start -= di.dy_start;
          if dist1_start < 0 {
            npix += 1;
          }
          if dist2_start < 0 {
            npix += 1;
          }
          dx += 1;
          if li.dist[dx] > li.width {
            break;
          }
        }
        if npix == 0 {
          break;
        }
        npix = 0;
        li.step -= 1;
        if li.step < -li.max_extent {
          break;
        }
      }
    } else {
      loop {
        li.li.dec();
        li.x -= lp.inc;
        li.y = (li.lp.y1 + li.li.y.into_raw()) >> POLY_SUBPIXEL_SHIFT;

        if lp.inc > 0 {
          di.dec_x(li.y - li.old_y);
        } else {
          di.inc_x(li.y - li.old_y);
        }

        li.old_y = li.y;

        let mut dist1_start = di.dist_start;
        let mut dist2_start = di.dist_start;

        let mut dy = 0;
        if dist1_start < 0 {
          npix += 1;
        }
        loop {
          dist1_start -= di.dx_start;
          dist2_start += di.dx_start;
          if dist1_start < 0 {
            npix += 1;
          }
          if dist2_start < 0 {
            npix += 1;
          }
          dy += 1;
          if li.dist[dy] > li.width {
            break;
          }
        }
        if npix == 0 {
          break;
        }
        npix = 0;
        li.step -= 1;
        if li.step < -li.max_extent {
          break;
        }
      }
    }
    li.li.adjust_forward();
    li.step -= li.max_extent;
    Self { li, di }
  }
  //pub fn count(&self) -> i64 {        self.li.count    }
  pub fn vertical(&self) -> bool {
    self.li.lp.vertical
  }
  pub fn step_hor<R: RenderOutline<Cover = U8>>(&mut self, ren: &mut R) -> bool {
    let s1 = self.li.step_hor_base(&mut self.di);
    let mut p0 = MAX_HALF_WIDTH + 2;
    let mut p1 = p0;

    let mut dist_start = self.di.dist_start;
    let mut dist_end = self.di.dist_end;

    let mut npix = 0;
    self.li.covers[p1] = RealLike::ZERO;
    if dist_end > 0 {
      if dist_start <= 0 {
        self.li.covers[p1] = ren.cover(s1);
      }
      npix += 1;
    }
    p1 += 1;

    let mut dy = 1;
    let mut dist = self.li.dist[dy] - s1;
    while dist <= self.li.width {
      dist_start -= self.di.dx_start;
      dist_end -= self.di.dx_end;
      self.li.covers[p1] = RealLike::ZERO;
      if dist_end > 0 && dist_start <= 0 {
        self.li.covers[p1] = ren.cover(dist);
        npix += 1;
      }
      p1 += 1;
      dy += 1;
      dist = self.li.dist[dy] - s1;
    }

    dy = 1;
    dist_start = self.di.dist_start;
    dist_end = self.di.dist_end;
    dist = self.li.dist[dy] + s1;
    while dist <= self.li.width {
      dist_start += self.di.dx_start;
      dist_end += self.di.dx_end;
      p0 -= 1;
      self.li.covers[p0] = RealLike::ZERO;
      if dist_end > 0 && dist_start <= 0 {
        self.li.covers[p0] = ren.cover(dist);
        npix += 1;
      }
      dy += 1;
    }
    ren.blend_solid_vspan(
      self.li.x,
      self.li.y - dy as i64 + 1,
      (p1 - p0) as i64,
      &self.li.covers[p0..],
    );
    self.li.step -= 1;
    npix != 0 && self.li.step < self.li.count
  }
  pub fn step_ver<R: RenderOutline<Cover = U8>>(&mut self, ren: &mut R) -> bool {
    let s1 = self.li.step_ver_base(&mut self.di);
    let mut p0 = MAX_HALF_WIDTH + 2;
    let mut p1 = p0;

    let mut dist_start = self.di.dist_start;
    let mut dist_end = self.di.dist_end;

    let mut npix = 0;
    self.li.covers[p1] = RealLike::ZERO;
    if dist_end > 0 {
      if dist_start <= 0 {
        self.li.covers[p1] = ren.cover(s1);
      }
      npix += 1;
    }
    p1 += 1;

    let mut dx = 1;
    let mut dist = self.li.dist[dx] - s1;
    while dist <= self.li.width {
      dist_start += self.di.dy_start;
      dist_end += self.di.dy_end;
      self.li.covers[p1] = RealLike::ZERO;
      if dist_end > 0 && dist_start <= 0 {
        self.li.covers[p1] = ren.cover(dist);
        npix += 1;
      }
      p1 += 1;
      dx += 1;
      dist = self.li.dist[dx] - s1;
    }

    dx = 1;
    dist_start = self.di.dist_start;
    dist_end = self.di.dist_end;
    dist = self.li.dist[dx] + s1;
    while dist <= self.li.width {
      dist_start -= self.di.dy_start;
      dist_end -= self.di.dy_end;
      p0 -= 1;
      self.li.covers[p0] = RealLike::ZERO;
      if dist_end > 0 && dist_start <= 0 {
        self.li.covers[p0] = ren.cover(dist);
        npix += 1;
      }
      dx += 1;
      dist = self.li.dist[dx] + s1;
    }
    ren.blend_solid_hspan(
      self.li.x - dx as i64 + 1,
      self.li.y,
      (p1 - p0) as i64,
      &self.li.covers[p0..p1],
    );
    self.li.step -= 1;
    npix != 0 && self.li.step < self.li.count
  }
}
