extern crate agg;

use agg::prelude::*;

fn draw_black_frame(pix: &mut agg::Pixfmt<agg::Rgb8>) {
  let w = pix.width();
  let h = pix.height();
  // println!("w,h: {} {}", w, h);
  let black = agg::Rgb8::BLACK;
  for i in 0..h {
    pix.copy_pixel(0, i, black);
    pix.copy_pixel(w - 1, i, black);
  }
  for i in 0..w {
    pix.copy_pixel(i, 0, black);
    pix.copy_pixel(i, h - 1, black);
  }
}

#[test]
fn t02_pixel_formats() {
  //let rbuf = agg::RenderingBuffer::new(320, 220, 3);
  let mut pix = agg::Pixfmt::<agg::Rgb8>::create(320, 220);
  pix.clear();
  draw_black_frame(&mut pix);

  for i in 0..pix.height() / 2 {
    pix.copy_pixel(i, i, Rgb8::from_raw(127, 200, 98));
  }

  pix.to_file("tests/tmp/t02_pixel_formats.png").unwrap();
  assert!(agg::ppm::img_diff("tests/tmp/t02_pixel_formats.png", "images/t02_pixel_formats.png").unwrap());
}
