use crate::color::Color;

#[derive(Copy, Clone, Debug, Builder)]
#[builder(default)]
pub struct Material {
    pub color: Color,
    pub albedo: f32,
}

impl Material {
    pub fn build() -> MaterialBuilder {
        MaterialBuilder::default()
    }
}

impl Default for Material {
    fn default() -> Self {
        Self {
            color: Color::new(1.0, 0.4, 0.0),

            // 0.18
            // 0.25
            albedo: 0.5,
        }
    }
}
