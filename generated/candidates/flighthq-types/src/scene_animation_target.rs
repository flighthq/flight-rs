// @generated from upstream/packages/types/src/SceneAnimationTarget.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{SceneAnimationPath, SceneNode};

// Source: upstream/packages/types/src/SceneAnimationTarget.ts:9 (sha256:01ab1226d7ebad39e82c031c642a554df71ce554250b32dd76289a8ee735cabc)
#[derive(Clone, Default)]
pub struct SceneAnimationTarget {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub node: SceneNode,
    pub path: SceneAnimationPath,
}
impl PartialEq for SceneAnimationTarget {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
