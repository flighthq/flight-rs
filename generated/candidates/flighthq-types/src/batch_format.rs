// @generated from upstream/packages/types/src/BatchFormat.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/BatchFormat.ts:13 (sha256:749046733b1cea90eb40a1665ebf979a05516907d91e4df375b547c59346e209)
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[repr(transparent)]
pub struct BatchFormat(pub u32);

impl BatchFormat {
    #[allow(non_upper_case_globals)]
    pub const Quad: Self = Self(0_u32);

    #[allow(non_upper_case_globals)]
    pub const VertexStream: Self = Self(1_u32);
}

impl std::ops::BitAnd for BatchFormat {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl std::ops::BitOr for BatchFormat {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl std::ops::BitXor for BatchFormat {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self {
        Self(self.0 ^ rhs.0)
    }
}
impl std::ops::Not for BatchFormat {
    type Output = Self;
    fn not(self) -> Self {
        Self(!self.0)
    }
}
impl PartialEq<f64> for BatchFormat {
    fn eq(&self, rhs: &f64) -> bool {
        self.0 as f64 == *rhs
    }
}
