use image::Rgba;

/// The maximum value a color can have.
const COLOR_MAX: u16 = ::std::u8::MAX as u16;

/// An RGB color.
pub struct Color {
    r: u16,
    g: u16,
    b: u16,
}

impl Color {
    pub fn new(r: u16, g: u16, b: u16) -> Self {
        Self { r, g, b }
    }

    pub fn to_rgba(&self) -> Rgba<u8> {
        // TODO: do not convert between u8/u16 here
        Rgba([self.r as u8, self.g as u8, self.b as u8, 0])
    }
}

impl Default for Color {
    fn default() -> Self {
        Self {
            r: COLOR_MAX / 2,
            g: COLOR_MAX / 2,
            b: COLOR_MAX / 2,
        }
    }
}
