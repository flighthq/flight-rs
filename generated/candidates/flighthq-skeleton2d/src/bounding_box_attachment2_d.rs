// @generated from upstream/packages/skeleton2d/src/boundingBoxAttachment2D.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::skin_skeleton2_d_attachment_points;
use flighthq_types::{BoundingBoxAttachment2D, Skeleton2D};

// Source: upstream/packages/skeleton2d/src/boundingBoxAttachment2D.ts:14 (sha256:abae0d9c17b108b0f9f32e85524d20a7b231f8ce2254435178082d3ac227d6cd)
pub fn compute_skeleton2_d_bounding_box_attachment_vertices(
    out: &mut Vec<f32>,
    attachment: &BoundingBoxAttachment2D,
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
        "BoundingBoxAttachment2D".to_owned(),
    );
}
