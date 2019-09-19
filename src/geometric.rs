use crate::color::Color;
use crate::math::{Intersectable, Point3, Ray, Vector3};

pub enum Entity {
    Plane(Plane),
    Sphere(Sphere),
}

impl Entity {
    // TODO: use a trait for this
    pub fn color(&self) -> &Color {
        match *self {
            Entity::Sphere(ref s) => &s.color,
            Entity::Plane(ref p) => &p.color,
        }
    }

    pub fn albedo(&self) -> f32 {
        // 0.18
        // 0.25
        0.5
    }
}

impl Intersectable for Entity {
    fn intersect(&self, ray: &Ray) -> Option<f64> {
        match *self {
            Entity::Sphere(ref s) => s.intersect(ray),
            Entity::Plane(ref p) => p.intersect(ray),
        }
    }

    fn surface_normal(&self, point: &Point3) -> Vector3 {
        match *self {
            Entity::Sphere(ref s) => s.surface_normal(point),
            Entity::Plane(ref p) => p.surface_normal(point),
        }
    }
}

/// A geometric shape, an infinite plane.
pub struct Plane {
    pub center: Point3,
    pub normal: Vector3,
    pub color: Color,
}

impl Intersectable for Plane {
    fn intersect(&self, ray: &Ray) -> Option<f64> {
        let normal = &self.normal;
        let denom = normal.dot(&ray.direction);
        if denom > 1e-6 {
            let v = self.center - ray.origin;
            let distance = v.dot(&normal) / denom;
            if distance >= 0.0 {
                return Some(distance);
            }
        }
        None
    }

    fn surface_normal(&self, _: &Point3) -> Vector3 {
        -self.normal
    }
}

/// A geometric shape, a sphere.
pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    pub color: Color,
}

impl Default for Sphere {
    fn default() -> Self {
        Self {
            center: Point3::new(0.0, 0.0, -5.0),
            radius: 1.0,
            color: Color::new(1.0, 0.4, 0.0),
        }
    }
}

impl Intersectable for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<f64> {
        let l = self.center - ray.origin;
        let adj = l.dot(&ray.direction);
        let d2 = l.dot(&l) - (adj * adj);
        let radius2 = self.radius * self.radius;
        if d2 > radius2 {
            return None;
        }
        let thc = (radius2 - d2).sqrt();
        let t0 = adj - thc;
        let t1 = adj + thc;

        if t0 < 0.0 && t1 < 0.0 {
            return None;
        }

        let distance = if t0 < t1 { t0 } else { t1 };
        Some(distance)
    }

    fn surface_normal(&self, point: &Point3) -> Vector3 {
        (*point - self.center).normalize()
    }
}
