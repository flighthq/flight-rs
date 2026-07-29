// @generated from upstream/packages/types/src/WgpuMaterialRenderer.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{Material, MaterialData, WgpuRenderState};

// Source: upstream/packages/types/src/WgpuMaterialRenderer.ts:15 (sha256:076f59ee0f05fca1621199e0b4db7cdafa86abc8e5c8b7f43296974efee7867b)
#[derive(Clone)]
pub struct WgpuMaterialRenderer {
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
impl PartialEq for WgpuMaterialRenderer {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
