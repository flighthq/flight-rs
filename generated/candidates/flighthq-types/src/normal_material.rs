// @generated from upstream/packages/types/src/NormalMaterial.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{AlphaType, BlendMode, Kind, MaterialAlphaMode, Texture};

// Source: upstream/packages/types/src/NormalMaterial.ts:7 (sha256:c70d8469431bf75424c4a4a457ce0d99ad98a217f88c1bfe6c687de4407e4031)
#[derive(Clone)]
pub struct NormalMaterial {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: Kind,
    pub name: Option<String>,
    pub alpha_cutoff: f64,
    pub alpha_mode: MaterialAlphaMode,
    pub alpha_type: AlphaType,
    pub blend_mode: BlendMode,
    pub double_sided: bool,
    pub normal_map: Option<Texture>,
    pub normal_scale: f64,
}
impl PartialEq for NormalMaterial {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/NormalMaterial.ts:12 (sha256:e33ef8cb9d2ca55ee4933ac852acfd8ce48eab2eb3db5fbd0dd97820bc5919e0)
pub const NORMAL_MATERIAL_KIND: &'static str = "NormalMaterial";
