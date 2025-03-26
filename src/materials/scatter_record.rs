use super::*;

pub struct ScatterRecord {
    attenuation: Color,
    pdf: Option<Box<dyn pdf::Pdf>>,
    specular_ray: Option<Ray>,
}

impl ScatterRecord {
    pub fn new(attenuation: Color, pdf: Box<dyn pdf::Pdf>) -> Self {
        Self {
            attenuation,
            pdf: Some(pdf),
            specular_ray: None,
        }
    }

    pub fn new_specular(attenuation: Color, specular_ray: Ray) -> Self {
        Self {
            attenuation,
            pdf: None,
            specular_ray: Some(specular_ray),
        }
    }

    pub fn pdf(&self) -> Option<&dyn pdf::Pdf> {
        let pdf = self.pdf.as_ref()?;
        Some(pdf.as_ref())
    }

    pub fn specular(&self) -> Option<&Ray> {
        self.specular_ray.as_ref()
    }

    pub fn attenuation(&self) -> &Color {
        &self.attenuation
    }
}
