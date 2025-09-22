extern crate agg;
use agg::prelude::*;

mod utils;
use utils::assets::{parse_lion, transform_paths};

mod tests {
  use super::*;
  #[test]
  fn lion_png() {
    let (w, h) = (400, 400);

    let (paths, colors) = parse_lion(false);
    let pixf = agg::Pixfmt::<agg::Rgb8>::create(w, h);
    let mut ren_base = agg::RenderingBase::new(pixf);
    ren_base.clear(agg::Rgb8::WHITE);
    let mut ren = agg::RenderingScanlineBinSolid::new(&mut ren_base, Rgb8::RED);

    let mut ras = agg::RasterizerScanline::new();

    let t = transform_paths(paths, w as f64, h as f64);

    agg::render_all_paths(&mut ras, &mut ren, &t, &colors);

    ren.to_file("tests/tmp/lion.png").unwrap();

    if !agg::ppm::img_diff("tests/tmp/lion.png", "images/lion.png").unwrap() {
      panic!("PNG Images differ");
    }
  }

  #[test]
  fn lion_cw() {
    let (w, h) = (400, 400);

    let (paths, colors) = parse_lion(true);
    let pixf = agg::Pixfmt::<agg::Rgb8>::create(w, h);
    let mut ren_base = agg::RenderingBase::new(pixf);
    ren_base.clear(agg::Rgb8::WHITE);
    let mut ren = agg::RenderingScanlineBinSolid::new(&mut ren_base, Rgb8::RED);

    let mut ras = agg::RasterizerScanline::new();

    let t = transform_paths(paths, w as f64, h as f64);

    agg::render_all_paths(&mut ras, &mut ren, &t, &colors);

    ren.to_file("tests/tmp/lion_cw.png").unwrap();

    assert!(agg::ppm::img_diff("tests/tmp/lion_cw.png", "images/lion_cw.png").unwrap());
  }
  // compare -verbose -metric AE lion.ppm ./tests/lion.ppm blarg.ppm

  #[test]
  fn lion_cw_aa() {
    let (w, h) = (400, 400);

    let (paths, colors) = parse_lion(true);
    let pixf = agg::Pixfmt::<agg::Rgb8>::create(w, h);
    let mut ren_base = agg::RenderingBase::new(pixf);
    ren_base.clear(agg::Rgb8::WHITE);
    let mut ren = agg::RenderingScanlineAASolid::new(&mut ren_base, Rgb8::RED);

    let mut ras = agg::RasterizerScanline::new();

    let t = transform_paths(paths, w as f64, h as f64);

    agg::render_all_paths(&mut ras, &mut ren, &t, &colors);

    ren.to_file("tests/tmp/lion_cw_aa.png").unwrap();

    assert!(agg::ppm::img_diff("tests/tmp/lion_cw_aa.png", "images/lion_cw_aa.png").unwrap());
  }
  // compare -verbose -metric AE lion.ppm ./tests/lion.ppm blarg.ppm

  #[test]
  fn lion_cw_aa_srgba() {
    let (w, h) = (400, 400);

    let (paths, colors) = parse_lion(true);
    let pixf = agg::Pixfmt::<agg::Rgb8>::create(w, h);
    let mut ren_base = agg::RenderingBase::new(pixf);
    //ren_base.clear( agg::Srgba8::new([255, 255, 255, 255]) );
    ren_base.clear(agg::Rgb8::WHITE);
    let mut ren = agg::RenderingScanlineAASolid::new(&mut ren_base, Rgb8::RED);

    let mut ras = agg::RasterizerScanline::new();

    let colors = colors.into_iter().map(|c| c.srgba8()).collect::<Vec<_>>();
    let t = transform_paths(paths, w as f64, h as f64);

    agg::render_all_paths(&mut ras, &mut ren, &t, &colors);

    ren.to_file("tests/tmp/lion_cw_aa_srgba.png").unwrap();

    assert!(agg::ppm::img_diff("tests/tmp/lion_cw_aa_srgba.png", "images/lion_cw_aa_srgba.png").unwrap());
  }
  // compare -verbose -metric AE lion.ppm ./tests/lion.ppm blarg.ppm

  #[test]
  fn lion_outline_width1() {
    let (w, h) = (400, 400);

    let (paths, colors) = parse_lion(true);
    let pixf = agg::Pixfmt::<agg::Rgb8>::create(w, h);
    let mut ren_base = agg::RenderingBase::new(pixf);
    //ren_base.clear( agg::Srgba8::new([255, 255, 255, 255]) );
    ren_base.clear(agg::Rgb8::WHITE);
    let mut ren = agg::RenderingScanlineAASolid::new(&mut ren_base, Rgb8::RED);

    let mut ras = agg::RasterizerScanline::new();

    let colors = colors.into_iter().map(|c| c.srgba8()).collect::<Vec<_>>();
    let t = transform_paths(paths, w as f64, h as f64);

    let mut stroke: Vec<_> = t.into_iter().map(agg::Stroke::new).collect();
    stroke.iter_mut().for_each(|p| p.width(1.0));
    agg::render_all_paths(&mut ras, &mut ren, &stroke, &colors);

    ren.to_file("tests/tmp/lion_outline_width1.png").unwrap();
    assert!(agg::ppm::img_diff("tests/tmp/lion_outline_width1.png", "images/lion_outline_width1.png").unwrap());
  }

  #[test]
  fn lion_outline() {
    let (w, h) = (400, 400);

    let (paths, colors) = parse_lion(true);
    let pixf = agg::Pixfmt::<agg::Rgb8>::create(w, h);
    let mut ren_base = agg::RenderingBase::new(pixf);
    //ren_base.clear( agg::Srgba8::new([255, 255, 255, 255]) );
    ren_base.clear(Rgb8::WHITE);
    let mut ren = agg::RenderingScanlineAASolid::new(&mut ren_base, Rgb8::RED);

    let mut ras = agg::RasterizerScanline::new();

    let colors = colors.into_iter().map(|c| c.srgba8()).collect::<Vec<_>>();
    let t = transform_paths(paths, w as f64, h as f64);

    let mut stroke: Vec<_> = t.into_iter().map(agg::Stroke::new).collect();
    stroke.iter_mut().for_each(|p| p.width(7.0));
    agg::render_all_paths(&mut ras, &mut ren, &stroke, &colors);

    ren.to_file("tests/tmp/lion_outline.png").unwrap();
    assert!(agg::ppm::img_diff("tests/tmp/lion_outline.png", "images/lion_outline.png").unwrap());
  }
  // compare -verbose -metric AE lion.ppm ./tests/lion.ppm diff.ppm
}
