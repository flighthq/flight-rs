// @generated from upstream/packages/types/src/VertexDisplaceModifier.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{ModifierKind, ModifierSlot, Texture, Vector3Like};

// Source: upstream/packages/types/src/VertexDisplaceModifier.ts:10 (sha256:19152c19bdd9931db0c88cecc284140f9f9782c883b08a75585f81d1ec5a4dd2)
#[derive(Clone, Default)]
pub struct VertexDisplaceModifierSourceValues {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub height_map: String,
    pub sine: String,
}
impl PartialEq for VertexDisplaceModifierSourceValues {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub static VERTEX_DISPLACE_MODIFIER_SOURCE: std::sync::LazyLock<
    VertexDisplaceModifierSourceValues,
> = std::sync::LazyLock::new(|| VertexDisplaceModifierSourceValues {
    __flight_identity: std::sync::Arc::new(()),
    height_map: "HeightMap".to_owned(),
    sine: "Sine".to_owned(),
});

// Source: upstream/packages/types/src/VertexDisplaceModifier.ts:15 (sha256:63b40fd9115022e1a0623d7422b4d5e7a9348cb474328b05a65f0244020fb586)
pub type VertexDisplaceModifierSource = crate::OpaqueHostValue;

// Source: upstream/packages/types/src/VertexDisplaceModifier.ts:28 (sha256:6e37b62b50d5b48500aae731c8675045a8e2096273488315f353d68df10c6e8c)
#[derive(Clone, Default)]
pub struct VertexDisplaceModifier {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: ModifierKind,
    pub slot: ModifierSlot,
    pub source: VertexDisplaceModifierSource,
    pub amplitude: f64,
    pub axis: Option<Vector3Like>,
    pub map: Option<Texture>,
    pub frequency: Option<f64>,
    pub speed: Option<f64>,
    pub direction: Option<Vector3Like>,
}
impl PartialEq for VertexDisplaceModifier {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/VertexDisplaceModifier.ts:40 (sha256:93a062d1b6f87cdfa2c5770c0f55b0fe826f2b87a91cf155167e1e766654359e)
pub const VERTEX_DISPLACE_MODIFIER_KIND: &'static str = "VertexDisplaceModifier";
