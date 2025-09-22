use crate::Color;
use crate::FromColor;
use crate::NamedColor;
use crate::POLY_SUBPIXEL_SCALE;
use crate::Pixel;
use crate::RenderingBase;
use crate::Subpixel;
use crate::color::Rgba8;
use crate::renders::BresehamInterpolator;

/// Primative Renderer
#[derive(Debug)]
pub struct RendererPrimatives<'a, T, C = Rgba8>
where
  T: 'a,
{
  base: &'a mut RenderingBase<T>,
  fill_color: C,
  line_color: C,
  x: Subpixel,
  y: Subpixel,
}

impl<'a, T> RendererPrimatives<'a, T, Rgba8>
where
  T: Pixel,
{
  /// Create a new RendererPrimatives with black line and white fill
  pub fn new_black(base: &'a mut RenderingBase<T>) -> Self {
    Self::new(base, Rgba8::BLACK)
  }
}

impl<'a, T, C> RendererPrimatives<'a, T, C>
where
  T: Pixel,
  C: Color + FromColor,
{
  pub fn new(base: &'a mut RenderingBase<T>, color: C) -> Self {
    Self {
      base,
      fill_color: color,
      line_color: color,
      x: Subpixel::from(0),
      y: Subpixel::from(0),
    }
  }

  /// Set line color
  #[must_use]
  pub fn with_line_color<C2: Color>(mut self, line_color: C2) -> Self {
    self.line_color = C::from_color(line_color);
    self
  }
  /// Set fill color
  #[must_use]
  pub fn with_fill_color<C2: Color>(mut self, fill_color: C2) -> Self {
    self.fill_color = C::from_color(fill_color);
    self
  }
  pub(crate) fn coord(&self, c: f64) -> Subpixel {
    Subpixel::from((c * POLY_SUBPIXEL_SCALE as f64).round() as i64)
  }
  pub(crate) fn move_to(&mut self, x: Subpixel, y: Subpixel) {
    self.x = x;
    self.y = y;
  }
  pub(crate) fn line_to(&mut self, x: Subpixel, y: Subpixel) {
    let (x0, y0) = (self.x, self.y);
    self.line(x0, y0, x, y);
    self.x = x;
    self.y = y;
  }
  fn line(&mut self, x1: Subpixel, y1: Subpixel, x2: Subpixel, y2: Subpixel) {
    //let cover_shift = POLY_SUBPIXEL_SCALE;
    //let cover_size = 1 << cover_shift;
    //let cover_mask = cover_size - 1;
    //let cover_full = cover_mask;
    let cover = T::cover_full();
    let color = self.line_color;
    let mut li = BresehamInterpolator::new(x1, y1, x2, y2);
    if li.len == 0 {
      return;
    }
    if li.ver {
      for _ in 0..li.len {
        //self.base.pixf.set((li.x2 as usize, li.y1 as usize), color);
        self.base.blend_hline(li.x2, li.y1, li.x2, color, cover);
        li.vstep();
      }
    } else {
      for _ in 0..li.len {
        self.base.blend_hline(li.x1, li.y2, li.x1, color, cover);
        li.hstep();
      }
    }
  }
}
