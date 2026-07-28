// @generated from upstream/packages/types/src/VertexColorMaterial.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{AlphaType, BlendMode, Kind, MaterialAlphaMode};

// Source: upstream/packages/types/src/VertexColorMaterial.ts:6 (sha256:9d8cc36eaecca7c66c0ce29413c59ec9780d206a161003fb62f06b580efd7d2b)
#[derive(Clone)]
pub struct VertexColorMaterial {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: Kind,
    pub name: Option<String>,
    pub alpha_cutoff: f64,
    pub alpha_mode: MaterialAlphaMode,
    pub alpha_type: AlphaType,
    pub blend_mode: BlendMode,
    pub double_sided: bool,
    pub tint: f64,
}
impl PartialEq for VertexColorMaterial {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/VertexColorMaterial.ts:10 (sha256:663ec1ea4d52c1b7c256befac8cd831121c0c0bd0dc46b845b553ec773b8becd)
pub const VERTEX_COLOR_MATERIAL_KIND: &'static str = "VertexColorMaterial";
