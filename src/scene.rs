use crate::geometric::Entity;
use crate::light::Light;
use crate::math::{Intersectable, Intersection, Ray};

/// Defines a scene to render.
#[derive(Clone, Debug, Deserialize)]
pub struct Scene {
    pub camera: Camera,
    pub entities: Vec<Entity>,
    pub lights: Vec<Light>,
}

impl Scene {
    pub fn trace(&self, ray: &Ray) -> Option<Intersection> {
        self.entities
            .iter()
            .filter_map(|s| {
                s.intersect(ray).map(|d| Intersection {
                    distance: d,
                    entity: s,
                })
            })
            .min_by(|i1, i2| i1.distance.partial_cmp(&i2.distance).unwrap())
    }
}

/// Scene camera configuration.
#[derive(Copy, Clone, Debug, Deserialize)]
pub struct Camera {
    pub width: u32,
    pub height: u32,
    pub fov: f64,
}

impl Camera {
    /// The the total number of pixels this camera covers.
    pub fn pixels(&self) -> u32 {
        self.width * self.height
    }
}
