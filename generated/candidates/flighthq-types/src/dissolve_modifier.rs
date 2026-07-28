// @generated from upstream/packages/types/src/DissolveModifier.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{EmissiveModifierFacing, FogModifierMode, Vector3Like, VertexDisplaceModifierSource};
use crate::{ModifierKind, ModifierSlot, Texture, Vector2Like};

// Source: upstream/packages/types/src/DissolveModifier.ts:11 (sha256:b4447f68b4d80c5a7fc46ba4dfaedef76ea959785551545cb6cb49842f894138)
#[derive(Clone, Default)]
pub struct DissolveModifier {
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
    pub map: Option<Texture>,
}
impl PartialEq for DissolveModifier {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/DissolveModifier.ts:21 (sha256:3bf663d2eb41c8257b420e470dff1f42d74906ed878b971bb68245fb58583590)
pub const DISSOLVE_MODIFIER_KIND: &'static str = "DissolveModifier";
