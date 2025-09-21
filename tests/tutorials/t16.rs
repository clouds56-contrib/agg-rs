extern crate agg;
use agg::prelude::*;

#[test]
fn t16_path_stroke_no_clip() {
  let (w, h) = (100, 100);

  let pixf = agg::Pixfmt::<agg::Rgb8>::create(w, h);

  let mut ren_base = agg::RenderingBase::new(pixf);

  ren_base.clear(agg::Rgb8::WHITE);

  let mut ren = agg::RenderingScanlineAASolid::new(&mut ren_base, Rgb8::RED);

  let mut ras = agg::RasterizerScanline::new();

  //ras.clip_box(40.0, 0.0, w as f64-40.0, h as f64);

  ras.reset();
  ras.move_to(10.0, 10.0);
  ras.line_to(50.0, 90.0);
  ras.line_to(90.0, 10.0);

  agg::render_scanlines(&mut ras, &mut ren);

  let mut ps = agg::Path::new();
  ps.remove_all();
  ps.move_to(10.0, 10.0);
  ps.line_to(50.0, 90.0);
  ps.line_to(90.0, 10.0);
  ps.line_to(10.0, 10.0);

  let mut pg = agg::Stroke::new(ps);

  pg.width(2.0);
  ras.add_path(&pg);

  agg::render_scanlines_aa_solid(&mut ras, &mut ren_base, agg::Rgb8::BLACK);

  ren_base.to_file("tests/tmp/agg_test_16.png").unwrap();

  assert!(agg::ppm::img_diff("tests/tmp/agg_test_16.png", "images/agg_test_16.png").unwrap());
}
