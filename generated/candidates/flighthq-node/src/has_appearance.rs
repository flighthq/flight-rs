// @generated from upstream/packages/node/src/hasAppearance.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::HasAppearance;

// Source: upstream/packages/node/src/hasAppearance.ts:3 (sha256:78f77813f37884625f00483faf457470c5d24a544ad1a3cee65fd8aea28840b5)
pub fn init_appearance_trait(target: &mut HasAppearance, obj: Option<HasAppearance>) -> () {
    target.alpha = (obj.as_ref().map(|value| value.alpha)).unwrap_or(1.0_f64);
    target.visible = (obj.as_ref().map(|value| value.visible)).unwrap_or(true);
}
