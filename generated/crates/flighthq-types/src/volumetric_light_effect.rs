// @generated from upstream/packages/types/src/VolumetricLightEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::Kind;

// Source: upstream/packages/types/src/VolumetricLightEffect.ts:2 (sha256:0c4674272a20a98d17994e001cca3d6c51b1a060f0922a66415643e2ba7d4045)
#[derive(Clone, Default)]
pub struct VolumetricLightEffect {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: Kind,
    pub density: Option<f64>,
    pub light_color: Option<f64>,
    pub light_x: Option<f64>,
    pub light_y: Option<f64>,
    pub samples: Option<f64>,
    pub scattering: Option<f64>,
}
impl PartialEq for VolumetricLightEffect {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
