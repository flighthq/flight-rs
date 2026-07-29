// @generated from upstream/packages/types/src/ParticleCollider.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{CircleCollider, PlaneCollider, RectangleCollider, SphereCollider};

// Source: upstream/packages/types/src/ParticleCollider.ts:7 (sha256:8f54bd14b8c32f1c74b8a0d5887d6402ad030550010ecd8090207ebda51cdae7)
pub type ParticleCollider = crate::FlightUnion2<
    CircleCollider,
    crate::FlightUnion2<PlaneCollider, crate::FlightUnion2<RectangleCollider, SphereCollider>>,
>;
