// @generated from upstream/packages/types/src/WgpuRenderStats.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/WgpuRenderStats.ts:2 (sha256:334ac23cd19daf6c55979c39f014391abb9473b2ef7c89cdae57aa847910e9ec)
#[derive(Clone)]
pub struct WgpuRenderStats {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub draw_call_count: f64,
    pub instance_count: f64,
    pub batch_flush_count: f64,
    pub texture_upload_count: f64,
}
impl PartialEq for WgpuRenderStats {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
