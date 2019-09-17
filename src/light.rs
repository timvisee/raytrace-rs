use crate::color::Color;
use crate::math::Vector3;

pub struct Light {
    pub direction: Vector3,
    pub color: Color,
    pub intensity: f32,
}
