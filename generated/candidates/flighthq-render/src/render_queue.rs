// @generated from upstream/packages/render/src/renderQueue.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::get_render_state_runtime;
use flighthq_node::get_node_runtime;
use flighthq_types::{
    Adjustment, BlendMode, ColorTransform, DisplayObjectClipHooks, InteractionSignals, Matrix,
    Node, NodeInteractionState, NodeSignals, NodeTraitsKey, RenderProxy, RenderQueue,
    RenderQueueEntry, RenderSortKey, RenderState, Renderable, SceneGraphSyncPolicy,
};

#[inline]
fn __flight_js_to_u32(value: f64) -> u32 {
    if !value.is_finite() || value == 0.0 {
        return 0;
    }
    value.trunc().rem_euclid(4294967296.0_f64) as u32
}

#[inline]
fn __flight_js_to_i32(value: f64) -> i32 {
    __flight_js_to_u32(value) as i32
}

#[derive(Clone, Default)]
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

#[derive(Clone, Default)]
pub struct FlightPartialRecord2 {
    pub __flight_identity: std::sync::Arc<()>,
    pub allow_smoothing: Option<bool>,
    pub background_color: Option<f64>,
    pub background_color_rgba: Option<Vec<f64>>,
    pub background_color_string: Option<String>,
    pub current_clip_depth: Option<f64>,
    pub display_object_clip_hooks: Option<DisplayObjectClipHooks>,
    pub pixel_ratio: Option<f64>,
    pub render_alpha: Option<f64>,
    pub render_blend_mode: Option<BlendMode>,
    pub render_transform2_d: Option<Matrix>,
    pub scene_graph_sync_policy: Option<SceneGraphSyncPolicy>,
    pub round_pixels: Option<bool>,
}
impl PartialEq for FlightPartialRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/render/src/renderQueue.ts:21 (sha256:4146245451006bf68200c59338e3ca689b3c4cda59f5e3d6ecada88114c2dbe6)
pub fn build_render_queue(state: &RenderState, source: &Renderable, out: &mut RenderQueue) -> () {
    clear_render_queue(out);
    let runtime = get_render_state_runtime(state);
    let mut stack_length = 1.0_f64;
    {
        let __flight_index = (0.0_f64) as usize;
        let __flight_value = (*source).clone();
        if __flight_index == _BUILD_STACK.len() {
            _BUILD_STACK.push(__flight_value);
        } else {
            _BUILD_STACK[__flight_index] = __flight_value;
        }
    };
    let mut scene_order = 0.0_f64;
    while (stack_length > 0.0_f64) {
        let current = _BUILD_STACK[{
            stack_length -= 1.0;
            stack_length
        } as usize]
            .clone();
        let proxy = runtime
            .render_proxy_map
            .iter()
            .find(|(key, _)| key == &(current).clone())
            .map(|(_, value)| value.clone());
        if (proxy).is_none() {
            continue;
        }
        if (!proxy.as_ref().unwrap().visible) {
            continue;
        }
        if ((proxy.as_ref().unwrap().renderer).clone()).is_some() {
            push_render_queue_entry(out, proxy.as_ref().unwrap(), scene_order);
        }
        {
            scene_order += 1.0;
            scene_order
        };
        let children = (get_node_runtime(&{
            let __flight_source = &(current);
            Node {
                __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
                data: (__flight_source.data).clone(),
                enabled: __flight_source.enabled,
                kind: (__flight_source.kind).clone(),
                name: (__flight_source.name).clone(),
            }
        })
        .children)
            .clone();
        if (children).is_some() {
            {
                let mut i = ((children.as_ref().unwrap().len() as f64) - 1.0_f64);
                while (i >= 0.0_f64) {
                    {
                        let __flight_index = ({
                            stack_length += 1.0;
                            stack_length
                        }) as usize;
                        let __flight_value = flighthq_types::Renderable::A(
                            children.as_ref().unwrap()[i as usize].clone(),
                        );
                        if __flight_index == _BUILD_STACK.len() {
                            _BUILD_STACK.push(__flight_value);
                        } else {
                            _BUILD_STACK[__flight_index] = __flight_value;
                        }
                    };
                    {
                        i -= 1.0;
                        i
                    };
                }
            }
        }
    }
}

// Source: upstream/packages/render/src/renderQueue.ts:51 (sha256:c8617b5b227d1ff26cd0b74f344c0e9941c08550a0ba9274337872d26c7b51bc)
pub fn clear_render_queue(queue: &mut RenderQueue) -> () {
    queue.entry_count = 0.0_f64;
}

// Source: upstream/packages/render/src/renderQueue.ts:56 (sha256:7f2ba1b9aab0a062f4a517e7dd11ee1e673a6b1452e19d896206e69d4b1c7d4f)
pub fn compare_render_queue_entries(a: &RenderQueueEntry, b: &RenderQueueEntry) -> f64 {
    return (a.sort_key - b.sort_key);
}

// Source: upstream/packages/render/src/renderQueue.ts:62 (sha256:133cbe800e0ebb681e160316c549e5f46b991349be2042062e57cfe8855f3ceb)
pub fn create_render_queue() -> RenderQueue {
    return RenderQueue {
        __flight_identity: std::sync::Arc::new(()),
        entries: vec![],
        entry_count: 0.0_f64,
    };
}

// Source: upstream/packages/render/src/renderQueue.ts:74 (sha256:ad802a7df727321ff928a6a490abb016dd32040658c7eb04886a57a1e8f26a45)
pub fn pack_render_sort_key(layer: f64, depth: f64, is_transparent: bool) -> RenderSortKey {
    let layer_bits = __flight_js_to_i32(
        (__flight_js_to_i32((0.0_f64).max(
            (32767.0_f64).min((__flight_js_to_i32(layer) | __flight_js_to_i32(0.0_f64)) as f64),
        )) & __flight_js_to_i32(32767.0_f64)) as f64,
    )
    .wrapping_shl((__flight_js_to_u32(16.0_f64) & 31)) as f64;
    let transparent_bit = if is_transparent {
        __flight_js_to_i32(1.0_f64).wrapping_shl((__flight_js_to_u32(15.0_f64) & 31)) as f64
    } else {
        0.0_f64
    };
    let depth_bits =
        (__flight_js_to_i32((0.0_f64).max((32767.0_f64).min((depth * 32767.0_f64).round())))
            & __flight_js_to_i32(32767.0_f64)) as f64;
    return (__flight_js_to_i32(
        (__flight_js_to_i32(layer_bits) | __flight_js_to_i32(transparent_bit)) as f64,
    ) | __flight_js_to_i32(depth_bits)) as f64;
}

// Source: upstream/packages/render/src/renderQueue.ts:83 (sha256:4c353b9fd165df77fd21a061d844f15702320db73f5e3e0d38bbd7dc05d2aa6d)
pub fn push_render_queue_entry(
    queue: &mut RenderQueue,
    proxy: &RenderProxy,
    sort_key: RenderSortKey,
) -> () {
    let entry = RenderQueueEntry {
        __flight_identity: std::sync::Arc::new(()),
        proxy: (*proxy).clone(),
        sort_key: sort_key,
    };
    if (queue.entry_count < (queue.entries.len() as f64)) {
        {
            let __flight_index = (queue.entry_count) as usize;
            let __flight_value = entry;
            if __flight_index == queue.entries.len() {
                queue.entries.push(__flight_value);
            } else {
                queue.entries[__flight_index] = __flight_value;
            }
        };
    } else {
        queue.entries.push(entry);
    }
    {
        queue.entry_count += 1.0;
        queue.entry_count
    };
}

// Source: upstream/packages/render/src/renderQueue.ts:97 (sha256:655bd48bf63bcc9d373f15d7b09563c9b70f931d1415e7b0d471f418a5eed06a)
pub fn sort_render_queue(
    queue: &mut RenderQueue,
    compare: Option<
        std::sync::Arc<
            std::sync::Mutex<
                Box<dyn FnMut(RenderQueueEntry, RenderQueueEntry) -> f64 + Send + 'static>,
            >,
        >,
    >,
) -> () {
    let mut slice =
        ((queue.entries).clone())[(0.0_f64) as usize..(queue.entry_count) as usize].to_vec();
    {
        let mut __flight_values = slice;
        __flight_values
            .sort_by(|left, right| left.partial_cmp(right).unwrap_or(std::cmp::Ordering::Equal));
        __flight_values
    };
    {
        let mut i = 0.0_f64;
        while (i < (slice.len() as f64)) {
            {
                let __flight_index = (i) as usize;
                let __flight_value = slice[i as usize].clone();
                if __flight_index == queue.entries.len() {
                    queue.entries.push(__flight_value);
                } else {
                    queue.entries[__flight_index] = __flight_value;
                }
            };
            {
                i += 1.0;
                i
            };
        }
    }
}

// Source: upstream/packages/render/src/renderQueue.ts:109 (sha256:21199f911fc0665dd25800f833a5614ac18db83f1a24b20553b2b171d924c344)
fn compare_render_queue_entries_by_key(a: &RenderQueueEntry, b: &RenderQueueEntry) -> f64 {
    return (a.sort_key - b.sort_key);
}

// Source: upstream/packages/render/src/renderQueue.ts:115 (sha256:649dbaf3919ca571327388f57e638c5f98e9046cb50d4ec5809e04cc630411c2)
static _BUILD_STACK: std::sync::LazyLock<std::sync::Mutex<Vec<Renderable>>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(vec![]));
