use crate::geometric::Sphere;

/// Defines a scene to render.
pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub fov: f64,
    pub sphere: Sphere,
}

impl Default for Scene {
    fn default() -> Self {
        Self {
            width: 800,
            height: 600,
            fov: 90.0,
            sphere: Sphere::default(),
        }
    }
}
