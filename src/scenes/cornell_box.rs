use super::*;

#[derive(Default)]
pub struct CornellBox(SceneData);

impl Scene for CornellBox {
    fn default_camera(&self) -> CameraBuilder {
        CameraBuilder::uninitialized()
            .aspect_ratio(1.0)
            .image_width(600)
            .samples_per_pixel(500)
            .max_depth(50)
            .vfov(40.0)
            .look_from(Point3::new(278.0, 278.0, -800.0))
            .look_at(Point3::new(278.0, 278.0, 0.0))
            .vup(Vec3::new(0.0, 1.0, 0.0))
            .defocus_angle(0.0)
    }

    fn build(&self) -> (HittableList, HittableList) {
        let mut world = HittableList::with_capacity(8);

        let red = Arc::new(Lambertian::from_color(Color::new(0.65, 0.05, 0.05), 1.0));
        let white = Arc::new(Lambertian::from_color(Color::splat(0.73), 1.0));
        let green = Arc::new(Lambertian::from_color(Color::new(0.12, 0.45, 0.15), 1.0));
        let light = Arc::new(DiffuseLight::from_color(Color::splat(15.0)));

        // Cornell box walls
        world.push(Arc::new(Quad::new(
            Point3::new(555.0, 0.0, 0.0),
            Vec3::new(0.0, 555.0, 0.0),
            Vec3::new(0.0, 0.0, 555.0),
            green,
        )));
        world.push(Arc::new(Quad::new(
            Point3::new(0.0, 0.0, 555.0),
            Vec3::new(0.0, 555.0, 0.0),
            Vec3::new(0.0, 0.0, -555.0),
            red,
        )));
        world.push(Arc::new(Quad::new(
            Point3::new(0.0, 555.0, 0.0),
            Vec3::new(555.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 555.0),
            white.clone(),
        )));
        world.push(Arc::new(Quad::new(
            Point3::new(0.0, 0.0, 555.0),
            Vec3::new(555.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, -555.0),
            white.clone(),
        )));
        world.push(Arc::new(Quad::new(
            Point3::new(555.0, 0.0, 555.0),
            Vec3::new(-555.0, 0.0, 0.0),
            Vec3::new(0.0, 555.0, 0.0),
            white.clone(),
        )));

        let mut box1: Arc<dyn Primitive> = Arc::new(Cuboid::new(
            Point3::new(0.0, 0.0, 0.0),
            Point3::new(165.0, 330.0, 165.0),
            white.clone(),
        ));
        box1 = Arc::new(RotationY::new(box1, 15.0));
        box1 = Arc::new(Translation::new(box1, Vec3::new(265.0, 0.0, 295.0)));
        world.push(box1);

        // Esfera de vidrio
        let glass = Arc::new(Dielectric::new(1.5));
        world.push(Arc::new(Sphere::new(
            Point3::new(190.0, 90.0, 190.0),
            90.0,
            glass,
        )));

        // Lights
        let mut lights = HittableList::new();

        world.push(Arc::new(Quad::new(
            Point3::new(213.0, 554.0, 227.0),
            Vec3::new(130.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 105.0),
            light,
        )));

        lights.push(Arc::new(Quad::new(
            Point3::new(343.0, 554.0, 332.0),
            Vec3::new(-130.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, -105.0),
            self.0.empty(),
        )));
        lights.push(Arc::new(Sphere::new(
            Point3::new(190.0, 90.0, 190.0),
            90.0,
            self.0.empty(),
        )));

        (world, lights)
    }
}
