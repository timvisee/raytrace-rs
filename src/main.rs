#[macro_use]
extern crate derive_builder;
#[macro_use]
extern crate lazy_static;

use std::f32::consts::PI;

use image::{DynamicImage, GenericImage, Rgba};
use rayon::prelude::*;

mod color;
mod geometric;
mod light;
mod material;
mod math;
mod scene;

use color::{Color, BLACK};
use geometric::Entity;
use material::Surface;
use math::*;
use scene::*;

/// The maximum depth/recursion for caste rays.
pub const RAY_RECURSION: u32 = 16;

/// The shadow/reflect/transform bias length.
// TODO: configure this per scene
pub const BIAS: f64 = 1e-13;

fn main() {
    // Load a scene
    let scene = Scene::default();

    // Render scene to an image, save it to a file
    render(&scene)
        .save("render.png")
        .expect("failed to save render to image file");
}

/// Render the given scene.
///
/// This renders the given scene to a newly created dynamic image.
pub fn render(scene: &Scene) -> DynamicImage {
    // TODO: efficiently load raw image from transmuted buffer here, instead of rebuilding the
    // image from the generated pixelmap pixel-by-pixel

    // Create a pixelmap of pixels
    let pixels: Vec<Rgba<u8>> = (0..scene.width * scene.height)
        .into_par_iter()
        .map(|i| (i / scene.height, i % scene.height))
        .map(|(x, y)| {
            let ray = Ray::new_prime(x, y, scene);
            cast_ray(scene, &ray, 0).to_rgba()
        })
        .collect();

    // Build the dynamic image from the pixels
    pixels
        .into_iter()
        .enumerate()
        .map(|(i, pixel)| ((i as u32) / scene.height, (i as u32) % scene.height, pixel))
        .fold(
            DynamicImage::new_rgb8(scene.width, scene.height),
            |mut image, (x, y, pixel)| {
                image.put_pixel(x, y, pixel);
                image
            },
        )
}

/// Shade hit point on diffuse surface.
///
/// Calculate the observed color at a diffuse surface point.
///
/// The hit `entity`, specific `hit` and entity surface normal must be given.
fn shade_diffuse(scene: &Scene, entity: &Entity, hit: &Point3, surface_normal: Vector3) -> Color {
    // TODO: textured coordinates:
    // let texture_coords = entity.texture_coords(&hit);

    let mut color = *BLACK;
    for light in &scene.lights {
        let direction_to_light = light.direction_from(&hit);

        let shadow_ray = Ray {
            origin: hit + (surface_normal * BIAS),
            direction: direction_to_light,
        };
        let shadow_intersection = scene.trace(&shadow_ray);
        let in_light = shadow_intersection.is_none()
            || shadow_intersection.unwrap().distance > light.distance(hit);

        let light_intensity = if in_light { light.intensity(hit) } else { 0.0 };
        let material = entity.material();
        let light_power =
            (surface_normal.dot(&direction_to_light) as f32).max(0.0) * light_intensity;
        let light_reflected = material.albedo / PI;

        let light_color = light.color() * light_power * light_reflected;

        // TODO: textured coordinates:
        // color = color + (material.coloration.color(&texture_coords) * light_color);
        color = color + (material.color * light_color);
    }

    color.clamp()
}

/// Get observed color at given intersection.
///
/// This calculates the observed color from a ray at the given intersection.
///
/// A current depth should be given to limit ray recursion.
/// For prime rays, simply give a depth of `0`.
fn get_color(scene: &Scene, ray: &Ray, intersection: &Intersection, depth: u32) -> Color {
    let hit = ray.origin + (ray.direction * intersection.distance);
    let normal = intersection.entity.surface_normal(&hit);

    let material = intersection.entity.material();
    match material.surface {
        Surface::Diffuse => shade_diffuse(scene, intersection.entity, &hit, normal),
        Surface::Specular { reflectivity } => {
            let mut color = shade_diffuse(scene, intersection.entity, &hit, normal);
            let reflection_ray = Ray::create_reflection(&normal, &ray.direction, hit, BIAS);
            color = color * (1.0 - reflectivity);
            color = color + (cast_ray(scene, &reflection_ray, depth + 1) * reflectivity);
            color
        }
        Surface::Transparent {
            index,
            transparency,
        } => {
            let mut refraction_color = *BLACK;
            let kr = fresnel(&ray.direction, &normal, index) as f32;
            // TODO: textured coordinates:
            // let surface_color = material
            //     .coloration
            //     .color(&intersection.entity.texture_coords(&hit));
            let surface_color = material.color;

            if kr < 1.0 {
                let transmission_ray =
                    Ray::create_transmission(normal, ray.direction, hit, index, BIAS).unwrap();
                refraction_color = cast_ray(scene, &transmission_ray, depth + 1);
            }

            let reflection_ray = Ray::create_reflection(&normal, &ray.direction, hit, BIAS);
            let reflection_color = cast_ray(scene, &reflection_ray, depth + 1);
            let mut color = reflection_color * kr + refraction_color * (1.0 - kr);
            color = color * transparency * surface_color;
            color
        }
    }
}

/// Calcualte fresnel lens value.
fn fresnel(incident: &Vector3, normal: &Vector3, index: f32) -> f64 {
    let i_dot_n = incident.dot(&normal);
    let mut eta_i = 1.0;
    let mut eta_t = f64::from(index);
    if i_dot_n > 0.0 {
        eta_i = eta_t;
        eta_t = 1.0;
    }

    let sin_t = eta_i / eta_t * (1.0 - i_dot_n * i_dot_n).max(0.0).sqrt();
    if sin_t > 1.0 {
        // Total internal reflection
        1.0
    } else {
        let cos_t = (1.0 - sin_t * sin_t).max(0.0).sqrt();
        let cos_i = cos_t.abs();
        let r_s = ((eta_t * cos_i) - (eta_i * cos_t)) / ((eta_t * cos_i) + (eta_i * cos_t));
        let r_p = ((eta_i * cos_i) - (eta_t * cos_t)) / ((eta_i * cos_i) + (eta_t * cos_t));
        (r_s * r_s + r_p * r_p) / 2.0
    }
}

/// Cast a ray in the scene, get observed color.
///
/// A current depth should be given to limit ray recursion.
/// For prime rays, simply give a depth of `0`.
pub fn cast_ray(scene: &Scene, ray: &Ray, depth: u32) -> Color {
    // We're just seeing black if max ray recursion is reached
    if depth >= RAY_RECURSION {
        return *BLACK;
    }

    // Find ray intersection, get intersection color
    scene
        .trace(&ray)
        .map(|i| get_color(scene, &ray, &i, depth))
        .unwrap_or(*BLACK)
}
