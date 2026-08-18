// @generated from upstream/packages/types/src/WgpuScene3DForwardLightList.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::Scene3DLightBlock;

// Source: upstream/packages/types/src/WgpuScene3DForwardLightList.ts:5 (sha256:17ceb715b43550352d696a593afa725cc73be305760cc4dca9a12301a183b75e)
#[derive(Clone, Default)]
pub struct WgpuScene3DForwardLightList {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub mesh_count: f64,
    pub mesh_light_blocks: Vec<Scene3DLightBlock>,
}
impl PartialEq for WgpuScene3DForwardLightList {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
