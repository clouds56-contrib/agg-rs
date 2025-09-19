//! Colors

use std::fmt::Debug;

use crate::Color;
use crate::math::multiply_u8;

pub trait ColorValueType: Copy + Debug + PartialEq<Self> {
    fn color_min() -> Self { Self::from_color_f64(0.0) }
    fn color_max() -> Self { Self::from_color_f64(1.0) }

    fn as_color_f64(self) -> f64;
    fn as_color_u8(self) -> u8 { (self.as_color_f64() * 255.0).round() as u8 }
    fn from_color_f64(v: f64) -> Self;
    fn from_color_u8(v: u8) -> Self { Self::from_color_f64(v as f64 / 255.0) }
    fn as_color_<T: ColorValueType>(self) -> T {
        T::from_color_f64(self.as_color_f64())
    }

    fn luminance_f64(r: Self, g: Self, b: Self) -> f64 {
        luminance(r.as_color_f64(), g.as_color_f64(), b.as_color_f64())
    }
    fn luminance_<T: ColorValueType>(r: Self, g: Self, b: Self) -> T { Self::luminance_f64(r, g, b).as_color_() }
    fn luminance(r: Self, g: Self, b: Self) -> Self { Self::luminance_(r, g, b) }

    fn to_srgb_f64(self) -> f64 { rgb_to_srgb(self.as_color_f64()) }
    fn to_srgb_<T: ColorValueType>(self) -> T { self.to_srgb_f64().as_color_() }
    fn to_srgb(self) -> Self { self.to_srgb_() }
    fn from_srgb_f64(self) -> f64 { srgb_to_rgb(self.as_color_f64()) }
    fn from_srgb_<T: ColorValueType>(self) -> T { self.from_srgb_f64().as_color_() }
    fn from_srgb(self) -> Self { self.from_srgb_() }

    fn color_mul(self, v: Self) -> Self {
        let a = self.as_color_f64();
        let b = v.as_color_f64();
        Self::from_color_f64(a * b)
    }
}
impl ColorValueType for u8 {
    fn color_min() -> Self { 0 }
    fn color_max() -> Self { 255 }
    fn as_color_f64(self) -> f64 { f64::from(self) / 255.0 }
    fn as_color_u8(self) -> u8 { self }
    fn from_color_f64(v: f64) -> Self { (v * 255.0).round() as u8 }
    fn from_color_u8(v: u8) -> Self { v }
    fn color_mul(self, v: Self) -> Self { multiply_u8(self, v) }
}
impl ColorValueType for u16 {
    fn color_min() -> Self { 0 }
    fn color_max() -> Self { 65535 }
    fn as_color_f64(self) -> f64 { f64::from(self) / 65535.0 }
    fn from_color_f64(v: f64) -> Self { (v * 65535.0).round() as u16 }
}
impl ColorValueType for f32 {
    fn as_color_f64(self) -> f64 { f64::from(self) }
    fn from_color_f64(v: f64) -> Self { v as f32 }
}
impl ColorValueType for f64 {
    fn as_color_f64(self) -> f64 { self }
    fn from_color_f64(v: f64) -> Self { v }
}

pub trait ConvertColorValue<T1: ColorValueType, T2: ColorValueType> {
    fn convert_color_value(v: T1) -> T2 {
        T2::from_color_f64(v.as_color_f64())
    }
}

pub trait NamedColor {
    fn white() -> Self;
    fn black() -> Self;
    fn gray_color<T: ColorValueType>(g: T, a: T) -> Self;
}

/// Convert from sRGB to RGB for a single component
fn srgb_to_rgb(x: f64) -> f64 {
    if x <= 0.04045 {
        x / 12.92
    } else {
        ((x + 0.055) / 1.055).powf(2.4)
    }
}
/// Convert from RGB to sRGB for a single component
fn rgb_to_srgb(x: f64) -> f64 {
    if x <= 0.003_130_8 {
        x * 12.92
    } else {
        1.055 * x.powf(1.0/2.4) - 0.055
    }
}

/// Color as Red, Green, Blue, and Alpha
#[derive(Debug,Default,Copy,Clone,PartialEq)]
pub struct Rgba<T> {
    /// Red
    pub r: T,
    /// Green
    pub g: T,
    /// Blue
    pub b: T,
    /// Alpha
    pub a: T,
}

pub type Rgba8 = Rgba<u8>;
pub type Rgba16 = Rgba<u16>;
pub type Rgba32 = Rgba<f32>;
pub type Rgba64 = Rgba<f64>;

impl<T: ColorValueType> Rgba<T> {
    pub fn from_color<C: Color>(c: C) -> Self {
        Self::new(c.red(), c.green(), c.blue(), c.alpha())
    }
    pub fn new(r: T, g: T, b: T, a: T) -> Self {
        Self { r, g, b, a }
    }
    pub fn clear(&mut self) {
        self.r = T::color_min();
        self.g = T::color_min();
        self.b = T::color_min();
        self.a = T::color_min();
    }
    pub fn into_slice(&self) -> [T;4] {
        [self.r, self.g, self.b, self.a]
    }
    pub fn premultiply(&self) -> RgbaPre<T> {
        match self.a {
            a if a == T::color_max() => {
                RgbaPre::new(self.r, self.g, self.b, self.a)
            },
            a if a == T::color_min() => {
                RgbaPre::new(T::color_min(), T::color_min(), T::color_min(), self.a)
            },
            a => {
                let r = self.r.color_mul(a);
                let g = self.g.color_mul(a);
                let b = self.b.color_mul(a);
                RgbaPre::new(r, g, b, self.a)
            }
        }
    }
}

impl<T: ColorValueType> NamedColor for Rgba<T> {
    fn white() -> Self {
        Self::new(T::color_max(), T::color_max(), T::color_max(), T::color_max())
    }
    fn black() -> Self {
        Self::new(T::color_min(), T::color_min(), T::color_min(), T::color_max())
    }
    fn gray_color<T2: ColorValueType>(gray: T2, alpha: T2) -> Self {
        let gray = gray.as_color_();
        Self::new(gray, gray, gray, alpha.as_color_())
    }
}

impl Rgba8 {
    /// Crate new color from a wavelength and gamma
    pub fn from_wavelength_gamma(w: f64, gamma: f64) -> Self {
        let c = Rgb8::from_wavelength_gamma(w, gamma);
        Self::from_color(c)
    }
}


/// Gray scale
#[derive(Debug,Copy,Clone,Default,PartialEq)]
pub struct Gray<T> {
    pub value: T,
    pub alpha: T,
}

pub type Gray8  = Gray<u8>;
pub type Gray16 = Gray<u16>;
pub type Gray32 = Gray<f32>;
pub type Gray64 = Gray<f64>;

impl<T: ColorValueType> Gray<T> {
    pub fn from_color<C: Color>(c: C) -> Self {
        let lum = luminance(c.red(), c.green(), c.blue());
        Self::new_with_alpha( lum.as_color_(), c.alpha() )
    }
    /// Create a new gray scale value
    pub fn new(value: T) -> Self {
        Self { value, alpha: T::color_max() }
    }
    pub fn new_with_alpha(value: T, alpha: T) -> Self {
        Self { value, alpha }
    }
    pub fn from_slice(v: &[T]) -> Self {
        Self { value: v[0], alpha: v[1] }
    }
    pub fn into_slice(&self) -> [T;2] {
        [self.value, self.alpha]
    }
}

impl<T: ColorValueType> NamedColor for Gray<T> {
    fn white() -> Self {
        Self::new(T::color_max())
    }
    fn black() -> Self {
        Self::new(T::color_min())
    }
    fn gray_color<T2: ColorValueType>(g: T2, a: T2) -> Self {
        Self::new_with_alpha(g.as_color_(), a.as_color_())
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

/// Color as Red, Green, Blue
#[derive(Debug,Default,Copy,Clone,PartialEq)]
pub struct Rgb<T> {
    pub r: T,
    pub g: T,
    pub b: T,
}

pub type Rgb8  = Rgb<u8>;
pub type Rgb16 = Rgb<u16>;
pub type Rgb32 = Rgb<f32>;
pub type Rgb64 = Rgb<f64>;

impl<T: ColorValueType> Rgb<T> {
    pub fn from_color<C: Color>(c: C) -> Self {
        Self::new(c.red(), c.green(), c.blue())
    }
    pub fn new(r: T, g: T, b: T) -> Self {
        Self { r, g, b }
    }
    pub fn from_slice(v: &[T]) -> Self {
        Self { r: v[0], g: v[1], b: v[2] }
    }
    pub fn into_slice(&self) -> [T;3] {
        [self.r, self.g, self.b]
    }
}

impl<T: ColorValueType> NamedColor for Rgb<T> {
    fn white() -> Self {
        Self::new(T::color_max(), T::color_max(), T::color_max())
    }
    fn black() -> Self {
        Self::new(T::color_min(), T::color_min(), T::color_min())
    }
    fn gray_color<T2: ColorValueType>(gray: T2, _: T2) -> Self {
        let gray = gray.as_color_();
        Self::new(gray, gray, gray)
    }
}

impl Rgb8 {
    pub fn from_wavelength_gamma(w: f64, gamma: f64) -> Self {
        let (r,g,b) =
            if w >= 380.0 && w <= 440.0 {
                (-1.0 * (w-440.0) / (440.0-380.0), 0.0, 1.0)
            } else if w >= 440.0 && w <= 490.0 {
                (0.0, (w-440.0)/(490.0-440.0), 1.0)
            } else if w >= 490.0 && w <= 510.0 {
                (0.0, 1.0, -1.0 * (w-510.0)/(510.0-490.0))
            } else if w >= 510.0 && w <= 580.0 {
                ((w-510.0)/(580.0-510.0), 1.0, 0.0)
            } else if w >= 580.0 && w <= 645.0 {
                (1.0, -1.0*(w-645.0)/(645.0-580.0), 0.0)
            } else if w >= 645.0 && w <= 780.0 {
                (1.0, 0.0, 0.0)
            } else {
                (0.,0.,0.)
            };
        let scale =
            if w > 700.0 {
                0.3 + 0.7 * (780.0-w)/(780.0-700.0)
            } else if w < 420.0 {
                0.3 + 0.7 * (w-380.0)/(420.0-380.0)
            } else {
                1.0
            };
        let r = (r * scale).powf(gamma) * 255.0;
        let g = (g * scale).powf(gamma) * 255.0;
        let b = (b * scale).powf(gamma) * 255.0;
        Self::new ( r as u8, g as u8, b as u8 )
    }
}


/// Color as Red, Green, Blue, and Alpha with pre-multiplied components
#[derive(Debug,Default,Copy,Clone,PartialEq)]
pub struct RgbaPre<T> {
    pub r: T,
    pub g: T,
    pub b: T,
    pub a: T,
}

pub type RgbaPre8  = RgbaPre<u8>;
pub type RgbaPre16 = RgbaPre<u16>;
pub type RgbaPre32 = RgbaPre<f32>;
pub type RgbaPre64 = RgbaPre<f64>;

impl<T: ColorValueType> RgbaPre<T> {
    pub fn from_color<C: Color>(color: C) -> Self {
        Self {r: color.red(), g: color.green(), b: color.blue(), a: color.alpha() }
    }
    pub fn new(r: T, g: T, b: T, a: T) -> Self {
        Self {r, g, b, a}
    }
    pub fn into_slice(&self) -> [T;4] {
        [self.r, self.g, self.b, self.a]
    }
}

impl<T: ColorValueType> NamedColor for RgbaPre<T> {
    fn white() -> Self {
        Rgba::white().premultiply()
    }
    fn black() -> Self {
        Rgba::black().premultiply()
    }
    fn gray_color<T2: ColorValueType>(gray: T2, alpha: T2) -> Self {
        Rgba::gray_color(gray, alpha).premultiply()
    }
}

/// Color as standard Red, Green, Blue, Alpha
///
/// See <https://en.wikipedia.org/wiki/SRGB>
///
#[derive(Debug,Default,Copy,Clone,PartialEq)]
pub struct Srgba<T> {
    /// Red
    r: T,
    /// Green
    g: T,
    /// Blue
    b: T,
    /// Alpha
    a: T,
}

pub type Srgba8  = Srgba<u8>;
pub type Srgba16 = Srgba<u16>;
pub type Srgba32 = Srgba<f32>;
pub type Srgba64 = Srgba<f64>;

impl<T: ColorValueType> Srgba<T> {
    pub fn from_color<C: Color>(c: C) -> Self {
        let r = c.red64().to_srgb_();
        let g = c.green64().to_srgb_();
        let b = c.blue64().to_srgb_();
        let a = c.alpha64().as_color_();
        Self::new(r,g,b,a)
    }
    /// Create a new Srgba8 color
    pub fn new(r: T, g: T, b: T, a: T) -> Self {
        Self { r, g, b, a }
    }
}

impl<T: ColorValueType> Color for Rgba<T> {
    fn red<T2: ColorValueType>(&self) -> T2 { self.r.as_color_() }
    fn green<T2: ColorValueType>(&self) -> T2 { self.g.as_color_() }
    fn blue<T2: ColorValueType>(&self) -> T2 { self.b.as_color_() }
    fn alpha<T2: ColorValueType>(&self) -> T2 { self.a.as_color_() }
    fn is_premultiplied(&self) -> bool { false }
}
impl<T: ColorValueType> Color for Rgb<T> {
    fn red<T2: ColorValueType>(&self) -> T2 { self.r.as_color_() }
    fn green<T2: ColorValueType>(&self) -> T2 { self.g.as_color_() }
    fn blue<T2: ColorValueType>(&self) -> T2 { self.b.as_color_() }
    fn alpha<T2: ColorValueType>(&self) -> T2 { T2::color_max() }
    fn is_premultiplied(&self) -> bool { false }
}
impl<T: ColorValueType> Color for RgbaPre<T> {
    fn red<T2: ColorValueType>(&self) -> T2 { self.r.as_color_() }
    fn green<T2: ColorValueType>(&self) -> T2 { self.g.as_color_() }
    fn blue<T2: ColorValueType>(&self) -> T2 { self.b.as_color_() }
    fn alpha<T2: ColorValueType>(&self) -> T2 { self.a.as_color_() }
    fn is_premultiplied(&self) -> bool { true }
}
impl<T: ColorValueType> Color for Srgba<T> {
    fn red<T2: ColorValueType>(&self) -> T2 { self.r.from_srgb_() }
    fn green<T2: ColorValueType>(&self) -> T2 { self.g.from_srgb_() }
    fn blue<T2: ColorValueType>(&self) -> T2 { self.b.from_srgb_() }
    fn alpha<T2: ColorValueType>(&self) -> T2 { self.a.as_color_() }
    fn is_premultiplied(&self) -> bool { false }
}
impl<T: ColorValueType> Color for Gray<T> {
    fn red<T2: ColorValueType>(&self) -> T2 { self.value.as_color_() }
    fn green<T2: ColorValueType>(&self) -> T2 { self.value.as_color_() }
    fn blue<T2: ColorValueType>(&self) -> T2 { self.value.as_color_() }
    fn alpha<T2: ColorValueType>(&self) -> T2 { self.alpha.as_color_() }
    fn is_premultiplied(&self) -> bool { false }
}

#[cfg(test)]
mod tests {
    use crate::{ColorValueType, NamedColor};

    use super::Gray8;
    use super::Rgb8;
    use super::Rgba8;
    use super::RgbaPre8;
    use super::Srgba8;

    #[test]
    fn value_type_test() {
        for i in 0..=255u8 {
            assert_eq!(i.as_color_::<u8>(), i);
        }
        for i in 0..=65535u16 {
            assert_eq!(i.as_color_::<u16>(), i);
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
            let c = Rgb8::new(*r,*g,*b);
            let gray = Gray8::from_color(c);
            assert_eq!(gray.value, *z);
        }
    }
    #[test]
    fn rgb8_test() {
        let w = Rgb8::white();
        assert_eq!(w, Rgb8{r: 255, g:255, b: 255});
        let w = Rgb8::black();
        assert_eq!(w, Rgb8{r: 0, g:0, b: 0});
        let w = Rgb8::gray_color(128u8, 255);
        assert_eq!(w, Rgb8{r: 128, g:128, b: 128});
        let w = Rgb8::from_slice(&[1,2,3]);
        assert_eq!(w, Rgb8{r: 1, g:2, b: 3});
        let w = Rgb8::new(0, 90, 180);
        assert_eq!(w, Rgb8{r: 0, g:90, b: 180});
    }
    #[test]
    fn gray_test() {
        let g = Gray8::new(34);
        assert_eq!(g, Gray8{ value: 34, alpha: 255 });
        let g = Gray8::new_with_alpha(134, 100);
        assert_eq!(g, Gray8{ value: 134, alpha: 100 });
        let g = Gray8::from_slice(&[10,20]);
        assert_eq!(g, Gray8{ value: 10, alpha: 20 });
    }
    #[test]
    fn rgba8_test() {
        let c = Rgba8::white();
        assert_eq!(c, Rgba8{r:255,g:255,b:255,a:255});
        let c = Rgba8::black();
        assert_eq!(c, Rgba8{r:0,g:0,b:0,a:255});
        let c = Rgba8::new(255, 90, 84, 72);
        assert_eq!(c, Rgba8{r:255,g:90,b:84,a:72});
        let mut c = c;
        c.clear();
        assert_eq!(c, Rgba8{r:0,g:0,b:0,a:0});
        let c = Rgba8::new(255,255,255,128);
        let p = c.premultiply();
        assert_eq!(p, RgbaPre8 { r: 128, g: 128, b: 128, a: 128 } )
    }
    #[test]
    fn srgb_test() {
        let s = Srgba8::new(50,150,250,128);
        assert_eq!(s, Srgba8{r:50,g:150,b:250,a:128});
        let t = Rgba8::from_color(s);
        assert_eq!(t, Rgba8{r:8,g:78,b:244,a:128});
    }
}
