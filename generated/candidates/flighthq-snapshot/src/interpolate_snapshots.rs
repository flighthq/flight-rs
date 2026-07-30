// @generated from upstream/packages/snapshot/src/interpolateSnapshots.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_math::{clamp, lerp};
use flighthq_types::{Snapshot, SnapshotSchema};

// Source: upstream/packages/snapshot/src/interpolateSnapshots.ts:20 (sha256:d68e78531cf50891d283ec2957b0dd672afab0abf49177629135df889b68657c)
pub fn interpolate_snapshots<T: Clone>(
    a: Snapshot<T>,
    b: Snapshot<T>,
    t: f64,
    out: T,
    schema: Option<SnapshotSchema>,
) -> () {
    if ((((((a).is_none())
        || (match &(a) {
            crate::OpaqueHostValue::Undefined => "undefined",
            crate::OpaqueHostValue::Null | crate::OpaqueHostValue::Object => "object",
            crate::OpaqueHostValue::Bool(_) => "boolean",
            crate::OpaqueHostValue::Number(_) => "number",
            crate::OpaqueHostValue::String(_) => "string",
        } != "object"))
        || ((b).is_none()))
        || (match &(b) {
            crate::OpaqueHostValue::Undefined => "undefined",
            crate::OpaqueHostValue::Null | crate::OpaqueHostValue::Object => "object",
            crate::OpaqueHostValue::Bool(_) => "boolean",
            crate::OpaqueHostValue::Number(_) => "number",
            crate::OpaqueHostValue::String(_) => "string",
        } != "object"))
        || ((out).is_none()))
        || ("object" != "object")
    {
        return;
    }
    interpolate_snapshots_into(
        out,
        a,
        b,
        clamp(t, 0.0_f64, 1.0_f64),
        ((schema).clone()).clone(),
        "".to_owned(),
    );
}

// Source: upstream/packages/snapshot/src/interpolateSnapshots.ts:42 (sha256:7c1199b279247721ee987a35f36801819725323ccd6881cc0bd96287f729ab09)
fn interpolate_snapshots_into(
    out: crate::OpaqueHostValue,
    a: crate::OpaqueHostValue,
    b: crate::OpaqueHostValue,
    t: f64,
    schema: Option<SnapshotSchema>,
    prefix: String,
) -> () {
    let mut out_record = crate::host_value::<Vec<(String, crate::OpaqueHostValue)>>("host.cast");
    let a_record = crate::host_value::<Vec<(String, crate::OpaqueHostValue)>>("host.cast");
    let b_record = crate::host_value::<Vec<(String, crate::OpaqueHostValue)>>("host.cast");
    if false {
        crate::host_set("host.length", (b.len() as f64));
    }
    for key in (crate::host_value::<()>("host.keys")).iter().cloned() {
        let path = if (prefix == "") {
            key
        } else {
            format!("{}.{}", prefix, key)
        };
        let a_value = a_record
            .iter()
            .find(|(key, _)| key == &key)
            .map(|(_, value)| value)
            .expect("TypeScript Record key was absent")
            .clone();
        let b_value = b_record
            .iter()
            .find(|(key, _)| key == &key)
            .map(|(_, value)| value)
            .expect("TypeScript Record key was absent")
            .clone();
        if (match &(a_value) {
            crate::OpaqueHostValue::Undefined => "undefined",
            crate::OpaqueHostValue::Null | crate::OpaqueHostValue::Object => "object",
            crate::OpaqueHostValue::Bool(_) => "boolean",
            crate::OpaqueHostValue::Number(_) => "number",
            crate::OpaqueHostValue::String(_) => "string",
        } == "number")
            && (match &(b_value) {
                crate::OpaqueHostValue::Undefined => "undefined",
                crate::OpaqueHostValue::Null | crate::OpaqueHostValue::Object => "object",
                crate::OpaqueHostValue::Bool(_) => "boolean",
                crate::OpaqueHostValue::Number(_) => "number",
                crate::OpaqueHostValue::String(_) => "string",
            } == "number")
        {
            out_record
                .iter()
                .find(|(key, _)| key == &key)
                .map(|(_, value)| value)
                .expect("TypeScript Record key was absent") =
                if is_snapshot_path_interpolated(((schema).clone()).clone(), (path).clone()) {
                    lerp((a_value).clone(), (b_value).clone(), t)
                } else {
                    (b_value).clone()
                };
            continue;
        }
        if (((((a_value).is_some())
            && (match &(a_value) {
                crate::OpaqueHostValue::Undefined => "undefined",
                crate::OpaqueHostValue::Null | crate::OpaqueHostValue::Object => "object",
                crate::OpaqueHostValue::Bool(_) => "boolean",
                crate::OpaqueHostValue::Number(_) => "number",
                crate::OpaqueHostValue::String(_) => "string",
            } == "object"))
            && ((b_value).is_some()))
            && (match &(b_value) {
                crate::OpaqueHostValue::Undefined => "undefined",
                crate::OpaqueHostValue::Null | crate::OpaqueHostValue::Object => "object",
                crate::OpaqueHostValue::Bool(_) => "boolean",
                crate::OpaqueHostValue::Number(_) => "number",
                crate::OpaqueHostValue::String(_) => "string",
            } == "object"))
            && (false == false)
        {
            let mut container = ensure_snapshot_container(
                out_record
                    .iter()
                    .find(|(key, _)| key == &key)
                    .map(|(_, value)| value)
                    .expect("TypeScript Record key was absent")
                    .clone(),
                false,
            );
            out_record
                .iter()
                .find(|(key, _)| key == &key)
                .map(|(_, value)| value)
                .expect("TypeScript Record key was absent") = (container).clone();
            interpolate_snapshots_into(
                (container).clone(),
                (a_value).clone(),
                (b_value).clone(),
                t,
                ((schema).clone()).clone(),
                (path).clone(),
            );
            continue;
        }
        out_record
            .iter()
            .find(|(key, _)| key == &key)
            .map(|(_, value)| value)
            .expect("TypeScript Record key was absent") = clone_snapshot_value((b_value).clone());
    }
}

// Source: upstream/packages/snapshot/src/interpolateSnapshots.ts:81 (sha256:e0d02f0ced0fd389d8abf218f761f6fd9ff18653b5e428be42e6b821114a1b4e)
fn is_snapshot_path_interpolated(schema: Option<SnapshotSchema>, path: String) -> bool {
    return ((schema).is_none()) || ((schema.as_ref().unwrap().includes)(path));
}

// Source: upstream/packages/snapshot/src/interpolateSnapshots.ts:87 (sha256:0772057dfbe92de054e11b940b63bf178839349dc06db250afa562498bc5ab0a)
#[derive(Clone, Default)]
struct EnsureSnapshotContainerRecord1 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for EnsureSnapshotContainerRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

fn ensure_snapshot_container(
    existing: crate::OpaqueHostValue,
    is_array: bool,
) -> crate::OpaqueHostValue {
    if (((existing).is_some())
        && (match &(existing) {
            crate::OpaqueHostValue::Undefined => "undefined",
            crate::OpaqueHostValue::Null | crate::OpaqueHostValue::Object => "object",
            crate::OpaqueHostValue::Bool(_) => "boolean",
            crate::OpaqueHostValue::Number(_) => "number",
            crate::OpaqueHostValue::String(_) => "string",
        } == "object"))
        && (false == is_array)
    {
        return (existing).clone();
    }
    return if is_array {
        vec![]
    } else {
        EnsureSnapshotContainerRecord1 {
            __flight_identity: std::sync::Arc::new(()),
        }
    };
}

// Source: upstream/packages/snapshot/src/interpolateSnapshots.ts:96 (sha256:67a7ee50c608bb7854484d022037db391fdf34fb9f8f3ea9ede421a8cac1a48a)
fn clone_snapshot_value(value: crate::OpaqueHostValue) -> crate::OpaqueHostValue {
    if ((value).is_none())
        || (match &(value) {
            crate::OpaqueHostValue::Undefined => "undefined",
            crate::OpaqueHostValue::Null | crate::OpaqueHostValue::Object => "object",
            crate::OpaqueHostValue::Bool(_) => "boolean",
            crate::OpaqueHostValue::Number(_) => "number",
            crate::OpaqueHostValue::String(_) => "string",
        } != "object")
    {
        return value;
    }
    return crate::host_value::<crate::OpaqueHostValue>("host.call");
}
