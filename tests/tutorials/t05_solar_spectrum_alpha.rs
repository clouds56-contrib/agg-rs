extern crate agg;

use agg::prelude::*;

#[test]
fn t05_solar_spectrum_alpha() {
  let mut pix = agg::Pixfmt::<agg::Rgb8>::create(320, 200);
  pix.clear();
  pix.fill(agg::Rgb8::BLACK);
  let mut alpha = agg::Pixfmt::<agg::Gray8>::create(320, 200);

  let w = pix.width();
  let h = pix.height();

  for i in 0..h {
    let v = (255 * i / h) as u8;
    alpha.copy_hline(0, i, w, agg::Gray8::from_raw(v, 255));
  }

  let mut span = vec![agg::Rgb8::WHITE; w];
  for (i, pixel) in span.iter_mut().enumerate() {
    *pixel = agg::rgb8_from_wavelength_gamma(380.0 + 400.0 * i as f64 / w as f64, 0.8);
  }

  let mut mix = agg::AlphaMaskAdaptor::new(pix, alpha);

  for i in 0..h {
    mix.blend_color_hspan(0, i, w, &span, 0);
  }
  mix.rgb.to_file("tests/tmp/agg_test_05.png").unwrap();

  assert!(agg::ppm::img_diff("tests/tmp/agg_test_05.png", "images/agg_test_05.png").unwrap());
}
