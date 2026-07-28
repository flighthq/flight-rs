// @generated from upstream/packages/types/src/SpritesheetAnimationData.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::SpritesheetAnimationDirection;

// Source: upstream/packages/types/src/SpritesheetAnimationData.ts:3 (sha256:dfe210ac4871351e75459a262cd969098d4ff4081e39eeb4de236436b5a71976)
#[derive(Clone)]
pub struct SpritesheetAnimationData {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub direction: SpritesheetAnimationDirection,
    pub frame_duration: f64,
    pub frame_durations: Option<Vec<f64>>,
    pub frame_names: Vec<String>,
    pub loop_: bool,
    pub name: String,
    pub origin_x: f64,
    pub origin_y: f64,
}
impl PartialEq for SpritesheetAnimationData {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
