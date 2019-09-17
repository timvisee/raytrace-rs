use crate::color::Color;
use crate::math::{Intersectable, Point3, Ray};

/// A geometric shape, a sphere.
pub struct Sphere {
    center: Point3,
    radius: f64,
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
    fn intersect(&self, ray: &Ray) -> bool {
        // Create a line segment between the ray origin and the center of the sphere
        let line = self.center - ray.origin;
        // Use l as a hypotenuse and find the length of the adjacent side
        let adj2 = line.dot(&ray.direction);
        // Find the length-squared of the opposite side
        // This is equivalent to (but faster than) (l.length() * l.length()) - (adj2 * adj2)
        let d2 = line.dot(&line) - (adj2 * adj2);
        // If that length-squared is less than radius squared, the ray intersects the sphere
        d2 < (self.radius * self.radius)
    }
}
