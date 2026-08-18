// @generated from upstream/packages/types/src/DirectionalLightOptions.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::Vector3Like;

// Source: upstream/packages/types/src/DirectionalLightOptions.ts:3 (sha256:ae22f5138e53cd2df7b843f130bdd9febf39d2cad6f130edf563bb11fc9f4431)
#[derive(Clone, Default)]
pub struct DirectionalLightOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub casts_shadow: Option<bool>,
    pub color: Option<f64>,
    pub direction: Option<Vector3Like>,
    pub intensity: Option<f64>,
    pub normal_bias: Option<f64>,
    pub pcf_radius: Option<f64>,
    pub shadow_bias: Option<f64>,
}
impl PartialEq for DirectionalLightOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
