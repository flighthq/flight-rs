// @generated from upstream/packages/types/src/Spring.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/Spring.ts:12 (sha256:e6c4ef7d0f6f1c23cddd5dfc4e9723cb4146faf0b2b5371c776ac77a074c9a45)
#[derive(Clone)]
pub struct Spring {
    pub value: f64,
    pub velocity: f64,
}

// Source: upstream/packages/types/src/Spring.ts:25 (sha256:fee861e5d20e4ce9607bb0446175ab87b87791f7547b08b1f3c33a2e103cbe10)
#[derive(Clone)]
pub struct SpringConfig {
    pub damping_ratio: f64,
    pub frequency: f64,
}

// Source: upstream/packages/types/src/Spring.ts:33 (sha256:ae030432d98239780f8d39ed48a9cf0df4fca1c299a3cb4f52c35a05cac894a3)
#[derive(Clone)]
pub struct Spring2D {
    pub x: Spring,
    pub y: Spring,
}

// Source: upstream/packages/types/src/Spring.ts:40 (sha256:e70a0349c30d39a3bf2a037216b2b969734f5728aa2d3dd1d09d57cdc84ec771)
#[derive(Clone)]
pub struct Spring3D {
    pub x: Spring,
    pub y: Spring,
    pub z: Spring,
}
