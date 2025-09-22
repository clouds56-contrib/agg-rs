use std::marker::PhantomData;

use crate::{
  math::{lerp_u8, multiply_u8}, Color, FromRaw2, FromRaw3, Gray8, Pixel, Pixfmt, RealLike, RenderingBase, Rgb8
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
  fn bpp() -> usize {
    Pixfmt::<Rgb8>::bpp()
  }
  fn blend_pix<C: Color, T: RealLike>(&mut self, id: (usize, usize), c: C, cover: T) {
    // TODO: use BlendPix trait
    let alpha = multiply_u8(c.alpha8(), (cover.to_f64() * 255.0) as u8);

    let c = c.rgb();
    let c0 = self.component(c);
    let p0 = self.mix_pix(id, c0, alpha);
    self.set(id, p0);
  }
}
