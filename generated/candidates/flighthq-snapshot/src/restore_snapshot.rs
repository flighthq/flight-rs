// @generated from upstream/packages/snapshot/src/restoreSnapshot.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::Snapshot;

// Source: upstream/packages/snapshot/src/restoreSnapshot.ts:16 (sha256:954eed7a3f0b79bd82df32ee60ee06123deba9c5c21cdac3f733be2ba7fc841c)
pub fn restore_snapshot<T: Clone>(snapshot: Snapshot<T>, target: T) -> () {
    if ((((snapshot).is_none()
        || (match &(snapshot) {
            crate::OpaqueHostValue::Undefined => "undefined",
            crate::OpaqueHostValue::Null | crate::OpaqueHostValue::Object => "object",
            crate::OpaqueHostValue::Bool(_) => "boolean",
            crate::OpaqueHostValue::Number(_) => "number",
            crate::OpaqueHostValue::String(_) => "string",
        } != "object"))
        || (target).is_none())
        || ("object" != "object"))
    {
        return;
    }
    restore_snapshot_into(target, snapshot);
}

// Source: upstream/packages/snapshot/src/restoreSnapshot.ts:25 (sha256:447f7fc71db1c8eadf2fac9e4e20bc8974ce1d9bcdb10c90ac0e6ef8441e6765)
fn restore_snapshot_into(target: crate::OpaqueHostValue, source: crate::OpaqueHostValue) -> () {
    if (array.is_array)(source) {
        let mut target_array = target;
        let source_array = source;
        target_array.truncate((source_array.len() as f64) as usize);
        {
            let mut index = 0.0_f64;
            while (index < (source_array.len() as f64)) {
                {
                    let __flight_index = (index) as usize;
                    let __flight_value = restore_snapshot_value(
                        target_array[index as usize].clone(),
                        source_array[index as usize].clone(),
                    );
                    if __flight_index == target_array.len() {
                        target_array.push(__flight_value);
                    } else {
                        target_array[__flight_index] = __flight_value;
                    }
                };
                {
                    index += 1.0_f64;
                    index
                };
            }
        }
        return;
    }
    let mut target_object = target;
    let source_object = source;
    for key in (crate::host_value::<()>("host.keys")).iter().cloned() {
        target_object
            .iter()
            .find(|(key, _)| key == &key)
            .map(|(_, value)| value)
            .expect("TypeScript Record key was absent") = restore_snapshot_value(
            target_object
                .iter()
                .find(|(key, _)| key == &key)
                .map(|(_, value)| value)
                .expect("TypeScript Record key was absent")
                .clone(),
            source_object
                .iter()
                .find(|(key, _)| key == &key)
                .map(|(_, value)| value)
                .expect("TypeScript Record key was absent")
                .clone(),
        );
    }
}

// Source: upstream/packages/snapshot/src/restoreSnapshot.ts:44 (sha256:89a8ff2c64c58f690578ab29d8c7da6ba30d4e2b971d0a3662eaf71f1727f234)
fn restore_snapshot_value(
    target_value: crate::OpaqueHostValue,
    source_value: crate::OpaqueHostValue,
) -> crate::OpaqueHostValue {
    if ((source_value).is_none()
        || (match &(source_value) {
            crate::OpaqueHostValue::Undefined => "undefined",
            crate::OpaqueHostValue::Null | crate::OpaqueHostValue::Object => "object",
            crate::OpaqueHostValue::Bool(_) => "boolean",
            crate::OpaqueHostValue::Number(_) => "number",
            crate::OpaqueHostValue::String(_) => "string",
        } != "object"))
    {
        return (source_value).clone();
    }
    let source_is_array = (array.is_array)(source_value);
    if (((target_value).is_some()
        && (match &(target_value) {
            crate::OpaqueHostValue::Undefined => "undefined",
            crate::OpaqueHostValue::Null | crate::OpaqueHostValue::Object => "object",
            crate::OpaqueHostValue::Bool(_) => "boolean",
            crate::OpaqueHostValue::Number(_) => "number",
            crate::OpaqueHostValue::String(_) => "string",
        } == "object"))
        && ((array.is_array)(target_value) == source_is_array))
    {
        restore_snapshot_into((target_value).clone(), (source_value).clone());
        return (target_value).clone();
    }
    return crate::host_value::<crate::OpaqueHostValue>("host.call");
}
