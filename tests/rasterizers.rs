extern crate agg;

use agg::prelude::*;

fn rgb64(r: f64, g: f64, b: f64, a: f64) -> agg::Rgba8 {
  agg::Rgba64::from_raw(r, g, b, a).rgba8()
}

#[test]
fn rasterizers() {
  let (w, h) = (500, 330);

  let m_x = [100. + 120., 369. + 120., 143. + 120.];
  let m_y = [60., 170., 310.0];

  let pixf = agg::Pixfmt::<agg::Rgb8>::create(w, h);
  let mut ren_base = agg::RenderingBase::new(pixf);
  ren_base.clear(agg::Rgb8::WHITE);

  //let gamma = 1.0;
  let alpha = 0.5;

  let mut ras = agg::RasterizerScanline::new();

  // Anti-Aliased
  {
    let mut ren_aa = agg::RenderingScanlineAASolid::new_black(&mut ren_base);
    let mut path = agg::Path::new();

    path.move_to(m_x[0], m_y[0]);
    path.line_to(m_x[1], m_y[1]);
    path.line_to(m_x[2], m_y[2]);
    path.close_polygon();
    ren_aa.color(rgb64(0.7, 0.5, 0.1, alpha));
    ras.add_path(&path);
    agg::render_scanlines(&mut ras, &mut ren_aa);
  }

  // Aliased
  {
    let mut ren_bin = agg::RenderingScanlineBinSolid::new_black(&mut ren_base);
    let mut path = agg::Path::new();

    path.move_to(m_x[0] - 200., m_y[0]);
    path.line_to(m_x[1] - 200., m_y[1]);
    path.line_to(m_x[2] - 200., m_y[2]);
    path.close_polygon();
    ren_bin.color(rgb64(0.1, 0.5, 0.7, alpha));
    ras.add_path(&path);
    //ras.
    agg::render_scanlines(&mut ras, &mut ren_bin);
  }
  ren_base.to_file("tests/tmp/rasterizers.png").unwrap();
  assert!(agg::utils::img_diff("tests/tmp/rasterizers.png", "images/rasterizers.png").unwrap());
}

#[test]
fn rasterizers_gamma() {
  let (w, h) = (500, 330);

  let m_x = [100. + 120., 369. + 120., 143. + 120.];
  let m_y = [60., 170., 310.0];

  let pixf = agg::Pixfmt::<agg::Rgb8>::create(w, h);
  let mut ren_base = agg::RenderingBase::new(pixf);
  ren_base.clear(agg::Rgb8::WHITE);

  let gamma = 1.0;
  let alpha = 0.5;

  let mut ras = agg::RasterizerScanline::new();

  // Anti-Aliased
  {
    let mut ren_aa = agg::RenderingScanlineAASolid::new_black(&mut ren_base);
    let mut path = agg::Path::new();

    path.move_to(m_x[0], m_y[0]);
    path.line_to(m_x[1], m_y[1]);
    path.line_to(m_x[2], m_y[2]);
    path.close_polygon();
    ren_aa.color(rgb64(0.7, 0.5, 0.1, alpha));
    ras.add_path(&path);
    // Power Function
    ras.gamma(|v| v.powf(gamma * 2.0));
    agg::render_scanlines(&mut ras, &mut ren_aa);
  }

  // Aliased
  {
    let mut ren_bin = agg::RenderingScanlineBinSolid::new_black(&mut ren_base);
    let mut path = agg::Path::new();

    path.move_to(m_x[0] - 200., m_y[0]);
    path.line_to(m_x[1] - 200., m_y[1]);
    path.line_to(m_x[2] - 200., m_y[2]);
    path.close_polygon();
    ren_bin.color(rgb64(0.1, 0.5, 0.7, alpha));
    ras.add_path(&path);
    // Threshold
    ras.gamma(|v| if v < gamma { 0.0 } else { 1.0 });
    agg::render_scanlines(&mut ras, &mut ren_bin);
  }
  ren_base.to_file("tests/tmp/rasterizers_gamma.png").unwrap();
  assert!(agg::utils::img_diff("tests/tmp/rasterizers_gamma.png", "images/rasterizers_gamma.png").unwrap());
}
