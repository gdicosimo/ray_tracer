use std::{
    error::Error,
    fmt::{self, Debug, Display, Formatter, Result as FmtResult},
};

use crate::{collections::ArrayList, geometry::Bvh, math::*};

#[derive(Debug, Clone)]
pub enum UnitVecError {
    ZeroVector,
}

// ─────────────────────────────

impl Debug for Coord {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        self.0.fmt(f)
    }
}

impl Display for Coord {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        self.0.fmt(f)
    }
}

// ─────────────────────────────

impl Debug for Point3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.debug_tuple("Point3").field(&self.0).finish()
    }
}

impl Display for Point3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "Point3 {}", self.0)
    }
}

// ─────────────────────────────

impl Debug for Vec3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.debug_tuple("Vec3 ").field(&self.0).finish()
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "Vec3 {}", self.0)
    }
}

// ─────────────────────────────

impl Debug for UnitVec3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.debug_tuple("UnitVec3").field(&self.0).finish()
    }
}

impl Display for UnitVec3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "UnitVec3: {}", self.0)
    }
}

impl Error for UnitVecError {}

impl fmt::Display for UnitVecError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UnitVecError::ZeroVector => write!(f, "It can not be normalized."),
        }
    }
}

// ─────────────────────────────

impl<T: Display> Display for ArrayList<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "[")?;

        for i in 0..self.len() {
            if i > 0 {
                write!(f, "")?;
            }
            write!(f, "{}", self.get(i).unwrap())?
        }

        write!(f, "]")
    }
}

impl<T: Debug> Debug for ArrayList<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.debug_list().entries(self.iter()).finish()
    }
}

// ─────────────────────────────

impl Display for Interval {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "[{:.2}, {:.2}]", self.min, self.max)
    }
}

// ─────────────────────────────

impl Display for Aabb {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Aabb {{ x: {}, y: {}, z: {} }}",
            self.x(),
            self.y(),
            self.z()
        )
    }
}

// ─────────────────────────────

impl Display for Bvh {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.fmt_recursive(f, 0)
    }
}

impl Bvh {
    fn fmt_recursive(&self, f: &mut Formatter<'_>, indent_level: usize) -> fmt::Result {
        let indent = "  ".repeat(indent_level);

        match self {
            Bvh::Empty => write!(f, "{}Empty BVH", indent),
            Bvh::Leaf(primitive) => {
                write!(
                    f,
                    "{}Leaf: Primitive with BBox: {}",
                    indent,
                    primitive.bounding_box()
                )
            }
            Bvh::Node { bbox, left, right } => {
                writeln!(f, "{}Node: bbox: {}", indent, bbox)?;
                writeln!(f, "{}  Left:", indent)?;
                left.fmt_recursive(f, indent_level + 2)?;
                writeln!(f, "\n{}  Right:", indent)?;
                right.fmt_recursive(f, indent_level + 2)
            }
        }
    }
}
