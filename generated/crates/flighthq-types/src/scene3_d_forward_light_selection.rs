// @generated from upstream/packages/types/src/Scene3DForwardLightSelection.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{PointLight, SpotLight};

// Source: upstream/packages/types/src/Scene3DForwardLightSelection.ts:9 (sha256:8a8917d87d1e9f3bc9fe8e873e99c17984e9b9d84dc790c0049b851acb32e92c)
#[derive(Clone, Default)]
pub struct Scene3DForwardLightSelection {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub indices: Vec<f64>,
    pub point: Vec<PointLight>,
    pub spot: Vec<SpotLight>,
}
impl PartialEq for Scene3DForwardLightSelection {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
