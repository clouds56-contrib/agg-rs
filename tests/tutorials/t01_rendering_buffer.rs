extern crate agg;

use agg::{pixfmt::RenderingBuffer, prelude::*};

/// Draw a black frame around the rendering buffer, assuming it has
/// RGB-structure, one byte per color component
fn draw_black_frame(rbuf: &mut RenderingBuffer) {
  let w = rbuf.width;
  let h = rbuf.height;
  for i in 0..h {
    rbuf.get_pixel_mut(0, i).fill(0); // Left Side
    rbuf.get_pixel_mut(w - 1, i).fill(0); // Right Side
  }
  for i in 0..w {
    rbuf.get_pixel_mut(i, 0).fill(0); // Top Side
    rbuf.get_pixel_mut(i, h - 1).fill(0); // Bottom Side
  }
}

#[test]
fn t01_rendering_buffer() {
  // Allocate the buffer.
  let mut rbuf = RenderingBuffer::new(320, 220, 3);

  rbuf.data.fill(255);
  draw_black_frame(&mut rbuf);

  for i in 0..rbuf.height / 2 {
    //let p = rbuf.row_ptr(i);
    let p = rbuf.get_pixel_mut(i, i);
    p[0] = 127;
    p[1] = 200;
    p[2] = 98;
  }

  let pix = agg::Pixfmt::<Rgb8>::new(rbuf);
  pix.to_file("tests/tmp/t01_rendering_buffer.png").unwrap();
  assert!(agg::ppm::img_diff("tests/tmp/t01_rendering_buffer.png", "images/t01_rendering_buffer.png").unwrap());
}
