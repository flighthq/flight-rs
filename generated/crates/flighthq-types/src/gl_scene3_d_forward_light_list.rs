// @generated from upstream/packages/types/src/GlScene3DForwardLightList.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::Scene3DLightBlock;

// Source: upstream/packages/types/src/GlScene3DForwardLightList.ts:7 (sha256:c58b106a2fcc285adc14a39f9994261ee76537152a420cc28cb4931c037fb721)
#[derive(Clone, Default)]
pub struct GlScene3DForwardLightList {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub mesh_count: f64,
    pub mesh_light_blocks: Vec<Scene3DLightBlock>,
}
impl PartialEq for GlScene3DForwardLightList {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
