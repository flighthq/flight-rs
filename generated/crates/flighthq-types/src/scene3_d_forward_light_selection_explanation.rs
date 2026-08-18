// @generated from upstream/packages/types/src/Scene3DForwardLightSelectionExplanation.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/Scene3DForwardLightSelectionExplanation.ts:4 (sha256:55f4ea877dc7153e6bae11dd4cc3044c7ea7ea1193517d363313887efdd7eed8)
#[derive(Clone, Default)]
pub struct Scene3DForwardLightSelectionExplanation {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub point_light_count: f64,
    pub reason: String,
    pub selection_prepared: bool,
    pub spot_light_count: f64,
}
impl PartialEq for Scene3DForwardLightSelectionExplanation {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
