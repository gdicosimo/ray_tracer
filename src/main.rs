use ray_tracer::util::{parse_config, save_as_png_from_floats, ImageError};

fn main() -> Result<(), ImageError> {
    let (scene, camera, output) = parse_config();

    let pixels = scene.render(&camera);

    let (width, height) = (camera.width(), camera.height());

    save_as_png_from_floats(width, height, &pixels, &output)?;
    Ok(())
}
