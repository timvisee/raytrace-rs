#[macro_use]
extern crate clap;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;

use std::fs::File;
use std::path::{Path, PathBuf};
use std::process;
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

use clap::{App, Arg};
use notify::{DebouncedEvent, RecursiveMode, Watcher};
use serde_yaml;
use took::Timer;

mod color;
mod geometric;
mod light;
mod material;
mod math;
mod render;
mod scene;

/// Application entrypoint.
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
        render(open, &scene_path, &output_path);

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

/// Render scene from file.
///
/// This renders the scene at the given `scene_path`, and outputs the render result to
/// `output_path`.
fn render(open: bool, scene_path: &Path, output_path: &Path) {
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
    eprintln!("Rendering scene on {} CPU cores...", num_cpus::get());
    let timer = Timer::new();
    let render = render::render(&scene);
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
    timer.took().describe("Rendering finished,");

    // Open render file
    if open {
        eprintln!("Opening render file...");
        open::that(output_path).expect("failed to open render output file");
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
            DebouncedEvent::NoticeRemove(_) | DebouncedEvent::Write(_) => break,
            _ => {}
        }
    }

    // Wait a little longer, ensure the file written
    thread::sleep(Duration::from_millis(200));
}
