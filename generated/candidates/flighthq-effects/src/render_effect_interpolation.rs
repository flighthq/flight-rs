// @generated from upstream/packages/effects/src/renderEffectInterpolation.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::RenderEffect;

// Source: upstream/packages/effects/src/renderEffectInterpolation.ts:8 (sha256:b26786a790ec3eb47d74e895a3db768227b866728ec9deeed5a5186e4b68e5bd)
pub fn can_lerp_render_effects(a: &RenderEffect, b: &RenderEffect) -> bool {
    return ((a.kind).clone() == (b.kind).clone());
}

// Source: upstream/packages/effects/src/renderEffectInterpolation.ts:21 (sha256:0f4369ed84c4b548926bbb4d1d189572e4b8fabfacf0a4147eead88d77a3f41d)
pub fn lerp_render_effect(
    a: &RenderEffect,
    b: &RenderEffect,
    t: f64,
    out: &mut RenderEffect,
) -> bool {
    if ((a.kind).clone() != (b.kind).clone()) {
        return false;
    }
    let tc = (0.0_f64).max((1.0_f64).min(t));
    let mut numeric_keys: Vec<String> = Vec::new();
    let mut boolean_keys: Vec<String> = Vec::new();
    let mut string_keys: Vec<String> = Vec::new();
    let a_rec = a;
    let b_rec = b;
    for key in (a_rec
        .iter()
        .map(|(entry_key, _)| entry_key.clone())
        .collect::<Vec<_>>())
    .iter()
    .cloned()
    {
        if ((key).clone() == "kind") {
            continue;
        }
        let va = a_rec
            .iter()
            .find(|(entry_key, _)| entry_key == &(key).clone())
            .map(|(_, value)| value.clone())
            .clone();
        let vb = b_rec
            .iter()
            .find(|(entry_key, _)| entry_key == &(key).clone())
            .map(|(_, value)| value.clone())
            .clone();
        if (((va).as_ref().map_or("undefined", |_| "object")).to_owned() == "number")
            || (((vb).as_ref().map_or("undefined", |_| "object")).to_owned() == "number")
        {
            {
                let __flight_value = (key).clone();
                if !numeric_keys.contains(&__flight_value) {
                    numeric_keys.push(__flight_value);
                }
            };
        } else {
            if (((va).as_ref().map_or("undefined", |_| "object")).to_owned() == "boolean")
                || (((vb).as_ref().map_or("undefined", |_| "object")).to_owned() == "boolean")
            {
                {
                    let __flight_value = (key).clone();
                    if !boolean_keys.contains(&__flight_value) {
                        boolean_keys.push(__flight_value);
                    }
                };
            } else {
                if (((va).as_ref().map_or("undefined", |_| "object")).to_owned() == "string")
                    || (((vb).as_ref().map_or("undefined", |_| "object")).to_owned() == "string")
                {
                    {
                        let __flight_value = (key).clone();
                        if !string_keys.contains(&__flight_value) {
                            string_keys.push(__flight_value);
                        }
                    };
                }
            }
        }
    }
    for key in (b_rec
        .iter()
        .map(|(entry_key, _)| entry_key.clone())
        .collect::<Vec<_>>())
    .iter()
    .cloned()
    {
        if ((key).clone() == "kind") {
            continue;
        }
        if ((!numeric_keys.iter().any(|item| item == &(key).clone()))
            && (!boolean_keys.iter().any(|item| item == &(key).clone())))
            && (!string_keys.iter().any(|item| item == &(key).clone()))
        {
            let vb = b_rec
                .iter()
                .find(|(entry_key, _)| entry_key == &(key).clone())
                .map(|(_, value)| value.clone())
                .clone();
            if (((vb).as_ref().map_or("undefined", |_| "object")).to_owned() == "number") {
                {
                    let __flight_value = (key).clone();
                    if !numeric_keys.contains(&__flight_value) {
                        numeric_keys.push(__flight_value);
                    }
                };
            } else {
                if (((vb).as_ref().map_or("undefined", |_| "object")).to_owned() == "boolean") {
                    {
                        let __flight_value = (key).clone();
                        if !boolean_keys.contains(&__flight_value) {
                            boolean_keys.push(__flight_value);
                        }
                    };
                } else {
                    if (((vb).as_ref().map_or("undefined", |_| "object")).to_owned() == "string") {
                        {
                            let __flight_value = (key).clone();
                            if !string_keys.contains(&__flight_value) {
                                string_keys.push(__flight_value);
                            }
                        };
                    }
                }
            }
        }
    }
    let mut out_record = crate::host_value::<Vec<(String, crate::OpaqueHostValue)>>("host.cast");
    for key in (numeric_keys).iter().cloned() {
        let va = a_rec
            .iter()
            .find(|(entry_key, _)| entry_key == &key)
            .map(|(_, value)| value.clone())
            .clone();
        let vb = b_rec
            .iter()
            .find(|(entry_key, _)| entry_key == &key)
            .map(|(_, value)| value.clone())
            .clone();
        if ((va).is_some()) && ((vb).is_some()) {
            {
                let __flight_key = key;
                let __flight_value = {
                    let __flight_portable_source = (*(va.as_ref().unwrap())
                        + ((*(vb.as_ref().unwrap()) - *(va.as_ref().unwrap())) * tc));
                    crate::FlightValue::Number(*(&__flight_portable_source) as f64)
                };
                if let Some((_, value)) =
                    out_record.iter_mut().find(|(key, _)| key == &__flight_key)
                {
                    *value = __flight_value;
                } else {
                    out_record.push((__flight_key, __flight_value));
                }
            };
        } else {
            {
                let __flight_key = key;
                let __flight_value = {
                    let __flight_portable_source = if (tc < 0.5_f64) { va } else { vb };
                    match (&__flight_portable_source).as_ref() {
                        Some(value) => crate::FlightValue::Number(*(value) as f64),
                        None => crate::FlightValue::Null,
                    }
                };
                if let Some((_, value)) =
                    out_record.iter_mut().find(|(key, _)| key == &__flight_key)
                {
                    *value = __flight_value;
                } else {
                    out_record.push((__flight_key, __flight_value));
                }
            };
        }
    }
    for key in (boolean_keys).iter().cloned() {
        {
            let __flight_key = key;
            let __flight_value = {
                let __flight_portable_source = if (tc < 0.5_f64) {
                    a_rec
                        .iter()
                        .find(|(entry_key, _)| entry_key == &key)
                        .map(|(_, value)| value.clone())
                        .clone()
                } else {
                    b_rec
                        .iter()
                        .find(|(entry_key, _)| entry_key == &key)
                        .map(|(_, value)| value.clone())
                        .clone()
                };
                match (&__flight_portable_source).as_ref() {
                    Some(value) => (value).clone(),
                    None => crate::FlightValue::Null,
                }
            };
            if let Some((_, value)) = out_record.iter_mut().find(|(key, _)| key == &__flight_key) {
                *value = __flight_value;
            } else {
                out_record.push((__flight_key, __flight_value));
            }
        };
    }
    for key in (string_keys).iter().cloned() {
        {
            let __flight_key = key;
            let __flight_value = {
                let __flight_portable_source = if (tc < 0.5_f64) {
                    a_rec
                        .iter()
                        .find(|(entry_key, _)| entry_key == &key)
                        .map(|(_, value)| value.clone())
                        .clone()
                } else {
                    b_rec
                        .iter()
                        .find(|(entry_key, _)| entry_key == &key)
                        .map(|(_, value)| value.clone())
                        .clone()
                };
                match (&__flight_portable_source).as_ref() {
                    Some(value) => (value).clone(),
                    None => crate::FlightValue::Null,
                }
            };
            if let Some((_, value)) = out_record.iter_mut().find(|(key, _)| key == &__flight_key) {
                *value = __flight_value;
            } else {
                out_record.push((__flight_key, __flight_value));
            }
        };
    }
    return true;
}
