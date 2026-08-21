// @generated from upstream/packages/skeleton2d/src/deformPathAttachment2D.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::skin_skeleton2_d_attachment_points;
use flighthq_types::{Path, PathAttachment2D, Skeleton2D};

// Source: upstream/packages/skeleton2d/src/deformPathAttachment2D.ts:28 (sha256:91ae321fb7dd184e9a25bccba649c8de631cbdde2c6d99de6297ff9d2b2cdfc4)
pub fn deform_skeleton2_d_path_attachment(
    out: &mut Path,
    attachment: &PathAttachment2D,
    skeleton: &Skeleton2D,
    bone_index: f64,
    deform: Option<Vec<f32>>,
) -> () {
    out.commands
        .truncate((attachment.commands.len() as f64) as usize);
    {
        let mut i = 0.0_f64;
        while (i < (attachment.commands.len() as f64)) {
            {
                let __flight_index = (i) as usize;
                let __flight_value = attachment.commands[i as usize].clone();
                if __flight_index == out.commands.len() {
                    out.commands.push(__flight_value);
                } else {
                    out.commands[__flight_index] = __flight_value;
                }
            };
            {
                i += 1.0;
                i
            };
        }
    }
    out.winding = (attachment.winding).clone();
    out.data
        .truncate((attachment.point_count * 2.0_f64) as usize);
    skin_skeleton2_d_attachment_points(
        &(crate::FlightUnion2::<Vec<f32>, Vec<f64>>::B((out.data).clone())),
        &(attachment.skin),
        &(attachment.vertices),
        skeleton,
        bone_index,
        &(deform),
        "PathAttachment2D".to_owned(),
    );
}
