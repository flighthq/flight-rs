// @generated from upstream/packages/materials/src/material.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_entity::create_entity;
use flighthq_types::{Kind, Material};

// Source: upstream/packages/materials/src/material.ts:6 (sha256:8bc3d74ced8427811d249dc4de6728f09afd84ead12470693ea66ea2e75ef998)
pub fn clone_material(source: &Material) -> Material {
    let mut clone = create_entity(Some(Material {
        __flight_identity: std::sync::Arc::new(()),
        __flight_entity_runtime: Default::default(),
        kind: (source.kind).clone(),
        name: None,
        ..Default::default()
    }));
    copy_material_fields(&mut clone, source, (source.kind).clone());
    return clone;
}

// Source: upstream/packages/materials/src/material.ts:14 (sha256:f4f6d26d442dce89d4d3d58c685d24f2d54b254a9799dc6aadaa62b09b70e94b)
pub fn copy_material(out: &mut Material, source: &Material) -> () {
    if (out == source) {
        return;
    }
    copy_material_fields(out, source, (source.kind).clone());
}

// Source: upstream/packages/materials/src/material.ts:19 (sha256:48b5bace9344eb193e68ae1cf5d25fb9aa194d64f8d368db2ea056d68511e4ec)
pub fn create_material(kind: Kind) -> Material {
    let mut material = create_entity(Some(Material {
        __flight_identity: std::sync::Arc::new(()),
        __flight_entity_runtime: Default::default(),
        kind: (kind).clone(),
        name: None,
        ..Default::default()
    }));
    material.name = None;
    return material;
}

// Source: upstream/packages/materials/src/material.ts:28 (sha256:1a48de1f6d32e81a24970552a896f290aea96a49b2f3fb841ec7f97863c86b42)
pub fn equals_material(a: &Material, b: &Material) -> bool {
    if (a == b) {
        return true;
    }
    if ((a.kind).clone() != (b.kind).clone()) {
        return false;
    }
    let a_fields = crate::host_value::<Vec<(String, crate::FlightValue)>>("host.cast");
    let b_fields = crate::host_value::<Vec<(String, crate::FlightValue)>>("host.cast");
    for key in (a_fields
        .iter()
        .map(|(entry_key, _)| entry_key.clone())
        .collect::<Vec<_>>())
    .iter()
    .cloned()
    {
        if (key == "kind") {
            continue;
        }
        if (a_fields
            .iter()
            .find(|(entry_key, _)| entry_key == &(key).clone())
            .map(|(_, value)| value.clone())
            .clone()
            != b_fields
                .iter()
                .find(|(entry_key, _)| entry_key == &(key).clone())
                .map(|(_, value)| value.clone())
                .clone())
        {
            return false;
        }
    }
    return true;
}

// Source: upstream/packages/materials/src/material.ts:45 (sha256:f7243c63cc0fe4a303d3cf43e3225823954c4137ee1703db6ba763009c1b6520)
pub fn get_material_of_kind<T: Clone>(material: &Option<Material>, kind: Kind) -> Option<T> {
    return if ((material).is_some()) && ((material.as_ref().unwrap().kind).clone() == kind) {
        Some((material).clone().unwrap())
    } else {
        None
    };
}

// Source: upstream/packages/materials/src/material.ts:52 (sha256:3ecd641f0460053cb1a7b61cfa11afbd981fc937c81b55090927a360c95052c0)
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
        let value = src_fields
            .iter()
            .find(|(entry_key, _)| entry_key == &(key).clone())
            .map(|(_, value)| value.clone())
            .clone();
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
