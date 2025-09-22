const FLIP: bool = true;

use agg::prelude::*;

#[test]
fn example_alpha_mask() {
  let (width, height) = (480, 350);
  let pix = agg::Pixfmt::<agg::Rgba8>::create(width, height);
  let mut ren_base = agg::RenderingBase::new(pix);
  ren_base.clear(agg::Rgba8::WHITE);

  let mut ren = agg::RenderingScanlineAASolid::new_black(&mut ren_base);

  let mtx = agg::Transform::new();
}
