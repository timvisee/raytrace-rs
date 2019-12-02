use crate::algebra::{Identity, Vector};
use crate::geometric::Entity;
use crate::scene::Scene;

/// A 3 dimentoinal ray.
#[derive(Copy, Clone, Debug)]
pub struct Ray {
    pub origin: Vector,
    pub direction: Vector,
}

impl Ray {
    /// Create a new ray from the given `origin`, going into `direction`.
    pub fn new(origin: Vector, direction: Vector) -> Self {
        Self { origin, direction }
    }

    /// Create a prime ray from the given screen pixel positionray from the given screen pixel
    /// position.
    pub fn new_prime(x: u32, y: u32, scene: &Scene) -> Self {
        let camera = scene.camera;

        // TODO: review these values
        // TODO: is this assert needed?
        assert!(camera.width > camera.height);
        let fov_adjustment = (camera.fov.to_radians() / 2.0).tan();
        let aspect_ratio = f64::from(camera.width) / f64::from(camera.height);
        let sensor_x = (((f64::from(x) + 0.5) / f64::from(camera.width) * 2.0 - 1.0)
            * aspect_ratio)
            * fov_adjustment;
        let sensor_y =
            (1.0 - ((f64::from(y) + 0.5) / f64::from(camera.height)) * 2.0) * fov_adjustment;

        // Construct the row
        Self::new(
            Vector::identity(),
            Vector(sensor_x, sensor_y, -1.0).normalize(),
        )
    }

    /// Create a reflection ray.
    ///
    /// Used to create a propegating ray that is reflected on a surface.
    /// Information about the normal, incident ray and intersection point must be given.
    ///
    /// # Parameters
    ///
    /// - `normal`: surface normal at hit point of intersected entity.
    /// - `insident`: the ray incident direction.
    /// - `intersection`: the intersection point on the entity we hit.
    /// - `bias`: the reflection bias to mitigate float precision errors.
    pub fn create_reflection(
        normal: Vector,
        incident: Vector,
        intersection: Vector,
        bias: f64,
    ) -> Self {
        Self::new(
            intersection,
            incident - (normal * incident.dot(normal) * 2.0),
        )
        .bias(bias)
    }

    /// Create a transmission ray.
    ///
    /// Used to create a propegating ray that is refracted through a surface.
    /// Information about the normal, incident ray, intersection point and refractive index must be
    /// given.
    ///
    /// # Parameters
    ///
    /// - `normal`: surface normal at hit point of intersected entity.
    /// - `insident`: the ray incident direction.
    /// - `intersection`: the intersection point on the entity we hit.
    /// - `index`: the refractive index of the surface.
    /// - `bias`: the reflection bias to mitigate float precision errors.
    pub fn create_transmission(
        normal: Vector,
        incident: Vector,
        intersection: Vector,
        index: f32,
        bias: f64,
    ) -> Option<Self> {
        let mut ref_n = normal;
        let mut eta_t = f64::from(index);
        let mut eta_i = 1.0;
        let mut i_dot_n = incident.dot(normal);
        if i_dot_n < 0.0 {
            // Outside the surface
            i_dot_n = -i_dot_n;
        } else {
            // Inside the surface; invert the normal and swap the indices of refraction
            ref_n = -normal;
            eta_i = eta_t;
            eta_t = 1.0;
        }

        let eta = eta_i / eta_t;
        let k = 1.0 - (eta * eta) * (1.0 - i_dot_n * i_dot_n);
        if k < 0.0 {
            None
        } else {
            Some(Self::new(
                intersection + (ref_n * -bias),
                (ref_n * i_dot_n + incident) * eta - ref_n * k.sqrt(),
            ))
        }
    }

    /// Bias ray origin by given length.
    ///
    /// This moves the ray origin into the ray direction by the given `bias`.
    /// Used to mitigate flaot precision issues.
    pub fn bias(&self, bias: f64) -> Ray {
        let mut ray = *self;
        ray.origin += ray.direction * bias;
        ray
    }
}

/// Intersection with an entity.
///
/// This represents an intersection with `entity` from a ray.
#[derive(Copy, Clone, Debug)]
pub struct Intersection<'a> {
    /// Distance to the intersection point at `entity` from the ray origin.
    pub distance: f64,

    /// The hit point normal.
    pub normal: Vector,

    /// The entity that was intersected.
    pub entity: &'a Entity,
}

pub trait Intersectable {
    /// Check for ray intersection with this entity.
    ///
    /// This check whether the given `ray` intersects with this entity, and if there's an
    /// intersection the distance to the hit point from the ray origin along with the hit point
    /// surface normal is returned.
    fn intersect(&self, ray: &Ray) -> Option<(f64, Vector)>;
}
