// @generated from upstream/packages/types/src/MeshAttachment2D.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::Skin2D;

// Source: upstream/packages/types/src/MeshAttachment2D.ts:17 (sha256:83a61ff7e39383c4a97d8682e07fd66014ca1e66d37304498a662e17ceea98d1)
#[derive(Clone, Default)]
pub struct MeshAttachment2D {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: String,
    pub name: Option<String>,
    pub skin: Option<Skin2D>,
    pub triangles: Vec<u16>,
    pub uvs: Vec<f32>,
    pub vertex_count: f64,
    pub vertices: Option<Vec<f32>>,
}
impl PartialEq for MeshAttachment2D {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/MeshAttachment2D.ts:25 (sha256:726f78a7d08e415b17b7888eb0bf4f7a9a57b466b8378a59d8cd2f6938e1c2bd)
pub const MESH_ATTACHMENT2_D_KIND: &'static str = "MeshAttachment2D";
