use image::{DynamicImage, GenericImage, Rgba};

mod color;
mod geometric;
mod math;
mod scene;

use math::*;
use scene::*;

fn main() {
    let scene = Scene::default();

    let image = render(&scene);

    image
        .save("render.png")
        .expect("failed to save render to image file");
}

pub fn render(scene: &Scene) -> DynamicImage {
    let mut image = DynamicImage::new_rgb8(scene.width, scene.height);
    let black = Rgba([0, 0, 0, 0]);

    for x in 0..scene.width {
        for y in 0..scene.height {
            let ray = Ray::new_screen(x, y, scene);

            if scene.sphere.intersect(&ray) {
                image.put_pixel(x, y, scene.sphere.color.to_rgba())
            } else {
                image.put_pixel(x, y, black);
            }
        }
    }

    image
}
