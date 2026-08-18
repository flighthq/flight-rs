// @generated from upstream/packages/types/src/RenderProxyResolver.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{Node2D, RenderProxy2D, RenderState};

// Source: upstream/packages/types/src/RenderProxyResolver.ts:5 (sha256:5e7da443f4d5ef0f399e84292de8463bdc75544ffdea4248d5b3f2265a9a5b61)
#[derive(Clone)]
pub struct RenderProxyResolver {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub resolve: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(RenderState, Node2D, RenderProxy2D) -> Option<bool> + Send + 'static>,
        >,
    >,
}
impl PartialEq for RenderProxyResolver {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
