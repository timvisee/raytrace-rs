use std::ops::{Add, Mul};

use image::Rgba;

lazy_static! {
    pub static ref BLACK: Color = Color::new(0.0, 0.0, 0.0);
    pub static ref GRAY: Color = Color::new(0.5, 0.5, 0.5);
    pub static ref WHITE: Color = Color::new(1.0, 1.0, 1.0);
}

/// An RGB color.
#[derive(Copy, Clone, Debug, Deserialize)]
pub struct Color(f32, f32, f32);

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self(r, g, b)
    }

    pub fn clamp(&self) -> Color {
        Color::new(
            self.0.min(1.0).max(0.0),
            self.1.min(1.0).max(0.0),
            self.2.min(1.0).max(0.0),
        )
    }

    pub fn to_rgba(&self) -> Rgba<u8> {
        // TODO: do not convert between u8/u16 here
        let color = self.clamp();
        Rgba([
            (color.0 * 255.0) as u8,
            (color.1 * 255.0) as u8,
            (color.2 * 255.0) as u8,
            255,
        ])
    }
}

impl Mul for Color {
    type Output = Color;

    fn mul(self, other: Color) -> Color {
        Color::new(self.0 * other.0, self.1 * other.1, self.2 * other.2)
    }
}

impl Mul<f32> for Color {
    type Output = Color;

    fn mul(self, other: f32) -> Color {
        Color::new(self.0 * other, self.1 * other, self.2 * other)
    }
}

impl Mul<Color> for f32 {
    type Output = Color;
    fn mul(self, other: Color) -> Color {
        other * self
    }
}

impl Add for Color {
    type Output = Color;
    fn add(self, other: Color) -> Color {
        Color::new(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}
