// @generated from upstream/packages/skeleton2d/src/clippingAttachment2D.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::skin_skeleton2_d_attachment_points;
use flighthq_types::{ClippingAttachment2D, Skeleton2D};

#[derive(Clone, Default)]
pub struct SharedStructuralRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub end: f64,
    pub start: f64,
}
impl PartialEq for SharedStructuralRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/skeleton2d/src/clippingAttachment2D.ts:13 (sha256:acb36ce8eccddd6ddbae12cd4c40c9021098bf6c2986fbae3962ac36b79a8894)
pub fn compute_skeleton2_d_clipping_attachment_vertices(
    out: &mut Vec<f32>,
    attachment: &ClippingAttachment2D,
    skeleton: &Skeleton2D,
    bone_index: f64,
    deform: Option<Vec<f32>>,
) -> () {
    {
        let mut __flight_argument_0 =
            crate::FlightUnion2::<Vec<f32>, Vec<f64>>::A(std::mem::take(out));
        let __flight_result = skin_skeleton2_d_attachment_points(
            &mut __flight_argument_0,
            &(attachment.skin),
            &(attachment.vertices),
            skeleton,
            bone_index,
            &(deform),
            "ClippingAttachment2D".to_owned(),
        );
        *(out) = match __flight_argument_0 {
            crate::FlightUnion2::A(value) => value,
            crate::FlightUnion2::B(_) => panic!("TypeScript union narrowing failed"),
        };
        __flight_result
    };
}

// Source: upstream/packages/skeleton2d/src/clippingAttachment2D.ts:38 (sha256:c65199749a9caaf460f43a08e94afcd2333c44610c2e476a322464c700bbad42)
pub fn get_skeleton2_d_clipping_attachment_slot_range(
    attachment: &ClippingAttachment2D,
    slot_index: f64,
    slot_count: f64,
) -> SharedStructuralRecord1 {
    let start = (slot_index + 1.0_f64);
    let declared = attachment.end_slot_index;
    let end = if (declared < start) {
        slot_count
    } else {
        (declared + 1.0_f64).min(slot_count)
    };
    return SharedStructuralRecord1 {
        __flight_identity: std::sync::Arc::new(()),
        end: (start).max(end),
        start: start,
    };
}
