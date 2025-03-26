use super::*;

pub struct DiffuseLight {
    texture: Arc<dyn Texture>,
}

impl DiffuseLight {
    pub fn from_color(albedo: Color) -> Self {
        Self {
            texture: Arc::new(SolidColor::from_color(albedo)),
        }
    }
    pub fn from_texture(texture: Arc<dyn Texture>) -> Self {
        Self { texture }
    }
}

impl Material for DiffuseLight {
    fn emitted(&self, hit_record: &HitRecord) -> Color {
        if !hit_record.front_face() {
            return BLACK;
        }

        self.texture
            .value(hit_record.u(), hit_record.v(), hit_record.point())
    }
}
