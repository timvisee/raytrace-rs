use crate::color::Color;
use crate::geometric::{Entity, Plane, Sphere};
use crate::light::{DirectionalLight, Light, SphericalLight};
use crate::math::{Intersectable, Intersection, Point3, Ray, Vector3};

/// Defines a scene to render.
pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub fov: f64,
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

impl Default for Scene {
    fn default() -> Self {
        Self {
            width: 1920,
            height: 1080,
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
                // Entity::Sphere(Sphere {
                //     center: Point3::new(0.0, 0.0, -5.0),
                //     radius: 1.0,
                //     color: Color::new(1.0, 0.4, 0.0),
                // }),
                // Entity::Sphere(Sphere {
                //     center: Point3::new(-3.0, 1.0, -6.0),
                //     radius: 1.0,
                //     color: Color::new(1.0, 0.0, 0.4),
                // }),
                // Entity::Sphere(Sphere {
                //     center: Point3::new(2.0, 1.0, -4.0),
                //     radius: 1.5,
                //     color: Color::new(0.4, 1.0, 0.4),
                // }),
                Entity::Plane(Plane {
                    center: Point3::new(0.0, -2.5, 0.0),
                    normal: Vector3::new(0.0, -1.0, 0.0),
                    color: Color::new(0.2, 0.2, 0.2),
                }),
                Entity::Sphere(Sphere {
                    center: Point3::new(-3.0, -0.5, 2.5),
                    radius: 2.0,
                    color: Color::new(1.0, 1.0, 1.0),
                }),
                Entity::Sphere(Sphere {
                    center: Point3::new(3.0, 1.8, 6.0),
                    radius: 4.0,
                    color: Color::new(0.0, 0.0, 1.0),
                }),
            ],
            lights: vec![
                Light::Directional(DirectionalLight {
                    direction: Vector3::new(-0.4, -1.0, -0.3),
                    color: Color::new(1.0, 1.0, 1.0),
                    intensity: 10.0,
                }),
                Light::Directional(DirectionalLight {
                    direction: Vector3::new(0.5, -1.0, -0.3),
                    color: Color::new(1.0, 0.0, 1.0),
                    intensity: 10.0,
                }),
                // Light::Spherical(SphericalLight {
                //     position: Point3::new(-1.0, -1.5, -3.0),
                //     color: Color::new(0.0, 1.0, 1.0),
                //     intensity: 800.0,
                // }),
                // Light::Spherical(SphericalLight {
                //     position: Point3::new(2.5, 0.5, -1.0),
                //     color: Color::new(1.0, 0.5, 0.0),
                //     intensity: 1000.0,
                // }),
            ],
        }
    }
}
