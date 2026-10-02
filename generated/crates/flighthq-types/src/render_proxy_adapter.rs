// @generated from upstream/packages/types/src/RenderProxyAdapter.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{NodeAny, RenderProxy2D, RenderState};

// Source: upstream/packages/types/src/RenderProxyAdapter.ts:5 (sha256:ff90cbfce0c573808c7e56d5d91a1f774d015ceb1b7e3a73461a6bac2127cadf)
#[derive(Clone)]
pub struct RenderProxyAdapter {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub adapt: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(RenderState, NodeAny, RenderProxy2D) -> Option<bool> + Send + 'static>,
        >,
    >,
}
impl PartialEq for RenderProxyAdapter {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
