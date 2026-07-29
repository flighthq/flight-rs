// @generated from upstream/packages/types/src/EmissiveModifier.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{FogModifierMode, Vector3Like, VertexDisplaceModifierSource};
use crate::{ModifierKind, ModifierSlot, Texture, Vector2Like};

// Source: upstream/packages/types/src/EmissiveModifier.ts:10 (sha256:0fbd2c29a8bac6b5ec11aef11661aedb7b5b3b938be90cb944238b4362bdf6e0)
#[derive(Clone, Default)]
pub struct EmissiveModifierFacingValues {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub away_from_light: String,
    pub ignore: String,
    pub toward_light: String,
}
impl PartialEq for EmissiveModifierFacingValues {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub static EMISSIVE_MODIFIER_FACING: std::sync::LazyLock<EmissiveModifierFacingValues> =
    std::sync::LazyLock::new(|| EmissiveModifierFacingValues {
        __flight_identity: std::sync::Arc::new(()),
        away_from_light: "AwayFromLight".to_owned(),
        ignore: "Ignore".to_owned(),
        toward_light: "TowardLight".to_owned(),
    });

// Source: upstream/packages/types/src/EmissiveModifier.ts:16 (sha256:18edd2a4d73a3112dbd34748439b33a494abc67eca0dc77d561fb3d0674f8ef7)
pub type EmissiveModifierFacing = crate::OpaqueHostValue;

// Source: upstream/packages/types/src/EmissiveModifier.ts:23 (sha256:1a8dbcef5fd253b0791b984f6f9941f1d377d618e6cffc3d199824faebde91f9)
#[derive(Clone, Default)]
pub struct EmissiveModifier {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: ModifierKind,
    pub slot: ModifierSlot,
    pub source: VertexDisplaceModifierSource,
    pub amplitude: f64,
    pub axis: Option<Vector3Like>,
    pub frequency: Option<f64>,
    pub speed: Option<f64>,
    pub direction: Option<Vector3Like>,
    pub steps: f64,
    pub smoothness: Option<f64>,
    pub color: f64,
    pub power: Option<f64>,
    pub intensity: Option<f64>,
    pub bias: Option<f64>,
    pub mode: Option<FogModifierMode>,
    pub near: Option<f64>,
    pub far: Option<f64>,
    pub density: Option<f64>,
    pub tint: f64,
    pub fresnel_bias: Option<f64>,
    pub roughness: Option<f64>,
    pub strength: f64,
    pub mask: Option<Texture>,
    pub facing: Option<EmissiveModifierFacing>,
    pub facing_softness: Option<f64>,
    pub threshold: f64,
    pub edge_color: f64,
    pub edge_width: Option<f64>,
    pub scale: Option<f64>,
    pub scroll: Vector2Like,
    pub secondary_map: Option<Texture>,
    pub secondary_scroll: Option<Vector2Like>,
}
impl PartialEq for EmissiveModifier {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/EmissiveModifier.ts:33 (sha256:227351bc3b3cc78bd40ccd66e8d053426172aa116ba64a873bd37abcd90872a8)
pub const EMISSIVE_MODIFIER_KIND: &'static str = "EmissiveModifier";
