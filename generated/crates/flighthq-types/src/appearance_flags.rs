// @generated from upstream/packages/types/src/AppearanceFlags.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/AppearanceFlags.ts:1 (sha256:226f103f8f6aff1c6397b44cfc346c3492eca4b7458c9a0739d4ded39022217c)
#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct AppearanceFlags(pub u32);

impl AppearanceFlags {
    #[allow(non_upper_case_globals)]
    pub const None: Self = Self(0_u32);

    #[allow(non_upper_case_globals)]
    pub const Visible: Self = Self(1_u32);

    #[allow(non_upper_case_globals)]
    pub const Alpha: Self = Self(2_u32);

    #[allow(non_upper_case_globals)]
    pub const BlendMode: Self = Self(4_u32);

    #[allow(non_upper_case_globals)]
    pub const Clip: Self = Self(8_u32);

    #[allow(non_upper_case_globals)]
    pub const Scale9Grid: Self = Self(16_u32);

    #[allow(non_upper_case_globals)]
    pub const Any: Self = Self(2147483648_u32);

    pub fn any(flags: AppearanceFlags, test: AppearanceFlags) -> bool {
        return ((flags & test) != 0.0_f64);
    }

    pub fn has(flags: AppearanceFlags, test: AppearanceFlags) -> bool {
        return ((flags & test) == test);
    }

    pub fn add(flags: AppearanceFlags, add: AppearanceFlags) -> AppearanceFlags {
        return (flags | add);
    }

    pub fn remove(flags: AppearanceFlags, remove: AppearanceFlags) -> AppearanceFlags {
        return (flags & (!remove));
    }

    pub fn clear() -> AppearanceFlags {
        return AppearanceFlags::None;
    }
}

impl std::ops::BitAnd for AppearanceFlags {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl std::ops::BitOr for AppearanceFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl std::ops::BitXor for AppearanceFlags {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self {
        Self(self.0 ^ rhs.0)
    }
}
impl std::ops::Not for AppearanceFlags {
    type Output = Self;
    fn not(self) -> Self {
        Self(!self.0)
    }
}
impl PartialEq<f64> for AppearanceFlags {
    fn eq(&self, rhs: &f64) -> bool {
        self.0 as f64 == *rhs
    }
}
