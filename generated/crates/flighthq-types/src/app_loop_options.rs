// @generated from upstream/packages/types/src/AppLoopOptions.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/AppLoopOptions.ts:2 (sha256:4ff97cd0f9fdfae4199d1db2aa86593d287e5957c04b32291f03e858d15cf80b)
#[derive(Clone, Default)]
pub struct AppLoopOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub max_delta_time: Option<f64>,
    pub target_frame_rate: Option<f64>,
    pub background_frame_rate: Option<f64>,
    pub fixed_time_step: Option<f64>,
    pub max_updates_per_frame: Option<f64>,
}
impl PartialEq for AppLoopOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/AppLoopOptions.ts:16 (sha256:543715c76c3ea51cb7cc4ad926df0fe5ce33504a170e8f0be7fb6997262def5f)
pub type AppStepOptions = AppLoopOptions;
