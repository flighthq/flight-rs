// @generated from upstream/packages/skeleton2d/src/slotDeform2D.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::{Attachment2D, Skeleton2DSlotDeform, Slot2D};

// Source: upstream/packages/skeleton2d/src/slotDeform2D.ts:18 (sha256:f2d0dd34987b9095371517191c7053c5af828b9430a82203f0435bd208ef8bff)
pub fn get_skeleton2_d_slot_deform_offsets(slot: &Slot2D) -> Option<Vec<f32>> {
    let deform = (slot.deform).clone();
    if ((deform).is_none()) || ((deform).is_none()) {
        return None;
    }
    return if ((deform.as_ref().unwrap().attachment).clone() == (slot.attachment).clone()) {
        Some((deform.as_ref().unwrap().offsets).clone())
    } else {
        None
    };
}

// Source: upstream/packages/skeleton2d/src/slotDeform2D.ts:27 (sha256:2e4c62964171bad3204dcde121f56d44c8225b8779c24c7441dd59dfb84c338d)
pub fn set_skeleton2_d_slot_deform(
    slot: &mut Slot2D,
    attachment: &Option<Attachment2D>,
    offsets: &Option<Vec<f32>>,
) -> Option<Skeleton2DSlotDeform> {
    if (offsets).is_none() {
        slot.deform = None;
        return None;
    }
    let mut existing = (slot.deform).clone();
    if (((existing).is_some()) && ((existing).is_some()))
        && ((existing.as_mut().unwrap().offsets.len() as f64)
            == (offsets.as_ref().unwrap().len() as f64))
    {
        {
            let __flight_offset = (0.0_f64) as usize;
            let __flight_values: Vec<f32> = (offsets.as_ref().unwrap())
                .iter()
                .map(|value| (*value) as f32)
                .collect();
            existing.as_mut().unwrap().offsets
                [__flight_offset..__flight_offset + __flight_values.len()]
                .copy_from_slice(&__flight_values);
        };
        existing.as_mut().unwrap().attachment = (*attachment).clone();
        return Some((existing.as_mut().unwrap()).clone());
    }
    let record: Skeleton2DSlotDeform = Skeleton2DSlotDeform {
        __flight_identity: std::sync::Arc::new(()),
        attachment: (*attachment).clone(),
        offsets: crate::host_value::<Vec<f32>>("host.from"),
    };
    slot.deform = Some((record).clone());
    return Some((record).clone());
}
