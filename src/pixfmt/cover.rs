use crate::{Color, RealLike, UFixed};

/// TODO: do memory layout optimization for this enum
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Cover<T> {
  None,
  Mask(T),
  Full,
}

pub type CoverOf<C> = <C as Color>::Component;
pub type CoverIterOf<'a, C> = Covers<'a, <C as Color>::Component>;

impl From<u64> for Cover<u64> {
  fn from(v: u64) -> Self {
    if v == 0 {
      Cover::None
    } else if v >= Self::FULL {
      Cover::Full
    } else {
      Cover::Mask(v)
    }
  }
}

impl Cover<u64> {
  const BITS: usize = 8;
  const NONE: u64 = 0;
  const FULL: u64 = (1 << Self::BITS) - 1;

  pub fn into_<T: RealLike>(self) -> Cover<T> {
    match self {
      Cover::None => Cover::None,
      Cover::Mask(v) => Cover::Mask(T::from_f64(v as f64 / Self::FULL as f64)),
      Cover::Full => Cover::Full,
    }
  }
}

impl<T: RealLike> Cover<T> {
  pub fn new(v: T) -> Self {
    if v == T::ZERO {
      Cover::None
    } else if v == T::ONE {
      Cover::Full
    } else {
      Cover::Mask(v)
    }
  }

  pub fn from_raw(v: T::Raw) -> Self {
    Self::new(v.into())
  }

  pub fn to_f64(self) -> f64 {
    self.get().to_f64()
  }

  pub fn get(self) -> T {
    match self {
      Cover::None => T::ZERO,
      Cover::Mask(v) => v,
      Cover::Full => T::ONE,
    }
  }

  pub fn is_full(&self) -> bool {
    *self == Cover::Full
  }

  pub fn is_none(&self) -> bool {
    *self == Cover::None
  }
}

pub trait CoverLike<T: RealLike> {
  fn into_cover(self) -> Cover<T>;
}

pub enum CoverLikeImpl<T> {
  U64(Cover<u64>),
  Raw(T),
  Ref(Cover<T>),
}

impl<T: RealLike> CoverLike<T> for Cover<T> {
  fn into_cover(self) -> Cover<T> {
    self
  }
}

impl<T: RealLike> CoverLike<T> for Cover<u64> {
  fn into_cover(self) -> Cover<T> {
    self.into_()
  }
}

impl<T: RealLike> CoverLike<T> for T {
  fn into_cover(self) -> Cover<T> {
    Cover::new(self)
  }
}

impl<T> CoverLike<UFixed<T>> for T
where
  UFixed<T>: RealLike,
{
  fn into_cover(self) -> Cover<UFixed<T>> {
    Cover::new(self.into())
  }
}

// pub fn into_cover<Co: Into<CoverLikeImpl<T>>, T: RealLike>(c: Co) -> Cover<T> {
//   c.into().into()
// }

// impl<T: RealLike> From<Cover<T>> for CoverLikeImpl<T> {
//   fn from(c: Cover<T>) -> Self {
//     CoverLikeImpl::Ref(c)
//   }
// }

// impl<T> From<Cover<u64>> for CoverLikeImpl<T> {
//   fn from(c: Cover<u64>) -> Self {
//     CoverLikeImpl::U64(c)
//   }
// }

// impl<T> From<UFixed<T>> for CoverLikeImpl<UFixed<T>> {
//   fn from(c: UFixed<T>) -> Self {
//     CoverLikeImpl::Raw(c)
//   }
// }

// macro_rules! impl_cover_like_from_raw {
//   ($($t:ty)+) => {
//     $(
//       impl From<<$t as RealLike>::Raw> for CoverLike<$t> {
//         fn from(c: <$t as RealLike>::Raw) -> Self {
//           CoverLike::Raw(c.into())
//         }
//       }
//     )+
//   };
// }
// impl_cover_like_from_raw!(U8 U16 f32 f64);

impl<T: RealLike + Clone> From<CoverLikeImpl<T>> for Cover<T> {
  fn from(c: CoverLikeImpl<T>) -> Self {
    match c {
      CoverLikeImpl::U64(c) => c.into_(),
      CoverLikeImpl::Raw(c) => Cover::new(c),
      CoverLikeImpl::Ref(c) => c,
    }
  }
}

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
