// @generated from upstream/packages/types/src/CanvasMaterialRenderer.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{CanvasMaterialState, Material};

// Source: upstream/packages/types/src/CanvasMaterialRenderer.ts:12 (sha256:02126e04da889515e04d88db18964d8168265460dd95229b45a9cbb62db39d62)
#[derive(Clone)]
pub struct CanvasMaterialRenderer {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub get_state: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(Material) -> CanvasMaterialState + Send + 'static>>,
    >,
}
impl PartialEq for CanvasMaterialRenderer {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
