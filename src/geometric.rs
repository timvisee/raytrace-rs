use std::path::Path;

use crate::algebra::{Identity, Vector};
use crate::material::Material;
use crate::math::{Intersectable, Ray};

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
            Entity::Model(ref _m) => Material::default(),
        }
    }

    /// Load any external resources.
    pub fn load(&mut self) {
        match self {
            Entity::Sphere(_) => {}
            Entity::Plane(_) => {}
            Entity::Model(ref mut m) => m.load(),
        }
    }
}

impl Intersectable for Entity {
    fn intersect(&self, ray: &Ray) -> Option<f64> {
        match self {
            Entity::Sphere(ref s) => s.intersect(ray),
            Entity::Plane(ref p) => p.intersect(ray),
            Entity::Model(ref m) => m.intersect(ray),
        }
    }

    fn surface_normal(&self, point: Vector) -> Vector {
        match self {
            Entity::Sphere(ref s) => s.surface_normal(point),
            Entity::Plane(ref p) => p.surface_normal(point),
            Entity::Model(ref m) => m.surface_normal(point),
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
    fn intersect(&self, ray: &Ray) -> Option<f64> {
        const EPSILON: f64 = 0.000_000_1;

        let v0: Vector = self.positions[0];
        let v1: Vector = self.positions[1];
        let v2: Vector = self.positions[2];
        // compute plane's normal
        let v0v1 = v1 - v0;
        let v0v2 = v2 - v0;
        // no need to normalize
        let N = v0v1.cross(v0v2); // N
        let denom = N.dot(N);

        // Step 1: finding P

        // check if ray and plane are parallel ?
        let NdotRayDirection = N.dot(ray.direction);
        if NdotRayDirection.abs() < EPSILON {
            // almost 0
            return None; // they are parallel so they don't intersect !
        }

        // compute d parameter using equation 2
        let d = N.dot(v0);

        // compute t (equation 3)
        let t = (N.dot(ray.origin) + d) / NdotRayDirection;
        // check if the triangle is in behind the ray
        if t < 0.0 {
            return None; // the triangle is behind
        }

        // compute the intersection point using equation 1
        let P = ray.origin + ray.direction * t;

        // Step 2: inside-outside test
        let mut C; // vector perpendicular to triangle's plane

        // edge 0
        let edge0 = v1 - v0;
        let vp0 = P - v0;
        C = edge0.cross(vp0);
        if N.dot(C) < 0.0 {
            return None; // P is on the right side
        }

        // edge 1
        let edge1 = v2 - v1;
        let vp1 = P - v1;
        C = edge1.cross(vp1);
        let mut u = N.dot(C);
        if u < 0.0 {
            return None; // P is on the right side
        }

        // edge 2
        let edge2 = v0 - v2;
        let vp2 = P - v2;
        C = edge2.cross(vp2);
        let mut v = N.dot(C);
        if v < 0.0 {
            return None; // P is on the right side;
        }

        u /= denom;
        v /= denom;

        return Some(t); // this ray hits the triangle
    }

    fn surface_normal(&self, _: Vector) -> Vector {
        // TODO: implement this!
        Vector::identity()
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
    fn intersect(&self, ray: &Ray) -> Option<f64> {
        self.triangles
            .iter()
            .filter_map(|t| t.intersect(ray))
            .min_by(|i1, i2| i1.partial_cmp(&i2).unwrap())
    }

    fn surface_normal(&self, _: Vector) -> Vector {
        // TODO: implement this!
        Vector::identity()
    }
}

/// A model.
#[derive(Clone, Debug, Deserialize)]
pub struct Model {
    // TODO: replace this with mesh loaded from file path
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
    pub fn load(&mut self) {
        match Mesh::load_obj(&self.path, self.position) {
            Ok(meshes) => self.meshes = meshes,
            Err(err) => {
                eprintln!("Failed to load model: {}", err);
            }
        }
    }
}

impl Intersectable for Model {
    fn intersect(&self, ray: &Ray) -> Option<f64> {
        self.meshes
            .iter()
            .filter_map(|t| t.intersect(ray))
            .min_by(|i1, i2| i1.partial_cmp(&i2).unwrap())
    }

    fn surface_normal(&self, point: Vector) -> Vector {
        // TODO: implement this!
        Vector::identity()
    }
}
