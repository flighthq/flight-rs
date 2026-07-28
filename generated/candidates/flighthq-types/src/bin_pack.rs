// @generated from upstream/packages/types/src/BinPack.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/BinPack.ts:10 (sha256:3f8ae0c79c92de844aeac6b6aa563c3603e25fab5d0d19cf11decc44dfd0c14b)
pub type RectangleId = crate::FlightUnion2<String, f64>;

// Source: upstream/packages/types/src/BinPack.ts:14 (sha256:0d4dd4a03fe6768f388ff1d15945725582f976354ad7cc1f2df54aa966166763)
#[derive(Clone)]
pub struct PackableRectangle {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub id: RectangleId,
    pub width: f64,
    pub height: f64,
}
impl PartialEq for PackableRectangle {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/BinPack.ts:23 (sha256:f498a1a6c6c8deff2dddc5907f79f47542d9e319a34b20f6250f48370de6675a)
#[derive(Clone)]
pub struct PackedRectangle {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub id: RectangleId,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub rotated: bool,
}
impl PartialEq for PackedRectangle {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/BinPack.ts:33 (sha256:2ce3c70c7b5ec3d8a6559fbd1d31675efc82b30a5c1d78f8c43af58fa83e2ef7)
#[derive(Clone)]
pub struct BinPackOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub max_width: Option<f64>,
    pub max_height: Option<f64>,
    pub padding: Option<f64>,
    pub border: Option<f64>,
    pub power_of_two: Option<bool>,
    pub square: Option<bool>,
    pub allow_rotation: Option<bool>,
    pub growable: Option<bool>,
}
impl PartialEq for BinPackOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/BinPack.ts:61 (sha256:9a7fcfb77a423785d0d90037d8ab0662fd34dfc628dbea27c2b7f862914d4699)
#[derive(Clone)]
pub struct PackResult {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub placements: Vec<PackedRectangle>,
    pub width: f64,
    pub height: f64,
    pub unpacked: Vec<RectangleId>,
}
impl PartialEq for PackResult {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
