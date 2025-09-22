use agg::prelude::*;

#[test]
fn repro_transparent_white_over_black() {
  // Create tiny image 3x1, black background
  let pix = agg::Pixfmt::<Rgb8>::create(3, 1);
  let mut ren_base = agg::RenderingBase::new(pix);
  ren_base.clear(Rgb8::BLACK);

  // Semi-transparent white alpha = round(0.2*255) = 51
  let alpha = (0.2f64 * 255.0f64).round() as u8;
  let white = agg::Rgba8::from_raw(255, 255, 255, alpha);

  // Blend full coverage (cover = 255) across first two pixels
  ren_base.blend_hline(0, 0, 1, white, 1.0);

  // Read back pixels and assert expected result
  let p0 = ren_base.pixf.get((0, 0));
  let p1 = ren_base.pixf.get((1, 0));
  let p2 = ren_base.pixf.get((2, 0));

  // Expected: lerp(0,255,alpha) -> ~alpha for rgb, alpha channel becomes 255
  let expected = agg::Rgb8::from_raw(alpha, alpha, alpha);
  assert_eq!(p0, expected);
  assert_eq!(p1, expected);
  // untouched pixel remains white of clear (clear sets WHITE) but we cleared to BLACK, so p2 should be BLACK
  assert_eq!(p2, agg::Rgb8::BLACK);
}
