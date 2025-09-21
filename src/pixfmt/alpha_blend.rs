use std::marker::PhantomData;

use crate::{
  Color, FromRaw2, FromRaw3, Gray8, Pixel, Pixfmt, RenderingBase, Rgb8,
  math::{lerp_u8, multiply_u8},
};

pub struct PixfmtAlphaBlend<'a, T, C>
where
  T: Pixel,
{
  pub(crate) ren: &'a mut RenderingBase<T>,
  pub(crate) offset: usize,
  //step: usize,
  phantom: PhantomData<C>,
}

impl<'a, T, C> PixfmtAlphaBlend<'a, T, C>
where
  T: Pixel,
{
  pub fn new(ren: &'a mut RenderingBase<T>, offset: usize) -> Self {
    //let step = T::bpp();
    Self {
      ren,
      offset,
      phantom: PhantomData,
    }
  }
}
impl PixfmtAlphaBlend<'_, Pixfmt<Rgb8>, Gray8> {
  pub fn component(&self, c: Rgb8) -> Gray8 {
    match self.offset {
      0 => Gray8::from_raw(c.red8(), 255),
      1 => Gray8::from_raw(c.green8(), 255),
      2 => Gray8::from_raw(c.blue8(), 255),
      _ => unreachable!("incorrect offset for Rgb8"),
    }
  }
  pub fn mix_pix(&mut self, (x, y): (usize, usize), c: Gray8, alpha: u8) -> Gray8 {
    let p = self.component(Rgb8::from_slice(self.ren.pixf.rbuf.get_pixel(x, y)));
    Gray8::from_raw(lerp_u8(p.luma.0, c.luma.0, alpha), alpha)
  }
}

impl Pixel for PixfmtAlphaBlend<'_, Pixfmt<Rgb8>, Gray8> {
  type Color = Gray8;
  fn width(&self) -> usize {
    self.ren.pixf.width()
  }
  fn height(&self) -> usize {
    self.ren.pixf.height()
  }
  fn as_bytes(&self) -> &[u8] {
    self.ren.pixf.as_bytes()
  }
  fn to_file<P: AsRef<std::path::Path>>(&self, filename: P) -> Result<(), std::io::Error> {
    crate::ppm::write_file(self.as_bytes(), self.width(), self.height(), filename)
  }
  fn _set(&mut self, id: (usize, usize), n: usize, c: Self::Color) {
    let c = c.rgb8();
    let value = self.component(c).luma.0;
    let bpp = self.ren.pixf.rbuf.bpp;

    let rbuf = &mut self.ren.pixf.rbuf;
    rbuf
      .slice_mut(id, n)
      .iter_mut()
      .skip(self.offset)
      .step_by(bpp)
      .for_each(|p| *p = value);
  }
  fn cover_mask() -> u64 {
    Pixfmt::<Rgb8>::cover_mask()
  }
  fn bpp() -> usize {
    Pixfmt::<Rgb8>::bpp()
  }
  fn blend_pix<C: Color>(&mut self, id: (usize, usize), c: C, cover: u64) {
    let alpha = multiply_u8(c.alpha8(), cover as u8);

    let c = c.rgb();
    let c0 = self.component(c);
    let p0 = self.mix_pix(id, c0, alpha);
    self.set(id, p0);
  }

  fn blend_color_vspan<C: Color>(&mut self, x: i64, y: i64, len: i64, colors: &[C], covers: &[u64], cover: u64) {
    assert_eq!(len as usize, colors.len());
    let (x, y) = (x as usize, y as usize);
    if !covers.is_empty() {
      assert_eq!(colors.len(), covers.len());
      for (i, (&color, &cover)) in colors.iter().zip(covers.iter()).enumerate() {
        self.copy_or_blend_pix_with_cover((x, y + i), color, cover);
      }
    } else if cover == Self::cover_mask() {
      for (i, &color) in colors.iter().enumerate() {
        self.copy_or_blend_pix((x, y + i), color);
      }
    } else {
      for (i, &color) in colors.iter().enumerate() {
        self.copy_or_blend_pix_with_cover((x, y + i), color, cover);
      }
    }
  }
}
