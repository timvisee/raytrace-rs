fn main() {
    let scene = Scene::default();

    let image = render(&scene);

    image
        .save("render.png")
        .expect("failed to save render to image file");
}
