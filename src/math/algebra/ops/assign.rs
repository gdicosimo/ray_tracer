use super::*;

macro_rules! impl_asign_ops {
(
        trait = $trait:ident,
        trait_fn = $trait_fn:ident,
        op = $op:tt,
        impls = [ $( ($LHS:ty, $RHS:ty) ),* $(,)? ]
    ) => {
        // Assign Operations (like AddAssign, SubAssign)
        $(
            impl std::ops::$trait<$RHS> for $LHS {
                fn $trait_fn(&mut self, rhs: $RHS) {
                    self.0 = &self.0 $op &rhs.0
                }
            }
        )*
    };
(
        trait = $trait:ident,
        trait_fn = $trait_fn:ident,
        op = $op:tt,
        impls = [ $( ($VectorType:ty) ),* $(,)? ]
    ) => {
        // Assign Operations (like MulAssign, DivAssign)
        $(
            impl std::ops::$trait<f32> for $VectorType {
                fn $trait_fn(&mut self, scalar: f32) {
                    self.0 = scalar $op &self.0 ;
                }
            }
        )*
    };
}

impl_asign_ops!(
    trait = AddAssign,
    trait_fn = add_assign,
    op = +,
    impls = [(Vec3, Vec3), (Vec3, &Vec3)]
);

impl_asign_ops!(
    trait = AddAssign,
    trait_fn = add_assign,
    op = +,
    impls = [(Point3, Vec3), (Point3, &Vec3)]
);

impl_asign_ops!(
    trait = SubAssign,
    trait_fn = sub_assign,
    op = -,
    impls = [(Vec3, Vec3), (Vec3, &Vec3)]
);

impl_asign_ops!(
    trait = MulAssign,
    trait_fn = mul_assign,
    op = *,
    impls = [(Vec3)]
);

impl_asign_ops!(
    trait = DivAssign,
    trait_fn = div_assign,
    op = /,
    impls = [(Vec3)]
);

impl_asign_ops!(
    trait = MulAssign,
    trait_fn = mul_assign,
    op = *,
    impls = [(Point3)]
);
