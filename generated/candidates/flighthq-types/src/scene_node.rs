// @generated from upstream/packages/types/src/SceneNode.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    Adjustment, ColorTransform, InteractionSignals, Kind, Matrix4, Node, NodeData,
    NodeInteractionState, NodeSignals, NodeTraitsKey, Quaternion, Vector3,
};

// Source: upstream/packages/types/src/SceneNode.ts:4 (sha256:d7065d58253b94793256d5459106599ce301c85eef2ca88243f7d65333380b37)
pub const SCENE_NODE_KIND: &'static str = "SceneNode";

// Source: upstream/packages/types/src/SceneNode.ts:5 (sha256:e59b10ab032a8806eb3f07b403def56d6adc8d08691cf80873bc9ead2465c762)
#[derive(Clone)]
pub struct SceneNodeTraits {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub alpha: f64,
    pub visible: bool,
    pub position: Vector3,
    pub rotation: Quaternion,
    pub scale: Vector3,
}
impl PartialEq for SceneNodeTraits {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/SceneNode.ts:6 (sha256:6e5459ccd42b488779163a72435bac6de76645ff853dd8d832f9e843f9a146be)
#[derive(Clone)]
pub struct SceneNode {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub data: Option<NodeData>,
    pub enabled: bool,
    pub kind: Kind,
    pub name: Option<String>,
    pub alpha: f64,
    pub visible: bool,
    pub position: Vector3,
    pub rotation: Quaternion,
    pub scale: Vector3,
}
impl PartialEq for SceneNode {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/SceneNode.ts:7 (sha256:ebe7357fd0aec285ee7f27a5aa1d89299adb37a5ba3864dc5dbb1a02cd10f8b3)
#[derive(Clone)]
pub struct SceneNodeRuntime {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub binding: Option<crate::OpaqueHostValue>,
    pub appearance_id: f64,
    pub bounds_using_local_bounds_id: f64,
    pub bounds_using_local_transform_id: f64,
    pub can_add_child:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(Node, Node) -> bool + Send + 'static>>>,
    pub children: Option<Vec<Node>>,
    pub color_adjustments: Option<Vec<Adjustment>>,
    pub resolved_color_transform: Option<ColorTransform>,
    pub color_adjustments_channel_mixing: bool,
    pub traits: Option<NodeTraitsKey>,
    pub interaction_signals: Option<InteractionSignals>,
    pub local_bounds_id: f64,
    pub local_bounds_using_local_bounds_id: f64,
    pub local_content_id: f64,
    pub local_transform_id: f64,
    pub local_transform_using_local_transform_id: f64,
    pub node_signals: Option<NodeSignals>,
    pub interaction_state: Option<NodeInteractionState>,
    pub parent: Option<Node>,
    pub world_bounds_using_local_bounds_id: f64,
    pub world_bounds_using_world_transform_id: f64,
    pub world_transform_id: f64,
    pub world_transform_using_local_transform_id: f64,
    pub world_transform_using_parent_transform_id: f64,
    pub world_alpha: Option<f64>,
    pub world_alpha_using_appearance_id: f64,
    pub world_alpha_using_parent_appearance_id: f64,
    pub world_appearance_id: f64,
    pub local_matrix4: Option<Matrix4>,
    pub local_matrix4_detached: bool,
    pub world_matrix4: Option<Matrix4>,
}
impl PartialEq for SceneNodeRuntime {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/SceneNode.ts:8 (sha256:14f10b8560a15ea01d6b65dbb9a023a8f33fe056f455a0f9be1217081ef75c3b)
pub static SCENE_NODE_TRAITS_KEY: std::sync::LazyLock<crate::FlightSymbol> =
    std::sync::LazyLock::new(|| crate::FlightSymbol::new());
