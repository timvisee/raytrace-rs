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
        //// TODO: cleanup
        //let pa = &self.positions[self.a];
        //let pb = &self.positions[self.b];
        //let pc = &self.positions[self.c];
        //let na = &self.normals[self.a];
        //let nb = &self.normals[self.b];
        //let nc = &self.normals[self.c];
        //// let ta = &self.texcoords[self.a];
        //// let tb = &self.texcoords[self.b];
        //// let tc = &self.texcoords[self.c];

        //let e = [*pb - *pa, *pc - *pa];
        //let mut s = [Vector::broadcast(0.0); 2];
        //s[0] = ray.direction.cross(e[1]);
        //let div = match s[0].dot(e[0]) {
        //    // 0.0 => degenerate triangle, can't hit
        //    d if d == 0.0 => return None,
        //    d => 1.0 / d,
        //};

        //let d = ray.o - *pa;
        //let mut bary = [0.0; 3];
        //bary[1] = d.dot(&s[0]) * div;
        //// Check that the first barycentric coordinate is in the triangle bounds
        //if bary[1] < 0.0 || bary[1] > 1.0 {
        //    return None;
        //}

        //s[1] = &d.cross(&e[0]);
        //bary[2] = &ray.direction.dot(&s[1]) * div;
        //// Check the second barycentric coordinate is in the triangle bounds
        //if bary[2] < 0.0 || bary[1] + bary[2] > 1.0 {
        //    return None;
        //}

        //// We've hit the triangle with the ray, now check the hit location is in the ray range
        //let t = &e[1].dot(&s[1]) * div;
        //// if t < ray.min_t || t > ray.max_t {
        ////     return None;
        //// }
        //bary[0] = 1.0 - bary[1] - bary[2];
        //ray.max_t = t;
        //let p = ray.at(t);

        //// Now compute normal at this location on the triangle
        //let n = (bary[0] * *na + bary[1] * *nb + bary[2] * *nc).normalized();

        //// Compute parameterization of surface and various derivatives for texturing
        //// Triangles are parameterized by the obj texcoords at the vertices
        //let texcoord = bary[0] * *ta + bary[1] * *tb + bary[2] * *tc;

        //// Triangle points can be found by p_i = p_0 + u_i dp/du + v_i dp/dv
        //// we use this property to find the derivatives dp/du and dp/dv
        //let du = [ta.x - tc.x, tb.x - tc.x];
        //let dv = [ta.y - tc.y, tb.y - tc.y];
        //let det = du[0] * dv[1] - dv[0] * du[1];
        ////If the texcoords are degenerate pick arbitrary coordinate system
        //let (dp_du, dp_dv) = if det == 0.0 {
        //    linalg::coordinate_system(&linalg::cross(&e[1], &e[0]).normalized())
        //} else {
        //    let det = 1.0 / det;
        //    let dp = [*pa - *pc, *pb - *pc];
        //    let dp_du = (dv[1] * dp[0] - dv[0] * dp[1]) * det;
        //    let dp_dv = (-du[1] * dp[0] + du[0] * dp[1]) * det;
        //    (dp_du, dp_dv)
        //};
        //Some(DifferentialGeometry::with_normal(
        //    &p, &n, texcoord.x, texcoord.y, ray.time, &dp_du, &dp_dv, geom,
        //))

        // ------------------------

        // const EPSILON: f64 = 0.000_000_1;

        // let vertex0: Vector = self.positions[0];
        // let vertex1: Vector = self.positions[1];
        // let vertex2: Vector = self.positions[2];
        // let edge1 = vertex1 - vertex0;
        // let edge2 = vertex2 - vertex0;
        // let h = ray.direction.cross(edge2);
        // let a = edge1.dot(h);

        // if a > -EPSILON && a < EPSILON {
        //     return None;
        // }

        // let f = 1.0 / a;
        // let s = ray.origin - vertex0;
        // let u = f * s.dot(h);
        // if u < 0.0 || u > 1.0 {
        //     return None;
        // }
        // let q = s.cross(edge1);
        // let v = f * ray.direction.dot(q);

        // if v < 0.0 || u + v > 1.0 {
        //     return None;
        // }

        // // At this stage we can compute t to find out where the intersection point is on the line.
        // let t = f * edge2.dot(q);
        // if t > EPSILON && t < 1.0 / EPSILON {
        //     // ray intersection
        //     // return Some(ray.origin + ray.direction * t);
        //     return Some(t);
        // // return true;
        // } else {
        //     // This means that there is a line intersection but not a ray intersection
        //     return None;
        // }

        // ------------------------

        // h.cross(rayVector, edge2);
        // a = edge1.dot(h);
        // if (a > -EPSILON && a < EPSILON) {
        //     return false;    // This ray is parallel to this triangle.
        // }
        // f = 1.0 / a;
        // s.sub(rayOrigin, vertex0);
        // u = f * (s.dot(h));
        // if (u < 0.0 || u > 1.0) {
        //     return false;
        // }
        // q.cross(s, edge1);
        // v = f * rayVector.dot(q);
        // if (v < 0.0 || u + v > 1.0) {
        //     return false;
        // }
        // // At this stage we can compute t to find out where the intersection point is on the line.
        // double t = f * edge2.dot(q);
        // if (t > EPSILON && t < 1/EPSILON) // ray intersection
        // {
        //     outIntersectionPoint.set(0.0, 0.0, 0.0);
        //     outIntersectionPoint.scaleAdd(t, rayVector, rayOrigin);
        //     return true;
        // } else // This means that there is a line intersection but not a ray intersection.
        // {
        //     return false;
        // }

        const EPSILON: f64 = 0.000_000_1;

        // let v0: Vector = self.positions[0];
        // let v1: Vector = self.positions[1];
        // let v2: Vector = self.positions[2];
        // // compute plane's normal
        // let v0v1 = v1 - v0;
        // let v0v2 = v2 - v0;
        // // no need to normalize
        // let N = v0v1.cross(v0v2); // N
        // let denom = N.dot(N);

        // // Step 1: finding P

        // // check if ray and plane are parallel ?
        // let NdotRayDirection = N.dot(ray.direction);
        // if NdotRayDirection.abs() < EPSILON {
        //     // almost 0
        //     return None; // they are parallel so they don't intersect !
        // }

        // // compute d parameter using equation 2
        // let d = N.dot(v0);

        // // compute t (equation 3)
        // let t = (N.dot(ray.origin) + d) / NdotRayDirection;
        // // check if the triangle is in behind the ray
        // if t < 0.0 {
        //     return None; // the triangle is behind
        // }

        // // compute the intersection point using equation 1
        // let P = ray.origin + ray.direction * t;

        // // Step 2: inside-outside test
        // let mut C; // vector perpendicular to triangle's plane

        // // edge 0
        // let edge0 = v1 - v0;
        // let vp0 = P - v0;
        // C = edge0.cross(vp0);
        // if N.dot(C) < 0.0 {
        //     return None; // P is on the right side
        // }

        // // edge 1
        // let edge1 = v2 - v1;
        // let vp1 = P - v1;
        // C = edge1.cross(vp1);
        // let mut u = N.dot(C);
        // if u < 0.0 {
        //     return None; // P is on the right side
        // }

        // // edge 2
        // let edge2 = v0 - v2;
        // let vp2 = P - v2;
        // C = edge2.cross(vp2);
        // let mut v = N.dot(C);
        // if v < 0.0 {
        //     return None; // P is on the right side;
        // }

        // u /= denom;
        // v /= denom;

        // return Some(t); // this ray hits the triangle

        let v0: Vector = self.positions[0];
        let v1: Vector = self.positions[1];
        let v2: Vector = self.positions[2];
        let dir = ray.direction;
        let orig = ray.origin;

        let v0v1 = v1 - v0;
        let v0v2 = v2 - v0;
        let pvec = dir.cross(v0v2);
        let det = v0v1.dot(pvec);

        // ray and triangle are parallel if det is close to 0
        if det.abs() < EPSILON {
            return None;
        }

        let invDet = 1.0 / det;

        let tvec = orig - v0;
        let u = tvec.dot(pvec) * invDet;
        if u < 0.0 || u > 1.0 {
            return None;
        }

        let qvec = tvec.cross(v0v1);
        let v = dir.dot(qvec) * invDet;
        if (v < 0.0 || u + v > 1.0) {
            return None;
        }

        let t = v0v2.dot(qvec) * invDet;

        return Some(t);
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
