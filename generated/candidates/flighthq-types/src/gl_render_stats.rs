// @generated from upstream/packages/types/src/GlRenderStats.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/GlRenderStats.ts:1 (sha256:1f13b147835dbaa106b3a6a6d9c329a105bd72f05dc81e86f568f8c0522afca5)
#[derive(Clone)]
pub struct GlRenderStats {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub draw_calls: f64,
    pub triangles: f64,
    pub texture_binds: f64,
    pub program_switches: f64,
    pub framebuffer_binds: f64,
    pub uniform_uploads: f64,
}
impl PartialEq for GlRenderStats {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
