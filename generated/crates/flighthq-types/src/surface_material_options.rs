// @generated from upstream/packages/types/src/SurfaceMaterialOptions.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{BlendMode, MaterialAlphaMode};

// Source: upstream/packages/types/src/SurfaceMaterialOptions.ts:11 (sha256:412559f26a3db335faa1360987a6c969a485794cba58f53b28164b8dbe13934e)
#[derive(Clone, Default)]
pub struct SurfaceMaterialOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub alpha_cutoff: Option<f64>,
    pub alpha_mode: Option<MaterialAlphaMode>,
    pub blend_mode: Option<BlendMode>,
    pub double_sided: Option<bool>,
}
impl PartialEq for SurfaceMaterialOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
