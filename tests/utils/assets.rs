use agg::color::{FromRaw4, NamedColor};

pub fn parse_lion(arrange_orientations: bool) -> (Vec<agg::Path>, Vec<agg::Rgba8>) {
  let txt = std::fs::read_to_string("tests/assets/lion.txt").unwrap();
  let mut paths = vec![];
  let mut colors = vec![];
  let mut path = agg::Path::new();
  let mut color = agg::Rgba8::BLACK;
  let mut cmd = agg::PathCommand::Stop;

  for line in txt.lines() {
    let v: Vec<_> = line.split_whitespace().collect();
    if v.len() == 1 {
      let n = 0;
      let hex = v[0];
      let r = u8::from_str_radix(&hex[n..n + 2], 16).unwrap();
      let g = u8::from_str_radix(&hex[n + 2..n + 4], 16).unwrap();
      let b = u8::from_str_radix(&hex[n + 4..n + 6], 16).unwrap();
      if !path.vertices.is_empty() {
        path.close_polygon();
        paths.push(path);
        colors.push(color);
      }
      path = agg::Path::new();
      color = agg::Rgba8::from_raw(r, g, b, 255);
    } else {
      for val in v {
        if val == "M" {
          cmd = agg::PathCommand::MoveTo;
        } else if val == "L" {
          cmd = agg::PathCommand::LineTo;
        } else {
          let pts: Vec<_> = val.split(",").map(|x| x.parse::<f64>().unwrap()).collect();

          match cmd {
            agg::PathCommand::LineTo => path.line_to(pts[0], pts[1]),
            agg::PathCommand::MoveTo => {
              path.close_polygon();
              path.move_to(pts[0], pts[1]);
            }
            _ => unreachable!("oh no !!!"),
          }
        }
      }
    }
  }
  if !path.vertices.is_empty() {
    colors.push(color);
    path.close_polygon();
    paths.push(path);
  }
  assert_eq!(paths.len(), colors.len());
  if arrange_orientations {
    paths
      .iter_mut()
      .for_each(|p| p.arrange_orientations(agg::PathOrientation::Clockwise));
  }
  (paths, colors)
}

// Helper that recenters paths to the middle of a w x h pixel image and
// returns a Vec of ConvTransform wrappers ready for rendering.
pub fn transform_paths(paths: Vec<agg::Path>, w: f64, h: f64, rotate: f64) -> Vec<agg::ConvTransform> {
  if paths.is_empty() {
    return Vec::new();
  }
  let p = paths[0].vertices[0];
  let mut r = agg::Rectangle::new(p.x, p.y, p.x, p.y);
  for p in &paths {
    if let Some(rp) = agg::bounding_rect(p) {
      //eprintln!("dx,dy: {:?}", rp);
      r.expand_rect(&rp);
    }
  }
  //eprintln!("dx,dy: {:?}", r);
  let g_base_dx = (r.x2() - r.x1()) / 2.0;
  let g_base_dy = (r.y2() - r.y1()) / 2.0;
  assert!(g_base_dx == 119.0 && g_base_dy == 188.5);
  let mtx = agg::Transform::new()
    .then_translate(-g_base_dx, -g_base_dy)
    .then_rotate(rotate)
    .then_translate(w / 2.0, h / 2.0);
  //mtx.translate(0.0, 0.0);
  let t: Vec<_> = paths.into_iter().map(|p| agg::ConvTransform::new(p, mtx)).collect();
  println!("polygons: {}", t.len());
  t
}
