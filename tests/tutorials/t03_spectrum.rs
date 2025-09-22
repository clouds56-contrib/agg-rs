extern crate agg;

use agg::prelude::*;

fn draw_black_frame(pix: &mut agg::Pixfmt<agg::Rgb8>) {
  let w = pix.width();
  let h = pix.height();
  let black = agg::Rgb8::BLACK;
  pix.copy_hline(0, 0, w, black);
  pix.copy_hline(0, h - 1, w, black);

  pix.copy_vline(0, 0, h, black);
  pix.copy_vline(w - 1, 0, h, black);
}

fn wavelength_span(w: usize) -> Vec<agg::Rgb8> {
  (0..w)
    .map(|i| agg::rgb8_from_wavelength_gamma(380.0 + 400.0 * i as f64 / w as f64, 0.8))
    .collect()
}

#[test]
fn t03_spectrum() {
  let (w, h) = (320, 200);
  let mut pix = agg::Pixfmt::<Rgb8>::create(w, h);
  pix.fill(Rgb8::WHITE);
  draw_black_frame(&mut pix);

  let span = wavelength_span(w);

  for i in 0..h {
    pix.blend_color_hspan(0, i as i64, w as i64, &span, 1.0);
  }
  pix.to_file("tests/tmp/t03_spectrum.png").unwrap();
  assert!(agg::utils::img_diff("tests/tmp/t03_spectrum.png", "images/t03_spectrum.png").unwrap());
}

/// https://agg.sourceforge.net/antigrain.com/doc/basic_renderers/basic_renderers.agdoc.html#toc0008
fn draw_alpha(color: Rgb8, filename: &str) {
  let (w, h) = (320, 200);

  let mut pix = agg::Pixfmt::<Rgb8>::create(w, h);
  pix.fill(color);

  let mut alpha = agg::Pixfmt::<Gray8>::create(w, h);
  for i in 0..h {
    let v = (255 * i / h) as u8;
    alpha.copy_hline(0, i, w, agg::Gray8::from_raw(v, 255));
  }

  let mut mix = agg::PixfmtAlphaMask::new(pix, alpha);

  let span = wavelength_span(w);
  for i in 0..h as i64 {
    mix.blend_color_hspan(0, i, w as i64, &span, 1.);
  }
  mix.rgb.to_file(format!("tests/tmp/{}.png", filename)).unwrap();
  assert!(
    agg::utils::img_diff(
      format!("tests/tmp/{}.png", filename),
      format!("images/{}.png", filename)
    )
    .unwrap()
  );
}

#[test]
fn t03_spectrum_alpha() {
  draw_alpha(Rgb8::WHITE, "t03_spectrum_alpha_white");
  draw_alpha(Rgb8::BLACK, "t03_spectrum_alpha_black");
}
