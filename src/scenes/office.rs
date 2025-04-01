use super::*;

// This code is a bit messy but hey, the important thing is the rendering of the scene, isn't?

// ─────────────────────────────

const WHITE: Color = Color::new(0.8471, 0.8784, 0.8784); // #d8e0e0
const BLACK: Color = Color::new(0.023, 0.054, 0.152);
const STEEL_BLACK: Color = Color::new(0.325, 0.333, 0.329);
const DARK_TEAL: Color = Color::new(0.1255, 0.2431, 0.2431); // #1f3d3d
const MOSS_GREEN: Color = Color::new(0.4745, 0.6353, 0.4627); // #78a175
const GRAY: Color = Color::new(0.4431, 0.5490, 0.5255); // #718c86

// ─────────────────────────────
struct Palette {
    teal: Arc<dyn Material>,
    green: Arc<dyn Material>,
    white: Arc<dyn Material>,
    black: Arc<dyn Material>,
    gray: Arc<dyn Material>,
    steel_black: Arc<dyn Material>,
    melamine: Arc<dyn Material>,
}
// ─────────────────────────────
#[derive(Default)]
pub struct Svrnc {
    data: SceneData,
    mats: Palette,
}

impl Svrnc {
    const SCENE_WIDTH: f32 = 1584.0;
    const SCENE_HEIGHT: f32 = 396.0;
    const SCENE_DEPTH: f32 = 555.0;

    const CEIL_LOWER_B: f32 = 264.0;
    const CEIL_UPPER_B: f32 = 141.0;
    const CEIL_HEIGHT: f32 = 71.0;
    const CEIL_ANGLE: f32 = 63.0;
    const CEIL_RATIO: f32 = Self::CEIL_UPPER_B / Self::CEIL_LOWER_B;

    const PLINTH_HEIGHT: f32 = 20.0;
    const PLINTH_DEPTH: f32 = 10.0;
    const HEADER_HEIGHT: f32 = 20.0;
    const WALL_HEIGHT: f32 = Self::SCENE_HEIGHT - Self::PLINTH_HEIGHT - Self::HEADER_HEIGHT;

    const PANEL_WIDTH: f32 = 200.0;
    const PANEL_HEIGHT: f32 = 140.0;
    const PANEL_DEPTH: f32 = 10.0;

    const TABLE_WIDTH: f32 = 200.0;
    const TABLE_HEIGHT: f32 = 10.0;
    const TABLE_DEPTH: f32 = 120.0;
}

impl Scene for Svrnc {
    fn default_camera(&self) -> CameraBuilder {
        CameraBuilder::uninitialized()
            .aspect_ratio(4.0)
            .image_width(Self::SCENE_WIDTH as u32)
            .samples_per_pixel(3000)
            .max_depth(100)
            .vfov(40.0)
            .look_from(Point3::new(
                Self::SCENE_WIDTH * 0.5,
                Self::SCENE_HEIGHT * 0.5,
                -Self::SCENE_DEPTH,
            ))
            .look_at(Point3::new(
                Self::SCENE_WIDTH * 0.5,
                Self::SCENE_HEIGHT * 0.5,
                0.0,
            ))
            .vup(Vec3::new(0.0, 1.0, 0.0))
            .defocus_angle(0.0)
    }

    fn build(&self) -> (HittableList, HittableList) {
        let mut world = HittableList::with_capacity(16);
        let mut lights = HittableList::with_capacity(4);

        world.push(self.create_floor());

        let (ceilling, c_ligths) = self.create_ceiling();
        world.push(ceilling);
        lights.push(c_ligths);

        let (l_terminal, r_terminal) = self.create_terminal();
        world.push(l_terminal);
        world.push(r_terminal);

        world.push(self.create_back_wall());

        let (l_wall, r_wall) = self.create_opposite_walls();
        world.push(l_wall);
        world.push(r_wall);

        world.push(self.create_desktops());

        world.push(Arc::new(Sphere::new(
            Point3::new(300.0, 50.1, Self::SCENE_DEPTH / 3.0 + 85.0),
            50.0,
            self.data.aluminum(),
        )));

        world.push(Arc::new(Sphere::new(
            Point3::new(400.0, 50.1, Self::SCENE_DEPTH / 3.0),
            50.0,
            self.data.glass(),
        )));

        lights.push(Arc::new(Sphere::new(
            Point3::new(400.0, 50.1, Self::SCENE_DEPTH / 3.0),
            50.0,
            self.data.empty(),
        )));

        (world, lights)
    }
}

impl Svrnc {
    fn create_opposite_walls(&self) -> (Arc<dyn Primitive>, Arc<dyn Primitive>) {
        fn create_edges(svrnc: &Svrnc) -> Arc<dyn Primitive> {
            let mut edges = HittableList::with_capacity(4);

            let edge = Arc::new(Quad::new(
                Point3::new(0.1, Svrnc::PLINTH_HEIGHT, Svrnc::SCENE_DEPTH - 2.51),
                Vec3::new(0.0, Svrnc::WALL_HEIGHT, 0.0),
                Vec3::new(0.0, 0.0, 2.5),
                svrnc.data.aluminum(),
            ));

            for offset in (264..=Svrnc::SCENE_DEPTH as usize).step_by(264) {
                edges.push(Arc::new(Translation::new(
                    edge.clone(),
                    Vec3::new(0.0, 0.0, -(offset as f32)),
                )));
            }

            Arc::new(edges)
        }

        let mut block = HittableList::with_capacity(8);

        let wall = Arc::new(Quad::new(
            Point3::new(0.0, Self::PLINTH_HEIGHT, 0.0),
            Vec3::new(0.0, 0.0, Self::SCENE_DEPTH),
            Vec3::new(0.0, Self::WALL_HEIGHT, 0.0),
            self.mats.white(),
        ));

        let header = Arc::new(Quad::new(
            Point3::new(0.0, Self::SCENE_HEIGHT - Self::HEADER_HEIGHT, 0.0),
            Vec3::new(0.0, Self::HEADER_HEIGHT, 0.0),
            Vec3::new(0.0, 0.0, Self::SCENE_DEPTH),
            self.data.aluminum(),
        ));

        let m_header = Arc::new(Quad::new(
            Point3::new(0.1, Self::SCENE_HEIGHT - Self::HEADER_HEIGHT + 5.0, 0.0),
            Vec3::new(0.0, Self::HEADER_HEIGHT * 0.5, 0.0),
            Vec3::new(0.0, 0.0, Self::SCENE_DEPTH),
            self.mats.steel_black(),
        ));

        let plinth = Arc::new(Quad::new(
            Point3::new(-Self::PLINTH_DEPTH, 0.0, 0.0),
            Vec3::new(0.0, Self::PLINTH_HEIGHT, 0.0),
            Vec3::new(0.0, 0.0, Self::SCENE_DEPTH + Self::PLINTH_DEPTH),
            self.mats.teal(),
        ));

        let plinth_fill = Arc::new(Quad::new(
            Point3::new(-Self::PLINTH_DEPTH, Self::PLINTH_HEIGHT, 0.0),
            Vec3::new(-Self::PLINTH_HEIGHT, 0.0, 0.0),
            Vec3::new(0.0, 0.0, Self::SCENE_DEPTH + Self::PLINTH_DEPTH),
            self.mats.white(),
        ));

        //

        block.push(wall);
        block.push(header);
        block.push(m_header);
        block.push(plinth);
        block.push(plinth_fill);
        block.push(create_edges(self));

        let r_wall = Arc::new(block);

        let mut l_wall: Arc<dyn Primitive> = Arc::new(MirrorYZ::new(r_wall.clone()));
        l_wall = Arc::new(Translation::new(
            l_wall,
            Vec3::new(Self::SCENE_WIDTH, 0.0, 0.0),
        ));

        (l_wall, r_wall)
    }

    fn create_floor(&self) -> Arc<dyn Primitive> {
        let mut block = HittableList::with_capacity(8);

        let disk_w = Self::SCENE_WIDTH - (270.0 * 2.0) + 155.0;
        let disk_d = Self::SCENE_DEPTH * 0.40 - Self::PANEL_WIDTH * 0.5;
        let disk = Arc::new(Disk::new(
            Point3::new(disk_w, 0.1, disk_d),
            Vec3::new(0.0, 0.0, 55.0),
            Vec3::new(155.0, 0.0, 0.0),
            self.data.aluminum(),
        ));
        block.push(Arc::new(Translation::new(
            disk.clone(),
            Vec3::new(0.0, 0.0, 250.0),
        )));
        block.push(Arc::new(Translation::new(
            disk.clone(),
            Vec3::new(-310.0, 0.0, 0.0),
        )));
        block.push(Arc::new(Translation::new(
            disk.clone(),
            Vec3::new(-310.0, 0.0, 250.0),
        )));

        block.push(disk);

        block.push(Arc::new(Quad::new(
            Point3::new(-Self::PLINTH_DEPTH, 0.0, 0.0),
            Vec3::new(Self::SCENE_WIDTH + Self::PLINTH_DEPTH * 2.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, Self::SCENE_DEPTH + Self::PLINTH_DEPTH),
            self.mats.green(),
        )));

        Arc::new(block)
    }

    fn create_ceiling(&self) -> (Arc<dyn Primitive>, Arc<dyn Primitive>) {
        fn create_ceil(
            svrnc: &Svrnc,
            rot_height: f32,
            angle_rad: f32,
        ) -> (Arc<dyn Primitive>, Arc<dyn Primitive>) {
            let mut block = HittableList::with_capacity(16);

            let offset = 10.0;
            let r_edge = Arc::new(Quad::new(
                Point3::new(-offset, 0.0, -Svrnc::CEIL_LOWER_B),
                Vec3::new(10.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0, Svrnc::CEIL_LOWER_B),
                svrnc.mats.gray(),
            ));
            let b_edge = Arc::new(Quad::new(
                Point3::new(0.0, 0.0, 0.0),
                Vec3::new(Svrnc::CEIL_LOWER_B, 0.0, 0.0),
                Vec3::new(0.0, 0.0, offset),
                svrnc.mats.gray(),
            ));

            let trapezoid: Arc<dyn Primitive> = Arc::new(Trapezoid::new(
                Point3::splat(0.0),
                Vec3::new(Svrnc::CEIL_LOWER_B, 0.0, 0.0),
                Vec3::new(0.0, Svrnc::CEIL_HEIGHT, 0.0),
                Svrnc::CEIL_RATIO,
                svrnc.mats.gray(),
            ));

            let b_trapezoid = Arc::new(RotationX::new(trapezoid.clone(), Svrnc::CEIL_ANGLE));

            let mut f_trapezoid: Arc<dyn Primitive> =
                Arc::new(RotationX::new(trapezoid.clone(), -Svrnc::CEIL_ANGLE));
            f_trapezoid = Arc::new(Translation::new(
                f_trapezoid,
                Vec3::new(0.0, 0.0, -Svrnc::CEIL_LOWER_B),
            ));

            let mut r_trapezoid: Arc<dyn Primitive> =
                Arc::new(RotationY::new(b_trapezoid.clone(), -90.0));
            r_trapezoid = Arc::new(Translation::new(
                r_trapezoid,
                Vec3::new(0.0, 0.0, -Svrnc::CEIL_LOWER_B),
            ));

            let mut l_trapezoid: Arc<dyn Primitive> =
                Arc::new(RotationY::new(b_trapezoid.clone(), 90.0));
            l_trapezoid = Arc::new(Translation::new(
                l_trapezoid,
                Vec3::new(Svrnc::CEIL_LOWER_B, 0.0, 0.0),
            ));

            block.push(b_trapezoid);
            block.push(r_trapezoid);
            block.push(f_trapezoid);
            block.push(l_trapezoid);
            block.push(r_edge);
            block.push(b_edge);

            let light_width = Svrnc::CEIL_UPPER_B;
            let light_depth = Svrnc::CEIL_UPPER_B;

            let emissive_origin = Point3::new(
                light_width * angle_rad.cos() - 1.0,
                rot_height - 0.1,
                -light_depth - light_depth * angle_rad.cos(),
            );
            let emissive_u = Vec3::new(light_width, 0.0, 0.0);
            let emissive_v = Vec3::new(0.0, 0.0, light_depth);

            let light_material = Arc::new(DiffuseLight::from_color(Color::splat(5.5)));
            block.push(Arc::new(Quad::new(
                emissive_origin.clone(),
                emissive_u.clone(),
                emissive_v.clone(),
                light_material,
            )));

            let light = Arc::new(Quad::new(
                emissive_origin,
                emissive_v,
                emissive_u,
                svrnc.data.empty(),
            ));

            let ceil = Arc::new(Translation::new(
                Arc::new(block),
                Vec3::new(10.0, Svrnc::SCENE_HEIGHT, Svrnc::SCENE_DEPTH - 10.0),
            ));

            (ceil, light)
        }

        let mut ceilling = HittableList::with_capacity(16);
        let mut lights = HittableList::with_capacity(16);

        let base = Self::CEIL_LOWER_B;
        let ceilling_depth = Self::SCENE_DEPTH - base;
        let ceilling_widh = Self::SCENE_WIDTH;

        let angle_rad = f32::to_radians(Self::CEIL_ANGLE);
        let rot_height = Self::CEIL_HEIGHT * angle_rad.cos() - 3.0;

        let (ceil, light) = create_ceil(self, rot_height, angle_rad);

        for i in (0..ceilling_depth as usize).step_by(265) {
            for j in (0..ceilling_widh as usize).step_by(265) {
                let i_f = i as f32;
                let j_f = j as f32;

                ceilling.push(Arc::new(Translation::new(
                    ceil.clone(),
                    Vec3::new(j_f, 0.0, -i_f),
                )));

                lights.push(Arc::new(Translation::new(
                    light.clone(),
                    Vec3::new(j_f, 0.0, -i_f),
                )));
            }
        }

        //I've to fill the edges
        let l_edge = Arc::new(Quad::new(
            Point3::new(Self::SCENE_WIDTH - 10.0, Self::SCENE_HEIGHT, 0.0),
            Vec3::new(10.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, Self::SCENE_DEPTH),
            self.mats.gray(),
        ));
        let f_edge = Arc::new(Quad::new(
            Point3::new(0.0, Self::SCENE_HEIGHT, 0.0),
            Vec3::new(Self::SCENE_WIDTH, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 25.0),
            self.mats.gray(),
        ));

        let f_ceilling = Arc::new(Quad::new(
            Point3::new(0.0, Self::SCENE_HEIGHT + rot_height, 0.0),
            Vec3::new(Self::SCENE_WIDTH, 0.0, 0.0),
            Vec3::new(0.0, 0.0, Self::SCENE_DEPTH),
            self.mats.gray(),
        ));

        ceilling.push(f_ceilling);
        ceilling.push(l_edge);
        ceilling.push(f_edge);

        (Arc::new(ceilling), Arc::new(lights))
    }

    fn create_back_wall(&self) -> Arc<dyn Primitive> {
        let mut block = HittableList::with_capacity(8);

        let plinth = Arc::new(Quad::new(
            Point3::new(
                -Self::PLINTH_DEPTH,
                0.0,
                Self::SCENE_DEPTH + Self::PLINTH_DEPTH,
            ),
            Vec3::new(Self::SCENE_WIDTH + Self::PLINTH_DEPTH * 2.0, 0.0, 0.0),
            Vec3::new(0.0, Self::PLINTH_HEIGHT, 0.0),
            self.mats.teal(),
        ));

        let plinth_fill = Arc::new(Quad::new(
            Point3::new(
                -Self::PLINTH_DEPTH,
                Self::PLINTH_HEIGHT,
                Self::SCENE_DEPTH + Self::PLINTH_DEPTH,
            ),
            Vec3::new(0.0, 0.0, Self::PLINTH_HEIGHT),
            Vec3::new(Self::SCENE_WIDTH + Self::PLINTH_DEPTH * 2.0, 0.0, 0.0),
            self.mats.white(),
        ));

        let header = Arc::new(Quad::new(
            Point3::new(
                0.0,
                Self::SCENE_HEIGHT - Self::HEADER_HEIGHT,
                Self::SCENE_DEPTH,
            ),
            Vec3::new(Self::SCENE_WIDTH, 0.0, 0.0),
            Vec3::new(0.0, Self::HEADER_HEIGHT, 0.0),
            self.data.aluminum(),
        ));

        let m_header = Arc::new(Quad::new(
            Point3::new(
                0.0,
                Self::SCENE_HEIGHT - Self::HEADER_HEIGHT + 5.0,
                Self::SCENE_DEPTH - 0.1,
            ),
            Vec3::new(Self::SCENE_WIDTH, 0.0, 0.0),
            Vec3::new(0.0, Self::HEADER_HEIGHT * 0.5, 0.0),
            self.mats.steel_black(),
        ));

        let wall = Arc::new(Quad::new(
            Point3::new(0.0, Self::PLINTH_HEIGHT, Self::SCENE_DEPTH),
            Vec3::new(Self::SCENE_WIDTH, 0.0, 0.0),
            Vec3::new(0.0, Self::WALL_HEIGHT, 0.0),
            self.mats.white(),
        ));

        let edge = Arc::new(Quad::new(
            Point3::new(0.0, Self::PLINTH_HEIGHT, Self::SCENE_DEPTH - 0.1),
            Vec3::new(2.5, 0.0, 0.0),
            Vec3::new(0.0, Self::WALL_HEIGHT, 0.0),
            self.data.aluminum(),
        ));

        for offset in (264..Self::SCENE_WIDTH as usize).step_by(264) {
            block.push(Arc::new(Translation::new(
                edge.clone(),
                Vec3::new(offset as f32, 0.0, 0.0),
            )));
        }

        block.push(header);
        block.push(m_header);
        block.push(wall);
        block.push(plinth);
        block.push(plinth_fill);
        block.push(edge);

        Arc::new(block)
    }

    fn create_desktops(&self) -> Arc<dyn Primitive> {
        fn create_panel(svr: &Svrnc) -> Arc<dyn Primitive> {
            let mut block = HittableList::with_capacity(4);

            let offset_from_ground = 50.0;
            let mut panel: Arc<dyn Primitive> = Arc::new(Cuboid::new(
                Point3::splat(0.0),
                Point3::new(Svrnc::PANEL_WIDTH, Svrnc::PANEL_HEIGHT, Svrnc::PANEL_DEPTH),
                svr.mats.melamine(),
            ));
            panel = Arc::new(Translation::new(
                panel,
                Vec3::new(0.0, offset_from_ground, 0.0),
            ));

            let offset = 10.0;

            let f_frame = Arc::new(Quad::new(
                Point3::new(offset * 0.5, offset_from_ground + offset * 0.5, -0.1),
                Vec3::new(0.0, Svrnc::PANEL_HEIGHT - offset, 0.0),
                Vec3::new(Svrnc::PANEL_WIDTH - offset, 0.0, 0.0),
                svr.mats.teal(),
            ));

            let mut b_frame: Arc<dyn Primitive> = Arc::new(RotationY::new(f_frame.clone(), -180.0));
            b_frame = Arc::new(Translation::new(
                b_frame,
                Vec3::new(Svrnc::PANEL_WIDTH, 0.0, Svrnc::PANEL_DEPTH + 0.2),
            ));

            let edge = Arc::new(Quad::new(
                Point3::new(Svrnc::PANEL_WIDTH, offset, 0.0),
                Vec3::new(0.0, 0.0, Svrnc::PANEL_DEPTH),
                Vec3::new(0.0, Svrnc::PANEL_HEIGHT, 0.0),
                svr.mats.teal(),
            ));

            block.push(panel);
            block.push(f_frame);
            block.push(b_frame);
            block.push(edge);

            Arc::new(block)
        }

        fn create_desktop(svr: &Svrnc) -> Arc<dyn Primitive> {
            let mut block = HittableList::with_capacity(8);

            let offset_from_ground = 85.0 + Svrnc::TABLE_HEIGHT;
            let mut table: Arc<dyn Primitive> = Arc::new(Cuboid::new(
                Point3::splat(0.0),
                Point3::new(Svrnc::TABLE_WIDTH, Svrnc::TABLE_HEIGHT, Svrnc::TABLE_DEPTH),
                svr.mats.melamine(),
            ));
            table = Arc::new(Translation::new(
                table,
                Vec3::new(0.0, offset_from_ground - Svrnc::TABLE_HEIGHT, 0.0),
            ));

            let box_width = Svrnc::TABLE_WIDTH * 0.35 - 10.0;
            let wall_width = 5.0;

            let f_box_height = Svrnc::TABLE_HEIGHT * 3.0;
            let mut f_box: Arc<dyn Primitive> = Arc::new(Cuboid::new(
                Point3::splat(0.0),
                Point3::new(box_width, f_box_height, Svrnc::TABLE_DEPTH - 1.0),
                svr.mats.melamine(),
            ));
            f_box = Arc::new(Translation::new(
                f_box,
                Vec3::new(wall_width + 1.0, offset_from_ground - f_box_height, 1.0),
            ));

            let s_box_height = Svrnc::TABLE_HEIGHT * 4.0;
            let mut s_box: Arc<dyn Primitive> = Arc::new(Cuboid::new(
                Point3::splat(0.0),
                Point3::new(box_width, s_box_height, Svrnc::TABLE_DEPTH - 1.0),
                svr.mats.melamine(),
            ));

            s_box = Arc::new(Translation::new(
                s_box,
                Vec3::new(
                    wall_width + 1.0,
                    offset_from_ground - 1.0 - f_box_height - s_box_height,
                    1.0,
                ),
            ));

            let wall_height = f_box_height + s_box_height + 2.0;
            let mut r_wall: Arc<dyn Primitive> = Arc::new(Cuboid::new(
                Point3::splat(0.0),
                Point3::new(wall_width, wall_height, Svrnc::TABLE_DEPTH - 1.0),
                svr.mats.melamine(),
            ));

            r_wall = Arc::new(Translation::new(
                r_wall,
                Vec3::new(0.0, offset_from_ground - wall_height, 0.0),
            ));

            let l_wall = Arc::new(Translation::new(
                r_wall.clone(),
                Vec3::new(box_width + 1.0, 0.0, 0.0),
            ));

            block.push(f_box);
            block.push(s_box);
            block.push(r_wall);
            block.push(l_wall);
            block.push(table);

            Arc::new(block)
        }

        let mut block = HittableList::with_capacity(16);

        let location_box_d = Self::SCENE_DEPTH * 0.40;
        let location_box_w = Self::SCENE_WIDTH - 530.0;

        let mut cbox: Arc<dyn Primitive> = Arc::new(Cuboid::new(
            Point3::splat(0.0),
            Point3::new(100.0, 150.0, 100.0),
            self.mats.melamine(),
        ));
        cbox = Arc::new(Translation::new(
            cbox,
            Vec3::new(location_box_w, 0.0, location_box_d),
        ));

        let panel = create_panel(self);
        let r_panel = Arc::new(Translation::new(
            panel.clone(),
            Vec3::new(
                location_box_w - Self::PANEL_WIDTH - 0.1,
                0.0,
                location_box_d + 75.0,
            ),
        ));

        let mut l_panel: Arc<dyn Primitive> = Arc::new(RotationY::new(panel.clone(), 180.0));
        l_panel = Arc::new(Translation::new(
            l_panel.clone(),
            Vec3::new(
                location_box_w + Self::PANEL_WIDTH + 100.1,
                0.0,
                location_box_d + Self::PANEL_DEPTH + 25.0,
            ),
        ));

        let mut f_panel: Arc<dyn Primitive> = Arc::new(RotationY::new(panel.clone(), -90.0));
        f_panel = Arc::new(Translation::new(
            f_panel.clone(),
            Vec3::new(
                location_box_w + 25.0,
                0.0,
                location_box_d - Self::PANEL_WIDTH - 0.1,
            ),
        ));

        let mut b_panel: Arc<dyn Primitive> = Arc::new(RotationY::new(panel, 90.0));
        b_panel = Arc::new(Translation::new(
            b_panel,
            Vec3::new(
                location_box_w + 75.0,
                0.0,
                location_box_d + Self::PANEL_WIDTH + 100.1,
            ),
        ));

        let desktop = create_desktop(self);

        let r_desktop = Arc::new(Translation::new(
            desktop.clone(),
            Vec3::new(
                location_box_w - Self::TABLE_WIDTH - 0.1,
                0.0,
                location_box_d + 75.0 - Self::TABLE_DEPTH - Self::PANEL_DEPTH,
            ),
        ));

        let mut l_desktop: Arc<dyn Primitive> = Arc::new(RotationY::new(desktop.clone(), 180.0));
        l_desktop = Arc::new(Translation::new(
            l_desktop,
            Vec3::new(
                location_box_w + Self::TABLE_WIDTH + 100.1,
                0.0,
                location_box_d + Self::TABLE_DEPTH + (Self::PANEL_DEPTH + 25.0),
            ),
        ));

        let mut f_desktop: Arc<dyn Primitive> = Arc::new(RotationY::new(desktop.clone(), -90.0));
        f_desktop = Arc::new(Translation::new(
            f_desktop,
            Vec3::new(
                location_box_w + Self::TABLE_DEPTH + Self::PANEL_DEPTH + 25.0,
                0.0,
                location_box_d - Self::TABLE_WIDTH - 1.0,
            ),
        ));

        let mut b_desktop: Arc<dyn Primitive> = Arc::new(RotationY::new(desktop.clone(), 90.0));
        b_desktop = Arc::new(Translation::new(
            b_desktop,
            Vec3::new(
                location_box_w + (Self::PANEL_DEPTH - 75.0),
                0.0,
                location_box_d + Self::TABLE_WIDTH + 100.1,
            ),
        ));

        block.push(cbox);
        block.push(r_panel);
        block.push(l_panel);
        block.push(b_panel);
        block.push(f_panel);

        block.push(r_desktop);
        block.push(l_desktop);
        block.push(f_desktop);
        block.push(b_desktop);

        Arc::new(block)
    }

    fn create_terminal(&self) -> (Arc<dyn Primitive>, Arc<dyn Primitive>) {
        const DEPTH: f32 = Svrnc::TABLE_DEPTH * 0.5;
        const WIDTH: f32 = Svrnc::TABLE_WIDTH * 0.5 - 10.0;
        const HEIGHT: f32 = Svrnc::PANEL_HEIGHT * 0.33;
        const OFFSET: f32 = 1.0;
        const THICKNESS: f32 = 3.0;

        fn create_monitor(svrnc: &Svrnc) -> Arc<dyn Primitive> {
            let mut block = HittableList::with_capacity(32);

            let h_panel = Arc::new(Cuboid::new(
                Point3::new(THICKNESS, 0.0, 0.0),
                Point3::new(WIDTH - 2.0 * THICKNESS, THICKNESS, DEPTH * 0.33),
                svrnc.mats.white(),
            ));

            let v_panel: Arc<dyn Primitive> = Arc::new(Cuboid::new(
                Point3::ORIGIN,
                Point3::new(THICKNESS, HEIGHT * 0.5 - OFFSET, DEPTH * 0.33),
                svrnc.mats.white(),
            ));

            let top = Arc::new(Translation::new(
                h_panel.clone(),
                Vec3::new(0.0, HEIGHT - THICKNESS, 0.0),
            ));

            let u_right = Arc::new(Translation::new(
                v_panel.clone(),
                Vec3::new(0.0, HEIGHT * 0.5 + OFFSET, 0.0),
            ));

            let u_left = Arc::new(Translation::new(
                u_right.clone(),
                Vec3::new(WIDTH - 3.0, 0.0, 0.0),
            ));

            let bottom = h_panel;

            let b_right = v_panel;

            let b_left = Arc::new(Translation::new(
                b_right.clone(),
                Vec3::new(WIDTH - 3.0, 0.0, 0.0),
            ));

            block.push(top);
            block.push(bottom);
            block.push(u_left);
            block.push(u_right);
            block.push(b_left);
            block.push(b_right);

            //--------------------------

            const M_HEIGHT: f32 = HEIGHT * 0.5 - OFFSET - THICKNESS;
            let angle: f32 = f32::atan(M_HEIGHT * 0.2 / DEPTH * 0.66).to_degrees();

            let mut right_panel = HittableList::with_capacity(8);

            let b_panel = Arc::new(Quad::new(
                Point3::new(0.0, THICKNESS, DEPTH * 0.33),
                Vec3::new(0.0, 0.0, DEPTH * 0.66),
                Vec3::new(0.0, M_HEIGHT, 0.0),
                svrnc.mats.white(),
            ));

            let b_triangle = Arc::new(Triangle::new(
                Point3::new(0.0, THICKNESS, DEPTH * 0.33),
                Vec3::new(0.0, 0.0, DEPTH * 0.66),
                Vec3::new(0.0, -THICKNESS, 0.0),
                svrnc.mats.white(),
            ));

            let u_panel = Arc::new(Translation::new(
                b_panel.clone(),
                Vec3::new(0.0, M_HEIGHT + 2.0 + OFFSET, 0.0),
            ));

            let u_triangle = Arc::new(Triangle::new(
                Point3::new(0.0, HEIGHT - THICKNESS, DEPTH * 0.33),
                Vec3::new(0.0, 0.0, DEPTH * 0.66),
                Vec3::new(0.0, THICKNESS, 0.0),
                svrnc.mats.white(),
            ));

            let fill_back = Arc::new(Quad::new(
                Point3::new(0.0, THICKNESS, DEPTH),
                Vec3::new(0.0, HEIGHT - 2.0 * THICKNESS, 0.0),
                Vec3::new(WIDTH, 0.0, 0.0),
                svrnc.mats.white(),
            ));

            let mut fill_bottom: Arc<dyn Primitive> = Arc::new(Quad::new(
                Point3::new(0.0, 0.0, DEPTH * 0.33),
                Vec3::new(WIDTH, 0.0, 0.0),
                Vec3::new(0.0, 0.0, DEPTH * 0.66),
                svrnc.mats.white(),
            ));
            fill_bottom = Arc::new(RotationX::new(fill_bottom, angle));

            let mut fill_top: Arc<dyn Primitive> = Arc::new(Quad::new(
                Point3::new(0.0, HEIGHT, DEPTH * 0.33),
                Vec3::new(0.0, 0.0, DEPTH * 0.66),
                Vec3::new(WIDTH, 0.0, 0.0),
                svrnc.mats.white(),
            ));
            fill_top = Arc::new(RotationX::new(fill_top, -angle));

            right_panel.push(b_panel);
            right_panel.push(b_triangle);
            right_panel.push(u_panel);
            right_panel.push(u_triangle);

            let right_panel = Arc::new(right_panel);

            let mut left_panel: Arc<dyn Primitive> = Arc::new(MirrorYZ::new(right_panel.clone()));
            left_panel = Arc::new(Translation::new(left_panel, Vec3::new(WIDTH, 0.0, 0.0)));

            block.push(fill_back);
            block.push(fill_bottom);
            block.push(fill_top);
            block.push(left_panel);
            block.push(right_panel);

            //--------------------------

            const INNER_HEIGHT: f32 = HEIGHT - 6.0;
            const INNER_WIDTH: f32 = WIDTH - 6.0;
            const INNER_THICKNESS: f32 = 3.0;

            let h_panel = Arc::new(Cuboid::new(
                Point3::ORIGIN,
                Point3::new(INNER_WIDTH, INNER_THICKNESS, DEPTH * 0.33),
                svrnc.mats.black(),
            ));

            let v_panel: Arc<dyn Primitive> = Arc::new(Cuboid::new(
                Point3::ORIGIN,
                Point3::new(INNER_THICKNESS, INNER_HEIGHT, DEPTH * 0.33),
                svrnc.mats.black(),
            ));

            let top = Arc::new(Translation::new(
                h_panel.clone(),
                Vec3::new(3.0, INNER_HEIGHT, 3.0),
            ));

            let right = Arc::new(Translation::new(
                v_panel.clone(),
                Vec3::new(3.0, INNER_THICKNESS, 3.0),
            ));

            let left = Arc::new(Translation::new(
                v_panel,
                Vec3::new(INNER_WIDTH, INNER_THICKNESS, 3.0),
            ));

            let bottom = Arc::new(Translation::new(h_panel, Vec3::new(3.0, 3.0, 3.0)));

            block.push(top);
            block.push(bottom);
            block.push(left);
            block.push(right);

            //--------------------------

            const YET_INNER_HEIGHT: f32 = INNER_HEIGHT - INNER_THICKNESS * 2.0;
            const YET_INNER_WIDTH: f32 = INNER_WIDTH - INNER_THICKNESS * 2.0;
            const H_THICKNESS: f32 = THICKNESS + INNER_THICKNESS;
            const VL_THICKNESS: f32 = H_THICKNESS;
            const VR_THICKNESS: f32 = YET_INNER_WIDTH * 0.33;
            const YET_INNER_DEPTH: f32 = 10.0;

            let mut lv_panel: Arc<dyn Primitive> = Arc::new(Cuboid::new(
                Point3::ORIGIN,
                Point3::new(VL_THICKNESS, YET_INNER_HEIGHT, YET_INNER_DEPTH),
                svrnc.mats.teal(),
            ));
            lv_panel = Arc::new(Translation::new(
                lv_panel,
                Vec3::new(YET_INNER_WIDTH, INNER_THICKNESS + THICKNESS, 0.0),
            ));

            let mut rv_panel: Arc<dyn Primitive> = Arc::new(Cuboid::new(
                Point3::ORIGIN,
                Point3::new(VR_THICKNESS, YET_INNER_HEIGHT, YET_INNER_DEPTH),
                svrnc.mats.teal(),
            ));
            rv_panel = Arc::new(Translation::new(
                rv_panel,
                Vec3::new(
                    INNER_THICKNESS + THICKNESS,
                    INNER_THICKNESS + THICKNESS,
                    0.0,
                ),
            ));

            let panel: Arc<dyn Primitive> = Arc::new(Cuboid::new(
                Point3::ORIGIN,
                Point3::new(
                    YET_INNER_WIDTH - VL_THICKNESS - VR_THICKNESS,
                    H_THICKNESS,
                    YET_INNER_DEPTH,
                ),
                svrnc.mats.teal(),
            ));

            let top_panel = Arc::new(Translation::new(
                panel.clone(),
                Vec3::new(
                    VR_THICKNESS + INNER_THICKNESS + THICKNESS,
                    YET_INNER_HEIGHT,
                    0.0,
                ),
            ));

            let bottom_panel = Arc::new(Translation::new(
                panel,
                Vec3::new(
                    VR_THICKNESS + INNER_THICKNESS + THICKNESS,
                    INNER_THICKNESS + THICKNESS,
                    0.0,
                ),
            ));

            let light = Arc::new(DiffuseLight::from_color(Color::splat(1.0)));

            let screen = Arc::new(Quad::new(
                Point3::new(
                    VR_THICKNESS + INNER_THICKNESS + THICKNESS,
                    INNER_THICKNESS + H_THICKNESS + THICKNESS,
                    YET_INNER_DEPTH * 0.5,
                ),
                Vec3::new(0.0, YET_INNER_HEIGHT - H_THICKNESS * 2.0, 0.0),
                Vec3::new(YET_INNER_WIDTH - VL_THICKNESS - VR_THICKNESS, 0.0, 0.0),
                light,
            ));

            let glass = Arc::new(Quad::new(
                Point3::new(
                    VR_THICKNESS + INNER_THICKNESS + THICKNESS,
                    INNER_THICKNESS + H_THICKNESS + THICKNESS,
                    YET_INNER_DEPTH * 0.5 - 0.1,
                ),
                Vec3::new(0.0, YET_INNER_HEIGHT - H_THICKNESS * 2.0, 0.0),
                Vec3::new(YET_INNER_WIDTH - VL_THICKNESS - VR_THICKNESS, 0.0, 0.0),
                svrnc.data.glass(),
            ));

            block.push(lv_panel);
            block.push(rv_panel);
            block.push(top_panel);
            block.push(bottom_panel);

            block.push(screen);
            block.push(glass);

            let monitor = Arc::new(block);
            //--------------------------
            let mut block = HittableList::with_capacity(8);
            block.push(Arc::new(RotationX::new(monitor, -angle * 2.0)));

            const RADIUS: f32 = 6.0;
            const C_HEIGHT: f32 = 3.0;
            const SUPPORT_DEPTH: f32 = RADIUS * 2.0 + 5.0;
            const SUPPORT_OFFSET: f32 = (-HEIGHT * 0.5) + RADIUS;

            let v_panel = Arc::new(Cuboid::new(
                Point3::ORIGIN,
                Point3::new(THICKNESS, HEIGHT, SUPPORT_DEPTH),
                svrnc.mats.white(),
            ));

            let r_panel = Arc::new(Translation::new(
                v_panel.clone(),
                Vec3::new(
                    -THICKNESS - 0.1,
                    SUPPORT_OFFSET,
                    DEPTH * 0.5 - SUPPORT_DEPTH * 0.5,
                ),
            ));

            let l_panel = Arc::new(Translation::new(
                v_panel,
                Vec3::new(
                    WIDTH + 0.1,
                    SUPPORT_OFFSET,
                    DEPTH * 0.5 - SUPPORT_DEPTH * 0.5,
                ),
            ));

            let mut h_panel: Arc<dyn Primitive> = Arc::new(Cuboid::new(
                Point3::ORIGIN,
                Point3::new(WIDTH, THICKNESS, DEPTH * 0.5 - SUPPORT_DEPTH * 0.5),
                svrnc.mats.white(),
            ));
            h_panel = Arc::new(Translation::new(
                h_panel,
                Vec3::new(
                    0.0,
                    SUPPORT_OFFSET + THICKNESS,
                    DEPTH * 0.5 - SUPPORT_DEPTH * 0.5,
                ),
            ));

            let mut cyl: Arc<dyn Primitive> = Arc::new(Cylinder::new(
                Point3::ORIGIN,
                RADIUS,
                C_HEIGHT,
                svrnc.mats.black(),
            ));
            cyl = Arc::new(RotationX::new(cyl, 90.0));
            cyl = Arc::new(RotationY::new(cyl, 90.0));

            let r_cyl = Arc::new(Translation::new(
                cyl.clone(),
                Vec3::new(-THICKNESS - C_HEIGHT - 0.1, HEIGHT * 0.5, DEPTH * 0.5),
            ));

            let l_cyl = Arc::new(Translation::new(
                cyl.clone(),
                Vec3::new(WIDTH + C_HEIGHT + 0.1, HEIGHT * 0.5, DEPTH * 0.5),
            ));

            block.push(l_cyl);
            block.push(r_cyl);
            block.push(r_panel);
            block.push(l_panel);
            block.push(h_panel);

            //--------------------------
            let mut base: Arc<dyn Primitive> = Arc::new(Cuboid::new(
                Point3::ORIGIN,
                Point3::new(WIDTH, THICKNESS, DEPTH * 0.75),
                svrnc.mats.white(),
            ));
            base = Arc::new(Translation::new(
                base,
                Vec3::new(0.0, SUPPORT_OFFSET - THICKNESS * 2.0, DEPTH * 0.25),
            ));

            let mut c_base: Arc<dyn Primitive> = Arc::new(Cylinder::new(
                Point3::ORIGIN,
                SUPPORT_DEPTH * 0.5 - 1.0,
                THICKNESS,
                svrnc.mats.white(),
            ));
            c_base = Arc::new(Translation::new(
                c_base,
                Vec3::new(0.0, SUPPORT_OFFSET, DEPTH * 0.5),
            ));

            block.push(base);
            block.push(c_base);

            Arc::new(block)
        }

        const KEYBOARD_DEPTH: f32 = DEPTH * 0.5;
        const KEYBOARD_WIDTH: f32 = WIDTH + 10.0;
        const KEYBOARD_HEIGHT: f32 = THICKNESS * 3.0;

        fn create_keyboard(svrnc: &Svrnc) -> Arc<dyn Primitive> {
            let mut block = HittableList::with_capacity(16);

            let mut border: Arc<dyn Primitive> = Arc::new(Cuboid::new(
                Point3::ORIGIN,
                Point3::new(KEYBOARD_WIDTH + 2.0, THICKNESS, KEYBOARD_DEPTH + 2.0),
                svrnc.mats.white(),
            ));
            border = Arc::new(Translation::new(border, Vec3::new(-1.0, 0.0, -1.0)));

            let mut base: Arc<dyn Primitive> = Arc::new(Cuboid::new(
                Point3::ORIGIN,
                Point3::new(KEYBOARD_WIDTH, KEYBOARD_HEIGHT, KEYBOARD_DEPTH),
                svrnc.mats.white(),
            ));
            base = Arc::new(Translation::new(base, Vec3::new(0.0, THICKNESS, 0.0)));

            let r_triangle = Arc::new(Triangle::new(
                Point3::new(0.0, KEYBOARD_HEIGHT + THICKNESS, KEYBOARD_DEPTH),
                Vec3::new(0.0, KEYBOARD_HEIGHT * 0.5, 0.0),
                Vec3::new(0.0, 0.0, -KEYBOARD_DEPTH),
                svrnc.mats.white(),
            ));

            let mut l_triangle: Arc<dyn Primitive> = Arc::new(MirrorYZ::new(r_triangle.clone()));
            l_triangle = Arc::new(Translation::new(
                l_triangle,
                Vec3::new(KEYBOARD_WIDTH, 0.0, 0.0),
            ));

            let fill_back = Arc::new(Quad::new(
                Point3::new(0.0, KEYBOARD_HEIGHT + THICKNESS, KEYBOARD_DEPTH),
                Vec3::new(KEYBOARD_WIDTH, 0.0, 0.0),
                Vec3::new(0.0, KEYBOARD_HEIGHT * 0.5, 0.0),
                svrnc.mats.white(),
            ));

            let angle: f32 = f32::atan(KEYBOARD_HEIGHT * 0.5 / KEYBOARD_DEPTH).to_degrees();

            let mut fill_front: Arc<dyn Primitive> = Arc::new(Quad::new(
                Point3::new(0.0, KEYBOARD_HEIGHT + THICKNESS, 0.0),
                Vec3::new(0.0, 0.0, KEYBOARD_DEPTH),
                Vec3::new(KEYBOARD_WIDTH, 0.0, 0.0),
                svrnc.mats.white(),
            ));

            fill_front = Arc::new(RotationX::new(fill_front, angle));

            let offset = 5.0;
            let mut fill_top: Arc<dyn Primitive> = Arc::new(Quad::new(
                Point3::new(
                    offset * 0.5,
                    KEYBOARD_HEIGHT + THICKNESS + 1.0,
                    offset * 0.5,
                ),
                Vec3::new(0.0, 0.0, KEYBOARD_DEPTH - offset),
                Vec3::new(KEYBOARD_WIDTH - offset, 0.0, 0.0),
                svrnc.mats.teal(),
            ));
            fill_top = Arc::new(RotationX::new(fill_top, angle));

            let mut touchpad: Arc<dyn Primitive> = Arc::new(Hemisphere::new(
                Point3::new(
                    KEYBOARD_WIDTH * 0.125,
                    KEYBOARD_HEIGHT + THICKNESS + 1.0,
                    KEYBOARD_DEPTH * 0.5,
                ),
                5.0,
                svrnc.mats.black(),
                UnitVec3::J,
            ));
            touchpad = Arc::new(RotationX::new(touchpad, angle));

            block.push(border);
            block.push(base);
            block.push(r_triangle);
            block.push(l_triangle);
            block.push(fill_back);
            block.push(fill_front);
            block.push(fill_top);
            block.push(touchpad);

            Arc::new(block)
        }

        let location_box_d = Self::SCENE_DEPTH * 0.60;
        let location_box_w = Self::SCENE_WIDTH - 530.0;
        let offset_from_ground = 85.0 + Svrnc::TABLE_HEIGHT;

        let monitor = create_monitor(self);
        let keyboard = create_keyboard(self);

        let mut r_terminal = HittableList::with_capacity(2);
        let r_monitor = Arc::new(Translation::new(
            monitor.clone(),
            Vec3::new(
                location_box_w - Self::TABLE_WIDTH * 0.5 - WIDTH * 0.5,
                offset_from_ground + (HEIGHT * 0.5) + 2.0,
                location_box_d - Self::TABLE_DEPTH + DEPTH * 0.5 - Self::PANEL_DEPTH - 5.0,
            ),
        ));

        let r_keyboard = Arc::new(Translation::new(
            keyboard.clone(),
            Vec3::new(
                location_box_w - Self::TABLE_WIDTH * 0.5 - KEYBOARD_WIDTH * 0.5,
                offset_from_ground + 0.1,
                location_box_d - Self::TABLE_DEPTH - KEYBOARD_DEPTH * 0.5 - 5.0,
            ),
        ));
        r_terminal.push(r_monitor);
        r_terminal.push(r_keyboard);

        let mut l_terminal = HittableList::with_capacity(2);
        let mut l_monitor: Arc<dyn Primitive> = Arc::new(RotationY::new(monitor, -90.0));
        l_monitor = Arc::new(Translation::new(
            l_monitor,
            Vec3::new(
                location_box_w + DEPTH + Self::PANEL_DEPTH + Self::TABLE_DEPTH * 0.5 - 20.0,
                offset_from_ground + (HEIGHT * 0.5) + 2.0,
                location_box_d - Self::TABLE_WIDTH - WIDTH * 0.5 - 5.0,
            ),
        ));
        let mut l_keyboard: Arc<dyn Primitive> = Arc::new(RotationY::new(keyboard, -90.0));
        l_keyboard = Arc::new(Translation::new(
            l_keyboard,
            Vec3::new(
                location_box_w + Self::PANEL_DEPTH + Self::TABLE_DEPTH + KEYBOARD_DEPTH * 0.5,
                offset_from_ground + 0.1,
                location_box_d - Self::TABLE_WIDTH - KEYBOARD_WIDTH * 0.5 - 5.0,
            ),
        ));
        l_terminal.push(l_monitor);
        l_terminal.push(l_keyboard);

        (Arc::new(l_terminal), Arc::new(r_terminal))
    }
}

impl Palette {
    fn teal(&self) -> Arc<dyn Material> {
        self.teal.clone()
    }

    fn black(&self) -> Arc<dyn Material> {
        self.black.clone()
    }

    fn white(&self) -> Arc<dyn Material> {
        self.white.clone()
    }

    fn green(&self) -> Arc<dyn Material> {
        self.green.clone()
    }

    fn gray(&self) -> Arc<dyn Material> {
        self.gray.clone()
    }

    fn melamine(&self) -> Arc<dyn Material> {
        self.melamine.clone()
    }

    fn steel_black(&self) -> Arc<dyn Material> {
        self.steel_black.clone()
    }
}

impl Default for Palette {
    fn default() -> Self {
        Self {
            teal: Arc::new(Lambertian::from_color(DARK_TEAL, 1.0)),
            green: Arc::new(Lambertian::from_color(MOSS_GREEN, 1.0)),
            white: Arc::new(Lambertian::from_color(WHITE, 1.0)),
            black: Arc::new(Lambertian::from_color(BLACK, 1.0)),
            gray: Arc::new(Lambertian::from_color(GRAY, 1.0)),
            steel_black: Arc::new(Metal::new(STEEL_BLACK, 1.0)),
            melamine: Arc::new(Lambertian::from_texture(
                Arc::new(Melamine::new(color::WHITE, 10.0)),
                1.0,
            )),
        }
    }
}
