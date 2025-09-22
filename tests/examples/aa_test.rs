use agg::{math::lerp_u8, prelude::*};

fn path_from_slice(pts: &[f64]) -> agg::Path {
  assert!(pts.len().is_multiple_of(2));
  assert!(pts.len() >= 4);
  let mut path = agg::Path::new();
  path.move_to(pts[0] + 0.5, pts[1] + 0.5);
  for i in (2..pts.len()).step_by(2) {
    path.line_to(pts[i] + 0.5, pts[i + 1] + 0.5);
  }
  path
}

#[allow(clippy::too_many_arguments)]
fn dash_line<T: Pixel, C: Color + FromColor>(
  ren: &mut agg::RenderingScanlineAASolid<T, C>,
  ras: &mut agg::RasterizerScanline,
  x1: f64,
  y1: f64,
  x2: f64,
  y2: f64,
  line_width: f64,
  dash_len: f64,
) {
  ras.reset();
  let mut path = agg::Path::new();
  path.move_to(x1 + 0.5, y1 + 0.5);
  path.line_to(x2 + 0.5, y2 + 0.5);
  if dash_len > 0.0 {
    let mut dash = agg::Dash::new(path);
    dash.add_dash(dash_len, dash_len);
    let mut stroke = agg::Stroke::new(dash);
    stroke.width(line_width);
    stroke.line_cap(agg::LineCap::Round);
    ras.add_path(&stroke);
  } else {
    let mut stroke = agg::Stroke::new(path);
    stroke.width(line_width);
    stroke.line_cap(agg::LineCap::Round);
    ras.add_path(&stroke);
  }
  agg::render_scanlines(ras, ren);
}

#[test]
fn t26_aa_test() {
  let (width, height) = (480, 350);
  let pix = agg::Pixfmt::<agg::Rgb8>::create(width, height);
  let mut ren_base = agg::RenderingBase::new(pix);

  ren_base.clear(Rgb8::BLACK);

  // Radial Line Test
  let cx = width as f64 / 2.0;
  let cy = height as f64 / 2.0;
  let r = cx.min(cy);

  let mut ras = agg::RasterizerScanline::new();
  {
    let mut ren = agg::RenderingScanlineAASolid::new(&mut ren_base, Rgba8::BLACK);
    ren.color(agg::Rgba64::from_raw(1.0, 1.0, 1.0, 0.2));
    for i in (1..=180).rev() {
      let n = 2.0 * (i as f64) * std::f64::consts::PI / 180.0;
      dash_line(
        &mut ren,
        &mut ras,
        cx + r * n.sin(),
        cy + r * n.cos(),
        cx,
        cy,
        1.0,
        if i < 90 { i as f64 } else { 0.0 },
      );
    }
  }

  for i in 1..=20 {
    let k = i as f64;
    let mut ren = agg::RenderingScanlineAASolid::new(&mut ren_base, Rgb8::BLACK);
    // Integral Point Sizes 1..=20
    ras.reset();
    ren.color(agg::Rgb8::WHITE);
    let ell = agg::Ellipse::new(20.0 + k * (k + 1.0) + 0.5, 20.5, k / 2.0, k / 2.0, 8 + i);
    ras.add_path(&ell);
    agg::render_scanlines(&mut ras, &mut ren);

    // Fractional Point Sizes 0..=2
    ras.reset();
    let ell = agg::Ellipse::new(18. + (k * 4.0) + 0.5, 33. + 0.5, k / 20.0, k / 20.0, 8);
    ras.add_path(&ell);
    agg::render_scanlines(&mut ras, &mut ren);

    // Fractional Point Positioning
    ras.reset();
    let ell = agg::Ellipse::new(
      18. + (k * 4.0) + (k - 1.0) / 10.0 + 0.5,
      27. + (k - 1.0) / 10.0 + 0.5,
      0.5,
      0.5,
      8,
    );
    ras.reset();
    ras.add_path(&ell);
    agg::render_scanlines(&mut ras, &mut ren);

    // Integral Line Widths 1..=20
    let gradient_colors = color_gradient(
      agg::Rgb64::WHITE,
      agg::Rgb64::from_raw((i % 2) as f64, 0.5 * (i % 3) as f64, 0.25 * (i % 5) as f64),
      256,
    )
    .into_iter()
    .map(|c| c.rgb8())
    .collect::<Vec<_>>();
    let x1 = 20.0 + k * (k + 1.0);
    let y1 = 40.5;
    let x2 = 20.0 + k * (k + 1.0) + ((k - 1.0) * 4.0);
    let y2 = 100.5;
    let gradient_mtx = calc_linear_gradient_transform(x1, y1, x2, y2);
    let span = agg::SpanGradient::new(gradient_mtx, agg::GradientX {}, &gradient_colors, 0.0, 100.0);
    let mut ren_grad = agg::RenderingScanlineAA::new(&mut ren_base, span);
    let path = path_from_slice(&[x1, y1, x2, y2]);
    let mut stroke = agg::Stroke::new(path);
    stroke.width(k);
    stroke.line_cap(agg::LineCap::Round);
    ras.reset();
    ras.add_path(&stroke);
    agg::render_scanlines(&mut ras, &mut ren_grad);

    // Fractional Line Lengths H (Red/Blue)
    let gradient_colors = color_gradient(agg::Rgb64::RED, agg::Rgb64::BLUE, 256)
      .into_iter()
      .map(|c| c.rgb8())
      .collect::<Vec<_>>();
    let x1 = 17.5 + (k * 4.0);
    let y1 = 107.;
    let x2 = 17.5 + (k * 4.0) + k / 6.66666667;
    let y2 = 107.;
    let gradient_mtx = calc_linear_gradient_transform(x1, y1, x2, y2);
    let span = agg::SpanGradient::new(gradient_mtx, agg::GradientX {}, &gradient_colors, 0.0, 100.0);
    let mut ren_grad = agg::RenderingScanlineAA::new(&mut ren_base, span);
    let path = path_from_slice(&[x1, y1, x2, y2]);
    let mut stroke = agg::Stroke::new(path);
    stroke.width(1.0);
    stroke.line_cap(agg::LineCap::Round);
    ras.reset();
    ras.add_path(&stroke);
    agg::render_scanlines(&mut ras, &mut ren_grad);

    // Fractional Line Lengths V (Red/Blue)
    let x1 = 18.0 + (k * 4.0);
    let y1 = 112.5;
    let x2 = 18.0 + (k * 4.0);
    let y2 = 112.5 + k / 6.66666667;
    let gradient_mtx = calc_linear_gradient_transform(x1, y1, x2, y2);
    let span = agg::SpanGradient::new(gradient_mtx, agg::GradientX {}, &gradient_colors, 0.0, 100.0);
    let mut ren_grad = agg::RenderingScanlineAA::new(&mut ren_base, span);
    let path = path_from_slice(&[x1, y1, x2, y2]);
    let mut stroke = agg::Stroke::new(path);
    stroke.width(1.0);
    stroke.line_cap(agg::LineCap::Round);
    ras.reset();
    ras.add_path(&stroke);
    agg::render_scanlines(&mut ras, &mut ren_grad);

    // Fractional Line Positioning (Red)
    let colors = color_gradient(agg::Rgb64::RED, agg::Rgb64::WHITE, 256)
      .into_iter()
      .map(|c| c.rgb8())
      .collect::<Vec<_>>();
    let x1 = 21.5;
    let y1 = 120.0 + (k - 1.0) * 3.1;
    let x2 = 52.5;
    let y2 = 120.0 + (k - 1.0) * 3.1;
    let gradient_mtx = calc_linear_gradient_transform(x1, y1, x2, y2);
    let span = agg::SpanGradient::new(gradient_mtx, agg::GradientX {}, &colors, 0.0, 100.0);
    let mut ren_grad = agg::RenderingScanlineAA::new(&mut ren_base, span);
    let path = path_from_slice(&[x1, y1, x2, y2]);
    let mut stroke = agg::Stroke::new(path);
    stroke.width(1.0);
    stroke.line_cap(agg::LineCap::Round);
    ras.reset();
    ras.add_path(&stroke);
    agg::render_scanlines(&mut ras, &mut ren_grad);

    // Fractional Line Widths 2..0 (Green)
    let colors = color_gradient(agg::Rgb64::GREEN, agg::Rgb64::WHITE, 256)
      .into_iter()
      .map(|c| c.rgb8())
      .collect::<Vec<_>>();
    let x1 = 52.5;
    let y1 = 118.0 + (k * 3.0);
    let x2 = 83.5;
    let y2 = 118.0 + (k * 3.0);
    let gradient_mtx = calc_linear_gradient_transform(x1, y1, x2, y2);
    let span = agg::SpanGradient::new(gradient_mtx, agg::GradientX {}, &colors, 0.0, 100.0);
    let mut ren_grad = agg::RenderingScanlineAA::new(&mut ren_base, span);
    let path = path_from_slice(&[x1, y1, x2, y2]);
    let mut stroke = agg::Stroke::new(path);
    stroke.width(2.0 - (k - 1.0) / 10.0);
    stroke.line_cap(agg::LineCap::Round);
    ras.reset();
    ras.add_path(&stroke);
    agg::render_scanlines(&mut ras, &mut ren_grad);

    // Stippled Fractional Width 2..0 (Blue)
    let colors = color_gradient(agg::Rgb64::BLUE, agg::Rgb64::WHITE, 256)
      .into_iter()
      .map(|c| c.rgb8())
      .collect::<Vec<_>>();
    let x1 = 83.5;
    let y1 = 119.0 + (k * 3.0);
    let x2 = 114.5;
    let y2 = 119.0 + (k * 3.0);
    let gradient_mtx = calc_linear_gradient_transform(x1, y1, x2, y2);
    let span = agg::SpanGradient::new(gradient_mtx, agg::GradientX {}, &colors, 0.0, 100.0);
    let mut ren_grad = agg::RenderingScanlineAA::new(&mut ren_base, span);
    let path = path_from_slice(&[x1, y1, x2, y2]);
    let mut dash = agg::Dash::new(path);
    dash.add_dash(3.0, 3.0);
    let mut stroke = agg::Stroke::new(dash);
    stroke.width(2.0 - (k - 1.0) / 10.0);
    stroke.line_cap(agg::LineCap::Round);
    ras.reset();
    ras.add_path(&stroke);
    agg::render_scanlines(&mut ras, &mut ren_grad);

    let mut ren = agg::RenderingScanlineAASolid::new(&mut ren_base, Rgb8::BLACK);
    ren.color(agg::Rgb8::WHITE);
    if i <= 10 {
      // Integral line width, horz aligned (mipmap test)
      let path = path_from_slice(&[
        125.5,
        119.5 + (k + 2.0) * (k / 2.0),
        135.5,
        119.5 + (k + 2.0) * (k / 2.0),
      ]);
      let mut stroke = agg::Stroke::new(path);
      stroke.width(k);
      stroke.line_cap(agg::LineCap::Round);
      ras.reset();
      ras.add_path(&stroke);
      agg::render_scanlines(&mut ras, &mut ren);
    }
    // Fractional line width 0..2, 1 px H
    let path = path_from_slice(&[17.5 + (k * 4.0), 192.0, 18.5 + (k * 4.0), 192.0]);
    let mut stroke = agg::Stroke::new(path);
    stroke.width(k / 10.0);
    stroke.line_cap(agg::LineCap::Round);
    ras.reset();
    ras.add_path(&stroke);
    agg::render_scanlines(&mut ras, &mut ren);

    // Fractional line positioning, 1 px H
    let path = path_from_slice(&[
      17.5 + (k * 4.0) + (k - 1.0) / 10.0,
      186.0,
      18.5 + (k * 4.0) + (k - 1.0) / 10.0,
      186.0,
    ]);
    let mut stroke = agg::Stroke::new(path);
    stroke.width(1.0);
    stroke.line_cap(agg::LineCap::Round);
    ras.reset();
    ras.add_path(&stroke);
    agg::render_scanlines(&mut ras, &mut ren);
  }

  let mut ren = agg::RenderingScanlineAASolid::new(&mut ren_base, Rgb8::BLACK);
  ren.color(agg::Rgb8::WHITE);
  for i in 1..=13 {
    let k = i as f64;

    let gradient_colors = color_gradient(
      agg::Rgb64::WHITE,
      agg::Rgb64::from_raw((i % 2) as f64, 0.5 * (i % 3) as f64, 0.25 * (i % 5) as f64),
      256,
    )
    .into_iter()
    .map(|c| c.rgb8())
    .collect::<Vec<_>>();
    let y0 = height as f64 - 20. - k * (k + 2.0);
    let x1 = width as f64 - 150.;
    let y1 = height as f64 - 20. - k * (k + 1.5);
    let x2 = width as f64 - 20.;
    let y2 = height as f64 - 20. - k * (k + 1.0);
    // println!("triangle {i} from ({x1},{y1}) to ({x2},{y0}-{y2})");

    let gradient_mtx = calc_linear_gradient_transform(x1, y1, x2, y2);
    // println!("Gradient Mtx: {:?}", gradient_mtx);
    let span = agg::SpanGradient::new(gradient_mtx, agg::GradientX {}, &gradient_colors, 0.0, 100.0);
    let mut ren_grad = agg::RenderingScanlineAA::new(&mut ren_base, span);

    ras.reset();
    ras.move_to(x1, y1);
    ras.line_to(x2, y2);
    ras.line_to(x2, y0);
    agg::render_scanlines(&mut ras, &mut ren_grad);
  }

  // Save the image to a file
  ren_base.to_file("tests/tmp/aa_test.png").unwrap();
  assert!(agg::ppm::img_diff("tests/tmp/aa_test.png", "images/aa_test.png").unwrap());
}

#[allow(clippy::assign_op_pattern)]
fn calc_linear_gradient_transform(x1: f64, y1: f64, x2: f64, y2: f64) -> agg::Transform {
  let gradient_d2 = 100.0;
  let dx = x2 - x1;
  let dy = y2 - y1;
  let s = (dx * dx + dy * dy).sqrt() / gradient_d2;
  let mut mtx = agg::Transform::new();
  mtx = mtx * agg::Transform::scaling(s, s);
  mtx = mtx * agg::Transform::rotation(dy.atan2(dx));
  mtx = mtx * agg::Transform::translation(x1 + 0.5, y1 + 0.5);
  mtx = mtx.then_invert();

  let mtx2 = agg::Transform::new()
    .then_scale(s, s)
    .then_rotate(dy.atan2(dx))
    .then_translate(x1 + 0.5, y1 + 0.5)
    .then_invert();
  assert!(mtx == mtx2);

  mtx
}

fn color_gradient<C: Color>(begin: C, end: C, n: usize) -> Vec<Rgb64> {
  let begin = begin.srgba8();
  let end = end.srgba8();
  (0..n)
    .map(|i| {
      let a = (i as f64 / (n - 1) as f64 * 255.).round() as u8;
      let red = lerp_u8(begin.red.0, end.red.0, a);
      let green = lerp_u8(begin.green.0, end.green.0, a);
      let blue = lerp_u8(begin.blue.0, end.blue.0, a);
      Srgba8::from_raw(red, green, blue, 255).rgb()
    })
    .collect()
}
