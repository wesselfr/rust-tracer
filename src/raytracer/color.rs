use std::ops::*;

pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Color {
        Color { r, g, b }
    }
    pub fn to_u32(&self) -> u32 {
        let mut out: u32 = 0;
        out += ((self.r * 255.0) as u32) << 16;
        out += ((self.g * 255.0) as u32) << 8;
        out += (self.b * 255.0) as u32;
        out
    }
}

impl Add for Color {
    type Output = Self;
    fn add(self, other: Color) -> Self {
        Color {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
        }
    }
}

impl Mul<f32> for Color {
    type Output = Self;
    fn mul(self, other: f32) -> Self {
        Color {
            r: self.r * other,
            g: self.g * other,
            b: self.b * other,
        }
    }
}
