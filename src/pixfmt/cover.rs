use std::slice::SliceIndex;

use crate::RealLike;

pub enum Covers<'a, T> {
  Repeat(T),
  Slice(&'a [T]),
}

impl<'a, T> Covers<'a, T>
where
  T: RealLike,
{
  pub fn len(&self) -> Option<usize> {
    match self {
      Covers::Repeat(_) => None,
      Covers::Slice(c) => Some(c.len()),
    }
  }

  pub fn is_empty(&self) -> bool {
    self.len() == Some(0)
  }

  pub fn check_len(&self, len: usize) -> bool {
    match self.len() {
      Some(l) => l == len,
      None => true,
    }
  }

  pub fn slice<I: SliceIndex<[T], Output = [T]>>(&self, range: I) -> Self {
    match self {
      Covers::Repeat(c) => Covers::Repeat(*c),
      Covers::Slice(c) => Covers::Slice(&c[range]),
    }
  }
}

impl<T: RealLike> From<T> for Covers<'_, T> {
  fn from(c: T) -> Self {
    Covers::Repeat(c)
  }
}

impl<'a, T: RealLike> From<&'a [T]> for Covers<'a, T> {
  fn from(c: &'a [T]) -> Self {
    Covers::Slice(c)
  }
}

impl<'a, T> std::ops::Index<usize> for Covers<'a, T>
where
  T: RealLike,
{
  type Output = T;

  fn index(&self, index: usize) -> &Self::Output {
    match self {
      Covers::Repeat(c) => c,
      Covers::Slice(c) => &c[index],
    }
  }
}

impl<'a, T: RealLike> IntoIterator for Covers<'a, T> {
  type Item = T;
  type IntoIter = CoversIter<'a, T>;

  fn into_iter(self) -> Self::IntoIter {
    CoversIter { covers: self, idx: 0 }
  }
}

pub struct CoversIter<'a, T> {
  covers: Covers<'a, T>,
  idx: usize,
}

impl<'a, T> Iterator for CoversIter<'a, T>
where
  T: RealLike,
{
  type Item = T;

  fn next(&mut self) -> Option<Self::Item> {
    let result = match &self.covers {
      Covers::Repeat(c) => *c,
      Covers::Slice(c) => {
        if self.idx >= c.len() {
          return None;
        }
        c[self.idx]
      }
    };
    self.idx += 1;
    Some(result)
  }
}
