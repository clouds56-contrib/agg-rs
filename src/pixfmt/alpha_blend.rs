use std::marker::PhantomData;

use crate::{math::lerp_u8, Color, FromRaw2, FromRaw3, Gray8, Pixel, Pixfmt, RenderingBase, Rgb8};


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
  pub fn mix_pix(&mut self, id: (usize, usize), c: Gray8, alpha: u8) -> Gray8 {
    let p = self.component(Rgb8::from_slice(&self.ren.pixf.rbuf[id]));
    Gray8::from_raw(lerp_u8(p.luma.0, c.luma.0, alpha), alpha)
  }
}
