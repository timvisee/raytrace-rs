#[macro_use]
extern crate clap;
#[macro_use]
extern crate derive_builder;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;

use std::f32::consts::PI;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::process;
use std::sync::mpsc::channel;
use std::time::Duration;

use clap::{App, Arg};
use image::{DynamicImage, GenericImage, Rgba};
use notify::{DebouncedEvent, RecursiveMode, Watcher};
use rayon::prelude::*;
use serde_yaml;

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

fn main() {
    // CLI argument handling
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(
            Arg::with_name("SCENE")
                .help("Scene file to render")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("OUTPUT")
                .help("Image file to output render to")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("watch")
                .long("watch")
                .short("w")
                .help("Watch scene file, automatically rerender")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("open")
                .long("open")
                .short("o")
                .help("Open rendered scene image")
                .takes_value(false),
        )
        .get_matches();

    // Validate scene file
    let scene_path = PathBuf::from(matches.value_of("SCENE").unwrap());
    if !scene_path.is_file() {
        eprintln!(
            "Invalid scene file, not an existing file: '{}'",
            scene_path.to_str().unwrap_or("?"),
        );
        process::exit(1)
    }

    // Validate render output file
    let output_path = PathBuf::from(matches.value_of("OUTPUT").unwrap());
    if output_path.is_dir() {
        eprintln!(
            "Invalid output file, is an existing directory: '{}'",
            output_path.to_str().unwrap_or("?"),
        );
        process::exit(1)
    }

    // Check whether to open and watch
    let mut open = matches.is_present("open");
    let watch = matches.is_present("watch");

    loop {
        // Render the scene
        run(open, &scene_path, &output_path);

        // Do not watch, render a single time and quit
        if !watch {
            break;
        }

        // Wait for scene file change
        wait_on_change(&scene_path);

        // Do not open a second time
        open = false;
        eprintln!("");
    }
}

/// Wait for a given file to change.
///
/// This function blocks, until the given file is changed.
fn wait_on_change(path: &Path) {
    // Create scene file watcher, with channel to receive events
    let (tx, rx) = channel();
    let mut watcher =
        notify::watcher(tx, Duration::from_secs(1)).expect("failed to create file watcher");
    watcher
        .watch(&path, RecursiveMode::NonRecursive)
        .expect("failed to configure watcher for file changes");

    // Wait for scene file change
    loop {
        match rx.recv().expect("failed to watch file for changes") {
            DebouncedEvent::Write(_) => break,
            _ => {}
        }
    }
}

fn run(open: bool, scene_path: &Path, output_path: &Path) {
    // Load scene from file
    eprintln!("Loading scene file...");
    let scene_file = match File::open(scene_path) {
        Ok(file) => file,
        Err(err) => {
            eprintln!(
                "Failed to open scene file, could not open file at '{}'\nSkipping this render\n\nDetails:\n{}",
                scene_path.to_str().unwrap_or("?"),
                err,
            );
            return;
        }
    };
    let scene = match serde_yaml::from_reader(scene_file) {
        Ok(file) => file,
        Err(err) => {
            eprintln!(
                "Failed to parse scene file, skipping this render\n\nDetails:\n{}",
                err,
            );
            return;
        }
    };

    // Render scene to an image, save it to a file
    eprintln!("Rendering scene...");
    let render = render(&scene);
    match render.save(output_path) {
        Ok(_) => {}
        Err(err) => {
            eprintln!(
                "Failed to write render to output path, could not write at: '{}'\nSkipping this render\n\nDetails:\n{}",
                output_path.to_str().unwrap_or("?"),
                err,
            );
            return;
        }
    }
    eprintln!("Rendering finished");

    // Open render file
    if open {
        eprintln!("Opening render file...");
        open::that(output_path).expect("failed to open render output file");
    }
}

/// Render the given scene.
///
/// This renders the given scene to a newly created dynamic image.
pub fn render(scene: &Scene) -> DynamicImage {
    // TODO: efficiently load raw image from transmuted buffer here, instead of rebuilding the
    // image from the generated pixelmap pixel-by-pixel

    let camera = scene.camera;

    // Create a pixelmap of pixels
    let pixels: Vec<Rgba<u8>> = (0..camera.pixels())
        .into_par_iter()
        .map(|i| (i / scene.camera.height, i % scene.camera.height))
        .map(|(x, y)| {
            let ray = Ray::new_prime(x, y, scene);
            cast_ray(scene, &ray, 0).to_rgba()
        })
        .collect();

    // Build the dynamic image from the pixels
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
            origin: hit + (surface_normal * scene.bias),
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
            let reflection_ray = Ray::create_reflection(&normal, &ray.direction, hit, scene.bias);
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
                    Ray::create_transmission(normal, ray.direction, hit, index, scene.bias)
                        .unwrap();
                refraction_color = cast_ray(scene, &transmission_ray, depth + 1);
            }

            let reflection_ray = Ray::create_reflection(&normal, &ray.direction, hit, scene.bias);
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
    if depth >= scene.depth {
        return *BLACK;
    }

    // Find ray intersection, get intersection color
    scene
        .trace(&ray)
        .map(|i| get_color(scene, &ray, &i, depth))
        .unwrap_or(*BLACK)
}
