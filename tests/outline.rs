use agg::prelude::*;

#[test]
fn t24_outline_basic_render() {
    let pix = Pixfmt::<Rgb8>::new(100, 100);
    let mut ren_base = agg::RenderingBase::new(pix);
    ren_base.clear(Rgba8::WHITE);

    let mut ren = RendererPrimatives::with_base(&mut ren_base);
    ren.line_color(agg::Rgba8::BLACK);

    let mut path = agg::Path::new();
    path.move_to(10.0, 10.0);
    path.line_to(50.0, 90.0);
    path.line_to(90.0, 10.0);

    let mut ras = RasterizerOutline::with_primative(&mut ren);
    ras.add_path(&path);
    ren_base.to_file("tests/tmp/outline.png").unwrap();

    assert!(agg::ppm::img_diff("tests/tmp/outline.png", "images/outline.png").unwrap());
}

#[test]
fn t20_outline_render() {
    let pix = Pixfmt::<Rgb8>::new(100, 100);
    let mut ren_base = agg::RenderingBase::new(pix);
    ren_base.clear(Rgba8::WHITE);
    let mut ren = RendererOutlineAA::with_base(&mut ren_base);
    ren.color(agg::Rgba8::BLACK);
    ren.width(20.0);

    let mut path = agg::Path::new();
    path.move_to(10.0, 10.0);
    path.line_to(50.0, 90.0);
    path.line_to(90.0, 10.0);

    let mut ras = RasterizerOutlineAA::with_renderer(&mut ren);
    ras.round_cap(true);
    ras.add_path(&path);
    ren_base.to_file("tests/tmp/outline_aa.png").unwrap();

    assert!(agg::ppm::img_diff("tests/tmp/outline_aa.png", "images/outline_aa.png").unwrap());
}
