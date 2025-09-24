//! Rendering Cells

use crate::PixelLike;
use crate::POLY_SUBPIXEL_MASK;
use crate::POLY_SUBPIXEL_SCALE;
use crate::POLY_SUBPIXEL_SHIFT;

use std::cmp::max;
use std::cmp::min;

/// this is pixel coord without subpixel bits
pub type Position = i32;

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
  /// Return number of cells in a specific y row
  pub fn scanline_num_cells(&self, y: Position) -> usize {
    self.sorted_y[y as usize].len()
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
    if self.cells[n - 1].area == Area::ZERO && self.cells[n - 1].cover == Area::ZERO {
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
    let ex1 = x1.ipart();
    let ex2 = x2.ipart();
    let fx1 = x1.frac();
    let fx2 = x2.frac();

    let dy = Area::ONE.scale(y2 - y1);

    // Horizontal Line, trivial case. Happens often
    if dy == PixelLike::ZERO {
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
    let dx = Area::from_pixel(if rev { x1 - x2 } else { x2 - x1 });

    // Adjacent Cells on Same Line
    let (delta_y, mut xmod) = dy.scale(if rev { fx1 } else { P::ONE - fx1 }).div_mod_floor(dx);

    // write first cell, where
    //   area = (y2 - y1) * (1 - fx1) * (1 + fx1) / (x2 - x1)
    //   area = (y2 - y1) * fx1 * fx1 / (x1 - x2) if rev
    self.add_to_curr_cell(delta_y, delta_y.scale(if rev { fx1 } else { fx1 + P::ONE }));

    // TODO: if range.len() == 0 { return }

    // if there are more than 2 cells, we need to calculate the lift of line
    let delta_ex = if rev { -1 } else { 1 };
    let mut ex = ex1 + delta_ex;
    let mut y = Area::from_pixel(y1) + delta_y;
    if ex != ex2 {
      xmod -= dx;

      let (lift, rem) = dy.div_mod_floor(dx);
      while ex != ex2 {
        self.set_curr_cell(ex, ey);
        xmod += rem;
        let delta = if xmod >= Area::ZERO {
          xmod -= dx;
          lift + Area::EPSILON
        } else {
          lift
        };
        self.add_to_curr_cell(delta, delta); // delta.scale(P::ONE);
        y += delta_y;
        ex += delta_ex;
      }
    }
    // write last cell, here ex == ex2
    self.set_curr_cell(ex, ey);
    let delta_y = Area::from_pixel(y2) - y;
    self.add_to_curr_cell(delta_y, delta_y.scale(if rev { fx2 + P::ONE } else { fx2 }));
  }

  /// Draw a line from (x1,y1) to (x2,y2)
  ///
  /// Cells are added to the cells collection with cover and area values
  ///
  /// Input coordinates are at subpixel scale
  pub fn line<P: PixelLike>(&mut self, x1: P, y1: P, x2: P, y2: P) {
    let dx_limit = P::from_pixel(crate::types::types::I64F0::from_raw(16384));
    let dx = x2 - x1;
    // Split long lines in half
    if dx >= dx_limit || dx <= -dx_limit {
      let cx = (x1 + x2) / 2;
      let cy = (y1 + y2) / 2;
      self.line(x1, y1, cx, cy);
      self.line(cx, cy, x2, y2);
      // bug fix: add return here (compared to orignal C++ code)
      return;
    }
    let dy = y2 - y1;
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
      let n = self.cells.len();
      if self.cells[n - 1].area == 0 && self.cells[n - 1].cover == 0 {
        self.cells.pop();
      }
      return;
    }

    if dx == 0 {
      let ex = x1 >> POLY_SUBPIXEL_SHIFT;
      let two_fx = (x1 - (ex << POLY_SUBPIXEL_SHIFT)) << 1;

      let (first, incr) = if dy < 0 { (0, -1) } else { (POLY_SUBPIXEL_SCALE, 1) };
      //let x_from = x1;
      let delta = first - fy1;
      {
        let m_curr_cell = self.cells.last_mut().unwrap();
        m_curr_cell.cover += delta;
        m_curr_cell.area += two_fx * delta;
      }

      let mut ey1 = ey1 + incr;
      self.set_curr_cell(ex, ey1);
      let delta = first + first - POLY_SUBPIXEL_SCALE;
      let area = two_fx * delta;
      while ey1 != ey2 {
        {
          let m_curr_cell = self.cells.last_mut().unwrap();
          m_curr_cell.cover = delta;
          m_curr_cell.area = area;
        }
        ey1 += incr;
        self.set_curr_cell(ex, ey1);
      }
      let delta = fy2 - POLY_SUBPIXEL_SCALE + first;
      {
        let m_curr_cell = self.cells.last_mut().unwrap();
        m_curr_cell.cover += delta;
        m_curr_cell.area += two_fx * delta;
      }
      return;
    }
    // Render Multiple Lines
    let (p, first, incr, dy) = if dy < 0 {
      (fy1 * dx, 0, -1, -dy)
    } else {
      ((POLY_SUBPIXEL_SCALE - fy1) * dx, POLY_SUBPIXEL_SCALE, 1, dy)
    };
    let mut delta = p / dy;
    let mut xmod = p % dy;
    if xmod < 0 {
      delta -= 1;
      xmod += dy;
    }
    let mut x_from = x1 + delta;
    self.render_hline(ey1, x1, fy1, x_from, first);
    let mut ey1 = ey1 + incr;
    self.set_curr_cell(x_from >> POLY_SUBPIXEL_SHIFT, ey1);
    if ey1 != ey2 {
      let p = POLY_SUBPIXEL_SCALE * dx;
      let mut lift = p / dy;
      let mut rem = p % dy;
      if rem < 0 {
        lift -= 1;
        rem += dy;
      }
      xmod -= dy;
      while ey1 != ey2 {
        delta = lift;
        xmod += rem;
        if xmod >= 0 {
          xmod -= dy;
          delta += 1;
        }
        let x_to = x_from + delta;
        self.render_hline(ey1, x_from, POLY_SUBPIXEL_SCALE - first, x_to, first);
        x_from = x_to;
        ey1 += incr;
        self.set_curr_cell(x_from >> POLY_SUBPIXEL_SHIFT, ey1);
      }
    }
    self.render_hline(ey1, x_from, POLY_SUBPIXEL_SCALE - first, x2, fy2);
    self.pop_last_cell_if_empty();
  }
}
