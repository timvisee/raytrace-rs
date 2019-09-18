use image::{DynamicImage, GenericImage, Rgba};
use rayon::prelude::*;

mod color;
mod geometric;
mod light;
mod math;
mod scene;

use color::Color;
use math::*;
use scene::*;

pub const SHADOW_BIAS: f64 = 1e-13;

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

    // The background color
    let background = Rgba([0, 0, 0, 255]);

    // Create a pixelmap of pixels
    let pixels: Vec<Rgba<u8>> = (0..scene.width * scene.height)
        .into_par_iter()
        .map(|i| (i / scene.height, i % scene.height))
        .map(|(x, y)| {
            let ray = Ray::new_prime(x, y, scene);

            if let Some(intersection) = scene.trace(&ray) {
                let hit_point = ray.origin + (ray.direction * intersection.distance);
                let surface_normal = intersection.entity.surface_normal(&hit_point);

                let mut color = Color::new(0.0, 0.0, 0.0);
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

                color.to_rgba()
            } else {
                background
            }
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
