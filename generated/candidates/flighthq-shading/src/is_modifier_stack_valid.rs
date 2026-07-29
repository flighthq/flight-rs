// @generated from upstream/packages/shading/src/isModifierStackValid.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{ModifierRegistry, get_unregistered_modifier_kinds};
use flighthq_types::Modifier;

// Source: upstream/packages/shading/src/isModifierStackValid.ts:9 (sha256:7c4d95e3f0f37032d3dd806125e3e91614a57137659ea316aec3b1c86d04f7c2)
pub fn is_modifier_stack_valid(registry: &ModifierRegistry, stack: &Vec<Modifier>) -> bool {
    return ((get_unregistered_modifier_kinds(registry, stack).len() as f64) == 0.0_f64);
}
