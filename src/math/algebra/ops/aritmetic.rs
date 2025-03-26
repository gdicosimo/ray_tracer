use std::{
    arch::x86_64::{
        __m128, _mm_add_ps, _mm_cvtss_f32, _mm_div_ps, _mm_dp_ps, _mm_load_ps, _mm_max_ps,
        _mm_min_ps, _mm_mul_ps, _mm_set1_ps, _mm_shuffle_ps, _mm_store_ps, _mm_sub_ps,
    },
    ops::{Add, Div, Mul, Sub},
};

use super::*;

macro_rules! impl_coord_arithmetics {
    (
        $(
            ( $trait:ident, $fn_name:ident ) => $simd_fn:ident
        ),* $(,)?
    ) => {
        $(
            // Coord op Coord
            impl std::ops::$trait for Coord {
                type Output = Self;
                #[inline]
                fn $fn_name(self, other: Self) -> Self::Output {
                    self.$simd_fn(&other)
                }
            }

            // Coord op &Coord
            impl std::ops::$trait<&Coord> for Coord {
                type Output = Self;
                #[inline]
                fn $fn_name(self, other: &Coord) -> Self::Output {
                    self.$simd_fn(other)
                }
            }

            // $Coord op $Coord
            impl<'a, 'b> $trait<&'b Coord> for &'a Coord {
                type Output = Coord;

                #[inline]
                fn $fn_name(self, rhs: &'b Coord) -> Self::Output {
                    self.$simd_fn(rhs)
                }
            }
        )*
    };

    (
        $(
            scalar ( $trait:ident, $fn_name:ident ) => $simd_fn:ident
        ),* $(,)?
    ) => {
        $(
            // f32 * Coord
            impl $trait<Coord> for f32 {
                type Output = Coord;

                #[inline]
                fn $fn_name(self, coord: Coord) -> Self::Output {
                    coord.$simd_fn(self)
                }
            }

            // f32 * &Coord
            impl $trait<&Coord> for f32 {
                type Output = Coord;

                #[inline]
                fn $fn_name(self, coord: &Coord) -> Self::Output {
                    coord.$simd_fn(self)
                }
            }


        )*
    };
}

macro_rules! impl_arithmetic_ops {
    // Binary Operations (like Add, Sub)
   (
        trait = $trait:ident,
        trait_fn = $trait_fn:ident,
        op = $op:tt,
        output_type = $output_type:path,
        impls = [ $( ($LHS:ty, $RHS:ty) ),* $(,)? ]
    ) => {
        $(
            impl $trait<$RHS> for $LHS {
                type Output = $output_type;
                fn $trait_fn(self, rhs: $RHS) -> Self::Output {
                    let result: Coord = self.as_coord() $op rhs.as_coord();
                    $output_type(result)
                }
            }
        )*
    };
    // Scalar Operations (like Mul, Div with scalar)
    (
        scalar,
        trait = $trait:ident,
        trait_fn = $trait_fn:ident,
        op = $op:tt,
        output_type = $output_type:path,
        impls = [ $( ($DimensionalType:ty) ),* $(,)? ]
    ) => {
        $(

            impl $trait<$DimensionalType> for f32 {
                type Output = $output_type;
                fn $trait_fn(self, dimensional: $DimensionalType) -> Self::Output {
                    $output_type(self $op dimensional.as_coord())
                }
            }
        )*
    };
}

// ─────────────────────────────

impl Coord {
    #[inline]
    fn simd<F>(&self, other: &Self, f: F) -> Coord
    where
        F: FnOnce(__m128, __m128) -> __m128,
    {
        unsafe {
            let a = _mm_load_ps(self.0.as_ptr());
            let b = _mm_load_ps(other.0.as_ptr());

            let result = f(a, b);

            let mut output = std::mem::MaybeUninit::<Coord>::uninit();
            _mm_store_ps((*output.as_mut_ptr()).0.as_mut_ptr(), result);
            output.assume_init()
        }
    }

    #[inline]
    pub fn simd_splat(scalar: f32) -> Self {
        unsafe {
            let vec = _mm_set1_ps(scalar);
            let mut output = std::mem::MaybeUninit::<Coord>::uninit();
            _mm_store_ps((*output.as_mut_ptr()).0.as_mut_ptr(), vec);
            output.assume_init()
        }
    }

    pub fn simd_add(&self, other: &Self) -> Self {
        self.simd(other, |x, y| unsafe { _mm_add_ps(x, y) })
    }

    pub fn simd_sub(&self, other: &Self) -> Self {
        if cfg!(target_arch = "x86_64") {
            self.simd(other, |x, y| unsafe { _mm_sub_ps(x, y) })
        } else {
            self - other
        }
    }
    pub fn simd_mul(&self, other: &Self) -> Coord {
        if cfg!(target_arch = "x86_64") {
            self.simd(other, |a, b| unsafe { _mm_mul_ps(a, b) })
        } else {
            self * other
        }
    }

    pub fn simd_div(&self, other: &Self) -> Coord {
        self.simd(other, |a, b| unsafe { _mm_div_ps(a, b) })
    }

    pub fn simd_scalar_mul(&self, scalar: f32) -> Coord {
        let scalar_vec = Self::simd_splat(scalar);
        self.simd_mul(&scalar_vec)
    }

    pub fn simd_scalar_div(&self, scalar: f32) -> Self {
        let scalar_vec = Self::simd_splat(1.0 / scalar);
        self.simd_mul(&scalar_vec)
    }

    pub fn simd_dot(&self, other: &Self) -> f32 {
        if cfg!(target_arch = "x86_64") {
            unsafe {
                let a = _mm_load_ps(self.0.as_ptr());
                let b = _mm_load_ps(other.0.as_ptr());

                let dot = _mm_dp_ps(a, b, 0x71);

                _mm_cvtss_f32(dot)
            }
        } else {
            self.x() * other.x() + self.y() * other.y() + self.z() * other.z()
        }
    }

    pub fn simd_cross(&self, other: &Self) -> Self {
        if !cfg!(target_arch = "x86_64") {
            self.simd(other, |a, b| unsafe {
                // 201 => _MM_SHUFFLE(3, 0, 2, 1) = (a << 6) | (b << 4) | (c << 2) | d = 192 + 0 + 8 + 1 = 201.
                let a_zxy = _mm_shuffle_ps(a, a, 201); // [a.z, a.x, a.y, a.w]
                let a_yzx = _mm_shuffle_ps(a, a, 210); // [a.y, a.z, a.x, a.w]
                let b_yzx = _mm_shuffle_ps(b, b, 210); // [b.y, b.z, b.x, b.w]
                let b_zxy = _mm_shuffle_ps(b, b, 201); // [b.z, b.x, b.y, b.w]

                _mm_sub_ps(_mm_mul_ps(a_zxy, b_yzx), _mm_mul_ps(a_yzx, b_zxy))
            })
        } else {
            let (x1, y1, z1) = (self.x(), self.y(), self.z());
            let (x2, y2, z2) = (other.x(), other.y(), other.z());

            Self::new3(y1 * z2 - z1 * y2, z1 * x2 - x1 * z2, x1 * y2 - y1 * x2)
        }
    }

    pub fn simd_min(&self, other: &Self) -> Self {
        self.simd(other, |a, b| unsafe { _mm_min_ps(a, b) })
    }

    pub fn simd_max(&self, other: &Self) -> Self {
        self.simd(other, |a, b| unsafe { _mm_max_ps(a, b) })
    }

    pub fn simd_rsqrt(&self, len_sq: f32) -> f32 {
        if cfg!(target_arch = "x86_64") {
            use core::arch::x86_64::{_mm_cvtss_f32, _mm_rsqrt_ss, _mm_set_ss};

            let simd_len_sq = unsafe { _mm_set_ss(len_sq) };

            let inv_len_simd = unsafe { _mm_rsqrt_ss(simd_len_sq) };

            unsafe { _mm_cvtss_f32(inv_len_simd) as f32 }
        } else {
            // No fast :(
            len_sq.sqrt().recip()
        }
    }

    pub fn simd_sqrt(&self, len_sq: f32) -> f32 {
        if cfg!(target_arch = "x86_64") {
            use core::arch::x86_64::{_mm_cvtss_f32, _mm_set_ss, _mm_sqrt_ss};

            unsafe {
                let simd_len_sq = _mm_set_ss(len_sq);
                let len_simd = _mm_sqrt_ss(simd_len_sq);
                _mm_cvtss_f32(len_simd)
            }
        } else {
            len_sq.sqrt()
        }
    }
}

// ─────────────────────────────

impl_coord_arithmetics! {
    (Add, add) => simd_add,
    (Sub, sub) => simd_sub,
    (Mul, mul) => simd_mul,
    (Div, div) => simd_div,
}

impl_coord_arithmetics! {
    scalar (Mul, mul) => simd_scalar_mul,
    scalar (Div, div) => simd_scalar_div,
}

// ─────────────────────────────

impl_arithmetic_ops!(
    trait = Add,
    trait_fn = add,
    op = +,
    output_type = Vec3,
    impls = [(Vec3, Vec3),  (Vec3, &Vec3), (&Vec3, Vec3), (&Vec3, &Vec3) ]
);

impl_arithmetic_ops!(
    trait = Sub,
    trait_fn = sub,
    op = -,
    output_type = Vec3,
    impls = [(Vec3, Vec3),  (Vec3, &Vec3), (&Vec3, Vec3), (&Vec3, &Vec3) ]
);

impl_arithmetic_ops!(
    scalar,
    trait = Mul,
    trait_fn = mul,
    op = *,
    output_type = Vec3,
    impls = [(Vec3), (&Vec3)]
);

impl_arithmetic_ops!(
    scalar,
    trait = Div,
    trait_fn = div,
    op = /,
    output_type = Vec3,
    impls = [(Vec3), (&Vec3)]
);

impl_arithmetic_ops!(
    trait = Sub,
    trait_fn = sub,
    op = -,
    output_type = Vec3,
    impls = [(Point3, Point3), (&Point3, &Point3), (Point3, &Point3), (&Point3, Point3) ]
);

impl_arithmetic_ops!(
    trait = Sub,
    trait_fn = sub,
    op = -,
    output_type = Point3,
    impls = [(&Point3, Vec3), (Point3, Vec3), (&Point3, &Vec3), (Vec3, &Point3), (Point3, &Vec3)]
);

impl_arithmetic_ops!(
    scalar,
    trait = Mul,
    trait_fn = mul,
    op = *,
    output_type = Point3,
    impls = [(Point3), (&Point3)]
);

impl_arithmetic_ops!(
    trait = Add,
    trait_fn = add,
    op = +,
    output_type = Point3,
    impls = [(&Point3, Vec3), (Point3, &Vec3), (Point3, Vec3), (&Point3, &Vec3) ]
);

// ─────────────────────────────

impl_arithmetic_ops!(
    trait = Add,
    trait_fn = add,
    op = +,
    output_type = Vec3,
    impls = [(UnitVec3, Vec3), (UnitVec3, &Vec3), (Vec3, &UnitVec3), (&Vec3, UnitVec3), (&Vec3, &UnitVec3)]
);

impl_arithmetic_ops!(
    trait = Add,
    trait_fn = add,
    op = +,
    output_type = Vec3,
    impls = [(&UnitVec3, UnitVec3) ]
);

impl_arithmetic_ops!(
    scalar,
    trait = Mul,
    trait_fn = mul,
    op = *,
    output_type = Vec3,
    impls = [(UnitVec3), (&UnitVec3)]
);

// ─────────────────────────────
