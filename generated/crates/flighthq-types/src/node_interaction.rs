// @generated from upstream/packages/types/src/NodeInteraction.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{NodeAny, Path, Rectangle};

// Source: upstream/packages/types/src/NodeInteraction.ts:21 (sha256:c03b73e655b4f055b50765741f70f78de5156242edc71fdf4aedf8ee87b501b1)
pub type HitArea =
    crate::FlightUnion2<Rectangle, crate::FlightUnion2<Path, crate::FlightUnion2<NodeAny, String>>>;
