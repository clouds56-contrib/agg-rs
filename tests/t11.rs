extern crate agg;
use agg::{color::NamedColor, prelude::*};

#[test]
fn t11_full() {
  let (w, h) = (100, 100);

  let pixf = agg::Pixfmt::<agg::Rgb8>::new(w, h);

  let mut ren_base = agg::RenderingBase::new(pixf);

  ren_base.clear(agg::Rgb8::WHITE);

  let mut ren = agg::RenderingScanlineAASolid::new(&mut ren_base, Rgb8::RED);

  let mut ras = agg::RasterizerScanline::new();

  ras.move_to(10.0, 10.0);
  ras.line_to(50.0, 90.0);
  ras.line_to(90.0, 10.0);

  agg::render_scanlines(&mut ras, &mut ren);

  ren.to_file("tests/tmp/agg_test_11.png").unwrap();

  assert!(agg::ppm::img_diff("tests/tmp/agg_test_11.png", "images/agg_test_11.png").unwrap());
}
