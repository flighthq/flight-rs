// @generated from upstream/packages/types/src/Collision.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

#[derive(Clone, Default)]
pub struct SharedStructuralRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub fraction: f64,
    pub x: f64,
    pub y: f64,
    pub normal_x: f64,
    pub normal_y: f64,
}
impl PartialEq for SharedStructuralRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Collision.ts:13 (sha256:05692c0f748ddd3cd4f43d69d0b3bd72cc43f753768c393095142b7e1c0ea154)
pub type CollisionShapeKind = String;

// Source: upstream/packages/types/src/Collision.ts:16 (sha256:bc4f3e77fdb0246be1c932573eca4bd375c2ff8e57b78a9ff39ae4670dcd928e)
#[derive(Clone, Default)]
pub struct CollisionCircle {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub x: f64,
    pub y: f64,
    pub radius: f64,
}
impl PartialEq for CollisionCircle {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Collision.ts:24 (sha256:5dc41063d0c28d3e015af4cd5b3fa24ee8c9c52eab1b32376c918fc08222420b)
#[derive(Clone, Default)]
pub struct CollisionAabb {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub min_x: f64,
    pub min_y: f64,
    pub max_x: f64,
    pub max_y: f64,
}
impl PartialEq for CollisionAabb {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Collision.ts:33 (sha256:5633e504c3ac228a2a1532ed76a91619c369b92fd8863bc3d9756dc27797c8ff)
#[derive(Clone, Default)]
pub struct CollisionObb {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub x: f64,
    pub y: f64,
    pub half_w: f64,
    pub half_h: f64,
    pub rotation: f64,
}
impl PartialEq for CollisionObb {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Collision.ts:44 (sha256:578976524467fce4d7d33df394603220f57396581f3e3ef6198195c6593a35bd)
#[derive(Clone, Default)]
pub struct CollisionPolygon {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub points: Vec<f64>,
}
impl PartialEq for CollisionPolygon {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Collision.ts:50 (sha256:a17eaaa7e7ec2ca72d4db02553418ae15acfee218a5a61054be88f39cdcbfbbe)
#[derive(Clone, Default)]
pub struct CollisionSegment {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub x0: f64,
    pub y0: f64,
    pub x1: f64,
    pub y1: f64,
}
impl PartialEq for CollisionSegment {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Collision.ts:58 (sha256:6db08d48fd527c64faaac34576d72e914f30a29b6d84216e3584cb0a6f163ed4)
#[derive(Clone, Default)]
pub struct CollisionPoint {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub x: f64,
    pub y: f64,
}
impl PartialEq for CollisionPoint {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Collision.ts:66 (sha256:d5dcb6313a9922b2369d904d6071c1d42f2071693951137c23d6ad5b7735319c)
#[derive(Clone, Default)]
pub struct CollisionShape {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: String,
}
impl PartialEq for CollisionShape {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Collision.ts:80 (sha256:3faf5007f7f5fcf04ee37c934cfbdb99659201a81ab1a767ebe1727536076405)
#[derive(Clone, Default)]
pub struct CollisionManifold {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub overlapping: bool,
    pub normal_x: f64,
    pub normal_y: f64,
    pub depth: f64,
}
impl PartialEq for CollisionManifold {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Collision.ts:90 (sha256:0fc37ebc201db4f24d23947b080f8715be1cf57f2413f81e5ba05b274d3134d4)
#[derive(Clone, Default)]
pub struct CollisionRaycastHit {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub fraction: f64,
    pub x: f64,
    pub y: f64,
    pub normal_x: f64,
    pub normal_y: f64,
}
impl PartialEq for CollisionRaycastHit {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Collision.ts:102 (sha256:daa52fb2451e8e19fe83cb7d0c336ee6443aa4ce7bed4067a08e12c74835c407)
#[derive(Clone, Default)]
pub struct CollisionTimeOfImpact {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub fraction: f64,
    pub x: f64,
    pub y: f64,
    pub normal_x: f64,
    pub normal_y: f64,
}
impl PartialEq for CollisionTimeOfImpact {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Collision.ts:112 (sha256:f96b2f199770412e07f811e7dacbe451fbe0bb15778a627ce86e5a6d3405c4d8)
#[derive(Clone, Default)]
pub struct CollisionTestExplanation {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: Option<CollisionShapeKind>,
    pub overlapping: bool,
    pub shape_index: Option<f64>,
    pub status: CollisionTestStatus,
}
impl PartialEq for CollisionTestExplanation {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Collision.ts:120 (sha256:c549d89d7eaf43d488c36e5aa6cb4580b9a0f9bcc3c375404d8e5b0daf0719d7)
pub type CollisionTestStatus = String;

// Source: upstream/packages/types/src/Collision.ts:129 (sha256:16044c2423843182b7aaeec33a20cc7799d7d2abcecfdad397339c3a11fa7b5b)
pub type CollisionTestGuard = std::sync::Arc<
    std::sync::Mutex<Box<dyn FnMut(CollisionShape, CollisionShape) -> () + Send + 'static>>,
>;

// Source: upstream/packages/types/src/Collision.ts:138 (sha256:0972cae3f54ba0ec4d0e1833767f11ab476578197e212b0dcb988637891ba0fa)
#[derive(Clone, Default)]
pub struct CollisionContactPoint {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub x: f64,
    pub y: f64,
    pub depth: f64,
    pub feature_id: f64,
}
impl PartialEq for CollisionContactPoint {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Collision.ts:164 (sha256:6dfc439ab4ce910b63d1d1a0ad76eaa0bb434fe5a5a17db5b8af67a6ca5332ef)
#[derive(Clone, Default)]
pub struct CollisionContactManifold {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub overlapping: bool,
    pub normal_x: f64,
    pub normal_y: f64,
    pub depth: f64,
    pub point_count: f64,
    pub points: Vec<CollisionContactPoint>,
}
impl PartialEq for CollisionContactManifold {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
