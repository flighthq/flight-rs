// @generated from upstream/packages/render/src/renderProxyAdapter.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{get_render_state_runtime, install_render_adapt_hook, update_render_proxy_renderer};
use flighthq_node::invalidate_node_appearance;
use flighthq_types::{RenderProxyAdapter, RenderState, Renderable};

// Source: upstream/packages/render/src/renderProxyAdapter.ts:7 (sha256:530d65dcefc2ec1b91a7b5ba71ebfb9db7e1bff73c7f86fdaaac92f0bc17d6c2)
#[derive(Clone)]
struct ApplyRenderProxyAdapterRecord1 {
    __flight_identity: std::sync::Arc<()>,
    traverse_children: bool,
}
impl PartialEq for ApplyRenderProxyAdapterRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn apply_render_proxy_adapter(
    state: &RenderState,
    source: &Renderable,
    data: &mut ApplyRenderProxyAdapterRecord1,
) -> () {
    let render_adapter = get_render_state_runtime(state)
        .render_proxy_adapter_map
        .iter()
        .find(|(key, _)| key == &(*source).clone())
        .map(|(_, value)| value.clone());
    let mut traverse_children = true;
    if (render_adapter).is_some() {
        let result = ((render_adapter.as_ref().unwrap().adapt).clone())
            .lock()
            .unwrap()((*state).clone(), (*source).clone(), data);
        if (result).is_some() {
            traverse_children = result.as_ref().unwrap();
            update_render_proxy_renderer(state, data);
        }
    }
    data.traverse_children = traverse_children;
}

// Source: upstream/packages/render/src/renderProxyAdapter.ts:24 (sha256:d396f28bf6281fb87842ad414cd5530ecb213426553b4ab1570653fca44de6fa)
pub fn get_render_proxy_adapter(
    state: &RenderState,
    source: &Renderable,
) -> Option<RenderProxyAdapter> {
    return get_render_state_runtime(state)
        .render_proxy_adapter_map
        .iter()
        .find(|(key, _)| key == &(*source).clone())
        .map(|(_, value)| value.clone());
}

// Source: upstream/packages/render/src/renderProxyAdapter.ts:28 (sha256:78929be4bb073022ebe2aff4aa47002fe327dacfd9dffadb3479bcf438bf399c)
pub fn set_render_proxy_adapter(
    state: &RenderState,
    source: &Renderable,
    adapter: Option<RenderProxyAdapter>,
) -> () {
    if ((get_render_state_runtime(state).render_adapt_hook).clone() != apply_render_proxy_adapter) {
        install_render_adapt_hook(state, &mut apply_render_proxy_adapter);
    }
    let mut runtime = get_render_state_runtime(state);
    if (adapter).is_none() {
        {
            let __flight_key = (*source).clone();
            if let Some(__flight_index) = runtime
                .render_proxy_adapter_map
                .iter()
                .position(|(key, _)| key == &__flight_key)
            {
                runtime.render_proxy_adapter_map.remove(__flight_index);
                true
            } else {
                false
            }
        };
    } else {
        {
            let __flight_key = (*source).clone();
            let __flight_value = (adapter.as_ref().unwrap()).clone();
            if let Some((_, value)) = runtime
                .render_proxy_adapter_map
                .iter_mut()
                .find(|(key, _)| key == &__flight_key)
            {
                *value = __flight_value;
            } else {
                runtime
                    .render_proxy_adapter_map
                    .push((__flight_key, __flight_value));
            }
        };
    }
    invalidate_node_appearance(&source);
}
