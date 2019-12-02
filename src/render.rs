use std::f32::consts::PI;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

use image::{DynamicImage, GenericImage, Rgba};
use pbr::ProgressBar;
use rayon::prelude::*;

use crate::algebra::Vector;
use crate::color::{Color, BLACK};
use crate::geometric::Entity;
use crate::material::Surface;
use crate::math::*;
use crate::scene::Scene;

/// Render the given scene.
///
/// This renders the given scene to a newly created dynamic image.
pub fn render(scene: &Scene, show_progress: bool) -> DynamicImage {
    let camera = scene.camera;

    // Warn if there are no lights
    if scene.lights.is_empty() {
        eprintln!("Warning: no lights in scene, you won't be able to see anything");
    }

    // Set up a progress bar if we should show progress
    let count = camera.pixels() as u64;
    let mut pb = None;
    let mut progress = None;
    if show_progress {
        let thread_progress = Arc::new(AtomicU64::new(0));
        let thread_pb = Arc::new(Mutex::new(ProgressBar::new(camera.pixels() as u64)));
        pb = Some(thread_pb.clone());
        progress = Some(thread_progress.clone());
        thread::spawn(move || loop {
            // Get the current progress value, update progress bar
            let value = thread_progress.load(Ordering::Relaxed);
            if let Ok(mut pb) = thread_pb.lock() {
                pb.set(value);
            }

            // Stop when done
            if value >= count {
                break;
            }

            thread::sleep(Duration::from_millis(250));
        });
    }

    // Create a pixelmap of pixels
    let pixels: Vec<Rgba<u8>> = (0..count as u32)
        .into_par_iter()
        .map(|i| (i / scene.camera.height, i % scene.camera.height))
        .map(|(x, y)| {
            let ray = Ray::new_prime(x, y, scene);
            let color = observe_ray(scene, &ray, 0).to_rgba();

            // Update the progress
            if let Some(progress) = progress.as_ref() {
                progress.fetch_add(1, Ordering::Relaxed);
            }

            color
        })
        .collect();

    // Finish the progress bar
    if let Some(pb) = pb {
        pb.lock().unwrap().finish();
    }

    // Build the dynamic image from the pixels
    // TODO: find more efficient method, render directly to image buffer
    pixels
        .into_iter()
        .enumerate()
        .map(|(i, pixel)| {
            (
                (i as u32) / camera.height,
                (i as u32) % camera.height,
                pixel,
            )
        })
        .fold(
            DynamicImage::new_rgb8(camera.width, camera.height),
            |mut image, (x, y, pixel)| {
                image.put_pixel(x, y, pixel);
                image
            },
        )
}

/// Cast a ray in the scene, get observed color.
///
/// A current depth should be given to limit ray recursion.
/// For prime rays, simply give a depth of `0`.
fn observe_ray(scene: &Scene, ray: &Ray, depth: u32) -> Color {
    // We're just seeing black if max ray recursion is reached
    if depth >= scene.depth {
        return *BLACK;
    }

    // Find ray intersection, get intersection color
    scene
        .intersect(&ray)
        .map(|i| observe_intersection(scene, &ray, &i, depth))
        .unwrap_or(*BLACK)
}

/// Get observed color at given intersection.
///
/// This calculates the observed color from a ray at the given intersection.
///
/// A current depth should be given to limit ray recursion.
/// For prime rays, simply give a depth of `0`.
fn observe_intersection(
    scene: &Scene,
    ray: &Ray,
    intersection: &Intersection,
    depth: u32,
) -> Color {
    let hit = ray.origin + (ray.direction * intersection.distance);
    let normal = intersection.normal;

    let material = intersection.entity.material();
    match material.surface {
        Surface::Diffuse => shade_diffuse(scene, intersection.entity, hit, normal),
        Surface::Specular { reflectivity } => {
            let mut color = shade_diffuse(scene, intersection.entity, hit, normal);
            let reflection_ray = Ray::create_reflection(normal, ray.direction, hit, scene.bias);
            color = color * (1.0 - reflectivity);
            color = color + (observe_ray(scene, &reflection_ray, depth + 1) * reflectivity);
            color
        }
        Surface::Transparent {
            index,
            transparency,
        } => {
            let mut refraction_color = *BLACK;
            let kr = fresnel(ray.direction, normal, index) as f32;
            // TODO: textured coordinates:
            // let surface_color = material
            //     .coloration
            //     .color(&intersection.entity.texture_coords(&hit));
            let surface_color = material.color;

            if kr < 1.0 {
                let transmission_ray =
                    Ray::create_transmission(normal, ray.direction, hit, index, scene.bias)
                        .unwrap();
                refraction_color = observe_ray(scene, &transmission_ray, depth + 1);
            }

            let reflection_ray = Ray::create_reflection(normal, ray.direction, hit, scene.bias);
            let reflection_color = observe_ray(scene, &reflection_ray, depth + 1);
            let mut color = reflection_color * kr + refraction_color * (1.0 - kr);
            color = color * transparency * surface_color;
            color
        }
    }
}

/// Shade hit point on diffuse surface.
///
/// Calculate the observed color at a diffuse surface point.
///
/// The hit `entity`, specific `hit` and entity surface normal must be given.
fn shade_diffuse(scene: &Scene, entity: &Entity, hit: Vector, surface_normal: Vector) -> Color {
    // TODO: textured coordinates:
    // let texture_coords = entity.texture_coords(&hit);

    let mut color = *BLACK;
    for light in &scene.lights {
        let direction_to_light = light.direction_from(hit);

        let shadow_ray = Ray {
            origin: hit + (surface_normal * scene.bias),
            direction: direction_to_light,
        };
        let shadow_intersection = scene.intersect(&shadow_ray);
        let in_light = shadow_intersection.is_none()
            || shadow_intersection.unwrap().distance > light.distance(hit);

        let light_intensity = if in_light { light.intensity(hit) } else { 0.0 };
        let material = entity.material();
        let light_power =
            (surface_normal.dot(direction_to_light) as f32).max(0.0) * light_intensity;
        let light_reflected = material.albedo / PI;

        let light_color = light.color() * light_power * light_reflected;

        // TODO: textured coordinates:
        // color = color + (material.coloration.color(&texture_coords) * light_color);
        color = color + (material.color * light_color);
    }

    color.clamp()
}

/// Calcualte fresnel lens value.
fn fresnel(incident: Vector, normal: Vector, index: f32) -> f64 {
    let i_dot_n = incident.dot(normal);
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
