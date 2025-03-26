use crate::{
    collections::ArrayList,
    math::{Coord, Point3, UnitVec3, Vec3},
    util::fmt,
};

impl Point3 {
    pub fn as_array(&self) -> [f32; 3] {
        unsafe { *(self as *const Point3 as *const [f32; 3]) }
    }

    pub fn into_array(self) -> [f32; 3] {
        [self.0[0], self.0[1], self.0[2]]
    }

    pub fn to_vec3(&self) -> Vec3 {
        Vec3(self.0.clone())
    }

    pub fn into_vec3(self) -> Vec3 {
        Vec3(self.0)
    }

    pub(crate) fn as_coord(&self) -> &Coord {
        &self.0
    }
}

impl std::str::FromStr for Point3 {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(',').collect();
        if parts.len() != 3 {
            return Err(format!("Invalid Point3 format: '{}'", s));
        }
        let x = parts[0].trim().parse::<f32>().map_err(|e| e.to_string())?;
        let y = parts[1].trim().parse::<f32>().map_err(|e| e.to_string())?;
        let z = parts[2].trim().parse::<f32>().map_err(|e| e.to_string())?;
        Ok(Point3::new(x, y, z))
    }
}

// ─────────────────────────────

impl Vec3 {
    pub fn as_array(&self) -> [f32; 3] {
        unsafe { *(self as *const Vec3 as *const [f32; 3]) }
    }

    pub fn into_array(self) -> [f32; 3] {
        [self.0[0], self.0[1], self.0[2]]
    }

    pub fn try_into_unit(self) -> Result<UnitVec3, fmt::UnitVecError> {
        UnitVec3::try_from_vec3(&self)
    }

    pub fn unchecked_into_unit(self) -> UnitVec3 {
        UnitVec3::unchecked_from_vec3(&self)
    }

    pub fn unchecked_into_unit_radius(self, r: f32) -> UnitVec3 {
        UnitVec3::unchecked_from_radius(&self, r)
    }

    pub(crate) fn as_coord(&self) -> &Coord {
        &self.0
    }
}

// ─────────────────────────────

impl UnitVec3 {
    pub fn as_vec3(&self) -> Vec3 {
        self.0.clone()
    }
    pub fn as_vec3_ref(&self) -> &Vec3 {
        &self.0
    }
    pub fn into_vec3(self) -> Vec3 {
        self.0
    }

    pub fn as_array(&self) -> [f32; 3] {
        unsafe { *(self as *const UnitVec3 as *const [f32; 3]) }
    }

    pub fn into_array(self) -> [f32; 3] {
        [self.0[0], self.0[1], self.0[2]]
    }

    pub(crate) fn as_coord(&self) -> &Coord {
        self.0.as_coord()
    }
}

// ─────────────────────────────

impl<T> ArrayList<T> {
    pub const fn as_mut_slice(&mut self) -> &mut [T] {
        unsafe { std::slice::from_raw_parts_mut(self.ptr.as_mut(), self.len()) }
    }

    pub const fn as_slice(&self) -> &[T] {
        unsafe { std::slice::from_raw_parts_mut(self.ptr.as_ptr(), self.len()) }
    }
}
