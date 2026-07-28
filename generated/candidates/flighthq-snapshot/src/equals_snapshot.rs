// @generated from upstream/packages/snapshot/src/equalsSnapshot.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::Snapshot;

// Source: upstream/packages/snapshot/src/equalsSnapshot.ts:10 (sha256:264d4460cde7e82c345d46949d99d8fbe687d608ca025150d69dfd49f38a11d1)
pub fn equals_snapshot<T: Clone>(a: Snapshot<T>, b: Snapshot<T>) -> bool {
    return snapshot_values_equal(a, b);
}

// Source: upstream/packages/snapshot/src/equalsSnapshot.ts:14 (sha256:afeebde982685368418fc68f512c4d204470065e1b57ddf2158ed94b3cda9a62)
fn snapshot_values_equal(a: crate::OpaqueHostValue, b: crate::OpaqueHostValue) -> bool {
    if (a == b) {
        return true;
    }
    if ((((a).is_none()) || ((b).is_none()))
        || (match &(a) {
            crate::OpaqueHostValue::Undefined => "undefined",
            crate::OpaqueHostValue::Null | crate::OpaqueHostValue::Object => "object",
            crate::OpaqueHostValue::Bool(_) => "boolean",
            crate::OpaqueHostValue::Number(_) => "number",
            crate::OpaqueHostValue::String(_) => "string",
        } != "object"))
        || (match &(b) {
            crate::OpaqueHostValue::Undefined => "undefined",
            crate::OpaqueHostValue::Null | crate::OpaqueHostValue::Object => "object",
            crate::OpaqueHostValue::Bool(_) => "boolean",
            crate::OpaqueHostValue::Number(_) => "number",
            crate::OpaqueHostValue::String(_) => "string",
        } != "object")
    {
        return false;
    }
    let a_is_array = (array.is_array)(a);
    if (a_is_array != (array.is_array)(b)) {
        return false;
    }
    if a_is_array {
        let a_array = a;
        let b_array = b;
        if ((a_array.len() as f64) != (b_array.len() as f64)) {
            return false;
        }
        {
            let mut index = 0.0_f64;
            while (index < (a_array.len() as f64)) {
                if (!snapshot_values_equal(
                    a_array[index as usize].clone(),
                    b_array[index as usize].clone(),
                )) {
                    return false;
                }
                {
                    index += 1.0_f64;
                    index
                };
            }
        }
        return true;
    }
    let a_object = a;
    let b_object = b;
    let a_keys = crate::host_value::<()>("host.keys");
    if (a_keys.length != crate::host_value::<()>("host.keys").length) {
        return false;
    }
    for key in (a_keys).iter().cloned() {
        if (!crate::host_value::<()>("host.call")) {
            return false;
        }
        if (!snapshot_values_equal(
            a_object
                .iter()
                .find(|(key, _)| key == &key)
                .map(|(_, value)| value)
                .expect("TypeScript Record key was absent")
                .clone(),
            b_object
                .iter()
                .find(|(key, _)| key == &key)
                .map(|(_, value)| value)
                .expect("TypeScript Record key was absent")
                .clone(),
        )) {
            return false;
        }
    }
    return true;
}
