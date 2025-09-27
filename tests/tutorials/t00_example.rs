use agg::prelude::*;

const WIDTH: usize = 100;
const HEIGHT: usize = 100;

fn draw_triangle(ras: &mut agg::RasterizerScanline, clip_box: bool) {
  if clip_box {
    ras.clip_box(40.0, 0.0, WIDTH as f64 - 40.0, HEIGHT as f64);
  }
  ras.move_to(10.0, 10.0);
  ras.line_to(50.0, 90.0);
  ras.line_to(90.0, 10.0);
}

fn draw_stroke(ras: &mut agg::RasterizerScanline) {
  let mut ps = agg::Path::new();
  ps.remove_all();
  ps.move_to(10.0, 10.0);
  ps.line_to(50.0, 90.0);
  ps.line_to(90.0, 10.0);
  ps.line_to(10.0, 10.0);

  let mut pg = agg::Stroke::new(ps);

  pg.width(2.0);
  ras.add_path(&pg);
}

#[test]
fn t00_example() {
  // Create a blank image 10x10 pixels
  let pix = agg::Pixfmt::<agg::Rgb8>::create(WIDTH as _, HEIGHT as _);
  let mut ren_base = agg::RenderingBase::new(pix);
  ren_base.clear(agg::Rgb8::WHITE);

  // Draw a polygon from (10,10) - (50,90) - (90,10)
  let mut ras = agg::RasterizerScanline::new();
  draw_triangle(&mut ras, false);

  // Render the line to the image
  let mut ren = agg::RenderingScanlineAASolid::new_black(&mut ren_base);
  agg::render_scanlines(&mut ras, &mut ren);

  // Save the image to a file
  ren_base.to_file("tests/tmp/t00_example.png").unwrap();
  assert!(agg::utils::img_diff("tests/tmp/t00_example.png", "images/t00_example.png").unwrap());
}

#[test]
fn t00_example_red() {
  let pixf = agg::Pixfmt::<agg::Rgb8>::create(WIDTH as _, HEIGHT as _);
  let mut ren_base = agg::RenderingBase::new(pixf);
  ren_base.clear(agg::Rgb8::WHITE);

  let mut ras = agg::RasterizerScanline::new();
  draw_triangle(&mut ras, false);

  let mut ren = agg::RenderingScanlineAASolid::new(&mut ren_base, Rgb8::RED); // <-- red color
  agg::render_scanlines(&mut ras, &mut ren);

  ren.to_file("tests/tmp/t00_example_red.png").unwrap();

  assert!(agg::utils::img_diff("tests/tmp/t00_example_red.png", "images/t00_example_red.png").unwrap());
}

#[test]
fn t00_example_red_clip_box() {
  let pixf = agg::Pixfmt::<agg::Rgb8>::create(WIDTH as _, HEIGHT as _);
  let mut ren_base = agg::RenderingBase::new(pixf);
  ren_base.clear(agg::Rgb8::WHITE);

  let mut ras = agg::RasterizerScanline::new();
  draw_triangle(&mut ras, true); // <-- clip box

  let mut ren = agg::RenderingScanlineAASolid::new(&mut ren_base, Rgb8::RED);
  agg::render_scanlines(&mut ras, &mut ren);

  ren.to_file("tests/tmp/t00_example_red_clip_box.png").unwrap();

  assert!(
    agg::utils::img_diff(
      "tests/tmp/t00_example_red_clip_box.png",
      "images/t00_example_red_clip_box.png"
    )
    .unwrap()
  );
}

#[test]
fn t00_example_aliased() {
  flexi_logger::Logger::try_with_env_or_str("debug").unwrap().start().ok();

  let pixf = agg::Pixfmt::<agg::Rgb8>::create(WIDTH as _, HEIGHT as _);
  let mut ren_base = agg::RenderingBase::new(pixf);
  ren_base.clear(agg::Rgb8::WHITE);

  let mut ras = agg::RasterizerScanline::new();
  draw_triangle(&mut ras, true);

  let mut ren = agg::RenderingScanlineBinSolid::new(&mut ren_base, Rgba8::RED); // <-- aliased, or call BinSolid
  agg::render_scanlines(&mut ras, &mut ren);

  ren.to_file("tests/tmp/t00_example_aliased.png").unwrap();

  assert!(agg::utils::img_diff("tests/tmp/t00_example_aliased.png", "images/t00_example_aliased.png").unwrap());
}

#[test]
fn t00_example_path_stroke_clip() {
  let pixf = agg::Pixfmt::<agg::Rgb8>::create(WIDTH as _, HEIGHT as _);
  let mut ren_base = agg::RenderingBase::new(pixf);
  ren_base.clear(agg::Rgb8::WHITE);
  let mut ren = agg::RenderingScanlineAASolid::new(&mut ren_base, Rgb8::RED);
  let mut ras = agg::RasterizerScanline::new();

  draw_triangle(&mut ras, true); // <-- clip box
  agg::render_scanlines(&mut ras, &mut ren);

  draw_stroke(&mut ras); // <-- stroke the triangle
  agg::render_scanlines_aa_solid(&mut ras, &mut ren_base, agg::Rgb8::BLACK);

  ren_base.to_file("tests/tmp/t00_example_path_stroke_clip.png").unwrap();
  assert!(
    agg::utils::img_diff(
      "tests/tmp/t00_example_path_stroke_clip.png",
      "images/t00_example_path_stroke_clip.png"
    )
    .unwrap()
  );
}

#[test]
fn t00_example_path_stroke() {
  let pixf = agg::Pixfmt::<agg::Rgb8>::create(WIDTH as _, HEIGHT as _);
  let mut ren_base = agg::RenderingBase::new(pixf);
  ren_base.clear(agg::Rgb8::WHITE);
  let mut ren = agg::RenderingScanlineAASolid::new(&mut ren_base, Rgb8::RED);
  let mut ras = agg::RasterizerScanline::new();

  draw_triangle(&mut ras, false); // <-- no clip box
  agg::render_scanlines(&mut ras, &mut ren);

  draw_stroke(&mut ras); // <-- stroke the triangle
  agg::render_scanlines_aa_solid(&mut ras, &mut ren_base, agg::Rgb8::BLACK);

  ren_base.to_file("tests/tmp/t00_example_path_stroke.png").unwrap();
  assert!(
    agg::utils::img_diff(
      "tests/tmp/t00_example_path_stroke.png",
      "images/t00_example_path_stroke.png"
    )
    .unwrap()
  );
}
