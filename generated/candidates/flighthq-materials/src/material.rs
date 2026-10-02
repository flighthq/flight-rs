// @generated from upstream/packages/materials/src/material.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_entity::{allocate_entity, finish_entity};
use flighthq_types::{EntityConstruction, Kind, Material};

// Source: upstream/packages/materials/src/material.ts:6 (sha256:3ce775b0696548bce396bc8cf635a7de2dec5248a7b4966dfe22bec26a295226)
pub fn clone_material(source: &Material) -> Material {
    let mut clone = allocate_entity();
    crate::host_set("host.kind", (source.kind).clone());
    copy_material_fields(&mut clone, source, (source.kind).clone());
    return finish_entity((clone).clone());
}

// Source: upstream/packages/materials/src/material.ts:15 (sha256:f4f6d26d442dce89d4d3d58c685d24f2d54b254a9799dc6aadaa62b09b70e94b)
pub fn copy_material(out: &mut Material, source: &Material) -> () {
    if (out == source) {
        return;
    }
    copy_material_fields(out, source, (source.kind).clone());
}

// Source: upstream/packages/materials/src/material.ts:20 (sha256:b6207601eecee6c15f5987bf1886b36ac594a4e3b2f537b3782fde5aa8bd4387)
pub fn create_material(kind: Kind) -> Material {
    let mut material = allocate_entity();
    initialize_material((material).clone(), (kind).clone());
    return finish_entity((material).clone());
}

// Source: upstream/packages/materials/src/material.ts:29 (sha256:feae53144c886197182cabf32d8f15f15550f0676f5937770a5a15ce73cf82cf)
pub fn equals_material(a: &Material, b: &Material) -> bool {
    if (a == b) {
        return true;
    }
    if ((a.kind).clone() != (b.kind).clone()) {
        return false;
    }
    let a_fields = crate::host_value::<Vec<(String, crate::FlightValue)>>("host.cast");
    let b_fields = crate::host_value::<Vec<(String, crate::FlightValue)>>("host.cast");
    let a_keys = a_fields
        .iter()
        .map(|(entry_key, _)| entry_key.clone())
        .collect::<Vec<_>>();
    let b_keys = b_fields
        .iter()
        .map(|(entry_key, _)| entry_key.clone())
        .collect::<Vec<_>>();
    if ((a_keys.len() as f64) != (b_keys.len() as f64)) {
        return false;
    }
    for key in (a_keys).iter().cloned() {
        if (!crate::host_value::<()>("host.hasOwn")) {
            return false;
        }
        if ((key).clone() == "kind") {
            continue;
        }
        if ((a_fields
            .iter()
            .find(|(entry_key, _)| entry_key == &(key).clone())
            .map(|(_, value)| value.clone()))
        .expect("TypeScript Record key was absent")
            != (b_fields
                .iter()
                .find(|(entry_key, _)| entry_key == &(key).clone())
                .map(|(_, value)| value.clone()))
            .expect("TypeScript Record key was absent"))
        {
            return false;
        }
    }
    return true;
}

// Source: upstream/packages/materials/src/material.ts:50 (sha256:71fc461c19bff1416462397c78890c52fc43c55d19087fc95c208390dfbb052d)
pub fn get_material_of_kind<T: Clone + flighthq_types::FlightEntity>(
    material: &Option<Material>,
    kind: crate::OpaqueHostValue,
) -> Option<T> {
    return if ((material).is_some()) && ((material.as_ref().unwrap().kind).clone() == kind) {
        Some(
            flighthq_types::FlightEntity::__flight_downcast::<T>(&((material).clone().unwrap()))
                .expect("TypeScript entity cast lost its concrete Rust snapshot"),
        )
    } else {
        None
    };
}

// Source: upstream/packages/materials/src/material.ts:54 (sha256:5325789d96487ae077a653f8727dbf7f6b36cb977b0e090083dc866092be76de)
pub fn initialize_material(material: EntityConstruction<Material>, kind: Kind) -> () {
    crate::host_set("host.kind", kind);
    crate::host_set("host.name", None);
}

// Source: upstream/packages/materials/src/material.ts:62 (sha256:3ecd641f0460053cb1a7b61cfa11afbd981fc937c81b55090927a360c95052c0)
fn copy_material_fields(dst: &mut Material, src: &Material, kind: Kind) -> () {
    let mut dst_fields = crate::host_value::<Vec<(String, crate::FlightValue)>>("host.cast");
    let src_fields = crate::host_value::<Vec<(String, crate::FlightValue)>>("host.cast");
    for key in (src_fields
        .iter()
        .map(|(entry_key, _)| entry_key.clone())
        .collect::<Vec<_>>())
    .iter()
    .cloned()
    {
        if (key == "kind") {
            continue;
        }
        let value: Option<crate::FlightValue> = src_fields
            .iter()
            .find(|(entry_key, _)| entry_key == &(key).clone())
            .map(|(_, value)| value.clone());
        if ((key == "standard") && (((value).clone()).is_some()))
            && ((match ((value).clone()).as_ref() {
                None => "undefined",
                Some(value) => match value {
                    crate::FlightValue::Undefined => "undefined",
                    crate::FlightValue::Null
                    | crate::FlightValue::Array(_)
                    | crate::FlightValue::Record(_)
                    | crate::FlightValue::Error { .. }
                    | crate::FlightValue::Object => "object",
                    crate::FlightValue::Bool(_) => "boolean",
                    crate::FlightValue::Number(_) => "number",
                    crate::FlightValue::String(_) => "string",
                    crate::FlightValue::Function => "function",
                    crate::FlightValue::Symbol => "symbol",
                },
            })
            .to_owned()
                == "object")
        {
            {
                let __flight_key = (key).clone();
                let __flight_value = crate::FlightValue::Record({
                    let mut __flight_record = Vec::new();
                    let __flight_spread_0 = (value.as_ref().unwrap()).clone();
                    match __flight_spread_0 {
                        crate::FlightValue::Record(entries) => {
                            for (__flight_key, __flight_value) in entries {
                                if let Some((_, __flight_existing)) = __flight_record
                                    .iter_mut()
                                    .find(|(existing, _)| existing == &__flight_key)
                                {
                                    *__flight_existing = __flight_value;
                                } else {
                                    __flight_record.push((__flight_key, __flight_value));
                                }
                            }
                        }
                        crate::FlightValue::Array(values) => {
                            for (__flight_index, __flight_value) in values.into_iter().enumerate() {
                                let __flight_key = __flight_index.to_string();
                                if let Some((_, __flight_existing)) = __flight_record
                                    .iter_mut()
                                    .find(|(existing, _)| existing == &__flight_key)
                                {
                                    *__flight_existing = __flight_value;
                                } else {
                                    __flight_record.push((__flight_key, __flight_value));
                                }
                            }
                        }
                        crate::FlightValue::Undefined
                        | crate::FlightValue::Null
                        | crate::FlightValue::Bool(_)
                        | crate::FlightValue::Number(_)
                        | crate::FlightValue::Function
                        | crate::FlightValue::Symbol => {}
                        crate::FlightValue::String(_) => panic!(
                            "portable object spread of strings requires UTF-16 property lowering"
                        ),
                        crate::FlightValue::Error { .. } | crate::FlightValue::Object => {
                            panic!("portable object spread cannot inspect an opaque host object")
                        }
                    }
                    __flight_record
                });
                if let Some((_, value)) =
                    dst_fields.iter_mut().find(|(key, _)| key == &__flight_key)
                {
                    *value = __flight_value;
                } else {
                    dst_fields.push((__flight_key, __flight_value));
                }
            };
        } else {
            {
                let __flight_key = (key).clone();
                let __flight_value = {
                    let __flight_portable_source = (value).clone();
                    match (&__flight_portable_source).as_ref() {
                        Some(value) => (value).clone(),
                        None => crate::FlightValue::Null,
                    }
                };
                if let Some((_, value)) =
                    dst_fields.iter_mut().find(|(key, _)| key == &__flight_key)
                {
                    *value = __flight_value;
                } else {
                    dst_fields.push((__flight_key, __flight_value));
                }
            };
        }
    }
    {
        let __flight_key = "kind".to_owned();
        let __flight_value = {
            let __flight_portable_source = (kind).clone();
            crate::FlightValue::String((&__flight_portable_source).clone())
        };
        if let Some((_, value)) = dst_fields.iter_mut().find(|(key, _)| key == &__flight_key) {
            *value = __flight_value;
        } else {
            dst_fields.push((__flight_key, __flight_value));
        }
    };
}
