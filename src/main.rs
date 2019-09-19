#[macro_use]
extern crate lazy_static;

use image::{DynamicImage, GenericImage, Rgba};
use rayon::prelude::*;

mod color;
mod geometric;
mod light;
mod math;
mod scene;

use color::{Color, BLACK};
use math::*;
use scene::*;

pub const RAY_RECURSION: u32 = 16;
pub const SHADOW_BIAS: f64 = 1e-13;
pub const REFLECT_BIAS: f64 = SHADOW_BIAS;

fn main() {
    let scene = Scene::default();

    let image = render(&scene);

    image
        .save("render.png")
        .expect("failed to save render to image file");
}

pub fn render(scene: &Scene) -> DynamicImage {
    // TODO: efficiently load raw image from transmuted buffer here, instead of rebuilding the
    // image from the generated pixelmap pixel-by-pixel

    // Create a pixelmap of pixels
    let pixels: Vec<Rgba<u8>> = (0..scene.width * scene.height)
        .into_par_iter()
        .map(|i| (i / scene.height, i % scene.height))
        .map(|(x, y)| {
            // TODO: make recursion configurable
            let ray = Ray::new_prime(x, y, scene);
            cast_ray(scene, ray, RAY_RECURSION).to_rgba()
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

/// Do a ray cast in the given scene, and obtain the preceived color at the ray origin.
fn cast_ray(scene: &Scene, ray: Ray, recursion: u32) -> Color {
    // Find the ray intersection
    let intersection = match scene.trace(&ray) {
        Some(i) => i,
        None => return *BLACK,
    };

    let hit_point = ray.origin + (ray.direction * intersection.distance);
    let surface_normal = intersection.entity.surface_normal(&hit_point);

    let mut color = Color::new(0.0, 0.0, 0.0);

    // Reflection ray
    // TODO: check implementation correctness
    if recursion > 1 {
        // Get inverse impact and normal ray delta, calculate reflection direction
        let normal_delta = surface_normal - -ray.direction;
        let reflect_direction = (-ray.direction + normal_delta * 2.0).normalize();
        let ray = Ray::new(
            hit_point + reflect_direction * REFLECT_BIAS,
            reflect_direction,
        );

        // Cast reflection ray, obtain reflected color
        let reflect_color = cast_ray(scene, ray, recursion - 1);

        // let material = scene.entity.material();
        // TODO: what value to use here?
        let intensity = 1.0;
        let light_power = (surface_normal.dot(&reflect_direction) as f32).max(0.0) * intensity;
        let light_reflected = intersection.entity.albedo() / std::f32::consts::PI;

        let light_color = reflect_color * light_power * light_reflected;
        // color = color + (material.coloration.color(&texture_coords) * light_color);
        // color = color + (*intersection.entity.color() * light_color);
        color = color + light_color;
    }

    // Light and shadow rays
    for light in &scene.lights {
        let direction_to_light = light.direction_from(&hit_point);

        let shadow_ray = Ray {
            origin: hit_point + (surface_normal * SHADOW_BIAS),
            direction: direction_to_light,
        };
        let shadow_intersection = scene.trace(&shadow_ray);
        let in_light = shadow_intersection.is_none()
            || shadow_intersection.unwrap().distance > light.distance(&hit_point);

        let light_intensity = if in_light {
            light.intensity(&hit_point)
        } else {
            0.0
        };
        // let material = scene.entity.material();
        let light_power =
            (surface_normal.dot(&direction_to_light) as f32).max(0.0) * light_intensity;
        let light_reflected = intersection.entity.albedo() / std::f32::consts::PI;

        let light_color = light.color() * light_power * light_reflected;
        // color = color + (material.coloration.color(&texture_coords) * light_color);
        color = color + (*intersection.entity.color() * light_color);
    }

    color
}
