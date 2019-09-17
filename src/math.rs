use crate::scene::Scene;

// pub use nalgebra::base::Vector3;
// pub use nalgebra::geometry::Point3;

pub type Vector3 = ::nalgebra::base::Vector3<f64>;
pub type Point3 = ::nalgebra::geometry::Point3<f64>;

// /// A 3 dimentional point.
// #[derive(Copy, Clone, Debug, PartialEq)]
// pub struct Point(f64, f64, f64);

// impl Point {
//     /// Create a type at `(0, 0, 0)`.
//     pub fn zero() -> Self {
//         Self(0.0, 0.0, 0.0)
//     }

//     /// Normalize this point to have a distance of 1 from the zero point.
//     pub fn normalize(&mut self) {
//         // Determine the current vector length
//         let length = ((self.0 * self.0) + (self.1 * self.1) + (self.2 * self.2))
//             .sqrt()
//             .abs();

//         // Multiply to give the vector a length of 1
//         if length > 0.0 {
//             self.0 /= length;
//             self.1 /= length;
//             self.2 /= length;
//         } else {
//             self.0 = 0.0;
//             self.1 = 0.0;
//             self.2 = 0.0;
//         }
//     }
// }

// pub type Vector = Point;

/// A 3 dimentoinal ray.
pub struct Ray {
    pub origin: Point3,
    pub direction: Vector3,
}

impl Ray {
    /// Create a new ray originating from the screen/camera.
    pub fn new_screen(x: u32, y: u32, scene: &Scene) -> Self {
        // TODO: review these values
        // TODO: is this assert needed?
        assert!(scene.width > scene.height);
        let fov_adjustment = (scene.fov.to_radians() / 2.0).tan();
        let aspect_ratio = (scene.width as f64) / (scene.height as f64);
        let sensor_x =
            ((((x as f64 + 0.5) / scene.width as f64) * 2.0 - 1.0) * aspect_ratio) * fov_adjustment;
        let sensor_y = (1.0 - ((y as f64 + 0.5) / scene.height as f64) * 2.0) * fov_adjustment;

        // Construct the row
        Ray {
            origin: Point3::new(0.0, 0.0, 0.0),
            direction: Vector3::new(sensor_x, sensor_y, -1.0).normalize(),
        }
    }
}

pub trait Intersectable {
    fn intersect(&self, ray: &Ray) -> bool;
}

// TODO: is this obsolete? We're testing nalgebra internals at this point.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_normalize() {
        // // A zero point
        // let mut point = Vector3::new(0.0, 0.0, 0.0);
        // assert_eq!(point.normalize(), Vector3::new(NaN, NaN, NaN),);

        // A one point
        let mut point = Vector3::new(1.0, 1.0, 1.0);
        assert_eq!(
            point.normalize(),
            Vector3::new(0.5773502691896258, 0.5773502691896258, 0.5773502691896258)
        );

        // A non-stable point
        let mut point = Vector3::new(3.0, 1.0, 2.0);
        assert_eq!(
            point.normalize(),
            Vector3::new(0.8017837257372732, 0.2672612419124244, 0.5345224838248488),
        );
    }
}
