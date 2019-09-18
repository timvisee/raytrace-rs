use crate::color::Color;
use crate::math::{Point3, Vector3};

pub enum Light {
    Directional(DirectionalLight),
    Spherical(SphericalLight),
}

impl Light {
    /// Get the light color.
    pub fn color(&self) -> Color {
        match self {
            Self::Directional(d) => d.color,
            Self::Spherical(s) => s.color,
        }
    }

    pub fn direction_from(&self, hit_point: &Point3) -> Vector3 {
        match *self {
            Self::Directional(ref d) => -d.direction,
            Self::Spherical(ref s) => (s.position - *hit_point).normalize(),
        }
    }

    pub fn intensity(&self, hit_point: &Point3) -> f32 {
        match *self {
            Self::Directional(ref d) => d.intensity,
            Self::Spherical(ref s) => {
                let r2 = (s.position - *hit_point).norm() as f32;
                s.intensity / (4.0 * ::std::f32::consts::PI * r2)
            }
        }
    }

    pub fn distance(&self, hit_point: &Point3) -> f64 {
        match *self {
            Self::Directional(_) => ::std::f64::INFINITY,
            // TODO: is norm here correct, use a unit test for testing this
            Self::Spherical(ref s) => (s.position - *hit_point).norm(),
        }
    }
}

pub struct DirectionalLight {
    pub direction: Vector3,
    pub color: Color,
    pub intensity: f32,
}

pub struct SphericalLight {
    pub position: Point3,
    pub color: Color,
    pub intensity: f32,
}
