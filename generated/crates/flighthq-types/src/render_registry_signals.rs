// @generated from upstream/packages/types/src/RenderRegistrySignals.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{Kind, Signal};

// Source: upstream/packages/types/src/RenderRegistrySignals.ts:9 (sha256:1b0efd05f997c9e5717842f68cb3a73186a37fcfb8b75a8411c63ca30b8dc689)
#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct RenderRegistryTable(pub u32);

impl RenderRegistryTable {
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

impl std::ops::BitAnd for RenderRegistryTable {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl std::ops::BitOr for RenderRegistryTable {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl std::ops::BitXor for RenderRegistryTable {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self {
        Self(self.0 ^ rhs.0)
    }
}
impl std::ops::Not for RenderRegistryTable {
    type Output = Self;
    fn not(self) -> Self {
        Self(!self.0)
    }
}
impl PartialEq<f64> for RenderRegistryTable {
    fn eq(&self, rhs: &f64) -> bool {
        self.0 as f64 == *rhs
    }
}

// Source: upstream/packages/types/src/RenderRegistrySignals.ts:21 (sha256:3c946e44c51b8e43590159a736d9b75dca125d28f957375aee153071cf05059d)
#[derive(Clone)]
pub struct RenderRegistriesMiss {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: Kind,
    pub registry: RenderRegistryTable,
}
impl PartialEq for RenderRegistriesMiss {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/RenderRegistrySignals.ts:26 (sha256:df03aaffcdeeb82685fe948deeda3c10eedc65f9e5f6ec9bfff99f8e7f7031aa)
#[derive(Clone, Default)]
pub struct RenderRegistriesMissExplanation {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub misses: Vec<RenderRegistriesMiss>,
    pub status: String,
}
impl PartialEq for RenderRegistriesMissExplanation {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/RenderRegistrySignals.ts:31 (sha256:e61350aebf26e7cf874c778acc2c43faa532dbfd39402bdcfa3185fb38169d9d)
#[derive(Clone)]
pub struct RenderRegistrySignals {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub on_registry_miss: Signal<
        std::sync::Arc<
            std::sync::Mutex<Box<dyn FnMut(RenderRegistryTable, Kind) -> () + Send + 'static>>,
        >,
    >,
}
impl PartialEq for RenderRegistrySignals {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
