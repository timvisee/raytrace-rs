use std::f32::consts::PI;
use std::f64::INFINITY;

use crate::color::Color;
use crate::math::{Point3, Vector3};

#[derive(Copy, Clone, Debug, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Light {
    /// A directional light.
    Directional(DirectionalLight),

    /// A spherical point light.
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
        match self {
            Self::Directional(ref d) => -d.direction,
            Self::Spherical(ref s) => (s.position - hit_point).normalize(),
        }
    }

    pub fn intensity(&self, hit_point: &Point3) -> f32 {
        match self {
            Self::Directional(ref d) => d.intensity,
            Self::Spherical(ref s) => {
                let r2 = (s.position - hit_point).norm() as f32;
                s.intensity / (4.0 * PI * r2)
            }
        }
    }

    pub fn distance(&self, hit_point: &Point3) -> f64 {
        match self {
            Self::Directional(_) => INFINITY,
            // TODO: is norm here correct, use a unit test for testing this
            Self::Spherical(ref s) => (s.position - hit_point).norm(),
        }
    }
}

/// A directional light.
#[derive(Copy, Clone, Debug, Deserialize)]
pub struct DirectionalLight {
    pub direction: Vector3,
    pub color: Color,
    pub intensity: f32,
}

/// A spherical point light.
#[derive(Copy, Clone, Debug, Deserialize)]
pub struct SphericalLight {
    pub position: Point3,
    pub color: Color,
    pub intensity: f32,
}
