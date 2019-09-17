use crate::color::Color;
use crate::math::{Intersectable, Point3, Ray};

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
            color: Color::new(255, 100, 0),
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
}
