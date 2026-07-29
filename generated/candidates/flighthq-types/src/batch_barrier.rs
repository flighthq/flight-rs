// @generated from upstream/packages/types/src/BatchBarrier.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/BatchBarrier.ts:4 (sha256:33fa6c5cd42f152d3b3c2fc74ec614ab1f9eed5d45cbbad1a97152a58a75ea64)
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[repr(transparent)]
pub struct BatchBarrier(pub u32);

impl BatchBarrier {
    #[allow(non_upper_case_globals)]
    pub const Capacity: Self = Self(0_u32);

    #[allow(non_upper_case_globals)]
    pub const Clip: Self = Self(1_u32);

    #[allow(non_upper_case_globals)]
    pub const Filter: Self = Self(2_u32);

    #[allow(non_upper_case_globals)]
    pub const RenderTarget: Self = Self(3_u32);
}

impl std::ops::BitAnd for BatchBarrier {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl std::ops::BitOr for BatchBarrier {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl std::ops::BitXor for BatchBarrier {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self {
        Self(self.0 ^ rhs.0)
    }
}
impl std::ops::Not for BatchBarrier {
    type Output = Self;
    fn not(self) -> Self {
        Self(!self.0)
    }
}
impl PartialEq<f64> for BatchBarrier {
    fn eq(&self, rhs: &f64) -> bool {
        self.0 as f64 == *rhs
    }
}
