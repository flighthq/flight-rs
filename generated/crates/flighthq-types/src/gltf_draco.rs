// @generated from upstream/packages/types/src/GltfDraco.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/GltfDraco.ts:11 (sha256:fec37b8af3a3406d6640bec8d62ae4ffc5a516ac525c059814e030c79140b95c)
#[derive(Clone, Default)]
pub struct GltfDracoMesh {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub attributes: Vec<(String, Vec<f32>)>,
    pub indices: Option<Vec<u32>>,
    pub vertex_count: f64,
}
impl PartialEq for GltfDracoMesh {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/GltfDraco.ts:26 (sha256:334ca7889efaed1c18ea1a4c3fcf5a350bb9c0b9052180831a542bf7c6a3ab64)
pub type GltfDracoDecoder = std::sync::Arc<
    std::sync::Mutex<
        Box<dyn FnMut(Vec<u8>, Vec<(String, f64)>) -> Option<GltfDracoMesh> + Send + 'static>,
    >,
>;
