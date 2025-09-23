use crate::Span;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SpanLen<Coord> {
  Repeated(Coord), // negative length, repeated cover
  Slice(Coord), // positive length, variable covers
}

impl<Coord> SpanLen<Coord> {
  pub fn len(&self) -> usize where Coord: Copy + Into<usize> {
    match self {
      SpanLen::Repeated(len) | SpanLen::Slice(len) => (*len).into(),
    }
  }

  pub fn is_empty(&self) -> bool where Coord: Copy + Into<usize> {
    self.len() == 0
  }

  pub fn is_repeated(&self) -> bool {
    matches!(self, SpanLen::Repeated(_))
  }

  pub fn is_slice(&self) -> bool {
    matches!(self, SpanLen::Slice(_))
  }
}

#[derive(Debug)]
struct SpanPointer<Coord, Cover> {
  x: Coord,
  len: SpanLen<Coord>,
  covers: *const Cover,
}

struct SpanRef<'a, Coord, Cover> {
  x: Coord,
  len: SpanLen<Coord>,
  covers: &'a [Cover],
}

#[derive(Debug)]
pub struct Spans<Coord, Cover> {
  spans: Vec<SpanPointer<Coord, Cover>>,
  covers: Vec<Cover>,
}

impl<Coord, Cover> Default for Spans<Coord, Cover> {
  fn default() -> Self {
    Self { spans: Default::default(), covers: Default::default() }
  }
}

impl<Coord, Cover> Spans<Coord, Cover>
where
  Coord: Copy + Into<usize>,
  Cover: Clone,
{
  pub fn new(covers_len: usize, value: Cover) -> Self where Cover: Clone {
    Self {
      spans: Vec::new(),
      covers: vec![value; covers_len],
    }
  }
}
