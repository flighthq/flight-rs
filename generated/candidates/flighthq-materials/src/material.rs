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
        kind: (source.kind).clone(),
        name: None,
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
        kind: (kind).clone(),
        name: None,
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
    let a_fields = crate::host_value::<Vec<(String, crate::OpaqueHostValue)>>("host.cast");
    let b_fields = crate::host_value::<Vec<(String, crate::OpaqueHostValue)>>("host.cast");
    for key in (crate::host_value::<()>("host.keys")).iter().cloned() {
        if (key == "kind") {
            continue;
        }
        if (a_fields
            .iter()
            .find(|(key, _)| key == &key)
            .map(|(_, value)| value)
            .expect("TypeScript Record key was absent")
            .clone()
            != b_fields
                .iter()
                .find(|(key, _)| key == &key)
                .map(|(_, value)| value)
                .expect("TypeScript Record key was absent")
                .clone())
        {
            return false;
        }
    }
    return true;
}

// Source: upstream/packages/materials/src/material.ts:43 (sha256:3ecd641f0460053cb1a7b61cfa11afbd981fc937c81b55090927a360c95052c0)
fn copy_material_fields(dst: &mut Material, src: &Material, kind: Kind) -> () {
    let mut dst_fields = crate::host_value::<Vec<(String, crate::OpaqueHostValue)>>("host.cast");
    let src_fields = crate::host_value::<Vec<(String, crate::OpaqueHostValue)>>("host.cast");
    for key in (crate::host_value::<()>("host.keys")).iter().cloned() {
        if (key == "kind") {
            continue;
        }
        let value = src_fields
            .iter()
            .find(|(key, _)| key == &key)
            .map(|(_, value)| value)
            .expect("TypeScript Record key was absent")
            .clone();
        if ((key == "standard") && ((value).is_some()))
            && (match &(value) {
                crate::OpaqueHostValue::Undefined => "undefined",
                crate::OpaqueHostValue::Null | crate::OpaqueHostValue::Object => "object",
                crate::OpaqueHostValue::Bool(_) => "boolean",
                crate::OpaqueHostValue::Number(_) => "number",
                crate::OpaqueHostValue::String(_) => "string",
            } == "object")
        {
            dst_fields
                .iter()
                .find(|(key, _)| key == &key)
                .map(|(_, value)| value)
                .expect("TypeScript Record key was absent") = ((value).clone()).clone();
        } else {
            dst_fields
                .iter()
                .find(|(key, _)| key == &key)
                .map(|(_, value)| value)
                .expect("TypeScript Record key was absent") = (value).clone();
        }
    }
    dst_fields
        .iter()
        .find(|(key, _)| key == &"kind".to_owned())
        .map(|(_, value)| value)
        .expect("TypeScript Record key was absent") = kind;
}
