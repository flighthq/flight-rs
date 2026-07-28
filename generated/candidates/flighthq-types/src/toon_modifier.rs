// @generated from upstream/packages/types/src/ToonModifier.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    EmissiveModifierFacing, FogModifierMode, Texture, Vector2Like, Vector3Like,
    VertexDisplaceModifierSource,
};
use crate::{ModifierKind, ModifierSlot};

// Source: upstream/packages/types/src/ToonModifier.ts:10 (sha256:32cd129d4ac517481ab1aa7f5c0cd5b6bb8440220cfdd8973a653c570d128acf)
#[derive(Clone, Default)]
pub struct ToonModifier {
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
impl PartialEq for ToonModifier {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/ToonModifier.ts:17 (sha256:76d40005572533e5b8ddff8275be511c5f4809bf81e6deca631c5477b8d4156d)
pub const TOON_MODIFIER_KIND: &'static str = "ToonModifier";
