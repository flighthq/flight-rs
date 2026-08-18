// @generated from upstream/packages/types/src/BoundingBoxAttachment2D.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::Skin2D;

// Source: upstream/packages/types/src/BoundingBoxAttachment2D.ts:16 (sha256:36d6cb63146034035859919f3e62ba4d729db8ea28b7d46e67ca4ef11b8eb787)
#[derive(Clone, Default)]
pub struct BoundingBoxAttachment2D {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: String,
    pub name: Option<String>,
    pub point_count: f64,
    pub skin: Option<Skin2D>,
    pub vertices: Option<Vec<f32>>,
}
impl PartialEq for BoundingBoxAttachment2D {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/BoundingBoxAttachment2D.ts:23 (sha256:b8891393e9b8df535be5150330b395fb8904153fde301a9da0465e9a3ad83c76)
pub const BOUNDING_BOX_ATTACHMENT2_D_KIND: &'static str = "BoundingBoxAttachment2D";
