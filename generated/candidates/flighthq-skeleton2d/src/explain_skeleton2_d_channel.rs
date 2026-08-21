// @generated from upstream/packages/skeleton2d/src/explainSkeleton2DChannel.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::{AnimationChannel, Skeleton2DCoercedInterpolation};

// Source: upstream/packages/skeleton2d/src/explainSkeleton2DChannel.ts:12 (sha256:d4d84566a648258f4b4ea8cdf1815d1fad6226e4f75da647b913550b2e7d0419)
pub fn explain_skeleton2_d_channel_interpolation(
    channel: &AnimationChannel,
    subject: String,
) -> Option<Skeleton2DCoercedInterpolation> {
    if (!is_skeleton2_d_stepped_channel_subject((subject).clone())) {
        return None;
    }
    let stated = (channel.track.interpolation).clone();
    if (stated == STEP_INTERPOLATION) {
        return None;
    }
    return Some(Skeleton2DCoercedInterpolation {
        __flight_identity: std::sync::Arc::new(()),
        applied: ((STEP_INTERPOLATION).clone()).to_owned(),
        stated: stated,
        subject: (subject).clone(),
    });
}

// Source: upstream/packages/skeleton2d/src/explainSkeleton2DChannel.ts:29 (sha256:d3e7626eb4360d077e402952e34900df92781e163bed91b51f60c551ab933796)
pub fn is_skeleton2_d_stepped_channel_subject(subject: String) -> bool {
    return (subject == ATTACHMENT_SUBJECT) || (subject == DRAW_ORDER_SUBJECT);
}

// Source: upstream/packages/skeleton2d/src/explainSkeleton2DChannel.ts:35 (sha256:401b07a86a4f5c8734b02580076425abdd47bacbf4385b3cfc09d71c10ef8a68)
const ATTACHMENT_SUBJECT: &'static str = "Attachment";

// Source: upstream/packages/skeleton2d/src/explainSkeleton2DChannel.ts:36 (sha256:8dcbc82a9e1decc3eec900c68b7df1d0dbab08ae687892fca147b49ff8e098b1)
const DRAW_ORDER_SUBJECT: &'static str = "DrawOrder";

// Source: upstream/packages/skeleton2d/src/explainSkeleton2DChannel.ts:37 (sha256:2a65dd44fe1e532dafbbac24f32c30ec16768129d77b37d30e80ed27ad4ce3de)
const STEP_INTERPOLATION: &'static str = "Step";
