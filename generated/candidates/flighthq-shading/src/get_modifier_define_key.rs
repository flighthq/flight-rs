// @generated from upstream/packages/shading/src/getModifierDefineKey.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{ModifierRegistry, order_modifier_stack, resolve_modifier};
use flighthq_types::Modifier;

// Source: upstream/packages/shading/src/getModifierDefineKey.ts:22 (sha256:03fcc388b216ba2278b2d491129647c99deb4f666934c79cff9092d9940140b5)
pub fn get_modifier_define_key(
    stack: &mut Vec<Modifier>,
    registry: Option<ModifierRegistry>,
) -> String {
    let ordered = order_modifier_stack(stack);
    let mut key = "";
    for modifier in (ordered).iter().cloned() {
        let signature =
            get_define_signature(&modifier, Some(((registry).clone().unwrap()).clone()));
        let token = if ((signature.encode_utf16().count() as f64) > 0.0_f64) {
            format!("{}:{}", (modifier.kind).clone(), signature)
        } else {
            (modifier.kind).clone()
        };
        key = if ((key.encode_utf16().count() as f64) > 0.0_f64) {
            format!("{}+{}", key, token)
        } else {
            token
        };
    }
    return key;
}

// Source: upstream/packages/shading/src/getModifierDefineKey.ts:33 (sha256:51c5e64d3bf247ca22786eb11d98731c6c2f4d81b083e38340610b0a87b129db)
fn get_define_signature(modifier: &Modifier, registry: Option<ModifierRegistry>) -> String {
    if (registry).is_none() {
        return "".to_owned();
    }
    let definition = resolve_modifier(&registry.as_ref().unwrap(), (modifier.kind).clone());
    if ((definition).is_none())
        || (((definition.as_ref().unwrap().get_define_signature).clone()).is_none())
    {
        return "".to_owned();
    }
    return {
        let __flight_callback = definition
            .as_ref()
            .unwrap()
            .get_define_signature
            .as_ref()
            .unwrap()
            .clone();
        let __flight_result = __flight_callback.lock().unwrap()((*modifier).clone());
        __flight_result
    };
}
