// @generated from upstream/packages/snapshot/src/captureSnapshot.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::Snapshot;

// Source: upstream/packages/snapshot/src/captureSnapshot.ts:14 (sha256:5bf8bd69d748313c1d8000c2dffa018e591078ceb4261bc84bb4d35bc903498d)
pub fn capture_snapshot<T: Clone>(source: T) -> Snapshot<T> {
    let clone = crate::host_value::<T>("host.call");
    freeze_snapshot_deep(clone);
    return clone;
}

// Source: upstream/packages/snapshot/src/captureSnapshot.ts:22 (sha256:e70b6a21754fd48d6df07bd32853d3748b0d4537cee1b55dd3eea2dc8cd378c8)
fn freeze_snapshot_deep(value: crate::OpaqueHostValue) -> () {
    if ((value).is_none())
        || (match &(value) {
            crate::OpaqueHostValue::Undefined => "undefined",
            crate::OpaqueHostValue::Null | crate::OpaqueHostValue::Object => "object",
            crate::OpaqueHostValue::Bool(_) => "boolean",
            crate::OpaqueHostValue::Number(_) => "number",
            crate::OpaqueHostValue::String(_) => "string",
        } != "object")
    {
        return;
    }
    crate::host_value::<()>("host.freeze");
    if (array.is_array)(value) {
        {
            let mut index = 0.0_f64;
            while (index < crate::host_value::<f64>("host.length")) {
                freeze_snapshot_deep(crate::host_value::<crate::OpaqueHostValue>("host.index"));
                {
                    index += 1.0_f64;
                    index
                };
            }
        }
        return;
    }
    for key in (crate::host_value::<()>("host.keys")).iter().cloned() {
        freeze_snapshot_deep(crate::host_value::<crate::OpaqueHostValue>("host.index"));
    }
}
