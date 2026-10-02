// @generated from upstream/packages/types/src/GlEnvironmentCaptureOptions.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{Environment, Node3D};

// Source: upstream/packages/types/src/GlEnvironmentCaptureOptions.ts:4 (sha256:03c29215e91c58aaf44875c1f799dec38d114aaff25774456008f077be8bf6f4)
#[derive(Clone, Default)]
pub struct GlEnvironmentCaptureOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub environment: Option<Environment>,
    pub exclude_node: Option<Node3D>,
    pub far: Option<f64>,
    pub near: Option<f64>,
}
impl PartialEq for GlEnvironmentCaptureOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
