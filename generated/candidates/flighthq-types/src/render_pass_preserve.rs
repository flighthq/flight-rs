// @generated from upstream/packages/types/src/RenderPassPreserve.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/RenderPassPreserve.ts:12 (sha256:2805a51b880bf870f6ccf558ebde7f8b063f961cfa63130bc3e2379ee52220c6)
#[derive(Clone)]
pub struct RenderPassPreserve {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub preserve_color: Option<crate::FlightUnion2<bool, Vec<bool>>>,
    pub preserve_depth: Option<bool>,
}
impl PartialEq for RenderPassPreserve {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
