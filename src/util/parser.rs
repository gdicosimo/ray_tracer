use clap::Parser;

use crate::{camera::Camera, math::Point3, scenes::*};

macro_rules! apply_camera_settings {
    ($builder:expr, $config:expr, [ $( $field:ident ),* ]) => {{
        let mut builder = $builder;
        $(
            if let Some(value) = $config.$field {
                builder = builder.$field(value);
            }
        )*
        builder
    }};
}

#[derive(Parser, Debug)]
pub struct CameraConfig {
    /// Width of the output image in pixels
    /// Example: --image-width 1920 or -w 1920
    #[arg(short = 'w', long)]
    image_width: Option<u32>,

    /// Vertical field of view in degrees
    /// Smaller values = zoomed in, larger values = wider view
    /// Example: --vfov 90.0 or -v 90.0
    #[arg(short = 'v', long)]
    vfov: Option<f32>,

    /// Anti-aliasing samples per pixel
    /// Higher values reduce noise but increase render time
    /// Example: --samples-per-pixel 100 or -s 100
    #[arg(short = 'p', long)]
    samples_per_pixel: Option<u16>,

    /// Image aspect ratio (width/height)
    /// Alternative to specifying height directly
    /// Example: --aspect-ratio 1.77 or -a 1.77
    #[arg(short = 'r', long)]
    aspect_ratio: Option<f32>,

    /// Camera position in 3D space (x,y,z coordinates)
    /// Example: --look-from 278,273,-800 or -f 278,273,-800
    #[arg(short = 'f', long)]
    look_from: Option<Point3>,

    /// Target point the camera is looking at (x,y,z coordinates)
    /// Example: --look-at 278,273,0 or -t 278,273,0
    #[arg(short = 't', long)]
    look_at: Option<Point3>,
}

#[derive(Parser, Debug)]
#[command(
    author = "gdicosimo",
    version = "1.0",
    about = "Minimalistic ray tracer with scene selection and camera configuration",
    long_about = r#"
    long_about = r#"
    Minimalistic ray tracer with scene selection and camera configuration

    Coordinate System:
    - Origin (0,0,0) - Bottom-right corner
    - Maximum (1,1,1) - Top-right corner (canonical system)
    - Axes: X (right), Y (up), Z (depth into scene)


    Camera Behavior:
    - Uses scene's default camera if no camera parameters are provided
    - Provide any camera option (-v/-f/-t/etc) to override scene defaults
"#
)]
pub struct Args {
    /// Supported values: cornell_box, earth, svrnc, demo
    #[arg(short, long, default_value = "cornell_box")]
    scene: String,

    /// Output file path
    #[arg(short, long, default_value = "output.png")]
    output: String,

    #[command(flatten)]
    camera_config: CameraConfig,
}

pub fn parse_config() -> (Box<dyn Scene>, Camera, String) {
    let args = Args::parse();

    let scene = build_scene(args.scene.as_str());
    let output = args.output;
    let camera = build_camera(args.camera_config, scene.as_ref());

    (scene, camera, output)
}

fn build_scene(scene: &str) -> Box<dyn Scene> {
    match scene {
        "cornell_box" => Box::new(CornellBox::default()),
        "svrnc" => Box::new(Svrnc::default()),
        "earth" => Box::new(Earth),
        "rtiow" => Box::new(Rtiow::default()),
        unknown => {
            eprintln!("Unknown scene: {}. Using default (cornell_box).", unknown);
            Box::new(CornellBox::default())
        }
    }
}

fn build_camera(config: CameraConfig, scene: &dyn Scene) -> Camera {
    let builder = scene.default_camera();

    let builder = apply_camera_settings!(
        builder,
        config,
        [
            image_width,
            aspect_ratio,
            look_from,
            look_at,
            vfov,
            samples_per_pixel
        ]
    );

    builder.build()
}
