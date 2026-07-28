// @generated from upstream/packages/types/src/RenderDrawContext.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{BatchFormat, BlendMode, Kind, Material};

// Source: upstream/packages/types/src/RenderDrawContext.ts:5 (sha256:a0d757ff90b238ea3420832080114481ba2787fc2d75e8eb488e18f895b8ee51)
#[derive(Clone)]
pub struct RenderBatchKey {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub blend: Option<BlendMode>,
    pub format: BatchFormat,
    pub material: Option<Material>,
    pub renderer_kind: Kind,
    pub texture: Option<crate::OpaqueHostValue>,
}
impl PartialEq for RenderBatchKey {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/RenderDrawContext.ts:12 (sha256:64c7fd0c32d1edb4c773b2c5102350b78ef31ae0b5968d34b119d4b8b3706b94)
#[derive(Clone)]
pub struct RenderDrawContext {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub draw_call_count: f64,
    pub flush_count: f64,
    pub open_batch_key: Option<RenderBatchKey>,
    pub proxy_visited_count: f64,
}
impl PartialEq for RenderDrawContext {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
