// @generated from upstream/packages/skeleton2d/src/deformMeshAttachment2D.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::skin_skeleton2_d_attachment_points;
use flighthq_types::{MeshAttachment2D, Skeleton2D};

// Source: upstream/packages/skeleton2d/src/deformMeshAttachment2D.ts:29 (sha256:ed27c419661732031f9311fb5fe9011a2470ee3174a51bd9bd17218ab14bccd8)
pub fn deform_skeleton2_d_mesh_attachment(
    out: &mut Vec<f32>,
    attachment: &MeshAttachment2D,
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
            "MeshAttachment2D".to_owned(),
        );
        *(out) = match __flight_argument_0 {
            crate::FlightUnion2::A(value) => value,
            crate::FlightUnion2::B(_) => panic!("TypeScript union narrowing failed"),
        };
        __flight_result
    };
}
