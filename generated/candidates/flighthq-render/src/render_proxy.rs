// @generated from upstream/packages/render/src/renderProxy.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    get_render_state_runtime, update_render_proxy_appearance, update_render_proxy_color_transform,
    update_render_proxy_material, update_render_proxy2_d_transform,
};
use flighthq_entity::create_entity;
use flighthq_geometry::create_matrix;
use flighthq_node::{
    get_node_appearance_revision, get_node_local_content_revision,
    get_node_local_transform_revision, get_node_parent, get_node_runtime,
};
use flighthq_types::{
    BLEND_MODE as blend_mode_constant, Node, RenderProxy, RenderProxy2D, RenderState, Renderable,
};

// Source: upstream/packages/render/src/renderProxy.ts:28 (sha256:d894ec3d1e6e7a6a856346f4278e905842c32b9f1a96201950040c468c38fa0c)
type AdaptHook = std::sync::Arc<
    std::sync::Mutex<Box<dyn FnMut(RenderState, Renderable, RenderProxy2D) -> () + Send + 'static>>,
>;

// Source: upstream/packages/render/src/renderProxy.ts:32 (sha256:2cae7bbf9ab7e85e6310dd599b38f294d288a397f60aef8addf3bfae1db346b0)
pub type RenderProxyVisitor = std::sync::Arc<
    std::sync::Mutex<
        Box<
            dyn FnMut(RenderState, Renderable, RenderProxy2D, Option<RenderProxy2D>) -> ()
                + Send
                + 'static,
        >,
    >,
>;

// Source: upstream/packages/render/src/renderProxy.ts:39 (sha256:f08bf21049f8e93b4b772b411ecd9db384cb63a0869fd6aa8dc21bf7f4567e46)
pub fn create_render_proxy(state: &RenderState, source: &Renderable) -> RenderProxy {
    let runtime = get_render_state_runtime(state);
    let renderer = runtime
        .renderer_map
        .iter()
        .find(|(key, _)| key == &(source.kind).clone())
        .map(|(_, value)| value.clone());
    return create_entity(Some(RenderProxy {
        __flight_identity: std::sync::Arc::new(()),
        source: (*source).clone(),
        kind: (source.kind).clone(),
        next: None,
        alpha: 1.0_f64,
        appearance_frame_id: (-1.0_f64),
        blend_mode: Some((blend_mode_constant.normal).clone()),
        color_transform: None,
        material: None,
        material_data: None,
        last_appearance_id: (-1.0_f64),
        last_local_content_id: (-1.0_f64),
        last_local_transform_id: (-1.0_f64),
        name: None,
        renderer: (renderer).clone(),
        renderer_data: {
            let __flight_callback = renderer
                .as_ref()
                .unwrap()
                .create_data
                .as_ref()
                .unwrap()
                .clone();
            let __flight_result =
                __flight_callback.lock().unwrap()((*state).clone(), (*source).clone());
            __flight_result
        },
        renderer_data_source: Some((*source).clone()),
        renderer_map_id: runtime.renderer_map_id,
        transform_frame_id: (-1.0_f64),
        visible: true,
    }));
}

// Source: upstream/packages/render/src/renderProxy.ts:68 (sha256:1d50624211f554619cb4603869ccdc0833d651a1f97e656af52537147d9dc2dd)
#[derive(Clone)]
struct CreateRenderProxy2DRecord1 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreateRenderProxy2DRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_render_proxy2_d(
    state: &RenderState,
    source: &CreateRenderProxy2DRecord1,
) -> RenderProxy2D {
    let mut node = create_render_proxy(state, source);
    node.transform2_d = create_matrix(None, None, None, None, None, None);
    node.traverse_children = true;
    node.clip_depth = 0.0_f64;
    return node;
}

// Source: upstream/packages/render/src/renderProxy.ts:86 (sha256:7e70ba0c2d12b41cf87957fdbdbeed0d25db52ff3bc4c856a3db500ec934e4fd)
pub fn dispose_display_object_render(state: &RenderState, root: &Renderable) -> () {
    walk_render_subtree(state, root, &mut dispose_render_proxy);
}

// Source: upstream/packages/render/src/renderProxy.ts:94 (sha256:22de9356a88a410d49dea475fd1b5b937476af861f8383fee9113b320f52286f)
pub fn dispose_render_proxy(state: &RenderState, source: &Renderable) -> () {
    let node = get_render_state_runtime(state)
        .render_proxy_map
        .iter()
        .find(|(key, _)| key == &(*source).clone())
        .map(|(_, value)| value.clone());
    if (node).is_none() {
        return;
    }
    if ((node.as_ref().unwrap().renderer_data).clone()).is_some() {
        {
            let __flight_callback = node
                .as_ref()
                .unwrap()
                .renderer
                .as_ref()
                .and_then(|value| (value.destroy_data).clone());
            __flight_callback.as_ref().map(|callback| {
                callback.lock().unwrap()(
                    (*state).clone(),
                    ((node.as_ref().unwrap().renderer_data).clone()).unwrap(),
                )
            })
        };
    }
    {
        let __flight_key = (*source).clone();
        if let Some(__flight_index) = get_render_state_runtime(state)
            .render_proxy_map
            .iter()
            .position(|(key, _)| key == &__flight_key)
        {
            get_render_state_runtime(state)
                .render_proxy_map
                .remove(__flight_index);
            true
        } else {
            false
        }
    };
}

// Source: upstream/packages/render/src/renderProxy.ts:102 (sha256:099caff0f69c91cbabd32183dd8757f741a004aa325fa5b436e39529f5cfcf73)
#[derive(Clone)]
struct GetOrCreateRenderProxy2DRecord1 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for GetOrCreateRenderProxy2DRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn get_or_create_render_proxy2_d(state: &RenderState, source: &Renderable) -> RenderProxy2D {
    let mut runtime = get_render_state_runtime(state);
    let mut node = runtime
        .render_proxy_map
        .iter()
        .find(|(key, _)| key == &(*source).clone())
        .map(|(_, value)| value.clone());
    if (node).is_none() {
        node = Some(create_render_proxy2_d(state, &source));
        {
            let __flight_key = (*source).clone();
            let __flight_value = (node).clone().unwrap();
            if let Some((_, value)) = runtime
                .render_proxy_map
                .iter_mut()
                .find(|(key, _)| key == &__flight_key)
            {
                *value = __flight_value;
            } else {
                runtime
                    .render_proxy_map
                    .push((__flight_key, __flight_value));
            }
        };
    }
    if (node.as_mut().unwrap().renderer_map_id != runtime.renderer_map_id) {
        update_render_proxy_renderer(state, &mut node);
    }
    return (node).clone().unwrap();
}

// Source: upstream/packages/render/src/renderProxy.ts:116 (sha256:020621bf9373b9a3534c1a6ed4a39684dd8c9aed1523f4965915b4efe48842ea)
pub fn get_render_proxy2_d(state: &RenderState, source: &Renderable) -> Option<RenderProxy2D> {
    return get_render_state_runtime(state)
        .render_proxy_map
        .iter()
        .find(|(key, _)| key == &(*source).clone())
        .map(|(_, value)| value.clone());
}

// Source: upstream/packages/render/src/renderProxy.ts:120 (sha256:7611587d0f27185f43869bf78cc98b791918b49c57d271624c7e59f80326f3db)
pub fn install_render_adapt_hook(state: &RenderState, fn_: AdaptHook) -> () {
    get_render_state_runtime(state).render_adapt_hook = Some((fn_).clone());
}

// Source: upstream/packages/render/src/renderProxy.ts:124 (sha256:0cbb9b9044c1cdae331ef06656017c7a6453199ec7c2b4036c28e574838fb94c)
pub fn is_render_proxy_dirty(
    state: &RenderState,
    source: &Renderable,
    data: &RenderProxy,
    parent_data: Option<RenderProxy>,
) -> bool {
    let current_frame_id = get_render_state_runtime(state).current_frame_id;
    let parent_dirty = ((parent_data).is_some())
        && ((parent_data.as_ref().unwrap().transform_frame_id == current_frame_id)
            || (parent_data.as_ref().unwrap().appearance_frame_id == current_frame_id));
    let local_dirty = ((((state.scene_graph_sync_policy).clone() == "refreshDerivedState")
        || (data.last_local_transform_id != get_node_local_transform_revision(&source)))
        || (data.last_appearance_id != get_node_appearance_revision(&source)))
        || (data.last_local_content_id != get_node_local_content_revision(&source));
    return (parent_dirty) || (local_dirty);
}

// Source: upstream/packages/render/src/renderProxy.ts:142 (sha256:79c68d259d30f18e8d7d49715d973a1ebd294921c549dc66cddb1fb07c04a58f)
pub fn is_render_proxy_visible(data: &RenderProxy2D) -> bool {
    return ((data.visible) && (data.alpha > 0.0_f64))
        && (!(data.transform2_d.a == 0.0_f64) && (data.transform2_d.d == 0.0_f64));
}

// Source: upstream/packages/render/src/renderProxy.ts:151 (sha256:0268c67aefba9a5406053c5af12340b99402e8b64776a903cdbd3da95c7e2ee2)
pub fn prepare_display_object_render(state: &RenderState, source: &Renderable) -> bool {
    return walk_node(state, source, &mut update_render_proxy2_d);
}

// Source: upstream/packages/render/src/renderProxy.ts:159 (sha256:f96c231dd91e6650215d702ec62e607764c535f1833abdb4f905ba7dc31253cc)
pub fn update_node_clip(
    _state: &RenderState,
    source: &Renderable,
    data: &mut RenderProxy2D,
    parent_data: Option<RenderProxy2D>,
) -> () {
    let parent_depth = if (parent_data).is_some() {
        parent_data.as_ref().unwrap().clip_depth
    } else {
        0.0_f64
    };
    data.clip_depth = (parent_depth
        + if ((source.clip).clone()).is_some() {
            1.0_f64
        } else {
            0.0_f64
        });
}

// Source: upstream/packages/render/src/renderProxy.ts:171 (sha256:9adaf1d9bf17523bc630dd53ad044f3c8efb7e9c13065f40c67e7a07da705337)
pub fn update_render_proxy2_d(
    state: &RenderState,
    source: &Renderable,
    data: &mut RenderProxy2D,
    parent_data: Option<RenderProxy2D>,
) -> () {
    update_render_proxy_appearance(state, data, Some(((parent_data).clone().unwrap()).clone()));
    update_render_proxy2_d_transform(state, data, Some(((parent_data).clone().unwrap()).clone()));
    update_render_proxy_material(state, data, Some(((parent_data).clone().unwrap()).clone()));
    update_render_proxy_color_transform(
        state,
        data,
        Some(((parent_data).clone().unwrap()).clone()),
    );
    update_node_clip(state, source, data, ((parent_data).clone()).clone());
    data.last_local_content_id = get_node_local_content_revision(&source);
    {
        let __flight_callback = (get_render_state_runtime(state).render_adapt_hook).clone();
        __flight_callback.as_ref().map(|callback| {
            callback.lock().unwrap()((*state).clone(), (*source).clone(), (*data).clone())
        })
    };
}

// Source: upstream/packages/render/src/renderProxy.ts:187 (sha256:aee55b70125fc03b9970554499f43aed9ab9b6147612cd7249ca27ffab9a10b6)
pub fn update_render_proxy_renderer(state: &RenderState, node: &mut RenderProxy) -> () {
    let runtime = get_render_state_runtime(state);
    let renderer = runtime
        .renderer_map
        .iter()
        .find(|(key, _)| key == &(node.kind).clone())
        .map(|(_, value)| value.clone());
    if ((node.renderer).clone() != renderer)
        || (!(((node.renderer_data_source).clone()) == Some((node.source).clone())))
    {
        if ((node.renderer_data).clone()).is_some() {
            {
                let __flight_callback = node
                    .renderer
                    .as_ref()
                    .and_then(|value| (value.destroy_data).clone());
                __flight_callback.as_ref().map(|callback| {
                    callback.lock().unwrap()(
                        (*state).clone(),
                        ((node.renderer_data).clone()).unwrap(),
                    )
                })
            };
        }
        node.renderer = (renderer).clone();
        node.renderer_data = {
            let __flight_callback = renderer
                .as_ref()
                .unwrap()
                .create_data
                .as_ref()
                .unwrap()
                .clone();
            let __flight_result =
                __flight_callback.lock().unwrap()((*state).clone(), (node.source).clone());
            __flight_result
        };
        node.renderer_data_source = Some((node.source).clone());
    }
    node.renderer_map_id = runtime.renderer_map_id;
}

// Source: upstream/packages/render/src/renderProxy.ts:204 (sha256:7de92aa499bc121546f31a75b61bb9be148e051fb98ccc77e26c5ebc7bdcba0e)
pub fn walk_node(
    state: &RenderState,
    root: &Renderable,
    visit: &mut impl FnMut(RenderState, Renderable, RenderProxy2D, Option<RenderProxy2D>) -> (),
) -> bool {
    let mut runtime = get_render_state_runtime(state);
    {
        runtime.current_frame_id += 1.0;
        runtime.current_frame_id
    };
    let mut stack_length = 1.0_f64;
    {
        let __flight_index = (0.0_f64) as usize;
        let __flight_value = (*root).clone();
        if __flight_index == runtime.temp_stack.len() {
            runtime.temp_stack.push(__flight_value);
        } else {
            runtime.temp_stack[__flight_index] = __flight_value;
        }
    };
    let mut parent_data: Option<RenderProxy2D> = None;
    let mut last_parent: Option<Node> = None;
    let mut tree_dirty = false;
    while (stack_length > 0.0_f64) {
        let current = runtime.temp_stack[{
            stack_length -= 1.0;
            stack_length
        } as usize]
            .clone();
        if (!current.enabled) {
            continue;
        }
        if (current != root) {
            let parent = get_node_parent(&current);
            if (parent).is_none() {
                parent_data = None;
                last_parent = None;
            } else {
                if (parent.as_ref().unwrap() != last_parent) {
                    parent_data = Some(get_or_create_render_proxy2_d(
                        state,
                        &parent.as_ref().unwrap(),
                    ));
                    last_parent = Some(parent.as_ref().unwrap());
                }
            }
        }
        let data = get_or_create_render_proxy2_d(state, &current);
        if is_render_proxy_dirty(
            state,
            &current,
            &data,
            Some(((parent_data).clone().unwrap()).clone()),
        ) {
            visit(
                (*state).clone(),
                (current).clone(),
                (data).clone(),
                (parent_data).clone(),
            );
            tree_dirty = true;
        }
        if (!is_render_proxy_visible(&data)) {
            continue;
        }
        if data.traverse_children {
            let children = (get_node_runtime(&current).children).clone();
            if (children).is_some() {
                {
                    let mut i = ((children.as_ref().unwrap().len() as f64) - 1.0_f64);
                    while (i >= 0.0_f64) {
                        {
                            let __flight_index = ({
                                stack_length += 1.0;
                                stack_length
                            }) as usize;
                            let __flight_value = children.as_ref().unwrap()[i as usize].clone();
                            if __flight_index == runtime.temp_stack.len() {
                                runtime.temp_stack.push(__flight_value);
                            } else {
                                runtime.temp_stack[__flight_index] = __flight_value;
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
    return tree_dirty;
}

// Source: upstream/packages/render/src/renderProxy.ts:257 (sha256:6506360450d6d71b906f7362c97dd8b630fadec5b123e2e3dd8078fb8b37c65c)
fn walk_render_subtree(
    state: &RenderState,
    root: &Renderable,
    visit: &mut impl FnMut(RenderState, Renderable) -> (),
) -> () {
    let mut stack_length = 1.0_f64;
    {
        let __flight_index = (0.0_f64) as usize;
        let __flight_value = (*root).clone();
        if __flight_index == get_render_state_runtime(state).temp_stack.len() {
            get_render_state_runtime(state)
                .temp_stack
                .push(__flight_value);
        } else {
            get_render_state_runtime(state).temp_stack[__flight_index] = __flight_value;
        }
    };
    while (stack_length > 0.0_f64) {
        let current = get_render_state_runtime(state).temp_stack[{
            stack_length -= 1.0;
            stack_length
        } as usize]
            .clone();
        visit((*state).clone(), (current).clone());
        let children = (get_node_runtime(&current).children).clone();
        if (children).is_some() {
            {
                let mut i = ((children.as_ref().unwrap().len() as f64) - 1.0_f64);
                while (i >= 0.0_f64) {
                    {
                        let __flight_index = ({
                            stack_length += 1.0;
                            stack_length
                        }) as usize;
                        let __flight_value = children.as_ref().unwrap()[i as usize].clone();
                        if __flight_index == get_render_state_runtime(state).temp_stack.len() {
                            get_render_state_runtime(state)
                                .temp_stack
                                .push(__flight_value);
                        } else {
                            get_render_state_runtime(state).temp_stack[__flight_index] =
                                __flight_value;
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
