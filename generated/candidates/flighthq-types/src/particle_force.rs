// @generated from upstream/packages/types/src/ParticleForce.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{AttractorForce, DragForce, TurbulenceForce, VortexForce, WindForce};

// Source: upstream/packages/types/src/ParticleForce.ts:8 (sha256:7cfd216ace506c8fb07d46215d2b698d0279893103bffe044b934de64a730496)
pub type ParticleForce = crate::FlightUnion2<
    AttractorForce,
    crate::FlightUnion2<
        DragForce,
        crate::FlightUnion2<TurbulenceForce, crate::FlightUnion2<VortexForce, WindForce>>,
    >,
>;
