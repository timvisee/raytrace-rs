use crate::color::Color;
use crate::geometric::{Entity, Plane, Sphere};
use crate::math::{Intersectable, Intersection, Point3, Ray, Vector3};

/// Defines a scene to render.
pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub fov: f64,
    pub entities: Vec<Entity>,
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
                Entity::Sphere(Sphere::default()),
                Entity::Sphere(Sphere {
                    center: Point3::new(1.5, 0.1, -3.0),
                    radius: 1.0,
                    color: Color::new(1.0, 0.0, 0.4),
                }),
                Entity::Sphere(Sphere {
                    center: Point3::new(-3.0, -1.5, -8.0),
                    radius: 2.0,
                    color: Color::new(0.4, 1.0, 0.4),
                }),
                Entity::Plane(Plane {
                    center: Point3::new(0.0, -3.0, 0.0),
                    normal: Vector3::new(0.0, -1.0, -0.1),
                    color: Color::new(0.2, 0.2, 0.2),
                }),
            ],
        }
    }
}
