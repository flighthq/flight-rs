// @generated from upstream/packages/types/src/SpritesheetAnimationData.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::SpritesheetAnimationDirection;

// Source: upstream/packages/types/src/SpritesheetAnimationData.ts:3 (sha256:8aafff09324b8e1fbc04f89ce45cae82cde358425e32da980649bcdc12a999c4)
#[derive(Clone, Default)]
pub struct SpritesheetAnimationData {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub direction: SpritesheetAnimationDirection,
    pub frame_duration: f64,
    pub frame_durations: Option<Vec<f64>>,
    pub frame_names: Vec<String>,
    pub repeat_count: f64,
    pub name: String,
    pub origin_x: f64,
    pub origin_y: f64,
}
impl PartialEq for SpritesheetAnimationData {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
