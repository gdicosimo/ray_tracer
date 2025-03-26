use super::*;

pub struct Mixture<'ptr> {
    ptr1: &'ptr dyn Pdf,
    ptr2: &'ptr dyn Pdf,
}

impl<'ptr> Mixture<'ptr> {
    pub fn new(ptr1: &'ptr dyn Pdf, ptr2: &'ptr dyn Pdf) -> Self {
        Self { ptr1, ptr2 }
    }
}

impl Pdf for Mixture<'_> {
    fn value(&self, dir: Vec3) -> f32 {
        0.5 * self.ptr1.value(dir.clone()) + 0.5 * self.ptr2.value(dir)
    }

    fn generate(&self) -> Vec3 {
        if random_float() < 0.5 {
            self.ptr1.generate()
        } else {
            self.ptr2.generate()
        }
    }
}
