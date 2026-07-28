// @generated from upstream/packages/types/src/SceneNodeVisitor.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::SceneNode;

// Source: upstream/packages/types/src/SceneNodeVisitor.ts:2 (sha256:86ca853e0838537db084747dc4f08afe66eac61fb741deb10326eb0fbc05d6dc)
pub type SceneNodeVisitor =
    std::sync::Arc<dyn Fn(SceneNode, f64) -> crate::OpaqueHostValue + Send + Sync + 'static>;
