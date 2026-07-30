// @generated from upstream/packages/types/src/MeshMorphBindPose.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/MeshMorphBindPose.ts:11 (sha256:46ae94ec0d2f4ecb8234eaf136c57802dfddfe945afee70775a2cd3b7ed391ec)
#[derive(Clone, Default)]
pub struct MeshMorphBindPose {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub blended_normals: Option<Vec<f32>>,
    pub blended_positions: Vec<f32>,
    pub blended_tangents: Option<Vec<f32>>,
    pub normals: Option<Vec<f32>>,
    pub positions: Vec<f32>,
    pub tangents: Option<Vec<f32>>,
}
impl PartialEq for MeshMorphBindPose {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
