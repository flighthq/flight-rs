// @generated from upstream/packages/types/src/Skin.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{SceneNode, Skeleton3D};

// Source: upstream/packages/types/src/Skin.ts:13 (sha256:4237493d27e6444829b2009db82e94785c1ed46e5e9d134ec1b288566e759151)
#[derive(Clone)]
pub struct Skin {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub skeleton: Skeleton3D,
    pub skeleton_root: Option<SceneNode>,
}
impl PartialEq for Skin {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
