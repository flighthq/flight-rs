// @generated from upstream/packages/types/src/PerspectiveProjectionOptions.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/PerspectiveProjectionOptions.ts:2 (sha256:34ac68876d041d6ffeaa372aaf4fb1b6dfc40e83a7deda58e9368a6f82ebbb23)
#[derive(Clone, Default)]
pub struct PerspectiveProjectionOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub aspect: Option<f64>,
    pub fov_y: f64,
}
impl PartialEq for PerspectiveProjectionOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
