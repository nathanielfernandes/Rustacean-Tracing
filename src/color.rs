extern crate image;
use image::Rgb;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Copy, Clone, Debug)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Color {
        Color { r, g, b }
    }
    pub fn to_rgb(&self) -> Rgb<u8> {
        Rgb::from(self.to_slice())
    }

    pub fn to_slice(&self) -> [u8; 3] {
        [
            (255.0 * self.r).max(0.0).min(255.0) as u8,
            (255.0 * self.g).max(0.0).min(255.0) as u8,
            (255.0 * self.b).max(0.0).min(255.0) as u8,
        ]
    }

    pub fn to_vec_f32(&self) -> Vec<f32> {
        vec![
            (self.r).max(0.0).min(1.0) as f32,
            (self.g).max(0.0).min(1.0) as f32,
            (self.b).max(0.0).min(1.0) as f32,
        ]
    }

    pub fn to_vec_u8(&self) -> Vec<u8> {
        Vec::from(self.to_slice())
    }

    pub fn sqrt(&self) -> Color {
        Color {
            r: self.r.sqrt(),
            g: self.g.sqrt(),
            b: self.b.sqrt(),
        }
    }

    pub fn from_rgb(r: u8, g: u8, b: u8) -> Color {
        Color {
            r: r as f32 / 255.0,
            g: g as f32 / 255.0,
            b: b as f32 / 255.0,
        }
    }
}

pub const BLACK: Color = Color {
    r: 0.0,
    g: 0.0,
    b: 0.0,
};
pub const WHITE: Color = Color {
    r: 1.0,
    g: 1.0,
    b: 1.0,
};
pub const RED: Color = Color {
    r: 255.0,
    g: 0.0,
    b: 0.0,
};
pub const GREEN: Color = Color {
    r: 0.0,
    g: 1.0,
    b: 0.0,
};
pub const BLUE: Color = Color {
    r: 0.0,
    g: 0.0,
    b: 1.0,
};

impl Add for Color {
    type Output = Color;

    fn add(self, other: Color) -> Color {
        Color {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
        }
    }
}

impl Sub for Color {
    type Output = Color;

    fn sub(self, other: Color) -> Color {
        Color {
            r: self.r - other.r,
            g: self.g - other.g,
            b: self.b - other.b,
        }
    }
}

impl Mul for Color {
    type Output = Color;

    fn mul(self, other: Color) -> Color {
        Color {
            r: self.r * other.r,
            g: self.g * other.g,
            b: self.b * other.b,
        }
    }
}

impl Mul<f32> for Color {
    type Output = Color;

    fn mul(self, other: f32) -> Color {
        Color {
            r: self.r * other,
            g: self.g * other,
            b: self.b * other,
        }
    }
}

impl Mul<Color> for f32 {
    type Output = Color;

    fn mul(self, other: Color) -> Color {
        other * self
    }
}

impl Neg for Color {
    type Output = Color;

    fn neg(self) -> Color {
        Color {
            r: -self.r,
            g: -self.g,
            b: -self.b,
        }
    }
}

impl Div<f32> for Color {
    type Output = Color;

    fn div(self, other: f32) -> Color {
        Color {
            r: self.r / other,
            g: self.g / other,
            b: self.b / other,
        }
    }
}

impl Div for Color {
    type Output = Color;

    fn div(self, other: Color) -> Color {
        Color {
            r: self.r / other.r,
            g: self.g / other.g,
            b: self.b / other.b,
        }
    }
}

impl Div<Color> for f32 {
    type Output = Color;

    fn div(self, other: Color) -> Color {
        other / self
    }
}
