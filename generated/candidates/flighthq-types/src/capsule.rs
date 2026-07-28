// @generated from upstream/packages/types/src/Capsule.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/Capsule.ts:5 (sha256:fbaf014ea5264f7e12234a5516e989174163eb8de214050b034b3820750e1394)
#[derive(Clone)]
pub struct Capsule {
    pub end_x: f64,
    pub end_y: f64,
    pub end_z: f64,
    pub radius: f64,
    pub start_x: f64,
    pub start_y: f64,
    pub start_z: f64,
}

// Source: upstream/packages/types/src/Capsule.ts:15 (sha256:8aefabc79440206ca63284c068bba45cf890bb3669077dae6cd48a7be692908e)
pub type CapsuleLike = Capsule;
