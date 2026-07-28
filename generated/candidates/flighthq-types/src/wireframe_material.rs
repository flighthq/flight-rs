// @generated from upstream/packages/types/src/WireframeMaterial.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{AlphaType, BlendMode, Kind, MaterialAlphaMode};

// Source: upstream/packages/types/src/WireframeMaterial.ts:5 (sha256:99fe447361adb31ff8434deb7f21c124b509a725ca83f0248cfb14280692d699)
#[derive(Clone)]
pub struct WireframeMaterial {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: Kind,
    pub name: Option<String>,
    pub alpha_cutoff: f64,
    pub alpha_mode: MaterialAlphaMode,
    pub alpha_type: AlphaType,
    pub blend_mode: BlendMode,
    pub double_sided: bool,
    pub color: f64,
    pub thickness: f64,
}
impl PartialEq for WireframeMaterial {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/WireframeMaterial.ts:10 (sha256:7dbfd4ee27ef5f4bc7233365411505c7a4663bc33499cd7b4305b8769f71d6e5)
pub const WIREFRAME_MATERIAL_KIND: &'static str = "WireframeMaterial";
