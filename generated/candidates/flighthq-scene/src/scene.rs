// @generated from upstream/packages/scene/src/scene.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::create_scene_node;
use flighthq_entity::create_entity;
pub use flighthq_types::Scene;
use flighthq_types::{SCENE_NODE_KIND as scene_node_kind_constant, SceneNode};

// Source: upstream/packages/scene/src/scene.ts:13 (sha256:83e79015335e10a8587a9c1c61074e7a3478234bce69f8ce0b642a9133fffc3b)
#[derive(Clone)]
struct CreateSceneRecord1 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreateSceneRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_scene(obj: Option<SceneNode>) -> Scene {
    let root = create_scene_node(
        Some(scene_node_kind_constant),
        Some(((obj).clone().unwrap()).clone()),
    );
    return create_entity(Some(Scene {
        __flight_identity: std::sync::Arc::new(()),
        animations: {
            let mut __flight_record = Vec::new();
            __flight_record
        },
        metadata: None,
        root: (root).clone(),
    }));
}
