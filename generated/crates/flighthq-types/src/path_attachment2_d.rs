// @generated from upstream/packages/types/src/PathAttachment2D.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{PathWinding, Skin2D};

// Source: upstream/packages/types/src/PathAttachment2D.ts:36 (sha256:e9b6dcb1216bfc2e91a753d24eb0a7320ccca947cd062903bc3d24707bc4d426)
#[derive(Clone, Default)]
pub struct PathAttachment2D {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: String,
    pub name: Option<String>,
    pub commands: Vec<f64>,
    pub point_count: f64,
    pub skin: Option<Skin2D>,
    pub vertices: Option<Vec<f32>>,
    pub winding: PathWinding,
}
impl PartialEq for PathAttachment2D {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/PathAttachment2D.ts:47 (sha256:9f0023dbaaa5f22353f313915c44118b0e72d0164c3514483d1716673a71973b)
pub const PATH_ATTACHMENT2_D_KIND: &'static str = "PathAttachment2D";
