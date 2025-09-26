//! Rendering Cells

use crate::PixelLike;
use crate::Position;
use crate::PIXEL_SHIFT;

use std::cmp::max;
use std::cmp::min;

/// Rendering Cell
///
/// Effectively Represents a Pixel
#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub(crate) struct Cell<Area> {
  // cell_aa
  /// Cell x position
  pub x: Position,
  /// Cell y position
  pub y: Position,
  /// Cell coverage
  pub cover: Area,
  /// Cell area
  pub area: Area,
}

impl<Area> Cell<Area> {
  /// Create a new Cell
  ///
  /// Cover and Area are both 0
  fn new() -> Self where Area: PixelLike {
    Cell {
      x: Position::MAX,
      y: Position::MAX,
      cover: Area::ZERO,
      area: Area::ZERO,
    }
  }
  /// Create new cell at position (x,y)
  pub fn at(x: Position, y: Position) -> Self where Area: PixelLike {
    let mut c = Cell::new();
    c.x = x;
    c.y = y;
    c
  }
  /// Compare two cell positionsx
  pub fn equal(&self, x: Position, y: Position) -> bool {
    self.x - x == 0 && self.y - y == 0
  }
  // / Test if cover and area are equal to 0
  //pub fn is_empty(&self) -> bool {
  //    self.cover == 0 && self.area == 0
  //}
}

/// Collection of Cells
#[derive(Debug, Default)]
pub(crate) struct RasterizerCell<Area> {
  /// Cells
  cells: Vec<Cell<Area>>,
  /// Minimum x value of current cells
  pub min_x: Position,
  /// Maximum x value of current cells
  pub max_x: Position,
  /// Minimum y value of current cells
  pub min_y: Position,
  /// Maximum y value of current cells
  pub max_y: Position,
  /// Cells sorted by y position, then x position
  pub sorted_y: Vec<Vec<Cell<Area>>>,
}

impl<Area> RasterizerCell<Area> {
  /// Create new Cell collection
  pub fn new() -> Self {
    Self {
      cells: Vec::with_capacity(256),
      min_x: Position::MAX,
      min_y: Position::MAX,
      max_x: Position::MIN,
      max_y: Position::MIN,
      sorted_y: vec![],
    }
  }
}

impl<Area: PixelLike> RasterizerCell<Area> {
  /// Clear cells
  pub fn reset(&mut self) {
    self.max_x = Position::MIN;
    self.max_y = Position::MIN;
    self.min_x = Position::MAX;
    self.min_y = Position::MAX;
    self.sorted_y.clear(); // Not sure if this should be cleared
    self.cells.clear(); // Not sure if this should be cleared
  }

  /// Return total number of cells
  pub fn total_cells(&self) -> usize {
    self.cells.len()
  }
  /// Sort cells into sorted_y cells
  ///
  /// Cells are distributed into y bins, then sorted by x value
  pub fn sort_cells(&mut self) {
    if !self.sorted_y.is_empty() || self.max_y < 0 {
      return;
    }
    // Distribute into
    self.sorted_y = vec![Vec::with_capacity(8); (self.max_y + 1) as usize];
    for c in self.cells.iter() {
      if c.y >= 0 {
        let y = c.y as usize;
        self.sorted_y[y].push(*c);
      }
    }
    // Sort by the x value
    for i in 0..self.sorted_y.len() {
      self.sorted_y[i].sort_by(|a, b| (a.x).cmp(&b.x));
    }
  }
  /// Returns the cells of a specific y row
  pub fn scanline_cells(&self, y: Position) -> &[Cell<Area>] {
    &self.sorted_y[y as usize]
  }

  //pub fn add_curr_cell(&mut self, new_cell: Cell) {
  //    self.cells.push( new_cell );
  //}
  /// Determine if the last cell is equal to (x,y) and is empty
  // fn curr_cell_is_set(&self, x: i64, y: i64) -> bool {
  //     match self.cells.last() {
  //         None      => true,
  //         Some(cur) => {
  //             ! cur.equal(x,y) && ! cur.is_empty()
  //         }
  //     }
  // }
  /// Determine if the current cell is located at (x,y)
  fn curr_cell_not_equal(&self, x: Position, y: Position) -> bool {
    match self.cells.last() {
      None => true,
      Some(cur) => !cur.equal(x, y),
    }
  }
  /// Remove last cell is cover and area are equal to 0
  fn pop_last_cell_if_empty(&mut self) {
    let n = self.cells.len();
    if n == 0 {
      return;
    }
    if self.cells[n - 1].area == 0 && self.cells[n - 1].cover == 0 {
      self.cells.pop();
    } //else {
    //  self.show_last_cell();
    //}
  }
  /// Print the last cell
  // fn show_last_cell(&self) {
  //     if let Some(c) = self.cells.last() {
  //         println!("ADD_CURR_CELL: {} {} area {} cover {} len {}", c.x,c.y,c.area,c.cover, self.cells.len());
  //     }
  // }
  /// Create new cell at (x,y)
  ///
  /// Current cell is removed if empty (cover and area equal to 0)
  /// New cell is added to cell list
  fn set_curr_cell(&mut self, x: Position, y: Position) {
    // trace!("SET_CURR_CELL: ({},{})", x, y);
    if self.curr_cell_not_equal(x, y) {
      self.pop_last_cell_if_empty();
      self.cells.push(Cell::at(x, y));
    }
  }

  fn add_to_curr_cell(&mut self, cover: Area, area: Area) {
    if let Some(c) = self.cells.last_mut() {
      c.cover += cover;
      c.area += area;
    }
  }

  /// Create and update new cells
  /// assuming y1.ipart() == y2.ipart() == ey
  fn render_hline<P: PixelLike>(&mut self, ey: Position, x1: P, y1: P, x2: P, y2: P) {
    trace!("RENDER_HLINE: y={} from ({:.5},{:.5}) to ({:.5},{:.5})", ey, x1.to_f64(), y1.to_f64(), x2.to_f64(), y2.to_f64());
    let ex1 = x1.ipart();
    let ex2 = x2.ipart();
    let fx1 = x1.frac();
    let fx2 = x2.frac();

    let dy = Area::from_fixed(y2 - y1);

    // Horizontal Line, trivial case. Happens often
    if dy == 0 {
      // set current cell to end of line
      self.set_curr_cell(ex2, ey);
      return;
    }

    // Single Cell
    if ex1 == ex2 {
      // everything is located in a single cell. That is easy!
      self.add_to_curr_cell(dy, dy.scale(fx1 + fx2));
      return;
    }

    let rev = x2 < x1;
    let dx = Area::from_fixed(if rev { x1 - x2 } else { x2 - x1 });

    // Adjacent Cells on Same Line
    let (delta, mut xmod) = dy.scale(if rev { fx1 } else { P::ONE - fx1 }).div_mod_floor::<_, PIXEL_SHIFT>(dx);
    // write first cell, where
    //   area = (y2 - y1) * (1 - fx1) * (1 + fx1) / (x2 - x1)
    //   area = (y2 - y1) * fx1 * fx1 / (x1 - x2) if rev
    self.add_to_curr_cell(delta, delta.scale(if rev { fx1 } else { fx1 + P::ONE }));

    // TODO: if range.len() == 0 { return }

    // if there are more than 2 cells, we need to calculate the lift of line
    let delta_ex = if rev { -1 } else { 1 };
    let mut ex = ex1 + delta_ex;
    let mut y = Area::from_fixed(y1) + delta;
    if ex != ex2 {
      xmod -= dx >> PIXEL_SHIFT;

      let (lift, rem) = dy.div_mod_floor::<_, PIXEL_SHIFT>(dx);
      while ex != ex2 {
        self.set_curr_cell(ex, ey);
        xmod += rem;
        let delta_y = if xmod >= 0 {
          xmod -= dx >> PIXEL_SHIFT;
          lift + (Area::EPSILON << PIXEL_SHIFT)
        } else {
          lift
        };
        self.add_to_curr_cell(delta_y, delta_y); // delta.scale(P::ONE);
        y += delta_y;
        ex += delta_ex;
      }
    }
    // write last cell, here ex == ex2
    self.set_curr_cell(ex, ey);
    let delta = Area::from_fixed(y2) - y;
    self.add_to_curr_cell(delta, delta.scale(if rev { fx2 + P::ONE } else { fx2 }));
  }

  /// Draw a line from (x1,y1) to (x2,y2)
  ///
  /// Cells are added to the cells collection with cover and area values
  ///
  /// Input coordinates are at subpixel scale
  pub fn line<P: PixelLike>(&mut self, x1: P, y1: P, x2: P, y2: P) {
    debug!("LINE: ({:.5},{:.5}) to ({:.5},{:.5})", x1.to_f64(), y1.to_f64(), x2.to_f64(), y2.to_f64());
    const DX_LIMIT: Position = 16384;
    let dx = Area::from_fixed(x2 - x1);
    // Split long lines in half
    if dx.ipart().abs() >= DX_LIMIT {
      let cx = (x1 + x2) >> 1;
      let cy = (y1 + y2) >> 1;
      self.line(x1, y1, cx, cy);
      self.line(cx, cy, x2, y2);
      // bug fix: add return here (compared to orignal C++ code)
      return;
    }
    let dy = Area::from_fixed(y2 - y1);
    // Downshift
    let ex1 = x1.ipart();
    let ex2 = x2.ipart();
    let ey1 = y1.ipart();
    let ey2 = y2.ipart();
    let fy1 = y1.frac();
    let fy2 = y2.frac();

    self.min_x = min(ex2, min(ex1, self.min_x));
    self.min_y = min(ey2, min(ey1, self.min_y));
    self.max_x = max(ex2, max(ex1, self.max_x));
    self.max_y = max(ey2, max(ey1, self.max_y));

    self.set_curr_cell(ex1, ey1);
    // Horizontal Line
    if ey1 == ey2 {
      self.render_hline(ey1, x1, fy1, x2, fy2);
      self.pop_last_cell_if_empty();
      return;
    }

    let rev = dy < 0;

    // Vertical Line
    if dx == 0 {
      let ex = x1.ipart();
      let two_fx = x1.frac() << 1;

      let first = if rev { Area::ZERO } else { Area::ONE };
      let incr = if rev { -1 } else { 1 };
      // let (first, incr) = if rev { (0, -1) } else { (POLY_SUBPIXEL_SCALE, 1) };
      //let x_from = x1;
      let delta = first - Area::from_fixed(fy1);
      self.add_to_curr_cell(delta, delta.scale(two_fx));

      let mut ey1 = ey1 + incr;
      self.set_curr_cell(ex, ey1);
      let delta = first + first - Area::ONE;
      let area = delta.scale(two_fx);
      while ey1 != ey2 {
        self.add_to_curr_cell(delta, area);
        ey1 += incr;
        self.set_curr_cell(ex, ey1);
      }
      let delta = first + Area::from_fixed(fy2) - Area::ONE;
      self.add_to_curr_cell(delta, delta.scale(two_fx));
      return;
    }

    // Render Multiple Lines
    let dy = if rev { -dy } else { dy };
    let incr = if rev { -1 } else { 1 };
    let first = if rev { P::ZERO } else { P::ONE };
    let (delta, mut xmod) = dx.scale(if rev { fy1 } else { P::ONE - fy1 }).div_mod_floor::<_, PIXEL_SHIFT>(dy);
    let mut x_from = x1 + P::from_fixed(delta);
    self.render_hline(ey1, x1, fy1, x_from, first);
    let mut ey1 = ey1 + incr;
    self.set_curr_cell(x_from.ipart(), ey1);
    if ey1 != ey2 {
      let p = Area::from_fixed(dx);
      let (lift, rem) = p.div_mod_floor::<_, PIXEL_SHIFT>(dy);
      xmod -= dy >> PIXEL_SHIFT;
      while ey1 != ey2 {
        xmod += rem;
        let delta = if xmod >= 0 {
          xmod -= dy >> PIXEL_SHIFT;
          lift + (Area::EPSILON << PIXEL_SHIFT)
        } else {
          lift
        };
        let x_to = x_from + P::from_fixed(delta);
        self.render_hline(ey1, x_from, P::ONE - first, x_to, first);
        x_from = x_to;
        ey1 += incr;
        self.set_curr_cell(x_from.ipart(), ey1);
      }
    }
    self.render_hline(ey1, x_from, P::ONE - first, x2, fy2);
    self.pop_last_cell_if_empty();
  }
}
