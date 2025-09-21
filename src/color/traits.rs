use crate::ColorValue;

pub trait FromRaw2: Sized {
  type Raw;
  fn from_raw(v1: Self::Raw, v2: Self::Raw) -> Self;
  fn from_slice(slice: &[Self::Raw]) -> Self
  where
    Self::Raw: Copy,
  {
    assert!(slice.len() >= 2);
    Self::from_raw(slice[0], slice[1])
  }
}
pub trait IntoRaw2: Sized {
  type Raw;
  fn into_raw(self) -> (Self::Raw, Self::Raw);
  fn into_slice(self) -> [Self::Raw; 2]
  where
    Self::Raw: Copy,
  {
    let (v1, v2) = self.into_raw();
    [v1, v2]
  }
}

pub trait FromRaw3: Sized {
  type Raw;
  fn from_raw(red: Self::Raw, green: Self::Raw, blue: Self::Raw) -> Self;
  fn from_slice(slice: &[Self::Raw]) -> Self
  where
    Self::Raw: Copy,
  {
    assert!(slice.len() >= 3);
    Self::from_raw(slice[0], slice[1], slice[2])
  }
}
pub trait IntoRaw3: Sized {
  type Raw;
  fn into_raw(self) -> (Self::Raw, Self::Raw, Self::Raw);
  fn into_slice(self) -> [Self::Raw; 3]
  where
    Self::Raw: Copy,
  {
    let (v1, v2, v3) = self.into_raw();
    [v1, v2, v3]
  }
}

pub trait FromRaw4: Sized {
  type Raw;
  fn from_raw(red: Self::Raw, green: Self::Raw, blue: Self::Raw, alpha: Self::Raw) -> Self;
  fn from_slice(slice: &[Self::Raw]) -> Self
  where
    Self::Raw: Copy,
  {
    assert!(slice.len() >= 4);
    Self::from_raw(slice[0], slice[1], slice[2], slice[3])
  }
}
pub trait IntoRaw4: Sized {
  type Raw;
  fn into_raw(self) -> (Self::Raw, Self::Raw, Self::Raw, Self::Raw);
  fn into_slice(self) -> [Self::Raw; 4]
  where
    Self::Raw: Copy,
  {
    let (v1, v2, v3, v4) = self.into_raw();
    [v1, v2, v3, v4]
  }
}

pub trait NamedColor {
  const EMPTY: Self;
  const WHITE: Self;
  const BLACK: Self;
  const RED: Self;
  const GREEN: Self;
  const BLUE: Self;
  fn gray_color<T: ColorValue>(value: T) -> Self;
}
