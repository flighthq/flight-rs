// @generated from upstream/packages/types/src/Scene3DPickOptions.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::Mesh;

// Source: upstream/packages/types/src/Scene3DPickOptions.ts:7 (sha256:6914133f0535e650a5b7181614136d46c7fa03b9773fca158f515a2ae60b8e41)
#[derive(Clone, Default)]
pub struct Scene3DPickOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub cull_backfaces: Option<bool>,
    pub max_distance: Option<f64>,
    pub predicate:
        Option<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(Mesh) -> bool + Send + 'static>>>>,
}
impl PartialEq for Scene3DPickOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
