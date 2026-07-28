// @generated from upstream/packages/types/src/EnvReflectModifier.ts; do not edit.
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

// Source: upstream/packages/types/src/EnvReflectModifier.ts:14 (sha256:57ea745159357dca3900015d7ffddcedc6d076797e254ff4744078bf12bdfa9d)
#[derive(Clone, Default)]
pub struct EnvReflectModifier {
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
impl PartialEq for EnvReflectModifier {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/EnvReflectModifier.ts:23 (sha256:7c28ff397a54bc44bbd7c88e5def768d8b849f15f34174467996e3e4d0b08835)
pub const ENV_REFLECT_MODIFIER_KIND: &'static str = "EnvReflectModifier";
