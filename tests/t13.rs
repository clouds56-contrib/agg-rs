extern crate agg;

use agg::prelude::*;

#[test]
fn t13_aliased() {
  flexi_logger::Logger::try_with_env_or_str("debug").unwrap().start().ok();

  let (w, h) = (100, 100);

  let pixf = agg::Pixfmt::<agg::Rgb8>::new(w, h);

  let mut ren_base = agg::RenderingBase::new(pixf);

  ren_base.clear(agg::Rgb8::WHITE);

  let mut ren = agg::RenderingScanlineBinSolid::new(&mut ren_base, Rgba8::RED);

  let mut ras = agg::RasterizerScanline::new();

  ras.clip_box(40.0, 0.0, w as f64 - 40.0, h as f64);

  ras.move_to(10.0, 10.0);
  ras.line_to(50.0, 90.0);
  ras.line_to(90.0, 10.0);

  agg::render_scanlines(&mut ras, &mut ren);

  ren.to_file("tests/tmp/agg_test_13.png").unwrap();

  assert!(agg::ppm::img_diff("tests/tmp/agg_test_13.png", "images/agg_test_13.png").unwrap());
}
