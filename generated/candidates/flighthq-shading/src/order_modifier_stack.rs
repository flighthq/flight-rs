// @generated from upstream/packages/shading/src/orderModifierStack.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::{MODIFIER_SLOT as modifier_slot_constant, Modifier};

// Source: upstream/packages/shading/src/orderModifierStack.ts:16 (sha256:f9ea12475ca98f27d30fc292f28ee1675ac01a1c2a3e846c801cf3f7bbd60fa3)
#[derive(Clone, Default)]
struct ClosureContextRecord1 {
    __flight_identity: std::sync::Arc<()>,
    index: crate::OpaqueHostValue,
    modifier: Modifier,
}
impl PartialEq for ClosureContextRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn order_modifier_stack(stack: &mut Vec<Modifier>) -> Vec<Modifier> {
    let mut indexed = (stack)
        .iter()
        .cloned()
        .map(
            |modifier: Modifier, index: crate::OpaqueHostValue| -> ClosureContextRecord1 {
                ClosureContextRecord1 {
                    __flight_identity: std::sync::Arc::new(()),
                    index: (index).clone(),
                    modifier: (modifier).clone(),
                }
            },
        )
        .collect::<Vec<_>>();
    {
        let mut __flight_values = indexed;
        __flight_values.sort_by(|left, right| {
            let __flight_order = (|a: ClosureContextRecord1, b: ClosureContextRecord1| -> f64 {
                let rank_delta = (get_modifier_slot_rank((a.modifier.slot).clone())
                    - get_modifier_slot_rank((b.modifier.slot).clone()));
                return if (rank_delta != 0.0_f64) {
                    rank_delta
                } else {
                    ((a.index).clone() - (b.index).clone())
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
        .map(|entry: ClosureContextRecord1| -> Modifier { (entry.modifier).clone() })
        .collect::<Vec<_>>();
}

// Source: upstream/packages/shading/src/orderModifierStack.ts:27 (sha256:cfa55a4f9e8932e472b4a3e6706c7e493b995410a66bcd9c391a8f2343c12f8c)
fn get_modifier_slot_rank(slot: ModifierSlot) -> f64 {
    let rank = SLOT_RANK
        .iter()
        .find(|(entry_key, _)| entry_key == &(slot).clone())
        .map(|(_, value)| value.clone());
    return if (rank).is_some() {
        *(rank.as_ref().unwrap())
    } else {
        (SLOT_RANK.len() as f64)
    };
}

// Source: upstream/packages/shading/src/orderModifierStack.ts:32 (sha256:3687cac5fdd29e40645e04f4a5d83b46781da17890b7679bbdcb5f2b8eb68050)
static SLOT_RANK: std::sync::LazyLock<Vec<(ModifierSlot, f64)>> = std::sync::LazyLock::new(|| {
    vec![
        ((modifier_slot_constant.vertex).clone(), 0.0_f64),
        ((modifier_slot_constant.normal).clone(), 1.0_f64),
        ((modifier_slot_constant.diffuse).clone(), 2.0_f64),
        ((modifier_slot_constant.specular).clone(), 3.0_f64),
        ((modifier_slot_constant.emissive).clone(), 4.0_f64),
        ((modifier_slot_constant.effect).clone(), 5.0_f64),
    ]
});
