// @generated from upstream/packages/types/src/Skeleton2DAnimationTargetBinder.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{AnimationChannel, Skeleton2D};

// Source: upstream/packages/types/src/Skeleton2DAnimationTargetBinder.ts:14 (sha256:9d41d4d59c3d2800b95d8fa330cbb6392e0a9299bfa5c64a6b90a4951dcd1ae7)
pub type Skeleton2DAnimationTargetBinder = std::sync::Arc<
    std::sync::Mutex<
        Box<
            dyn FnMut(AnimationChannel, Skeleton2D, Skeleton2D, crate::FlightValue, f64) -> ()
                + Send
                + 'static,
        >,
    >,
>;
