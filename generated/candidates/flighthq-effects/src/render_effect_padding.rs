// @generated from upstream/packages/effects/src/renderEffectPadding.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_registry::{
    create_keyed_table, with_registry_table_entry, without_registry_table_entry,
};
use flighthq_render::get_render_state_runtime;
use flighthq_types::{
    Kind, REGISTRY_ENTRY_STATE as registry_entry_state_constant, RenderEffect, RenderEffectPadding,
    RenderEffectPaddingExplanation, RenderEffectPaddingResolver, RenderRegistry, RenderState,
};

// Source: upstream/packages/effects/src/renderEffectPadding.ts:16 (sha256:46c1cf81e3c6877cb9d40d3ad64c16546d6dfea285e5b4dc439ea714675f748c)
pub fn compute_render_effect_padding(
    state: &RenderState,
    effects: &crate::FlightUnion2<RenderEffect, Vec<RenderEffect>>,
) -> RenderEffectPadding {
    let list = if false {
        (*effects).clone()
    } else {
        vec![effects]
    };
    let explanation = explain_render_effect_padding(state, &(list));
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

// Source: upstream/packages/effects/src/renderEffectPadding.ts:28 (sha256:c71835a99b95afbb3186475c8d58fbe2ab7a26ed88bf8a4a9c549d9d7996ecbd)
pub fn explain_render_effect_padding(
    state: &RenderState,
    effects: &crate::FlightUnion2<RenderEffect, Vec<RenderEffect>>,
) -> RenderEffectPaddingExplanation {
    let list = if false {
        (*effects).clone()
    } else {
        vec![effects]
    };
    let entries = get_render_state_runtime(state)
        .inner
        .lock()
        .unwrap()
        .render_state_runtime
        .registries
        .effect_padding_resolvers
        .as_ref()
        .map(|value| (value.entries).clone());
    let mut bottom = 0.0_f64;
    let mut left = 0.0_f64;
    let mut right = 0.0_f64;
    let mut top = 0.0_f64;
    let mut missing_kinds: Vec<Kind> = vec![];
    for effect in (list).iter().cloned() {
        let entry = entries.as_ref().and_then(|entries| {
            entries
                .iter()
                .find(|(entry_key, _)| entry_key == &(effect.kind).clone())
                .map(|(_, value)| value.clone())
        });
        if (entry.as_ref().map(|value| (value.state).clone())
            != registry_entry_state_constant.bound)
        {
            if (!{
                let __flight_value = (effect.kind).clone();
                (missing_kinds).iter().any(|item| item == &__flight_value)
            }) {
                missing_kinds.push((effect.kind).clone());
            }
            continue;
        }
        let padding = (entry.as_ref().unwrap().value)(effect);
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

// Source: upstream/packages/effects/src/renderEffectPadding.ts:59 (sha256:b55d4d803b35eb99ca8917156c8c3fd8ca6d0c20d562e5c5fecc2278ee9e440d)
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

// Source: upstream/packages/effects/src/renderEffectPadding.ts:77 (sha256:81535baa8e1658f96c5f82ad67d4bcd7c63c7eee827ecc11d7d40480a4bbbf25)
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

// Source: upstream/packages/effects/src/renderEffectPadding.ts:83 (sha256:ff9fb6c0b399c806e0365da5da18dfc45910ecf5483042886e775dae91572a08)
pub fn register_render_effect_padding_resolver(
    state: &RenderState,
    kind: Kind,
    resolver: &Option<RenderEffectPaddingResolver>,
) -> () {
    let mut runtime = get_render_state_runtime(state);
    let table = (runtime
        .inner
        .lock()
        .unwrap()
        .render_state_runtime
        .registries
        .effect_padding_resolvers)
        .clone();
    if (resolver).is_none() {
        if ((table).clone()).is_some() {
            runtime
                .inner
                .lock()
                .unwrap()
                .render_state_runtime
                .registries
                .effect_padding_resolvers = Some(without_registry_table_entry(
                &table.as_ref().unwrap(),
                (kind).clone(),
            ));
        }
        return;
    }
    runtime
        .inner
        .lock()
        .unwrap()
        .render_state_runtime
        .registries
        .effect_padding_resolvers = Some(with_registry_table_entry(
        &(table).clone().unwrap_or(create_keyed_table(
            "RenderEffectPaddingResolver".to_owned(),
            "Zero".to_owned(),
        )),
        (kind).clone(),
        (resolver.as_ref().unwrap()).clone(),
    ));
}

// Source: upstream/packages/effects/src/renderEffectPadding.ts:101 (sha256:7ffe070fe9601935cbd9850f2d86d51e0dbec2cadc140975c5b26231fd048967)
fn sanitize_padding(value: f64) -> f64 {
    return if (value).is_finite() {
        (0.0_f64).max(value)
    } else {
        0.0_f64
    };
}
