// @generated from upstream/packages/scene/src/sceneNodeDispose.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_node::dispose_node;
use flighthq_types::SceneNode;

// Source: upstream/packages/scene/src/sceneNodeDispose.ts:14 (sha256:6ceb2c9b1f10930f91019087647b65f5d0e71d59e820422cff6fd1ea43e50077)
pub fn dispose_scene_node(node: &SceneNode) -> () {
    dispose_node(node);
}
