use agg::{prelude::*, sources::line_height};

/*
char=H left=1 top=10 w=8 h=10
char=e left=0 top=8 w=7 h=8
char=l left=0 top=10 w=3 h=10
char=l left=0 top=10 w=3 h=10
char=o left=0 top=8 w=7 h=8
char=  left=0 top=0 w=0 h=0
space adv.x=256 adv.y=0
char=W left=0 top=10 w=13 h=10
char=o left=0 top=8 w=7 h=8
char=r left=0 top=8 w=5 h=8
char=l left=0 top=10 w=3 h=10
char=d left=0 top=10 w=7 h=10
char=! left=1 top=10 w=2 h=10
char=! left=1 top=10 w=2 h=10
char=! left=1 top=10 w=2 h=10
*/

#[test]
fn t23_font() {
  let lib = agg::ft::Library::init().unwrap();
  let font = lib.new_face("tests/assets/DejaVuSans.ttf", 0).unwrap();
  // ["Regular", "Bold", "Oblique", "Bold Oblique", "Light", "Light Oblique"]
  assert_eq!(font.family_name().unwrap(), "DejaVu Sans");
  assert_eq!(font.style_name().unwrap(), "Book");
  font.set_pixel_sizes(0, 13).unwrap();
  assert_eq!(line_height(&font), 17.0);
  assert_eq!(font.height(), 2384);
  println!("{:?}", font.size_metrics().unwrap());

  let pix = agg::Pixfmt::<agg::Rgb8>::create(100, 100);
  let mut ren_base = agg::RenderingBase::new(pix);
  ren_base.clear(agg::Rgb8::WHITE);

  agg::draw_text("Hello World!!!", 50, 45, &font, &mut ren_base); // visual height 10

  let mut label = agg::Label::new("Hello World!!!", 50., 58., 13.0, &font)
    .unwrap()
    .xalign(agg::XAlign::Center)
    .yalign(agg::YAlign::Center);
  assert_eq!(label.size(), (91., 17.));
  label.draw(&mut ren_base); // visual height 11

  ren_base.blend_hline(50, 58, 50, agg::Rgba8::RED, 1.0);

  ren_base.to_file("tests/tmp/font.png").unwrap();
  assert!(agg::utils::img_diff("tests/tmp/font.png", "images/font.png").unwrap());
}
