// @generated from upstream/packages/types/src/VertexDisplaceModifierOptions.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{Texture, Vector3Like, VertexDisplaceModifierSource};

// Source: upstream/packages/types/src/VertexDisplaceModifierOptions.ts:5 (sha256:4831daeafb37b213acd119f1b235c36ab8b0c9539d23d2958379792fa2a48f98)
#[derive(Clone, Default)]
pub struct VertexDisplaceModifierOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub source: VertexDisplaceModifierSource,
    pub amplitude: f64,
    pub axis: Option<Vector3Like>,
    pub map: Option<Texture>,
    pub frequency: Option<f64>,
    pub speed: Option<f64>,
    pub direction: Option<Vector3Like>,
}
impl PartialEq for VertexDisplaceModifierOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
