// @generated from upstream/packages/types/src/ApplicationLoopOptions.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/ApplicationLoopOptions.ts:2 (sha256:6da530e1a42c7e3a45d4d16e018a2403a0457de4ad6ce1f2ab5ad26993e83fc5)
#[derive(Clone, Default)]
pub struct ApplicationLoopOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub max_delta_time: Option<f64>,
    pub target_frame_rate: Option<f64>,
    pub background_frame_rate: Option<f64>,
    pub fixed_time_step: Option<f64>,
    pub max_updates_per_frame: Option<f64>,
}
impl PartialEq for ApplicationLoopOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
