// @generated from upstream/packages/types/src/Node.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

pub use crate::NodeSignals;
use crate::{Adjustment, ColorTransform, InteractionSignals, Kind, NodeInteractionState};

// Source: upstream/packages/types/src/Node.ts:7 (sha256:0651d5b16f8e351f86dd94441ad185cc688c533fec52863c0ee8c4ded6229cb9)
// TypeScript value namespace NodeTraitsKey is represented by its generated Rust type.

// Source: upstream/packages/types/src/Node.ts:8 (sha256:39191231a60dcc3e755c8e4031f1f112ede2e3524412f41f03330316061c9c1d)
#[derive(Clone)]
pub struct NodeTraitsKey {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for NodeTraitsKey {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Node.ts:11 (sha256:fd19f6940c105f6178fe6e2c765287b08a166e07192d5c7651f67df9b9d4612e)
pub type NodeData = crate::OpaqueHostValue;

// Source: upstream/packages/types/src/Node.ts:12 (sha256:a0ca5c6eda1567093009cdd9b82a3d010054165b96eb6cc50e665fc0cde6b922)
pub type NodeDataFactory<D> =
    std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(Option<D>) -> D + Send + 'static>>>;

// Source: upstream/packages/types/src/Node.ts:13 (sha256:a77dc1f914baf7d31135c3537c5a89fe7b7eaedf57b66a4bb17a44317cf68633)
pub type NodeRuntimeFactory<R> =
    std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(Option<R>) -> R + Send + 'static>>>;

// Source: upstream/packages/types/src/Node.ts:14 (sha256:38f920314f6e0716d2c3896881e9117164a3e94a87f00fff6e36f9f5d69e1d0c)
#[derive(Clone)]
pub struct NodeTraits {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub data: Option<NodeData>,
    pub enabled: bool,
    pub kind: Kind,
    pub name: Option<String>,
}
impl PartialEq for NodeTraits {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Node.ts:20 (sha256:426dc2d14c89b2af765f48019e4ba9f16cf36c7064b91789ac3a6f185cbf142e)
#[derive(Clone)]
pub struct Node {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub data: Option<NodeData>,
    pub enabled: bool,
    pub kind: Kind,
    pub name: Option<String>,
}
impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Node.ts:23 (sha256:34bfde655eed1560880cd3bc97d772be061f0f244761b84782bcbe13bc77d0b0)
#[derive(Clone)]
pub struct NodeRuntime {
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
}
impl PartialEq for NodeRuntime {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Node.ts:60 (sha256:f3e33f9e2043ae1b7d82638dcbec7989c280f19fffb05a6c0ddc9d85892a3665)
pub const NODE_KIND: &'static str = "Node";

// Source: upstream/packages/types/src/Node.ts:61 (sha256:5d1ab7ee845efa54f0ac99eff03034223dba1f7df72bdf0d6aba7c8e6ad5e93c)
pub type NodeOf = Node;

// Source: upstream/packages/types/src/Node.ts:62 (sha256:73990d06f638eee557927d22f63242afcdd0043ed6db74ec4e8501d9a359fde4)
pub static NULL_SCENE: std::sync::LazyLock<crate::FlightSymbol> =
    std::sync::LazyLock::new(|| crate::FlightSymbol::new());

// Source: upstream/packages/types/src/Node.ts:64 (sha256:18887280f4f7836bc0fbe77395f606df1d806a6ddfdf9d4c35cc6058b7de7eb2)
pub type NodeAny = Node;
