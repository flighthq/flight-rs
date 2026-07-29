// @generated from upstream/packages/types/src/SurfaceRegion.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::Surface;

// Source: upstream/packages/types/src/SurfaceRegion.ts:3 (sha256:f1eded16cf834bd12772e6b22abdafe531feeab64b0e59d11e2ef44f42e39c41)
#[derive(Clone, Default)]
pub struct SurfaceRegion {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub height: f64,
    pub surface: Surface,
    pub width: f64,
    pub x: f64,
    pub y: f64,
}
impl PartialEq for SurfaceRegion {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
