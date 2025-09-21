//! Pixel Format

pub mod _pixfmt;
pub mod alpha_blend;
pub mod buffer;
pub mod pixel;

pub use _pixfmt::*;
pub use alpha_blend::*;
pub use buffer::*;
pub use pixel::*;

#[cfg(test)]
mod tests {
  use crate::FromRaw4;
  use crate::NamedColor;
  use crate::Pixel;
  use crate::Pixfmt;
  use crate::Rgb8;
  use crate::Rgba8;
  use crate::Rgba32;
  use crate::RgbaPre8;
  use crate::Source;
  use crate::Srgba8;
  #[test]
  fn pixfmt_test() {
    let mut p = Pixfmt::<Rgb8>::create(10, 10);
    assert_eq!(p.rbuf.data.len(), 300);

    p.copy_pixel(0, 0, Rgb8::BLACK);
    assert_eq!(p.get((0, 0)), Rgba8::BLACK);

    assert_ne!(p.get((1, 0)), Rgba8::WHITE);
    p.copy_pixel(1, 0, Rgb8::WHITE);
    assert_eq!(p.get((1, 0)), Rgba8::WHITE);

    let red = Rgba8::from_raw(255, 0, 0, 128);
    p.copy_hline(0, 1, 10, red);
    for i in 0..10 {
      assert_eq!(p.get((i, 1)), Rgba8::from_raw(255, 0, 0, 255));
    }
    let yellow = Srgba8::from_raw(128, 255, 0, 128);
    p.copy_hline(0, 2, 10, yellow);
    for i in 0..10 {
      assert_eq!(p.get((i, 2)), Rgba8::from_raw(55, 255, 0, 255));
    }
    let fuchsia = Rgba32::from_raw(0.0, 1.0, 1.0, 0.5);
    p.copy_hline(0, 3, 10, fuchsia);
    for i in 0..10 {
      assert_eq!(p.get((i, 3)), Rgba8::from_raw(0, 255, 255, 255));
    }
    p.clear();
    assert_eq!(p.get((0, 3)), Rgba8::from_raw(255, 255, 255, 255));

    let red = Rgba8::from_raw(255, 0, 0, 128);
    p.copy_vline(1, 0, 10, red);
    for i in 0..10 {
      assert_eq!(p.get((1, i)), Rgba8::from_raw(255, 0, 0, 255));
    }
    let yellow = Srgba8::from_raw(128, 255, 0, 128);
    p.copy_vline(2, 0, 10, yellow);
    for i in 0..10 {
      assert_eq!(p.get((2, i)), Rgba8::from_raw(55, 255, 0, 255));
    }
    let fuchsia = Rgba32::from_raw(0.0, 1.0, 1.0, 0.5);
    p.copy_vline(3, 0, 10, fuchsia);
    for i in 0..10 {
      assert_eq!(p.get((3, i)), Rgba8::from_raw(0, 255, 255, 255));
    }

    p.clear();
    p.copy_pixel(11, 11, Rgb8::BLACK);
    for i in 0..10 {
      for j in 0..10 {
        assert_eq!(p.get((i, j)), Rgba8::WHITE);
      }
    }
    p.copy_hline(0, 0, 20, Rgb8::BLACK);
    for i in 0..10 {
      assert_eq!(p.get((i, 0)), Rgba8::BLACK);
    }
    p.copy_hline(5, 1, 20, Rgb8::BLACK);
    for i in 5..10 {
      assert_eq!(p.get((i, 1)), Rgba8::BLACK);
    }

    p.clear();
    p.copy_vline(0, 0, 20, Rgb8::BLACK);
    for i in 0..10 {
      assert_eq!(p.get((0, i)), Rgba8::BLACK);
    }

    p.clear();
    p.copy_vline(1, 5, 20, Rgb8::BLACK);
    for i in 0..5 {
      assert_eq!(p.get((1, i)), Rgba8::WHITE, "pix({},{}): {:?}", 1, i, p.get((1, i)));
    }
    for i in 5..10 {
      assert_eq!(p.get((1, i)), Rgba8::BLACK, "pix({},{}): {:?}", 1, i, p.get((1, i)));
    }
    p.copy_vline(2, 3, 5, Rgb8::BLACK);
    for i in 0..3 {
      assert_eq!(p.get((2, i)), Rgba8::WHITE, "pix({},{}): {:?}", 2, i, p.get((2, i)));
    }
    for i in 3..8 {
      assert_eq!(p.get((2, i)), Rgba8::BLACK, "pix({},{}): {:?}", 2, i, p.get((2, i)));
    }
    for i in 8..10 {
      assert_eq!(p.get((2, i)), Rgba8::WHITE, "pix({},{}): {:?}", 2, i, p.get((2, i)));
    }
  }

  #[test]
  fn pixfmt_rgb8_test() {
    let mut pix = Pixfmt::<Rgb8>::create(1, 1);
    let black = Rgba8::BLACK;
    let white = Rgba8::WHITE;

    pix.copy_pixel(0, 0, Rgba8::from_raw(0, 0, 0, 255));
    assert_eq!(pix.get((0, 0)), black);

    let (alpha, beta, cover) = (255, 255, 255); // Copy Pixel
    pix.copy_pixel(0, 0, Rgba8::from_raw(0, 0, 0, alpha));
    pix.copy_or_blend_pix_with_cover((0, 0), Rgba8::from_raw(255, 255, 255, beta), cover);
    assert_eq!(pix.get((0, 0)), white);

    let (alpha, beta, cover) = (255, 255, 0); // Do Nothing, No Coverage
    pix.copy_pixel(0, 0, Rgba8::from_raw(0, 0, 0, alpha));
    pix.copy_or_blend_pix_with_cover((0, 0), Rgba8::from_raw(255, 255, 255, beta), cover);
    assert_eq!(pix.get((0, 0)), black);

    let (alpha, beta, cover) = (255, 0, 255); // Do Nothing, Transparent
    pix.copy_pixel(0, 0, Rgba8::from_raw(0, 0, 0, alpha));
    pix.copy_or_blend_pix_with_cover((0, 0), Rgba8::from_raw(255, 255, 255, beta), cover);
    assert_eq!(pix.get((0, 0)), black);

    let (alpha, beta, cover) = (255, 255, 128); // Partial Coverage, Blend
    pix.copy_pixel(0, 0, Rgba8::from_raw(0, 0, 0, alpha));
    pix.copy_or_blend_pix_with_cover((0, 0), Rgba8::from_raw(255, 255, 255, beta), cover);
    assert_eq!(pix.get((0, 0)), Rgba8::from_raw(128, 128, 128, 255));

    let (alpha, beta, cover) = (255, 128, 255); // Full Coverage, Alpha Color
    pix.copy_pixel(0, 0, Rgba8::from_raw(0, 0, 0, alpha));
    pix.copy_or_blend_pix_with_cover((0, 0), Rgba8::from_raw(255, 255, 255, beta), cover);
    assert_eq!(pix.get((0, 0)), Rgba8::from_raw(128, 128, 128, 255));

    let (alpha, beta, cover) = (128, 128, 255); // Partial Coverage, Blend
    pix.copy_pixel(0, 0, Rgba8::from_raw(255, 255, 255, alpha));
    assert_eq!(pix.get((0, 0)), Rgba8::from_raw(255, 255, 255, 255)); // Alpha channel is ignored
    pix.copy_or_blend_pix_with_cover((0, 0), Rgba8::from_raw(0, 0, 0, beta), cover);
    assert_eq!(pix.get((0, 0)), Rgba8::from_raw(127, 127, 127, 255));

    let (alpha, beta, cover) = (128, 128, 128); // Partial Coverage, Blend
    pix.copy_pixel(0, 0, Rgba8::from_raw(255, 255, 255, alpha));
    assert_eq!(pix.get((0, 0)), Rgba8::from_raw(255, 255, 255, 255)); // Alpha channel is ignored
    pix.copy_or_blend_pix_with_cover((0, 0), Rgba8::from_raw(0, 0, 0, beta), cover);
    assert_eq!(pix.get((0, 0)), Rgba8::from_raw(191, 191, 191, 255));
  }

  #[test]
  fn pixfmt_rgba8_test() {
    let mut pix = Pixfmt::<Rgba8>::create(1, 1);
    let black = Rgba8::BLACK;
    let white = Rgba8::WHITE;

    pix.copy_pixel(0, 0, Rgba8::from_raw(0, 0, 0, 255));
    assert_eq!(pix.get((0, 0)), black);

    let (alpha, beta, cover) = (255, 255, 255); // Copy Pixel
    pix.copy_pixel(0, 0, Rgba8::from_raw(0, 0, 0, alpha));
    pix.copy_or_blend_pix_with_cover((0, 0), Rgba8::from_raw(255, 255, 255, beta), cover);
    assert_eq!(pix.get((0, 0)), white);

    let (alpha, beta, cover) = (255, 255, 0); // Do Nothing, No Coverage
    pix.copy_pixel(0, 0, Rgba8::from_raw(0, 0, 0, alpha));
    pix.copy_or_blend_pix_with_cover((0, 0), Rgba8::from_raw(255, 255, 255, beta), cover);
    assert_eq!(pix.get((0, 0)), black);

    let (alpha, beta, cover) = (255, 0, 255); // Do Nothing, Transparent
    pix.copy_pixel(0, 0, Rgba8::from_raw(0, 0, 0, alpha));
    pix.copy_or_blend_pix_with_cover((0, 0), Rgba8::from_raw(255, 255, 255, beta), cover);
    assert_eq!(pix.get((0, 0)), black);

    let (alpha, beta, cover) = (255, 255, 128); // Partial Coverage, Blend
    pix.copy_pixel(0, 0, Rgba8::from_raw(0, 0, 0, alpha));
    pix.copy_or_blend_pix_with_cover((0, 0), Rgba8::from_raw(255, 255, 255, beta), cover);
    assert_eq!(pix.get((0, 0)), Rgba8::from_raw(128, 128, 128, 255));

    let (alpha, beta, cover) = (255, 128, 255); // Full Coverage, Alpha Color
    pix.copy_pixel(0, 0, Rgba8::from_raw(0, 0, 0, alpha));
    pix.copy_or_blend_pix_with_cover((0, 0), Rgba8::from_raw(255, 255, 255, beta), cover);
    assert_eq!(pix.get((0, 0)), Rgba8::from_raw(128, 128, 128, 255));

    let (alpha, beta, cover) = (128, 128, 255); // Partial Coverage, Blend
    pix.copy_pixel(0, 0, Rgba8::from_raw(255, 255, 255, alpha));
    assert_eq!(pix.get((0, 0)), Rgba8::from_raw(255, 255, 255, 128));
    pix.copy_or_blend_pix_with_cover((0, 0), Rgba8::from_raw(0, 0, 0, beta), cover);
    assert_eq!(pix.get((0, 0)), Rgba8::from_raw(127, 127, 127, 192));

    let (alpha, beta, cover) = (128, 128, 128); // Partial Coverage, Blend
    pix.copy_pixel(0, 0, Rgba8::from_raw(255, 255, 255, alpha));
    assert_eq!(pix.get((0, 0)), Rgba8::from_raw(255, 255, 255, 128)); // Alpha channel is ignored
    pix.copy_or_blend_pix_with_cover((0, 0), Rgba8::from_raw(0, 0, 0, beta), cover);
    assert_eq!(pix.get((0, 0)), Rgba8::from_raw(191, 191, 191, 160));
  }

  #[test]
  fn pixfmt_rgba8pre_test() {
    let mut pix = Pixfmt::<RgbaPre8>::create(1, 1);
    let black = Rgba8::BLACK;
    let white = Rgba8::WHITE;

    pix.copy_pixel(0, 0, Rgba8::from_raw(0, 0, 0, 255));
    assert_eq!(pix.get((0, 0)), black);

    let (alpha, beta, cover) = (255, 255, 255); // Copy Pixel
    pix.copy_pixel(0, 0, Rgba8::from_raw(0, 0, 0, alpha));
    pix.copy_or_blend_pix_with_cover((0, 0), Rgba8::from_raw(255, 255, 255, beta), cover);
    assert_eq!(pix.get((0, 0)), white);

    let (alpha, beta, cover) = (255, 255, 0); // Do Nothing, No Coverage
    pix.copy_pixel(0, 0, Rgba8::from_raw(0, 0, 0, alpha));
    pix.copy_or_blend_pix_with_cover((0, 0), Rgba8::from_raw(255, 255, 255, beta), cover);
    assert_eq!(pix.get((0, 0)), black);

    let (alpha, beta, cover) = (255, 0, 255); // Do Nothing, Transparent
    pix.copy_pixel(0, 0, Rgba8::from_raw(0, 0, 0, alpha));
    pix.copy_or_blend_pix_with_cover((0, 0), Rgba8::from_raw(255, 255, 255, beta), cover);
    assert_eq!(pix.get((0, 0)), black);

    let (alpha, beta, cover) = (255, 255, 128); // Partial Coverage, Blend
    pix.copy_pixel(0, 0, Rgba8::from_raw(0, 0, 0, alpha));
    pix.copy_or_blend_pix_with_cover((0, 0), Rgba8::from_raw(255, 255, 255, beta), cover);
    assert_eq!(pix.get((0, 0)), Rgba8::from_raw(128, 128, 128, 255));

    let (alpha, beta, cover) = (255, 128, 255); // Full Coverage, Alpha Color
    pix.copy_pixel(0, 0, Rgba8::from_raw(0, 0, 0, alpha));
    pix.copy_or_blend_pix_with_cover((0, 0), Rgba8::from_raw(255, 255, 255, beta), cover);
    assert_eq!(pix.get((0, 0)), Rgba8::from_raw(255, 255, 255, 255));

    let (alpha, beta, cover) = (128, 128, 255); // Partial Coverage, Blend
    pix.copy_pixel(0, 0, Rgba8::from_raw(255, 255, 255, alpha));
    assert_eq!(pix.get((0, 0)), Rgba8::from_raw(255, 255, 255, 128));
    pix.copy_or_blend_pix_with_cover((0, 0), Rgba8::from_raw(0, 0, 0, beta), cover);
    assert_eq!(pix.get((0, 0)), Rgba8::from_raw(127, 127, 127, 192));

    let (alpha, beta, cover) = (128, 128, 128); // Partial Coverage, Blend
    pix.copy_pixel(0, 0, Rgba8::from_raw(255, 255, 255, alpha));
    assert_eq!(pix.get((0, 0)), Rgba8::from_raw(255, 255, 255, 128)); // Alpha channel is ignored
    pix.copy_or_blend_pix_with_cover((0, 0), Rgba8::from_raw(0, 0, 0, beta), cover);
    assert_eq!(pix.get((0, 0)), Rgba8::from_raw(191, 191, 191, 160));
  }

  #[test]
  fn test_fill() {
    let (w, h) = (3, 5);
    let mut pix = Pixfmt::<RgbaPre8>::create(w, h);
    let black = Rgba8::BLACK;
    let white = Rgba8::WHITE;

    pix.clear();
    pix.fill(black);
    for y in 0..h {
      for x in 0..w {
        assert_eq!(pix.get((x, y)), black, "pix({},{}): {:?}", x, y, pix.get((x, y)));
      }
    }

    pix.copy_hline(0, 0, w, white);
    for x in 0..w {
      assert_eq!(pix.get((x, 0)), white, "pix({},0): {:?}", x, pix.get((x, 0)));
    }
  }
}
