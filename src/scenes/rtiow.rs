use super::*;

#[derive(Default)]
pub struct Rtiow(SceneData);

impl Scene for Rtiow {
    fn default_camera(&self) -> CameraBuilder {
        CameraBuilder::uninitialized()
            .aspect_ratio(1.0)
            .image_width(800)
            .samples_per_pixel(500)
            .max_depth(40)
            .vfov(40.0)
            .look_from(Point3::new(478.0, 278.0, -600.0))
            .look_at(Point3::new(278.0, 278.0, 0.0))
            .vup(Vec3::new(0.0, 1.0, 0.0))
            .background(color::BLACK)
            .defocus_angle(0.0)
    }

    fn build(&self) -> (HittableList, HittableList) {
        let mut boxes1 = HittableList::new();

        let ground = Arc::new(Lambertian::from_color(Color::new(0.48, 0.83, 0.53), 0.85));

        let boxes_per_side = 20;
        for i in 0..boxes_per_side {
            for j in 0..boxes_per_side {
                let w = 100.0;
                let x0 = -1000.0 + i as f32 * w;
                let z0 = -1000.0 + j as f32 * w;
                let y1 = random_float_between(10.0, 101.0);

                boxes1.push(Arc::new(Cuboid::new(
                    Point3::new(x0, 0.0, z0),
                    Point3::new(x0 + w, y1, z0 + w),
                    ground.clone(),
                )));
            }
        }

        let mut world = HittableList::new();
        let mut lights = HittableList::new();

        world.push(Arc::new(boxes1));

        let light = Arc::new(DiffuseLight::from_color(Color::new(7.0, 7.0, 7.0)));
        world.push(Arc::new(Quad::new(
            Point3::new(123.0, 554.0, 147.0),
            Vec3::new(300.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 265.0),
            light.clone(),
        )));

        lights.push(Arc::new(Quad::new(
            Point3::new(123.0, 554.0, 147.0),
            Vec3::new(300.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 265.0),
            self.0.empty(),
        )));

        let sphere_material = Arc::new(Lambertian::from_color(Color::new(0.7, 0.3, 0.1), 1.0));
        world.push(Arc::new(Sphere::new_moving(
            Point3::new(400.0, 400.0, 200.0),
            Point3::new(430.0, 400.0, 200.0),
            50.0,
            sphere_material.clone(),
        )));

        world.push(Arc::new(Sphere::new(
            Point3::new(260.0, 150.0, 45.0),
            50.0,
            Arc::new(Dielectric::new(1.5)),
        )));

        world.push(Arc::new(Sphere::new(
            Point3::new(0.0, 150.0, 145.0),
            50.0,
            Arc::new(Metal::new(Color::new(0.8, 0.8, 0.9), 1.0)),
        )));

        let boundary = Arc::new(Sphere::new(
            Point3::new(360.0, 150.0, 145.0),
            70.0,
            Arc::new(Dielectric::new(1.5)),
        ));
        world.push(Arc::new(ConstantMedium::from_color(
            boundary.clone(),
            0.2,
            Color::new(0.2, 0.4, 0.9),
        )));

        let earth_texture = Arc::new(ImageTexture::from_image("earthmap.jpg"));
        let earth_material = Arc::new(Lambertian::from_texture(earth_texture, 1.0));
        world.push(Arc::new(Sphere::new(
            Point3::new(400.0, 200.0, 400.0),
            100.0,
            earth_material,
        )));

        let noise_texture = Arc::new(NoiseTexture::new(0.2));
        world.push(Arc::new(Sphere::new(
            Point3::new(220.0, 280.0, 300.0),
            80.0,
            Arc::new(Lambertian::from_texture(noise_texture, 1.0)),
        )));

        let mut boxes2 = HittableList::new();
        let white = Arc::new(Lambertian::from_color(Color::new(0.73, 0.73, 0.73), 1.0));

        for _ in 0..1000 {
            boxes2.push(Arc::new(Sphere::new(
                Point3::random_between(0.0, 165.0),
                10.0,
                white.clone(),
            )));
        }

        world.push(Arc::new(Translation::new(
            Arc::new(RotationY::new(Arc::new(boxes2), 15.0)),
            Vec3::new(-100.0, 270.0, 395.0),
        )));

        (world, lights)
    }
}
