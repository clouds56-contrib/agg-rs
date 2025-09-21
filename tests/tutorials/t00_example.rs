#[test]
fn t00_example() {
  use agg::NamedColor;

  // Create a blank image 10x10 pixels
  let pix = agg::Pixfmt::<agg::Rgb8>::create(100, 100);
  let mut ren_base = agg::RenderingBase::new(pix);
  ren_base.clear(agg::Rgb8::WHITE);

  // Draw a polygon from (10,10) - (50,90) - (90,10)
  let mut ras = agg::RasterizerScanline::new();
  ras.move_to(10.0, 10.0);
  ras.line_to(50.0, 90.0);
  ras.line_to(90.0, 10.0);

  // Render the line to the image
  let mut ren = agg::RenderingScanlineAASolid::new_black(&mut ren_base);
  agg::render_scanlines(&mut ras, &mut ren);

  // Save the image to a file
  ren_base.to_file("tests/tmp/t00_example.png").unwrap();
  assert!(
    agg::ppm::img_diff(
      "tests/tmp/t00_example.png",
      "images/t00_example.png"
    )
    .unwrap()
  );
}
