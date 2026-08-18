// @generated from upstream/packages/types/src/GlPbrTransmissionSceneColor.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/GlPbrTransmissionSceneColor.ts:3 (sha256:1e3cadb5bd83513d7b7a2409cbf451c3acf9431c607abee493026ef8dab3593d)
#[derive(Clone, Default)]
pub struct GlPbrTransmissionSceneColor {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub height: f64,
    pub mip_level_count: f64,
    pub texture: crate::OpaqueHostValue,
    pub width: f64,
}
impl PartialEq for GlPbrTransmissionSceneColor {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
