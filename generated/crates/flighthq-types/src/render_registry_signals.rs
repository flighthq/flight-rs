// @generated from upstream/packages/types/src/RenderRegistrySignals.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{Kind, Signal};

// Source: upstream/packages/types/src/RenderRegistrySignals.ts:9 (sha256:50e820ba16f20ca593fbd06b2833fe41f57f2c485490ff5254b06d95526e40ba)
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[repr(transparent)]
pub struct RenderRegistry(pub u32);

impl RenderRegistry {
    #[allow(non_upper_case_globals)]
    pub const BlendRealization: Self = Self(0_u32);

    #[allow(non_upper_case_globals)]
    pub const EffectPaddingResolver: Self = Self(1_u32);

    #[allow(non_upper_case_globals)]
    pub const MaterialRenderer: Self = Self(2_u32);

    #[allow(non_upper_case_globals)]
    pub const MaterialTextureLister: Self = Self(3_u32);

    #[allow(non_upper_case_globals)]
    pub const ModifierSnippet: Self = Self(4_u32);

    #[allow(non_upper_case_globals)]
    pub const NodeRenderer: Self = Self(5_u32);

    #[allow(non_upper_case_globals)]
    pub const ShapeCommandHandler: Self = Self(6_u32);

    #[allow(non_upper_case_globals)]
    pub const ShapeRasterizer: Self = Self(7_u32);

    #[allow(non_upper_case_globals)]
    pub const TextureResolver: Self = Self(8_u32);
}

impl std::ops::BitAnd for RenderRegistry {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl std::ops::BitOr for RenderRegistry {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl std::ops::BitXor for RenderRegistry {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self {
        Self(self.0 ^ rhs.0)
    }
}
impl std::ops::Not for RenderRegistry {
    type Output = Self;
    fn not(self) -> Self {
        Self(!self.0)
    }
}
impl PartialEq<f64> for RenderRegistry {
    fn eq(&self, rhs: &f64) -> bool {
        self.0 as f64 == *rhs
    }
}

// Source: upstream/packages/types/src/RenderRegistrySignals.ts:21 (sha256:ae6c8a9183659933976a72e2ef24dc2025c65f10f65291defbc61b719c528d03)
#[derive(Clone)]
pub struct RenderRegistryMiss {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: Kind,
    pub registry: RenderRegistry,
}
impl PartialEq for RenderRegistryMiss {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/RenderRegistrySignals.ts:26 (sha256:6bbccee2ac8bb108bafbda601f75098ff978d7c8b17665c24f11e5688064cba0)
#[derive(Clone, Default)]
pub struct RenderRegistryMissExplanation {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub misses: Vec<RenderRegistryMiss>,
    pub status: String,
}
impl PartialEq for RenderRegistryMissExplanation {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/RenderRegistrySignals.ts:31 (sha256:c0e2041f574ce98f51e8e7cb554d744269fa1e3d45bf68323b363670f3c152e3)
#[derive(Clone)]
pub struct RenderRegistrySignals {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub on_registry_miss: Signal<
        std::sync::Arc<
            std::sync::Mutex<Box<dyn FnMut(RenderRegistry, Kind) -> () + Send + 'static>>,
        >,
    >,
}
impl PartialEq for RenderRegistrySignals {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
