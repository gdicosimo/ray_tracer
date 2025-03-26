use super::*;

#[derive(Default)]
pub struct Earth;

impl Scene for Earth {
    fn default_camera(&self) -> CameraBuilder {
        CameraBuilder::uninitialized()
            .aspect_ratio(1.0)
            .image_width(600)
            .samples_per_pixel(300)
            .max_depth(100)
            .vfov(25.0)
            .look_from(Point3::new(8.0, 2.0, 12.0))
            .look_at(Point3::new(0.0, 0.0, 0.0))
            .vup(Vec3::new(0.0, 1.0, 0.0))
            .background(Color::new(0.01, 0.01, 0.011))
            .defocus_angle(0.25)
    }

    fn build(&self) -> (HittableList, HittableList) {
        let mut world = HittableList::with_capacity(32);
        let mut lights = HittableList::with_capacity(4);

        let earth_texture = Arc::new(ImageTexture::from_image("earthmap.jpg"));
        let earth_material = Arc::new(Lambertian::from_texture(earth_texture, 1.0));
        let globe = Arc::new(Sphere::new(Point3::new(0.0, 0.0, 0.0), 2.0, earth_material));
        world.push(globe);

        let atmosphere = Arc::new(Sphere::new(
            Point3::new(0.0, 0.0, 0.0),
            2.05,
            Arc::new(Dielectric::new(1.1)),
        ));
        world.push(atmosphere);

        let star_count = 50;
        let star_material = Arc::new(DiffuseLight::from_color(Color::splat(1.2)));
        for _ in 0..star_count {
            let center = Point3::new(
                random_float_between(-30.0, 15.0),
                random_float_between(-15.0, 15.0),
                random_float_between(-50.0, -25.0),
            );
            let star = Arc::new(Sphere::new(
                center,
                random_float_between(0.05, 0.15), // Tamaños variables
                star_material.clone(),
            ));
            world.push(star);
        }

        lights.push(Arc::new(Quad::new(
            Point3::new(343.0, 554.0, 332.0),
            Vec3::new(-130.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, -105.0),
            Arc::new(Empty),
        )));

        let sun_light = Arc::new(DiffuseLight::from_color(Color::new(1.0, 0.9, 0.8)));
        world.push(Arc::new(Sphere::new(
            Point3::new(5.0, 10.0, 5.0),
            3.0,
            sun_light.clone(),
        )));

        let ambient_light = Arc::new(DiffuseLight::from_color(Color::new(0.2, 0.3, 0.5)));
        world.push(Arc::new(Sphere::new(
            Point3::new(0.0, -10.0, 0.0),
            7.0,
            ambient_light,
        )));

        (world, lights)
    }
}
