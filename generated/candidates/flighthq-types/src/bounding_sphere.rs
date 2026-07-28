// @generated from upstream/packages/types/src/BoundingSphere.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::Vector3;

// Source: upstream/packages/types/src/BoundingSphere.ts:6 (sha256:92c524ce02c86cb0a9e5328f47a35f2de5f28ff17c6a6e73d56cc9fb56bfef78)
#[derive(Clone)]
pub struct BoundingSphere {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub center: Vector3,
    pub radius: f64,
}
impl PartialEq for BoundingSphere {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/BoundingSphere.ts:11 (sha256:32e9a87d519d3d874f0ff7571b5de451c7edcad250ebc21fa0704fedcba65132)
pub type BoundingSphereLike = BoundingSphere;
