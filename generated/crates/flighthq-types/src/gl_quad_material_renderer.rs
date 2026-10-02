// @generated from upstream/packages/types/src/GlQuadMaterialRenderer.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{GlRenderState, Material, MaterialData};

// Source: upstream/packages/types/src/GlQuadMaterialRenderer.ts:19 (sha256:6d0119bc6bd1a4f663d8827bdb7874fbccf6f536747607d59a1e63b3f1d4ad52)
#[derive(Clone)]
pub struct GlQuadMaterialRenderer {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub instance_float_count: f64,
    pub bind: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(GlRenderState, Option<Material>) -> () + Send + 'static>>,
    >,
    pub pack_instance: Option<
        std::sync::Arc<
            std::sync::Mutex<
                Box<
                    dyn FnMut(GlRenderState, Option<MaterialData>, Vec<f32>, f64) -> ()
                        + Send
                        + 'static,
                >,
            >,
        >,
    >,
}
impl PartialEq for GlQuadMaterialRenderer {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
