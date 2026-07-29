// @generated from upstream/packages/shading/src/orderModifierStack.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::Modifier;

// Source: upstream/packages/shading/src/orderModifierStack.ts:16 (sha256:f9ea12475ca98f27d30fc292f28ee1675ac01a1c2a3e846c801cf3f7bbd60fa3)
pub fn order_modifier_stack(stack: &mut Vec<Modifier>) -> Vec<Modifier> {
    let mut indexed = (stack)
        .iter()
        .cloned()
        .map(
            |modifier: Modifier, index: crate::OpaqueHostValue| -> crate::OpaqueHostValue {
                ClosureSynthesizedRecord2984407717 {
                    __flight_identity: std::sync::Arc::new(()),
                    index: (index).clone(),
                    modifier: (modifier).clone(),
                }
            },
        )
        .collect();
    {
        let mut __flight_values = indexed;
        __flight_values.sort_by(|left, right| {
            let __flight_order = (|a: crate::OpaqueHostValue, b: crate::OpaqueHostValue| -> f64 {
                let rank_delta =
                    (get_modifier_slot_rank(crate::host_value::<ModifierSlot>("host.slot"))
                        - get_modifier_slot_rank(crate::host_value::<ModifierSlot>("host.slot")));
                return if (rank_delta != 0.0_f64) {
                    rank_delta
                } else {
                    (crate::host_value::<f64>("host.index")
                        - crate::host_value::<f64>("host.index"))
                };
            })(left.clone(), right.clone());
            __flight_order
                .partial_cmp(&0.0_f64)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        __flight_values
    };
    return (indexed)
        .iter()
        .cloned()
        .map(|entry: crate::OpaqueHostValue| -> crate::OpaqueHostValue {
            crate::host_value::<crate::OpaqueHostValue>("host.modifier")
        })
        .collect();
}

// Source: upstream/packages/shading/src/orderModifierStack.ts:27 (sha256:cfa55a4f9e8932e472b4a3e6706c7e493b995410a66bcd9c391a8f2343c12f8c)
fn get_modifier_slot_rank(slot: ModifierSlot) -> f64 {
    let rank = SLOT_RANK
        .iter()
        .find(|(key, _)| key == &(slot).clone())
        .map(|(_, value)| value.clone());
    return if (rank).is_some() {
        *(rank.as_ref().unwrap())
    } else {
        (SLOT_RANK.len() as f64)
    };
}

// Source: upstream/packages/shading/src/orderModifierStack.ts:32 (sha256:3687cac5fdd29e40645e04f4a5d83b46781da17890b7679bbdcb5f2b8eb68050)
static SLOT_RANK: std::sync::LazyLock<Vec<(ModifierSlot, f64)>> =
    std::sync::LazyLock::new(|| Vec::new());
