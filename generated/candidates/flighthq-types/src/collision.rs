// @generated from upstream/packages/types/src/Collision.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/Collision.ts:12 (sha256:05692c0f748ddd3cd4f43d69d0b3bd72cc43f753768c393095142b7e1c0ea154)
pub type CollisionShapeKind = String;

// Source: upstream/packages/types/src/Collision.ts:15 (sha256:bc4f3e77fdb0246be1c932573eca4bd375c2ff8e57b78a9ff39ae4670dcd928e)
#[derive(Clone)]
pub struct CollisionCircle {
    pub x: f64,
    pub y: f64,
    pub radius: f64,
}

// Source: upstream/packages/types/src/Collision.ts:23 (sha256:5dc41063d0c28d3e015af4cd5b3fa24ee8c9c52eab1b32376c918fc08222420b)
#[derive(Clone)]
pub struct CollisionAabb {
    pub min_x: f64,
    pub min_y: f64,
    pub max_x: f64,
    pub max_y: f64,
}

// Source: upstream/packages/types/src/Collision.ts:32 (sha256:5633e504c3ac228a2a1532ed76a91619c369b92fd8863bc3d9756dc27797c8ff)
#[derive(Clone)]
pub struct CollisionObb {
    pub x: f64,
    pub y: f64,
    pub half_w: f64,
    pub half_h: f64,
    pub rotation: f64,
}

// Source: upstream/packages/types/src/Collision.ts:43 (sha256:578976524467fce4d7d33df394603220f57396581f3e3ef6198195c6593a35bd)
#[derive(Clone)]
pub struct CollisionPolygon {
    pub points: Vec<f64>,
}

// Source: upstream/packages/types/src/Collision.ts:49 (sha256:a17eaaa7e7ec2ca72d4db02553418ae15acfee218a5a61054be88f39cdcbfbbe)
#[derive(Clone)]
pub struct CollisionSegment {
    pub x0: f64,
    pub y0: f64,
    pub x1: f64,
    pub y1: f64,
}

// Source: upstream/packages/types/src/Collision.ts:57 (sha256:6db08d48fd527c64faaac34576d72e914f30a29b6d84216e3584cb0a6f163ed4)
#[derive(Clone)]
pub struct CollisionPoint {
    pub x: f64,
    pub y: f64,
}

// Source: upstream/packages/types/src/Collision.ts:65 (sha256:d5dcb6313a9922b2369d904d6071c1d42f2071693951137c23d6ad5b7735319c)
pub type CollisionShape = crate::OpaqueHostValue;

// Source: upstream/packages/types/src/Collision.ts:79 (sha256:3faf5007f7f5fcf04ee37c934cfbdb99659201a81ab1a767ebe1727536076405)
#[derive(Clone)]
pub struct CollisionManifold {
    pub overlapping: bool,
    pub normal_x: f64,
    pub normal_y: f64,
    pub depth: f64,
}
