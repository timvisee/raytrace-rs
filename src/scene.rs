use crate::geometric::Sphere;
use crate::math::{Intersectable, Intersection, Ray};

use crate::color::Color;
use crate::math::Point3;

/// Defines a scene to render.
pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub fov: f64,
    pub entities: Vec<Sphere>,
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

impl Default for Scene {
    fn default() -> Self {
        Self {
            width: 800,
            height: 600,
            fov: 90.0,
            entities: vec![
                Sphere::default(),
                Sphere {
                    center: Point3::new(1.5, 0.1, -3.0),
                    radius: 1.0,
                    color: Color::new(255, 0, 100),
                },
                Sphere {
                    center: Point3::new(-3.0, -1.5, -8.0),
                    radius: 2.0,
                    color: Color::new(100, 255, 100),
                },
            ],
        }
    }
}
