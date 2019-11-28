use std::path::Path;

use crate::algebra::Vector;
use crate::material::Material;
use crate::math::{Intersectable, Ray};

#[derive(Copy, Clone, Debug, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Entity {
    /// A plane entity.
    Plane(Plane),

    /// A spherical entity.
    Sphere(Sphere),
}

impl Entity {
    // TODO: use a trait for this
    pub fn material(&self) -> Material {
        match self {
            Entity::Sphere(ref s) => s.material,
            Entity::Plane(ref p) => p.material,
        }
    }
}

impl Intersectable for Entity {
    fn intersect(&self, ray: &Ray) -> Option<f64> {
        match self {
            Entity::Sphere(ref s) => s.intersect(ray),
            Entity::Plane(ref p) => p.intersect(ray),
        }
    }

    fn surface_normal(&self, point: Vector) -> Vector {
        match self {
            Entity::Sphere(ref s) => s.surface_normal(point),
            Entity::Plane(ref p) => p.surface_normal(point),
        }
    }
}

/// A geometric shape, an infinite plane.
#[derive(Copy, Clone, Debug, Deserialize)]
pub struct Plane {
    /// Plane center in world space.
    pub center: Vector,

    /// Plane normal.
    pub normal: Vector,

    /// Plane material.
    pub material: Material,
}

impl Intersectable for Plane {
    fn intersect(&self, ray: &Ray) -> Option<f64> {
        let normal = self.normal;
        let denom = normal.dot(ray.direction);
        if denom > 1e-6 {
            let v = self.center - ray.origin;
            let distance = v.dot(normal) / denom;
            if distance >= 0.0 {
                return Some(distance);
            }
        }
        None
    }

    fn surface_normal(&self, _: Vector) -> Vector {
        -self.normal
    }
}

/// A geometric shape, a sphere.
#[derive(Copy, Clone, Debug, Deserialize)]
pub struct Sphere {
    /// Sphere center in world space.
    pub center: Vector,

    /// Sphere radius.
    #[serde(default = "one")]
    pub radius: f64,

    /// Sphere material.
    pub material: Material,
}

impl Intersectable for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<f64> {
        let l: Vector = self.center - ray.origin;
        let adj = l.dot(ray.direction);
        let d2 = l.dot(l) - (adj * adj);
        let radius2 = self.radius * self.radius;
        if d2 > radius2 {
            return None;
        }
        let thc = (radius2 - d2).sqrt();
        let t0 = adj - thc;
        let t1 = adj + thc;

        if t0 < 0.0 && t1 < 0.0 {
            None
        } else if t0 < 0.0 {
            Some(t1)
        } else if t1 < 0.0 {
            Some(t0)
        } else {
            let distance = if t0 < t1 { t0 } else { t1 };
            Some(distance)
        }
    }

    fn surface_normal(&self, point: Vector) -> Vector {
        (point - self.center).normalize()
    }
}

/// Returns one.
///
/// Helper function for serde defaults.
const fn one() -> f64 {
    1.0
}

/// Represents a triangle.
pub struct Triangle {
    positions: [Vector; 3],
    normals: [Vector; 3],
    // texcoords: [Point; 3],
}

impl Triangle {
    /// Constructor.
    pub fn new(positions: [Vector; 3], normals: [Vector; 3]) -> Self {
        Self { positions, normals }
    }
}

pub struct Mesh {
    triangles: Vec<Triangle>,
}

impl Mesh {
    pub fn new(positions: Vec<Vector>, normals: Vec<Vector>, indices: Vec<u32>) -> Self {
        let triangles = indices
            .chunks(3)
            .map(|i| {
                Triangle::new(
                    [
                        positions[i[0] as usize],
                        positions[i[1] as usize],
                        positions[i[2] as usize],
                    ],
                    [
                        normals[i[0] as usize],
                        normals[i[1] as usize],
                        normals[i[2] as usize],
                    ],
                )
            })
            .collect();
        Self { triangles }
    }

    pub fn load_obj(path: &Path) -> Result<Vec<Mesh>, &str> {
        // Load the obj file
        let models = match tobj::load_obj(path) {
            Ok((models, _)) => models,
            Err(err) => return Err("failed to load obj file"),
        };

        Ok(models
            .into_iter()
            .map(|m| {
                println!("Loading model {}...", m.name);
                let mesh = m.mesh;

                println!("{} has {} triangles", m.name, mesh.indices.len() / 3);
                let positions = mesh
                    .positions
                    .chunks(3)
                    .map(|i| Vector(i[0] as f64, i[1] as f64, i[2] as f64))
                    .collect();
                let normals = mesh
                    .normals
                    .chunks(3)
                    .map(|i| Vector(i[0] as f64, i[1] as f64, i[2] as f64))
                    .collect();
                // let texcoords = mesh
                //     .texcoords
                //     .chunks(2)
                //     .map(|i| Point::new(i[0], i[1]))
                //     .collect();
                Mesh::new(positions, normals, mesh.indices)
            })
            .collect())
    }
}
