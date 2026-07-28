// @generated from upstream/packages/types/src/TweenOptions.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::EasingFunction;

// Source: upstream/packages/types/src/TweenOptions.ts:3 (sha256:1a93638bf1b8e99e78d9808d729bfab6dc8c338484d0ba13486298519278bfad)
#[derive(Clone)]
pub struct TweenOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub delay: Option<f64>,
    pub ease: Option<EasingFunction>,
    pub overwrite: Option<bool>,
    pub reflect: Option<bool>,
    pub repeat: Option<f64>,
    pub reverse: Option<bool>,
    pub smart_rotation: Option<bool>,
    pub snapping: Option<bool>,
}
impl PartialEq for TweenOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
