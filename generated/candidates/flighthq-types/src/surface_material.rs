// @generated from upstream/packages/types/src/SurfaceMaterial.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::Texture;
use crate::{AlphaType, BlendMode, Kind};

// Source: upstream/packages/types/src/SurfaceMaterial.ts:8 (sha256:8b0ef03700d2be8726531d72c46a83974f88b1df8e6918a4709cc196cee2d32f)
pub type MaterialAlphaMode = String;

// Source: upstream/packages/types/src/SurfaceMaterial.ts:15 (sha256:38a674859bb34f75042080d1aa542c108f3b1477eb602f27aeb3a54e6b580fc4)
#[derive(Clone, Default)]
pub struct SurfaceMaterial {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: Kind,
    pub name: Option<String>,
    pub alpha_cutoff: f64,
    pub alpha_mode: MaterialAlphaMode,
    pub alpha_type: AlphaType,
    pub blend_mode: BlendMode,
    pub double_sided: bool,
    pub shader_key: String,
    pub textures: Option<Vec<(String, Texture)>>,
    pub uniforms: Option<Vec<(String, crate::FlightUnion2<f64, Vec<f64>>)>>,
}
impl PartialEq for SurfaceMaterial {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
