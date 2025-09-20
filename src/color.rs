//! Colors

use palette::blend::Premultiply;

use crate::{Color, ColorValue, U16, U8};

pub trait FromColor {
    fn from_color<C: Color>(c: C) -> Self;
}

pub trait FromRaw2: Sized {
    type Raw;
    fn from_raw(v1: Self::Raw, v2: Self::Raw) -> Self;
    fn from_slice(slice: &[Self::Raw]) -> Self where Self::Raw: Copy {
        assert!(slice.len() >= 2);
        Self::from_raw(slice[0], slice[1])
    }
}
pub trait IntoRaw2: Sized {
    type Raw;
    fn into_raw(self) -> (Self::Raw, Self::Raw);
    fn into_slice(self) -> [Self::Raw; 2] where Self::Raw: Copy {
        let (v1, v2) = self.into_raw();
        [v1, v2]
    }
}

pub trait FromRaw3: Sized {
    type Raw;
    fn from_raw(red: Self::Raw, green: Self::Raw, blue: Self::Raw) -> Self;
    fn from_slice(slice: &[Self::Raw]) -> Self where Self::Raw: Copy {
        assert!(slice.len() >= 3);
        Self::from_raw(slice[0], slice[1], slice[2])
    }
}
pub trait IntoRaw3: Sized {
    type Raw;
    fn into_raw(self) -> (Self::Raw, Self::Raw, Self::Raw);
    fn into_slice(self) -> [Self::Raw; 3] where Self::Raw: Copy {
        let (v1, v2, v3) = self.into_raw();
        [v1, v2, v3]
    }
}

pub trait FromRaw4: Sized {
    type Raw;
    fn from_raw(red: Self::Raw, green: Self::Raw, blue: Self::Raw, alpha: Self::Raw) -> Self;
    fn from_slice(slice: &[Self::Raw]) -> Self where Self::Raw: Copy {
        assert!(slice.len() >= 4);
        Self::from_raw(slice[0], slice[1], slice[2], slice[3])
    }
}
pub trait IntoRaw4: Sized {
    type Raw;
    fn into_raw(self) -> (Self::Raw, Self::Raw, Self::Raw, Self::Raw);
    fn into_slice(self) -> [Self::Raw; 4] where Self::Raw: Copy {
        let (v1, v2, v3, v4) = self.into_raw();
        [v1, v2, v3, v4]
    }
}

pub trait NamedColor {
    fn empty() -> Self;
    fn white() -> Self;
    fn black() -> Self;
    fn gray_color<T: ColorValue>(g: T, a: T) -> Self;
}

pub type Rgba<T> = palette::rgb::LinSrgba<T>;
pub type Rgba8 = Rgba<U8>;
pub type Rgba16 = Rgba<U16>;
pub type Rgba32 = Rgba<f32>;
pub type Rgba64 = Rgba<f64>;

impl<T: ColorValue> FromRaw4 for Rgba<T> {
    type Raw = T::Raw;
    fn from_raw(red: T::Raw, green: T::Raw, blue: T::Raw, alpha: T::Raw) -> Self {
        Self::new(red.into(), green.into(), blue.into(), alpha.into())
    }
}
impl<T: ColorValue> IntoRaw4 for Rgba<T> {
    type Raw = T::Raw;
    fn into_raw(self) -> (Self::Raw, Self::Raw, Self::Raw, Self::Raw) {
        (self.red.into(), self.green.into(), self.blue.into(), self.alpha.into())
    }
}
impl<T: ColorValue> FromColor for Rgba<T> {
    fn from_color<C: Color>(c: C) -> Self {
        Self::new(c.red_(), c.green_(), c.blue_(), c.alpha_())
    }
}

// impl<T: ColorValueType> Rgba<T> {
//     pub fn premultiply(&self) -> RgbaPre<T> {
//         match self.a {
//             a if a == T::ONE => {
//                 RgbaPre::new(self.r, self.g, self.b, self.a)
//             },
//             a if a == T::ZERO => {
//                 RgbaPre::new(T::ZERO, T::ZERO, T::ZERO, self.a)
//             },
//             a => {
//                 let r = self.r.color_mul(a);
//                 let g = self.g.color_mul(a);
//                 let b = self.b.color_mul(a);
//                 RgbaPre::new(r, g, b, self.a)
//             }
//         }
//     }
// }


impl<T: ColorValue> NamedColor for Rgba<T> {
    fn empty() -> Self {
        Self::new(T::ZERO, T::ZERO, T::ZERO, T::ZERO)
    }
    fn white() -> Self {
        Self::new(T::ONE, T::ONE, T::ONE, T::ONE)
    }
    fn black() -> Self {
        Self::new(T::ZERO, T::ZERO, T::ZERO, T::ONE)
    }
    fn gray_color<T2: ColorValue>(gray: T2, alpha: T2) -> Self {
        let gray = gray.as_color_();
        Self::new(gray, gray, gray, alpha.as_color_())
    }
}

// impl Rgba8 {
//     /// Crate new color from a wavelength and gamma
//     pub fn from_wavelength_gamma(w: f64, gamma: f64) -> Self {
//         let c = Rgb8::from_wavelength_gamma(w, gamma);
//         Self::from_color(c)
//     }
// }

pub type Gray<T> = palette::Alpha<palette::luma::LinLuma<palette::white_point::D65, T>, T>;
pub type Gray8  = Gray<U8>;
pub type Gray16 = Gray<U16>;
pub type Gray32 = Gray<f32>;
pub type Gray64 = Gray<f64>;

impl<T: ColorValue> FromRaw2 for Gray<T> {
    type Raw = T::Raw;
    fn from_raw(luma: T::Raw, alpha: T::Raw) -> Self {
        Self::new(luma.into(), alpha.into())
    }
}
impl<T: ColorValue> IntoRaw2 for Gray<T> {
    type Raw = T::Raw;
    fn into_raw(self) -> (Self::Raw, Self::Raw) {
        (self.luma.into(), self.alpha.into())
    }
}
impl<T: ColorValue> FromColor for Gray<T> {
    fn from_color<C: Color>(c: C) -> Self {
        let l = T::luminance(c.red_(), c.green_(), c.blue_());
        Self::new(l, c.alpha_())
    }
}

impl<T: ColorValue> NamedColor for Gray<T> {
    fn empty() -> Self {
        Self::new(T::ZERO, T::ZERO)
    }
    fn white() -> Self {
        Self::new(T::ONE, T::ONE)
    }
    fn black() -> Self {
        Self::new(T::ZERO, T::ONE)
    }
    fn gray_color<T2: ColorValue>(g: T2, a: T2) -> Self {
        Self::new(g.as_color_(), a.as_color_())
    }
}

pub fn luminance(red: f64, green: f64, blue: f64) -> f64 {
    0.2126 * red + 0.7152 * green + 0.0722 * blue
}

/// Lightness (max(R, G, B) + min(R, G, B)) / 2
pub fn lightness(red: f64, green: f64, blue: f64) -> f64 {
    let mut cmax = red;
    let mut cmin = red;
    if green > cmax { cmax = green; }
    if blue  > cmax { cmax = blue;  }
    if green < cmin { cmin = green; }
    if blue  < cmin { cmin = blue;  }

    (cmax + cmin) / 2.0
}
/// Average
pub fn average(red: f64, green: f64, blue: f64) -> f64 {
    (red + green + blue) / 3.0
}

pub type Rgb<T> = palette::rgb::LinSrgb<T>;
pub type Rgb8  = Rgb<U8>;
pub type Rgb16 = Rgb<U16>;
pub type Rgb32 = Rgb<f32>;
pub type Rgb64 = Rgb<f64>;

impl<T: ColorValue> FromRaw3 for Rgb<T> {
    type Raw = T::Raw;
    fn from_raw(red: T::Raw, green: T::Raw, blue: T::Raw) -> Self {
        Self::new(red.into(), green.into(), blue.into())
    }
}
impl<T: ColorValue> IntoRaw3 for Rgb<T> {
    type Raw = T::Raw;
    fn into_raw(self) -> (Self::Raw, Self::Raw, Self::Raw) {
        (self.red.into(), self.green.into(), self.blue.into())
    }
}
impl<T: ColorValue> FromColor for Rgb<T> {
    fn from_color<C: Color>(c: C) -> Self {
        Self::new(c.red_(), c.green_(), c.blue_())
    }
}

impl<T: ColorValue> NamedColor for Rgb<T> {
    fn empty() -> Self {
        Self::new(T::ZERO, T::ZERO, T::ZERO)
    }
    fn white() -> Self {
        Self::new(T::ONE, T::ONE, T::ONE)
    }
    fn black() -> Self {
        Self::new(T::ZERO, T::ZERO, T::ZERO)
    }
    fn gray_color<T2: ColorValue>(gray: T2, _: T2) -> Self {
        let gray = gray.as_color_();
        Self::new(gray, gray, gray)
    }
}

// impl Rgb8 {
//     pub fn from_wavelength_gamma(w: f64, gamma: f64) -> Self {
//         let (r,g,b) =
//             if w >= 380.0 && w <= 440.0 {
//                 (-1.0 * (w-440.0) / (440.0-380.0), 0.0, 1.0)
//             } else if w >= 440.0 && w <= 490.0 {
//                 (0.0, (w-440.0)/(490.0-440.0), 1.0)
//             } else if w >= 490.0 && w <= 510.0 {
//                 (0.0, 1.0, -1.0 * (w-510.0)/(510.0-490.0))
//             } else if w >= 510.0 && w <= 580.0 {
//                 ((w-510.0)/(580.0-510.0), 1.0, 0.0)
//             } else if w >= 580.0 && w <= 645.0 {
//                 (1.0, -1.0*(w-645.0)/(645.0-580.0), 0.0)
//             } else if w >= 645.0 && w <= 780.0 {
//                 (1.0, 0.0, 0.0)
//             } else {
//                 (0.,0.,0.)
//             };
//         let scale =
//             if w > 700.0 {
//                 0.3 + 0.7 * (780.0-w)/(780.0-700.0)
//             } else if w < 420.0 {
//                 0.3 + 0.7 * (w-380.0)/(420.0-380.0)
//             } else {
//                 1.0
//             };
//         let r = (r * scale).powf(gamma) * 255.0;
//         let g = (g * scale).powf(gamma) * 255.0;
//         let b = (b * scale).powf(gamma) * 255.0;
//         Self::new ( r as u8, g as u8, b as u8 )
//     }
// }


pub type RgbaPre<T> = palette::blend::PreAlpha<Rgb<T>>;
pub type RgbaPre8  = RgbaPre<U8>;
pub type RgbaPre16 = RgbaPre<U16>;
pub type RgbaPre32 = RgbaPre<f32>;
pub type RgbaPre64 = RgbaPre<f64>;

impl<T: ColorValue> FromRaw4 for RgbaPre<T> {
    type Raw = T::Raw;
    fn from_raw(red: T::Raw, green: T::Raw, blue: T::Raw, alpha: T::Raw) -> Self {
        Self {
            color: Rgb::new(red.into(), green.into(), blue.into()),
            alpha: alpha.into()
        }
    }
}
impl<T: ColorValue> IntoRaw4 for RgbaPre<T> {
    type Raw = T::Raw;
    fn into_raw(self) -> (Self::Raw, Self::Raw, Self::Raw, Self::Raw) {
        (self.red.into(), self.green.into(), self.blue.into(), self.alpha.into())
    }
}
impl<T: ColorValue> FromColor for RgbaPre<T> {
    fn from_color<C: Color>(c: C) -> Self {
        Self::new(Rgb::new(c.red_(), c.green_(), c.blue_()), c.alpha_())
    }
}

impl<T: ColorValue> NamedColor for RgbaPre<T>
where
    Rgb<T>: Premultiply<Scalar = T>,
{
    fn empty() -> Self {
        Rgba::empty().premultiply()
    }
    fn white() -> Self {
        Rgba::white().premultiply()
    }
    fn black() -> Self {
        Rgba::black().premultiply()
    }
    fn gray_color<T2: ColorValue>(gray: T2, alpha: T2) -> Self {
        Rgba::gray_color(gray, alpha).premultiply()
    }
}

pub type Srgba<T> = palette::rgb::Srgba<T>;
pub type Srgba8  = Srgba<U8>;
pub type Srgba16 = Srgba<U16>;
pub type Srgba32 = Srgba<f32>;
pub type Srgba64 = Srgba<f64>;

impl<T: ColorValue> FromRaw4 for Srgba<T> {
    type Raw = T::Raw;
    fn from_raw(red: T::Raw, green: T::Raw, blue: T::Raw, alpha: T::Raw) -> Self {
        Self::new(red.into(), green.into(), blue.into(), alpha.into())
    }
}
impl<T: ColorValue> IntoRaw4 for Srgba<T> {
    type Raw = T::Raw;
    fn into_raw(self) -> (Self::Raw, Self::Raw, Self::Raw, Self::Raw) {
        (self.red.into(), self.green.into(), self.blue.into(), self.alpha.into())
    }
}
impl<T: ColorValue> FromColor for Srgba<T> {
    fn from_color<C: Color>(c: C) -> Self {
        Self::new(c.red_(), c.green_(), c.blue_(), c.alpha_())
    }
}

impl<T: ColorValue> Color for Rgba<T> {
    fn red_<T2: ColorValue>(&self) -> T2 { self.red.as_color_() }
    fn green_<T2: ColorValue>(&self) -> T2 { self.green.as_color_() }
    fn blue_<T2: ColorValue>(&self) -> T2 { self.blue.as_color_() }
    fn alpha_<T2: ColorValue>(&self) -> T2 { self.alpha.as_color_() }
    fn is_premultiplied(&self) -> bool { false }
}
impl<T: ColorValue> Color for Rgb<T> {
    fn red_<T2: ColorValue>(&self) -> T2 { self.red.as_color_() }
    fn green_<T2: ColorValue>(&self) -> T2 { self.green.as_color_() }
    fn blue_<T2: ColorValue>(&self) -> T2 { self.blue.as_color_() }
    fn alpha_<T2: ColorValue>(&self) -> T2 { T2::ONE }
    fn is_premultiplied(&self) -> bool { false }
}
impl<T: ColorValue> Color for RgbaPre<T> where  Rgb<T>: Premultiply<Scalar=T> {
    fn red_<T2: ColorValue>(&self) -> T2 { self.red.as_color_() }
    fn green_<T2: ColorValue>(&self) -> T2 { self.green.as_color_() }
    fn blue_<T2: ColorValue>(&self) -> T2 { self.blue.as_color_() }
    fn alpha_<T2: ColorValue>(&self) -> T2 { self.alpha.as_color_() }
    fn is_premultiplied(&self) -> bool { true }
}
impl<T: ColorValue> Color for Srgba<T> {
    fn red_<T2: ColorValue>(&self) -> T2 { self.red.from_srgb_() }
    fn green_<T2: ColorValue>(&self) -> T2 { self.green.from_srgb_() }
    fn blue_<T2: ColorValue>(&self) -> T2 { self.blue.from_srgb_() }
    fn alpha_<T2: ColorValue>(&self) -> T2 { self.alpha.as_color_() }
    fn is_premultiplied(&self) -> bool { false }
}
impl<T: ColorValue> Color for Gray<T> {
    fn red_<T2: ColorValue>(&self) -> T2 { self.luma.as_color_() }
    fn green_<T2: ColorValue>(&self) -> T2 { self.luma.as_color_() }
    fn blue_<T2: ColorValue>(&self) -> T2 { self.luma.as_color_() }
    fn alpha_<T2: ColorValue>(&self) -> T2 { self.alpha.as_color_() }
    fn is_premultiplied(&self) -> bool { false }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn value_type_test() {
        for i in 0..=255u8 {
            assert_eq!(U8::new(i).as_color_::<U8>(), U8::new(i));
        }
        for i in 0..=65535u16 {
            assert_eq!(U16::new(i).as_color_::<U16>(), U16::new(i));
        }
    }

    #[test]
    fn rgb8_to_gray8_test() {
        let values = [[0,0,0,0u8],
                      [255,255,255,255],
                      [255,   0,   0,  54],
                      [0,   255,   0,  182],
                      [0,   0,   255,  18],
                      [255, 255,   0,  237],
                      [255, 0,   255,  73],
                      [0,   255, 255,  201],
                      [128,128,128,    128],
                      [128,   0,   0,  27],
                      [0,   128,   0,  92],
                      [0,   0,   128,  9],
                      [128, 128,   0,  119],
                      [128, 0,   128,  36],
                      [0,   128, 128,  101],
        ];
        for [r,g,b,z] in &values {
            let c = Rgb8::new(U8::new(*r),U8::new(*g),U8::new(*b));
            let gray = c.gray8();
            assert_eq!(gray.luma.0, *z);
        }
    }
    #[test]
    fn rgb8_test() {
        let w = Rgb8::white();
        assert_eq!(w.into_raw(), (255,255,255));
        assert_eq!((w.red8(), w.green8(), w.blue8()), (255,255,255));
        let w = Rgb8::black();
        assert_eq!(w.into_raw(), (0,0,0));
        assert_eq!((w.red8(), w.green8(), w.blue8()), (0,0,0));
        let w = Rgb8::gray_color(0.5, 1.0);
        assert_eq!(w.into_raw(), (128,128,128));
        assert_eq!((w.red8(), w.green8(), w.blue8()), (128,128,128));
        // let w = Rgb8::from_slice(&[1, 2, 3]);
        // assert_eq!(w, Rgb8::from_raw(1, 2, 3));
        let w = Rgb8::from_raw(0, 90, 180);
        assert_eq!(w.into_raw(), (0,90,180));
        assert_eq!((w.red8(), w.green8(), w.blue8()), (0,90,180));
    }
    #[test]
    fn gray_test() {
        let g = Gray8::from_raw(34, 255);
        assert_eq!(g.into_raw(), (34, 255));
        let g = Gray8::from_raw(134, 100);
        assert_eq!(g.into_raw(), (134, 100));
        // let g = Gray8::from_slice(&[10,20]);
        // assert_eq!(g.into_raw(), (10,20));
    }
    #[test]
    fn rgba8_test() {
        let c = Rgba8::white();
        assert_eq!(c.into_raw(), (255,255,255,255));
        assert_eq!((c.red8(), c.green8(), c.blue8(), c.alpha8()), (255,255,255,255));
        let c = Rgba8::black();
        assert_eq!(c.into_raw(), (0,0,0,255));
        assert_eq!((c.red8(), c.green8(), c.blue8(), c.alpha8()), (0,0,0,255));
        let c = Rgba8::from_raw(255, 90, 84, 72);
        assert_eq!(c.into_raw(), (255,90,84,72));
        assert_eq!((c.red8(), c.green8(), c.blue8(), c.alpha8()), (255,90,84,72));
        let c = Rgba8::empty();
        assert_eq!(c.into_raw(), (0,0,0,0));
        assert_eq!((c.red8(), c.green8(), c.blue8(), c.alpha8()), (0,0,0,0));
        let c = Rgba8::from_raw(255, 255, 255, 128);
        let p = c.premultiply();
        assert_eq!(p, RgbaPre8::from_raw(128, 128, 128, 128));
    }
    #[test]
    fn srgb_test() {
        let s = Srgba8::from_raw(50,150,250,128);
        assert_eq!(s.into_raw(), (50,150,250,128));
        assert_eq!((s.red8(), s.green8(), s.blue8(), s.alpha8()), (8,78,244,128));
        let t = s.rgba8();
        assert_eq!(t.into_raw(), (8,78,244,128));
        assert_eq!((t.red8(), t.green8(), t.blue8(), t.alpha8()), (8,78,244,128));
    }

    #[test]
    fn rgba_pre_test() {
        let c = RgbaPre8::white();
        assert_eq!(c.into_raw(), (255,255,255,255));
        assert_eq!((c.red8(), c.green8(), c.blue8(), c.alpha8()), (255,255,255,255));
        let c = RgbaPre8::black();
        assert_eq!(c.into_raw(), (0,0,0,255));
        assert_eq!((c.red8(), c.green8(), c.blue8(), c.alpha8()), (0,0,0,255));
        let c = RgbaPre8::from_raw(255, 90, 84, 72);
        assert_eq!(c.into_raw(), (255,90,84,72));
        assert_eq!((c.red8(), c.green8(), c.blue8(), c.alpha8()), (255,90,84,72));
        let c = Rgba8::from_raw(255, 90, 84, 72).premultiply();
        assert_eq!(c, RgbaPre8::from_raw(72, 25, 24, 72));
        assert_eq!((c.red8(), c.green8(), c.blue8(), c.alpha8()), (72,25,24,72));
    }
}
