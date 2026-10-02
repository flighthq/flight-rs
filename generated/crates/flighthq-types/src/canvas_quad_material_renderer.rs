// @generated from upstream/packages/types/src/CanvasQuadMaterialRenderer.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{CanvasMaterialState, Material};

// Source: upstream/packages/types/src/CanvasQuadMaterialRenderer.ts:12 (sha256:6214271e06fb451bb2f43a0afca29a0a90ef7df96e5d69448634ff5cbd4ffaa1)
#[derive(Clone)]
pub struct CanvasQuadMaterialRenderer {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub get_state: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(Material) -> CanvasMaterialState + Send + 'static>>,
    >,
}
impl PartialEq for CanvasQuadMaterialRenderer {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
