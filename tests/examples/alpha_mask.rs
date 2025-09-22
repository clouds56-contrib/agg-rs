use agg::prelude::*;

use crate::utils::assets::{parse_lion, transform_paths};

fn ell_data() -> Vec<[u32; 6]> {
  vec![
    [115, 377, 106, 103, 255, 81],
    [205, 249, 112, 106, 171, 186],
    [326, 163, 79, 110, 194, 124],
    [488, 11, 56, 92, 141, 231],
    [99, 62, 50, 102, 159, 51],
    [306, 22, 22, 49, 183, 13],
    [346, 211, 76, 113, 93, 37],
    [233, 184, 39, 41, 212, 94],
    [198, 13, 90, 35, 180, 155],
    [386, 62, 93, 76, 65, 116],
  ]
}

/// This test translates the behavior of examples/src/alpha_mask.cpp to Rust.
/// It constructs an 8-bit alpha mask by rasterizing several ellipses into
/// a gray rendering buffer, then uses an alpha-mask adaptor to composite
/// colored filled shapes into an RGBA buffer.
#[test]
fn example_alpha_mask() {
  // Image size
  let (w, h) = (512, 400);

  // 1) Create alpha pixfmt (Gray8) and rasterize ellipses into it
  let mut alpha_base = agg::Pixfmt::<agg::Gray8>::create(w, h).into_rendering_base();
  // clear to fully transparent (luma=0, alpha=255)
  alpha_base.clear(agg::Gray8::from_raw(0, 255));

  let mut ras = agg::RasterizerScanline::new();

  // Rasterize deterministic ellipses into the alpha buffer
  {
    let data = ell_data();
    for e in data.iter() {
      let cx = e[0] as f64;
      let cy = e[1] as f64;
      let rx = e[2] as f64;
      let ry = e[3] as f64;
      let col = (e[4] & 0xFF) as u8;
      let alpha = (e[5] & 0xFF) as u8;

      ras.reset();
      let ell = agg::Ellipse::new(cx, cy, rx, ry, 100);
      // TODO: in fact Sgray8
      let color = agg::Gray8::from_raw(col, alpha);
      ras.add_path(&ell);

      // Render into the gray pixfmt using a gray color (luma,alpha)
      let mut ren_alpha = agg::RenderingScanlineAASolid::new(&mut alpha_base, color);
      agg::render_scanlines(&mut ras, &mut ren_alpha);
    }
  }

  // Extract the alpha pixfmt back (we move it out of the RenderingBase)
  let alpha_pix = alpha_base.pixf;

  // 2) Create an RGBA pixfmt for the final image and an AlphaMaskAdaptor
  let mut rgb_pix = agg::Pixfmt::<agg::Rgb8>::create(w, h);
  rgb_pix.fill(agg::Rgb8::WHITE);

  // The PixfmtAlphaMask takes ownership of the rgb and alpha pixfmts
  let mut mix_base = agg::PixfmtAlphaMask::new(rgb_pix, alpha_pix).into_rendering_base();

  // 3) Rasterize the same ellipses and render using the alpha mask adaptor
  // Use the public AlphaMaskRenderer implemented in the alphamask module.
  // let mut ren = agg::AlphaMaskRenderer::new(&mut mix, agg::Rgba8::from_raw(255, 0, 0, 255));
  let (paths, colors) = parse_lion(true);
  let t = transform_paths(paths, w as f64, h as f64);

  let mut ren = agg::RenderingScanlineAASolid::new_black(&mut mix_base);

  agg::render_all_paths(&mut ras, &mut ren, &t, &colors);
  // Save the resulting RGBA buffer
  mix_base.to_file("tests/tmp/alpha_mask.png").unwrap();
  assert!(
    agg::ppm::img_diff(
      "tests/tmp/alpha_mask.png",
      "images/alpha_mask.png"
    )
    .unwrap()
  );
}
