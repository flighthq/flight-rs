// @generated from upstream/packages/types/src/GlMaterialRenderer.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{GlRenderState, Material, MaterialData};

// Source: upstream/packages/types/src/GlMaterialRenderer.ts:19 (sha256:7c64da4d491b7f9ed538cd0cf3e523810a44a748708c6b0c55f02c118314e35e)
#[derive(Clone)]
pub struct GlMaterialRenderer {
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
impl PartialEq for GlMaterialRenderer {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
