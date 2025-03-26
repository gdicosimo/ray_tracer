use super::*;

pub struct Primitive<'pdf> {
    origin: Point3,
    primitive: &'pdf dyn geometry::Primitive,
}

impl<'pdf> Primitive<'pdf> {
    pub fn new(primitive: &'pdf dyn geometry::Primitive, origin: &Point3) -> Self {
        Self {
            origin: origin.clone(),
            primitive,
        }
    }
}

impl Pdf for Primitive<'_> {
    fn value(&self, direction: Vec3) -> f32 {
        self.primitive.pdf_value(&self.origin, &direction)
    }

    fn generate(&self) -> Vec3 {
        self.primitive.random(&self.origin)
    }
}
