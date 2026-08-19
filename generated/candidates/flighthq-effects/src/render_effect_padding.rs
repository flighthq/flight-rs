// @generated from upstream/packages/effects/src/renderEffectPadding.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_render::get_render_state_runtime;
use flighthq_types::{
    BlendMode, Kind, Matrix, RenderEffect, RenderEffectPadding, RenderEffectPaddingExplanation,
    RenderEffectPaddingResolver, RenderRegistry, RenderState, Scene2DClipHooks,
    Scene3DGraphSyncPolicy,
};

#[derive(Clone, Default)]
pub struct FlightPartialRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub allow_smoothing: Option<bool>,
    pub background_color: Option<f64>,
    pub background_color_rgba: Option<Vec<f64>>,
    pub background_color_string: Option<String>,
    pub current_clip_depth: Option<f64>,
    pub display_object_clip_hooks: Option<Scene2DClipHooks>,
    pub pixel_ratio: Option<f64>,
    pub render_alpha: Option<f64>,
    pub render_blend_mode: Option<BlendMode>,
    pub render_transform2_d: Option<Matrix>,
    pub scene_graph_sync_policy: Option<Scene3DGraphSyncPolicy>,
    pub round_pixels: Option<bool>,
}
impl PartialEq for FlightPartialRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/effects/src/renderEffectPadding.ts:15 (sha256:46c1cf81e3c6877cb9d40d3ad64c16546d6dfea285e5b4dc439ea714675f748c)
pub fn compute_render_effect_padding(
    state: &RenderState,
    effects: &crate::FlightUnion2<RenderEffect, Vec<RenderEffect>>,
) -> RenderEffectPadding {
    let list = if false {
        (*effects).clone()
    } else {
        vec![effects]
    };
    let explanation = explain_render_effect_padding(state, &((list).clone()));
    let emit_miss = (get_render_state_runtime(state)
        .inner
        .lock()
        .unwrap()
        .render_state_runtime
        .registry_miss)
        .clone();
    if (emit_miss).is_some() {
        for kind in ((explanation.missing_kinds).clone()).iter().cloned() {
            emit_miss.as_ref().unwrap()(RenderRegistry::EffectPaddingResolver, (kind).clone());
        }
    }
    return (explanation.padding).clone();
}

// Source: upstream/packages/effects/src/renderEffectPadding.ts:27 (sha256:ba702800aa81325294025409496b3f1b2d8e00afc51cd780c7abcffda7fae978)
pub fn explain_render_effect_padding(
    state: &RenderState,
    effects: &crate::FlightUnion2<RenderEffect, Vec<RenderEffect>>,
) -> RenderEffectPaddingExplanation {
    let list = if false {
        (*effects).clone()
    } else {
        vec![effects]
    };
    let registry = (get_render_state_runtime(state)
        .inner
        .lock()
        .unwrap()
        .render_effect_padding_resolver_registry)
        .clone();
    let mut bottom = 0.0_f64;
    let mut left = 0.0_f64;
    let mut right = 0.0_f64;
    let mut top = 0.0_f64;
    let mut missing_kinds: Vec<Kind> = vec![];
    for effect in (list).iter().cloned() {
        let resolver = registry
            .as_mut()
            .unwrap()
            .iter()
            .find(|(key, _)| key == &(effect.kind).clone())
            .map(|(_, value)| value.clone());
        if (resolver).is_none() {
            if (!{
                let __flight_value = (effect.kind).clone();
                (missing_kinds).iter().any(|item| item == &__flight_value)
            }) {
                missing_kinds.push((effect.kind).clone());
            }
            continue;
        }
        let padding = resolver.as_ref().unwrap().lock().unwrap()(effect);
        bottom += sanitize_padding(padding.bottom);
        left += sanitize_padding(padding.left);
        right += sanitize_padding(padding.right);
        top += sanitize_padding(padding.top);
    }
    return RenderEffectPaddingExplanation {
        __flight_identity: std::sync::Arc::new(()),
        missing_kinds: (missing_kinds).clone(),
        padding: RenderEffectPadding {
            __flight_identity: std::sync::Arc::new(()),
            bottom: bottom,
            left: left,
            right: right,
            top: top,
        },
        status: if ((missing_kinds.len() as f64) == 0.0_f64) {
            "complete".to_owned()
        } else {
            "missing-resolver".to_owned()
        },
    };
}

// Source: upstream/packages/effects/src/renderEffectPadding.ts:58 (sha256:b55d4d803b35eb99ca8917156c8c3fd8ca6d0c20d562e5c5fecc2278ee9e440d)
pub fn get_directional_render_effect_padding(
    blur_x: f64,
    blur_y: f64,
    offset_x: f64,
    offset_y: f64,
) -> RenderEffectPadding {
    let gaussian = get_gaussian_render_effect_padding(blur_x, blur_y);
    let dx = if ((offset_x).abs() < 1e-10_f64) {
        0.0_f64
    } else {
        offset_x
    };
    let dy = if ((offset_y).abs() < 1e-10_f64) {
        0.0_f64
    } else {
        offset_y
    };
    return RenderEffectPadding {
        __flight_identity: std::sync::Arc::new(()),
        bottom: (gaussian.bottom + (0.0_f64).max(dy)).ceil(),
        left: (gaussian.left + (0.0_f64).max((-dx))).ceil(),
        right: (gaussian.right + (0.0_f64).max(dx)).ceil(),
        top: (gaussian.top + (0.0_f64).max((-dy))).ceil(),
    };
}

// Source: upstream/packages/effects/src/renderEffectPadding.ts:76 (sha256:81535baa8e1658f96c5f82ad67d4bcd7c63c7eee827ecc11d7d40480a4bbbf25)
pub fn get_gaussian_render_effect_padding(blur_x: f64, blur_y: f64) -> RenderEffectPadding {
    let horizontal = ((0.0_f64).max(blur_x) * 3.0_f64).ceil();
    let vertical = ((0.0_f64).max(blur_y) * 3.0_f64).ceil();
    return RenderEffectPadding {
        __flight_identity: std::sync::Arc::new(()),
        bottom: vertical,
        left: horizontal,
        right: horizontal,
        top: vertical,
    };
}

// Source: upstream/packages/effects/src/renderEffectPadding.ts:82 (sha256:54b848a5e508843cc7939aff5149e24d45e2f14cd3dbab289f7e380d006daaa9)
pub fn register_render_effect_padding_resolver(
    state: &RenderState,
    kind: Kind,
    resolver: Option<RenderEffectPaddingResolver>,
) -> () {
    let mut runtime = get_render_state_runtime(state);
    if (resolver).is_none() {
        {
            let __flight_key = (kind).clone();
            if let Some(__flight_index) = runtime
                .inner
                .lock()
                .unwrap()
                .render_effect_padding_resolver_registry
                .as_mut()
                .unwrap()
                .iter()
                .position(|(key, _)| key == &__flight_key)
            {
                runtime
                    .inner
                    .lock()
                    .unwrap()
                    .render_effect_padding_resolver_registry
                    .as_mut()
                    .unwrap()
                    .remove(__flight_index);
                true
            } else {
                false
            }
        };
    } else {
        ({
            let __flight_runtime = runtime;
            let __flight_value = Some(Vec::new());
            let mut __flight_storage = __flight_runtime.inner.lock().unwrap();
            __flight_storage.render_effect_padding_resolver_registry?? = __flight_value;
            __flight_storage
                .render_effect_padding_resolver_registry
                .clone()
        }
        .set)(kind, (resolver.as_ref().unwrap()).clone());
    }
}

// Source: upstream/packages/effects/src/renderEffectPadding.ts:92 (sha256:7ffe070fe9601935cbd9850f2d86d51e0dbec2cadc140975c5b26231fd048967)
fn sanitize_padding(value: f64) -> f64 {
    return if (value).is_finite() {
        (0.0_f64).max(value)
    } else {
        0.0_f64
    };
}
