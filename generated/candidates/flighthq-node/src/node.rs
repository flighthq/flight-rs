// @generated from upstream/packages/node/src/node.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{invalidate_node, remove_node_child};
use flighthq_entity::{create_entity_runtime, get_entity_runtime};
use flighthq_signals::{clear_signal, create_signal};
use flighthq_types::{
    Adjustment, ColorTransform, Entity, InteractionSignals, Kind, Node, NodeDataFactory,
    NodeInteractionState, NodeRuntime, NodeRuntimeFactory, NodeSignals, NodeTraitsKey,
};

#[derive(Clone)]
pub struct FlightPartialRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub binding: Option<crate::OpaqueHostValue>,
    pub appearance_id: Option<f64>,
    pub bounds_using_local_bounds_id: Option<f64>,
    pub bounds_using_local_transform_id: Option<f64>,
    pub can_add_child: Option<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(Node, Node) -> bool + Send + 'static>>>,
    >,
    pub children: Option<Vec<Node>>,
    pub color_adjustments: Option<Vec<Adjustment>>,
    pub resolved_color_transform: Option<ColorTransform>,
    pub color_adjustments_channel_mixing: Option<bool>,
    pub traits: Option<NodeTraitsKey>,
    pub interaction_signals: Option<InteractionSignals>,
    pub local_bounds_id: Option<f64>,
    pub local_bounds_using_local_bounds_id: Option<f64>,
    pub local_content_id: Option<f64>,
    pub local_transform_id: Option<f64>,
    pub local_transform_using_local_transform_id: Option<f64>,
    pub node_signals: Option<NodeSignals>,
    pub interaction_state: Option<NodeInteractionState>,
    pub parent: Option<Node>,
    pub world_bounds_using_local_bounds_id: Option<f64>,
    pub world_bounds_using_world_transform_id: Option<f64>,
    pub world_transform_id: Option<f64>,
    pub world_transform_using_local_transform_id: Option<f64>,
    pub world_transform_using_parent_transform_id: Option<f64>,
}
impl PartialEq for FlightPartialRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/node/src/node.ts:20 (sha256:c3cb6204a77925ebc8ab244b5280f1c3fe2a4498454d588dc636e1ffdf7d9d00)
pub fn create_node<Data: Clone, Runtime: Clone>(
    node_kind: Kind,
    obj: Option<Node>,
    create_data: Option<NodeDataFactory<Data>>,
    create_node_runtime_factory: Option<NodeRuntimeFactory<Runtime>>,
) -> Node {
    let runtime_factory = (create_node_runtime_factory).unwrap_or(create_node_runtime);
    let mut out = Node {
        __flight_identity: std::sync::Arc::new(()),
        data: if (create_data).is_some() {
            Some({
                let __flight_callback = (create_data.as_ref().unwrap()).clone();
                let __flight_result = __flight_callback.lock().unwrap()(Some(
                    (obj.as_ref().and_then(|value| (value.data).clone())).unwrap(),
                ));
                __flight_result
            })
        } else {
            None
        },
        name: obj.as_ref().and_then(|value| (value.name).clone()),
        kind: (node_kind).clone(),
    };
    out.enabled = (obj.as_ref().map(|value| value.enabled)).unwrap_or(true);
    return out;
}

// Source: upstream/packages/node/src/node.ts:41 (sha256:cb2d5a08e65afff3d0bfeb52db6d1f1c8e107ba2a240933ca6c586bc438aee8d)
pub fn create_node_runtime<Traits: Clone>(
    methods: Option<FlightPartialRecord1>,
) -> NodeRuntime<Traits> {
    let mut out = create_entity_runtime();
    out.appearance_id = 0.0_f64;
    out.bounds_using_local_bounds_id = (-1.0_f64);
    out.bounds_using_local_transform_id = (-1.0_f64);
    out.can_add_child = (methods
        .as_ref()
        .and_then(|value| (value.can_add_child).clone()))
    .unwrap_or(std::sync::Arc::new(std::sync::Mutex::new(Box::new(
        move |__flight_argument_0: Node, __flight_argument_1: Node| -> bool {
            default_node_runtime_can_add_child(&__flight_argument_0, &__flight_argument_1)
        },
    )
        as Box<dyn FnMut(Node, Node) -> bool + Send + 'static>)));
    out.children = None;
    out.color_adjustments = None;
    out.resolved_color_transform = None;
    out.color_adjustments_channel_mixing = false;
    out.node_signals = None;
    out.interaction_signals = None;
    out.interaction_state = None;
    out.local_bounds_id = 0.0_f64;
    out.local_bounds_using_local_bounds_id = (-1.0_f64);
    out.local_content_id = 0.0_f64;
    out.local_transform_id = 0.0_f64;
    out.local_transform_using_local_transform_id = (-1.0_f64);
    out.parent = None;
    out.world_bounds_using_local_bounds_id = (-1.0_f64);
    out.world_bounds_using_world_transform_id = (-1.0_f64);
    out.world_transform_id = 0.0_f64;
    out.world_transform_using_local_transform_id = (-1.0_f64);
    out.world_transform_using_parent_transform_id = (-1.0_f64);
    return out;
}

// Source: upstream/packages/node/src/node.ts:70 (sha256:d5c07278e2b47491acf360abb26941038e0cbd9a281cc612ca38d7493825c862)
pub fn create_node_signals() -> NodeSignals {
    return NodeSignals {
        __flight_identity: std::sync::Arc::new(()),
        on_child_added: create_signal(),
        on_child_removed: create_signal(),
        on_children_changed: create_signal(),
        on_children_order_changed: create_signal(),
        on_parent_changed: create_signal(),
    };
}

// Source: upstream/packages/node/src/node.ts:80 (sha256:6bbf327cafeda85d6162c473d678208a8512e66d774e850e048a566db5daac0d)
pub fn default_node_runtime_can_add_child(_target: &Node, _child: &Node) -> bool {
    return true;
}

// Source: upstream/packages/node/src/node.ts:97 (sha256:1765dee6602ac1b442a498d6fa60c88b896ed640064a8db0dae7bb54a6d207c1)
pub fn dispose_node(target: &Node) -> () {
    let mut runtime = get_entity_runtime(&Entity {
        __flight_identity: std::sync::Arc::clone(&(target).__flight_identity),
    });
    let parent = (runtime.parent).clone();
    if (parent).is_some() {
        remove_node_child(&parent.as_ref().unwrap(), target);
    }
    let children = (runtime.children).clone();
    if (children).is_some() {
        let snapshot = (children.as_ref().unwrap()).clone();
        {
            let mut i = 0.0_f64;
            while (i < (snapshot.len() as f64)) {
                dispose_node(&snapshot[i as usize]);
                {
                    i += 1.0;
                    i
                };
            }
        }
        runtime.children = None;
    }
    let mut node_signals = (runtime.node_signals).clone();
    if (node_signals).is_some() {
        clear_signal(&mut node_signals.as_mut().unwrap().on_child_added);
        clear_signal(&mut node_signals.as_mut().unwrap().on_child_removed);
        clear_signal(&mut node_signals.as_mut().unwrap().on_children_changed);
        clear_signal(&mut node_signals.as_mut().unwrap().on_children_order_changed);
        clear_signal(&mut node_signals.as_mut().unwrap().on_parent_changed);
        runtime.node_signals = None;
    }
    let interaction_signals = (runtime.interaction_signals).clone();
    if (interaction_signals).is_some() {
        runtime.interaction_signals = None;
    }
    runtime.interaction_state = None;
}

// Source: upstream/packages/node/src/node.ts:139 (sha256:551dfe1fd464044bd176d313788b22dba049dd9866f38a3a9a4a41ee67830fc6)
pub fn enable_node_signals(source: &Node) -> NodeSignals {
    let mut runtime = get_entity_runtime(&Entity {
        __flight_identity: std::sync::Arc::clone(&(source).__flight_identity),
    });
    return {
        runtime.node_signals?? = Some(create_node_signals());
        runtime.node_signals
    };
}

// Source: upstream/packages/node/src/node.ts:144 (sha256:f67f85bd1019e6ace600710e14cf73de5c78ca31e09efa904d6d2929a81a465e)
pub fn get_node_runtime<Traits: Clone>(source: &Node) -> NodeRuntime<Traits> {
    return get_entity_runtime(&Entity {
        __flight_identity: std::sync::Arc::clone(&(source).__flight_identity),
    });
}

// Source: upstream/packages/node/src/node.ts:150 (sha256:79dede77260e41ec94bd413b273e961790a50bc19e112c51346e204c425f95e3)
pub fn get_node_signals(source: &Node) -> Option<NodeSignals> {
    return (get_entity_runtime(&Entity {
        __flight_identity: std::sync::Arc::clone(&(source).__flight_identity),
    })
    .node_signals)
        .clone();
}

// Source: upstream/packages/node/src/node.ts:154 (sha256:a7426aaac4798ea0baa3f8db0450d703cf79b68adbb164d343a36ebb8d5351e1)
pub fn set_node_enabled(target: &mut Node, value: bool) -> () {
    target.enabled = value;
    invalidate_node(target);
}
