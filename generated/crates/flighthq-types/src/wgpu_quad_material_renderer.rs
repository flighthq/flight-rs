// @generated from upstream/packages/types/src/WgpuQuadMaterialRenderer.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{Material, MaterialData, WgpuRenderState};

// Source: upstream/packages/types/src/WgpuQuadMaterialRenderer.ts:15 (sha256:f389a435306830acb1d32bc3861f51a780c18812dcc065b7cbd31714cddb6ea4)
#[derive(Clone)]
pub struct WgpuQuadMaterialRenderer {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub instance_float_count: f64,
    pub get_shader_module: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(WgpuRenderState) -> crate::OpaqueHostValue + Send + 'static>,
        >,
    >,
    pub pack_instance: Option<
        std::sync::Arc<
            std::sync::Mutex<
                Box<
                    dyn FnMut(
                            WgpuRenderState,
                            Option<Material>,
                            Option<MaterialData>,
                            Vec<f32>,
                            f64,
                        ) -> ()
                        + Send
                        + 'static,
                >,
            >,
        >,
    >,
}
impl PartialEq for WgpuQuadMaterialRenderer {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
