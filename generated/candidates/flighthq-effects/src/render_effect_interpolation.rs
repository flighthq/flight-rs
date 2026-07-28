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
    for key in (crate::host_value::<()>("host.keys")).iter().cloned() {
        if (key == "kind") {
            continue;
        }
        let va = a_rec
            .iter()
            .find(|(key, _)| key == &key)
            .map(|(_, value)| value)
            .expect("TypeScript Record key was absent")
            .clone();
        let vb = b_rec
            .iter()
            .find(|(key, _)| key == &key)
            .map(|(_, value)| value)
            .expect("TypeScript Record key was absent")
            .clone();
        if (match &(va) {
            crate::OpaqueHostValue::Undefined => "undefined",
            crate::OpaqueHostValue::Null | crate::OpaqueHostValue::Object => "object",
            crate::OpaqueHostValue::Bool(_) => "boolean",
            crate::OpaqueHostValue::Number(_) => "number",
            crate::OpaqueHostValue::String(_) => "string",
        } == "number")
            || (match &(vb) {
                crate::OpaqueHostValue::Undefined => "undefined",
                crate::OpaqueHostValue::Null | crate::OpaqueHostValue::Object => "object",
                crate::OpaqueHostValue::Bool(_) => "boolean",
                crate::OpaqueHostValue::Number(_) => "number",
                crate::OpaqueHostValue::String(_) => "string",
            } == "number")
        {
            {
                let __flight_value = key;
                if !numeric_keys.contains(&__flight_value) {
                    numeric_keys.push(__flight_value);
                }
            };
        } else {
            if (match &(va) {
                crate::OpaqueHostValue::Undefined => "undefined",
                crate::OpaqueHostValue::Null | crate::OpaqueHostValue::Object => "object",
                crate::OpaqueHostValue::Bool(_) => "boolean",
                crate::OpaqueHostValue::Number(_) => "number",
                crate::OpaqueHostValue::String(_) => "string",
            } == "boolean")
                || (match &(vb) {
                    crate::OpaqueHostValue::Undefined => "undefined",
                    crate::OpaqueHostValue::Null | crate::OpaqueHostValue::Object => "object",
                    crate::OpaqueHostValue::Bool(_) => "boolean",
                    crate::OpaqueHostValue::Number(_) => "number",
                    crate::OpaqueHostValue::String(_) => "string",
                } == "boolean")
            {
                {
                    let __flight_value = key;
                    if !boolean_keys.contains(&__flight_value) {
                        boolean_keys.push(__flight_value);
                    }
                };
            } else {
                if (match &(va) {
                    crate::OpaqueHostValue::Undefined => "undefined",
                    crate::OpaqueHostValue::Null | crate::OpaqueHostValue::Object => "object",
                    crate::OpaqueHostValue::Bool(_) => "boolean",
                    crate::OpaqueHostValue::Number(_) => "number",
                    crate::OpaqueHostValue::String(_) => "string",
                } == "string")
                    || (match &(vb) {
                        crate::OpaqueHostValue::Undefined => "undefined",
                        crate::OpaqueHostValue::Null | crate::OpaqueHostValue::Object => "object",
                        crate::OpaqueHostValue::Bool(_) => "boolean",
                        crate::OpaqueHostValue::Number(_) => "number",
                        crate::OpaqueHostValue::String(_) => "string",
                    } == "string")
                {
                    {
                        let __flight_value = key;
                        if !string_keys.contains(&__flight_value) {
                            string_keys.push(__flight_value);
                        }
                    };
                }
            }
        }
    }
    for key in (crate::host_value::<()>("host.keys")).iter().cloned() {
        if (key == "kind") {
            continue;
        }
        if ((!numeric_keys.iter().any(|item| item == &key))
            && (!boolean_keys.iter().any(|item| item == &key)))
            && (!string_keys.iter().any(|item| item == &key))
        {
            let vb = b_rec
                .iter()
                .find(|(key, _)| key == &key)
                .map(|(_, value)| value)
                .expect("TypeScript Record key was absent")
                .clone();
            if (match &(vb) {
                crate::OpaqueHostValue::Undefined => "undefined",
                crate::OpaqueHostValue::Null | crate::OpaqueHostValue::Object => "object",
                crate::OpaqueHostValue::Bool(_) => "boolean",
                crate::OpaqueHostValue::Number(_) => "number",
                crate::OpaqueHostValue::String(_) => "string",
            } == "number")
            {
                {
                    let __flight_value = key;
                    if !numeric_keys.contains(&__flight_value) {
                        numeric_keys.push(__flight_value);
                    }
                };
            } else {
                if (match &(vb) {
                    crate::OpaqueHostValue::Undefined => "undefined",
                    crate::OpaqueHostValue::Null | crate::OpaqueHostValue::Object => "object",
                    crate::OpaqueHostValue::Bool(_) => "boolean",
                    crate::OpaqueHostValue::Number(_) => "number",
                    crate::OpaqueHostValue::String(_) => "string",
                } == "boolean")
                {
                    {
                        let __flight_value = key;
                        if !boolean_keys.contains(&__flight_value) {
                            boolean_keys.push(__flight_value);
                        }
                    };
                } else {
                    if (match &(vb) {
                        crate::OpaqueHostValue::Undefined => "undefined",
                        crate::OpaqueHostValue::Null | crate::OpaqueHostValue::Object => "object",
                        crate::OpaqueHostValue::Bool(_) => "boolean",
                        crate::OpaqueHostValue::Number(_) => "number",
                        crate::OpaqueHostValue::String(_) => "string",
                    } == "string")
                    {
                        {
                            let __flight_value = key;
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
            .find(|(key, _)| key == &key)
            .map(|(_, value)| value)
            .expect("TypeScript Record key was absent")
            .clone();
        let vb = b_rec
            .iter()
            .find(|(key, _)| key == &key)
            .map(|(_, value)| value)
            .expect("TypeScript Record key was absent")
            .clone();
        if ((va).is_some()) && ((vb).is_some()) {
            out_record
                .iter()
                .find(|(key, _)| key == &key)
                .map(|(_, value)| value)
                .expect("TypeScript Record key was absent") = (va + ((vb - va) * tc));
        } else {
            out_record
                .iter()
                .find(|(key, _)| key == &key)
                .map(|(_, value)| value)
                .expect("TypeScript Record key was absent") = if (tc < 0.5_f64) {
                (va).clone().unwrap()
            } else {
                (vb).clone().unwrap()
            };
        }
    }
    for key in (boolean_keys).iter().cloned() {
        out_record
            .iter()
            .find(|(key, _)| key == &key)
            .map(|(_, value)| value)
            .expect("TypeScript Record key was absent") = if (tc < 0.5_f64) {
            a_rec
                .iter()
                .find(|(key, _)| key == &key)
                .map(|(_, value)| value)
                .expect("TypeScript Record key was absent")
                .clone()
        } else {
            b_rec
                .iter()
                .find(|(key, _)| key == &key)
                .map(|(_, value)| value)
                .expect("TypeScript Record key was absent")
                .clone()
        };
    }
    for key in (string_keys).iter().cloned() {
        out_record
            .iter()
            .find(|(key, _)| key == &key)
            .map(|(_, value)| value)
            .expect("TypeScript Record key was absent") = if (tc < 0.5_f64) {
            a_rec
                .iter()
                .find(|(key, _)| key == &key)
                .map(|(_, value)| value)
                .expect("TypeScript Record key was absent")
                .clone()
        } else {
            b_rec
                .iter()
                .find(|(key, _)| key == &key)
                .map(|(_, value)| value)
                .expect("TypeScript Record key was absent")
                .clone()
        };
    }
    return true;
}
