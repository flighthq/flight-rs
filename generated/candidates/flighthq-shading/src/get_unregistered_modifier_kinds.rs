// @generated from upstream/packages/shading/src/getUnregisteredModifierKinds.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::resolve_modifier;
use flighthq_types::{Modifier, ModifierKind, ModifierRegistry};

// Source: upstream/packages/shading/src/getUnregisteredModifierKinds.ts:10 (sha256:c5caf813913f41804803764eda529e64de65aab7f373f8b4a2ac7dc86138e336)
pub fn get_unregistered_modifier_kinds(
    registry: &ModifierRegistry,
    stack: &Vec<Modifier>,
) -> Vec<ModifierKind> {
    let mut unregistered: Vec<ModifierKind> = vec![];
    for modifier in (stack).iter().cloned() {
        if (resolve_modifier(registry, (modifier.kind).clone())).is_some() {
            continue;
        }
        if {
            let __flight_value = (modifier.kind).clone();
            (unregistered).iter().any(|item| item == &__flight_value)
        } {
            continue;
        }
        unregistered.push((modifier.kind).clone());
    }
    return unregistered;
}
