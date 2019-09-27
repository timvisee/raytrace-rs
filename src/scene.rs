use crate::geometric::Entity;
use crate::light::Light;
use crate::math::{Intersectable, Intersection, Ray};

/// Defines a scene to render.
#[derive(Clone, Debug, Deserialize)]
pub struct Scene {
    /// The shadow/reflect/transform bias length.
    #[serde(default = "default_bias")]
    pub bias: f64,

    /// Maximum ray recursion depth.
    #[serde(default = "default_ray_depth")]
    pub depth: u32,

    /// Scene camera configuration.
    pub camera: Camera,

    /// Entities in this scene.
    pub entities: Vec<Entity>,

    /// Lights in this scene.
    pub lights: Vec<Light>,
}

impl Scene {
    /// Cast a ray in the scene, and get the first intersection.
    pub fn intersect(&self, ray: &Ray) -> Option<Intersection> {
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
    /// The screen width in pixels.
    pub width: u32,

    /// The screen height in pixels.
    pub height: u32,

    /// The camera field of view in degrees.
    #[serde(default = "default_fov")]
    pub fov: f64,
}

impl Camera {
    /// The the total number of pixels this camera covers.
    pub fn pixels(&self) -> u32 {
        self.width * self.height
    }
}

/// The maximum depth/recursion for casted rays.
///
/// Helper function for serde defaults.
fn default_ray_depth() -> u32 {
    16
}

/// The default shadow/reflect/transform bias length.
///
/// Helper function for serde defaults.
fn default_bias() -> f64 {
    1e-13
}

/// The default FOV for the camera.
///
/// Helper function for serde defaults.
fn default_fov() -> f64 {
    90.0
}
