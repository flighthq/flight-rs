// @generated from upstream/packages/types/src/Material3DOptions.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{BlendMode, MaterialAlphaMode};

// Source: upstream/packages/types/src/Material3DOptions.ts:11 (sha256:debf84cb4ee2c53ffc5c55ddf9d2eff26c39eae50949387218200f9e042788b0)
#[derive(Clone, Default)]
pub struct Material3DOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub alpha_cutoff: Option<f64>,
    pub alpha_mode: Option<MaterialAlphaMode>,
    pub blend_mode: Option<BlendMode>,
    pub double_sided: Option<bool>,
}
impl PartialEq for Material3DOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
