// @generated from upstream/packages/adjustments/src/adjustment.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::{AdjustmentKind, EntityConstruction};

// Source: upstream/packages/adjustments/src/adjustment.ts:3 (sha256:1910ab835f323bdb1ee919a7b33159690fd75f12cb0f287b2d9da3f730ea9d8c)
pub fn initialize_adjustment<T: Clone>(out: EntityConstruction<T>, kind: AdjustmentKind) -> () {
    crate::host_set("host.kind", kind);
}
