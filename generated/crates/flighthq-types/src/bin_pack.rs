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

// Source: upstream/packages/types/src/BinPack.ts:33 (sha256:02827650530a05814ac7db8af8205eebd86ecf1ccd58d1a4f06180b57fd558f5)
#[derive(Clone, Default)]
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
    pub heuristic: Option<BinPackHeuristic>,
    pub growable: Option<bool>,
}
impl PartialEq for BinPackOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/BinPack.ts:70 (sha256:9a7fcfb77a423785d0d90037d8ab0662fd34dfc628dbea27c2b7f862914d4699)
#[derive(Clone, Default)]
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

// Source: upstream/packages/types/src/BinPack.ts:81 (sha256:5ec6b8bc563362a6b85216ae7f5e24000b34456b537de576959ca668086f21e3)
pub type BinPackHeuristic = String;

// Source: upstream/packages/types/src/BinPack.ts:92 (sha256:7af9f6f32ceb939b8ca8d15b66aebf815bdb32f84310639682ce4392cc0403ce)
pub type UnpackedRectangleReason = String;

// Source: upstream/packages/types/src/BinPack.ts:94 (sha256:176e320195bd990ea5470d020fa86425782cd3e3b8bb7ab8e3b4d15288b81532)
#[derive(Clone)]
pub struct UnpackedRectangleExplanation {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub id: RectangleId,
    pub reason: UnpackedRectangleReason,
    pub usable_width: f64,
    pub usable_height: f64,
}
impl PartialEq for UnpackedRectangleExplanation {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
