use crate::geometric::Entity;
use crate::scene::Scene;

pub type Vector3 = nalgebra::base::Vector3<f64>;
pub type Point3 = nalgebra::geometry::Point3<f64>;

/// A 3 dimentoinal ray.
pub struct Ray {
    pub origin: Point3,
    pub direction: Vector3,
}

impl Ray {
    /// Create a new ray from the given `origin`, going into `direction`.
    pub fn new(origin: Point3, direction: Vector3) -> Self {
        Self { origin, direction }
    }

    /// Create a new ray originating from the prime/camera/screen.
    pub fn new_prime(x: u32, y: u32, scene: &Scene) -> Self {
        // TODO: review these values
        // TODO: is this assert needed?
        assert!(scene.width > scene.height);
        let fov_adjustment = (scene.fov.to_radians() / 2.0).tan();
        let aspect_ratio = (scene.width as f64) / (scene.height as f64);
        let sensor_x =
            ((((x as f64 + 0.5) / scene.width as f64) * 2.0 - 1.0) * aspect_ratio) * fov_adjustment;
        let sensor_y = (1.0 - ((y as f64 + 0.5) / scene.height as f64) * 2.0) * fov_adjustment;

        // Construct the row
        Self::new(
            Point3::new(0.0, 0.0, 0.0),
            Vector3::new(sensor_x, sensor_y, -1.0).normalize(),
        )
    }

    /// Reflect this ray, based on the given `normal` at the given hit `point`.
    pub fn reflect(&self, normal: Vector3, point: Point3) -> Self {
        let direction = self.direction - 2.0 * self.direction.dot(&normal) * normal;
        // TODO: assert if normalized direction is the same?
        Self::new(point, direction)
    }

    /// Bias the origin of this ray by the given amount.
    /// This moves the ray origin into the ray direction by the given `amount`.
    pub fn bias(&self, amount: f64) -> Ray {
        let mut ray = *self;
        ray.origin += ray.direction * amount;
        ray
    }
}

pub struct Intersection<'a> {
    pub distance: f64,
    pub entity: &'a Entity,
}

pub trait Intersectable {
    /// Check whether the given ray intersects with this entity in 3 dimentional space, and return
    /// the distance between the intersection and ray origin.
    fn intersect(&self, ray: &Ray) -> Option<f64>;

    fn surface_normal(&self, point: &Point3) -> Vector3;
}
