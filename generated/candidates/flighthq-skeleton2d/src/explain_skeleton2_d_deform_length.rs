// @generated from upstream/packages/skeleton2d/src/explainSkeleton2DDeformLength.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::{Skeleton2DDeformLengthExplanation, Skin2D};

// Source: upstream/packages/skeleton2d/src/explainSkeleton2DDeformLength.ts:14 (sha256:5f1aca4d4bddf81d4917eadae52ba653db781cccbf171cac9961e0fc44fc2892)
pub fn explain_skeleton2_d_deform_length(
    skin: &Option<Skin2D>,
    vertices: &Option<Vec<f32>>,
    deform: &Option<Vec<f32>>,
) -> Skeleton2DDeformLengthExplanation {
    let weighted = ((skin).is_some()) && ((skin.as_ref().unwrap()).is_some());
    let addressed = if weighted {
        ((skin.as_ref().unwrap().influences.len() as f64) / 2.0_f64)
    } else {
        (vertices.as_ref().map(|value| value.len() as f64))
            .clone()
            .unwrap_or(0.0_f64)
    };
    let offsets = if (deform).is_none() {
        0.0_f64
    } else {
        (deform.as_ref().unwrap().len() as f64)
    };
    return Skeleton2DDeformLengthExplanation {
        __flight_identity: std::sync::Arc::new(()),
        accepted: ((deform).is_some()) && (offsets >= addressed),
        addressed: addressed,
        addressing: if weighted {
            "weighted".to_owned()
        } else {
            "rigid".to_owned()
        },
        offsets: offsets,
    };
}
