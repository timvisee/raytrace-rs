use std::path::Path;

use crate::algebra::{Identity, Vector};
use crate::material::Material;
use crate::math::{Intersectable, Ray};

// TODO: use bias from scene?
const EPSILON: f64 = 1e-6;

#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Entity {
    /// A plane entity.
    Plane(Plane),

    /// A spherical entity.
    Sphere(Sphere),

    /// A model entity.
    Model(Model),
}

impl Entity {
    // TODO: use a trait for this
    pub fn material(&self) -> Material {
        match self {
            Entity::Sphere(ref s) => s.material,
            Entity::Plane(ref p) => p.material,
            Entity::Model(ref m) => m.material,
        }
    }

    /// Load any external resources.
    pub fn load<P: AsRef<Path>>(&mut self, workdir: P) {
        match self {
            Entity::Sphere(_) => {}
            Entity::Plane(_) => {}
            Entity::Model(ref mut m) => m.load(workdir),
        }
    }
}

impl Intersectable for Entity {
    fn intersect(&self, ray: &Ray) -> Option<(f64, Vector)> {
        match self {
            Entity::Sphere(ref s) => s.intersect(ray),
            Entity::Plane(ref p) => p.intersect(ray),
            Entity::Model(ref m) => m.intersect(ray),
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

impl Plane {
    /// Get intersection distance form ray origin.
    fn intersect_distance(&self, ray: &Ray) -> Option<f64> {
        let normal = self.normal;
        let denom = normal.dot(ray.direction);
        // TODO: use scene bias here?
        if denom > EPSILON {
            let v = self.center - ray.origin;
            let distance = v.dot(normal) / denom;
            if distance >= 0.0 {
                return Some(distance);
            }
        }
        None
    }
}

impl Intersectable for Plane {
    fn intersect(&self, ray: &Ray) -> Option<(f64, Vector)> {
        self.intersect_distance(ray).map(|d| (d, -self.normal))
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

impl Sphere {
    /// Get intersection distance form ray origin.
    fn intersect_distance(&self, ray: &Ray) -> Option<f64> {
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
}

impl Intersectable for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<(f64, Vector)> {
        self.intersect_distance(ray).map(|d| {
            let point = ray.origin + ray.direction * d;
            (d, (point - self.center).normalize())
        })
    }
}

/// Returns one.
///
/// Helper function for serde defaults.
const fn one() -> f64 {
    1.0
}

/// Represents a triangle.
#[derive(Clone, Debug, Deserialize)]
pub struct Triangle {
    positions: [Vector; 3],
    normals: Option<[Vector; 3]>,
    // texcoords: [Point; 3],
}

impl Triangle {
    /// Constructor.
    pub fn new(positions: [Vector; 3], normals: Option<[Vector; 3]>) -> Self {
        Self { positions, normals }
    }
}

impl Intersectable for Triangle {
    fn intersect(&self, ray: &Ray) -> Option<(f64, Vector)> {
        // Intersection check with Möller–Trumbore algorithm
        let v0 = self.positions[0];
        let v1 = self.positions[1];
        let v2 = self.positions[2];
        let edge1 = v1 - v0;
        let edge2 = v2 - v0;
        let h = ray.direction.cross(edge2);
        let a = edge1.dot(h);

        // No intersection if ray is parallel to triangle face
        if a.abs() < EPSILON {
            return None;
        }
        let f = 1.0 / a;
        let s = ray.origin - v0;
        let u = f * s.dot(h);
        if u < 0.0 || u > 1.0 {
            return None;
        }
        let q = s.cross(edge1);
        let v = f * ray.direction.dot(q);
        if v < 0.0 || u + v > 1.0 {
            return None;
        }
        let t = f * edge2.dot(q);

        // Ray intersection
        // This means that there is a line intersection but not a ray intersection.
        if t <= EPSILON || t >= 1.0 / EPSILON {
            return None;
        }

        // Calcualte the normal
        let normal = match &self.normals {
            // Interpolate vertex normals for smooth Gouraud normal
            Some(normals) => normals[0] * (1.0 - u - v) + normals[1] * u + normals[2] * v,

            // Calculate face normal
            None => {
                let v0: Vector = self.positions[0];
                let v1: Vector = self.positions[1];
                let v2: Vector = self.positions[2];
                (v1 - v0).cross(v2 - v0).normalize()
            }
        };

        Some((t, normal))
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct Mesh {
    triangles: Vec<Triangle>,
}

impl Mesh {
    pub fn new(positions: Vec<Vector>, normals: Vec<Vector>, indices: Vec<u32>) -> Self {
        let triangles = indices
            .chunks(3)
            .map(|i| {
                let positions = [
                    positions[i[0] as usize],
                    positions[i[1] as usize],
                    positions[i[2] as usize],
                ];
                let normals = if !normals.is_empty() {
                    Some([
                        normals[i[0] as usize],
                        normals[i[1] as usize],
                        normals[i[2] as usize],
                    ])
                } else {
                    None
                };
                Triangle::new(positions, normals)
            })
            .collect();

        Self { triangles }
    }

    /// Load a mesh from an .obj file at the given path.
    pub fn load_obj<'a, P: AsRef<Path>>(path: P, offset: Vector) -> Result<Vec<Mesh>, String> {
        // Load the obj file
        let models = match tobj::load_obj(path.as_ref()) {
            Ok((models, _)) => models,
            Err(err) => return Err(format!("Failed to load obj file: {}", err)),
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
                    .map(|p| Vector(p[0] as f64, p[1] as f64, p[2] as f64) + offset)
                    .collect();
                let normals = mesh
                    .normals
                    .chunks(3)
                    .map(|p| Vector(p[0] as f64, p[1] as f64, p[2] as f64))
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

impl Intersectable for Mesh {
    fn intersect(&self, ray: &Ray) -> Option<(f64, Vector)> {
        self.triangles
            .iter()
            .filter_map(|t| t.intersect(ray))
            .min_by(|i1, i2| i1.0.partial_cmp(&i2.0).unwrap())
    }
}

/// A model.
#[derive(Clone, Debug, Deserialize)]
pub struct Model {
    /// Path to the model file to load.
    pub path: String,

    /// Position of the model in world space.
    #[serde(default = "Vector::identity")]
    pub position: Vector,

    /// Model mesh.
    #[serde(default)]
    pub meshes: Vec<Mesh>,

    /// Model material.
    pub material: Material,
}

impl Model {
    /// Load any external resources.
    pub fn load<P: AsRef<Path>>(&mut self, workdir: P) {
        // Determine absolute path for relative model paths
        let mut path = workdir.as_ref().to_path_buf();
        path.push(&self.path);

        match Mesh::load_obj(&path, self.position) {
            Ok(meshes) => self.meshes = meshes,
            Err(err) => {
                eprintln!("Failed to load model, ignoring: {}", err);
            }
        }
    }
}

impl Intersectable for Model {
    fn intersect(&self, ray: &Ray) -> Option<(f64, Vector)> {
        self.meshes
            .iter()
            .filter_map(|t| t.intersect(ray))
            .min_by(|i1, i2| i1.0.partial_cmp(&i2.0).unwrap())
    }
}
