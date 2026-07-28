// @generated from upstream/packages/node/src/hasBlendMode.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::HasBlendMode;

// Source: upstream/packages/node/src/hasBlendMode.ts:3 (sha256:c3e028c7ceaad82cb426a3ae1e6c5ef92bf842e4a4c0bdf76c459fbc1c06f7d8)
pub fn init_blend_mode_trait(target: &mut HasBlendMode, obj: Option<HasBlendMode>) -> () {
    target.blend_mode = obj.as_ref().and_then(|value| (value.blend_mode).clone());
}
