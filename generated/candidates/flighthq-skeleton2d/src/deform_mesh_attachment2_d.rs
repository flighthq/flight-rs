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
    skin_skeleton2_d_attachment_points(
        &(crate::FlightUnion2::<Vec<f32>, Vec<f64>>::A((*out).clone())),
        &(attachment.skin),
        &(attachment.vertices),
        skeleton,
        bone_index,
        &(deform),
        "MeshAttachment2D".to_owned(),
    );
}
