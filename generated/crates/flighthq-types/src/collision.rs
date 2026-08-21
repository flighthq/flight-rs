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

#[derive(Clone, Default)]
pub struct CollisionBuiltInShape2DRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub x: f64,
    pub y: f64,
    pub kind: String,
}
impl PartialEq for CollisionBuiltInShape2DRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct CollisionBuiltInShape2DRecord2 {
    pub __flight_identity: std::sync::Arc<()>,
    pub x0: f64,
    pub y0: f64,
    pub x1: f64,
    pub y1: f64,
    pub kind: String,
}
impl PartialEq for CollisionBuiltInShape2DRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct CollisionBuiltInShape2DRecord3 {
    pub __flight_identity: std::sync::Arc<()>,
    pub points: Vec<f64>,
    pub kind: String,
}
impl PartialEq for CollisionBuiltInShape2DRecord3 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct CollisionBuiltInShape2DRecord4 {
    pub __flight_identity: std::sync::Arc<()>,
    pub x: f64,
    pub y: f64,
    pub half_w: f64,
    pub half_h: f64,
    pub rotation: f64,
    pub kind: String,
}
impl PartialEq for CollisionBuiltInShape2DRecord4 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct CollisionBuiltInShape2DRecord5 {
    pub __flight_identity: std::sync::Arc<()>,
    pub min_x: f64,
    pub min_y: f64,
    pub max_x: f64,
    pub max_y: f64,
    pub kind: String,
}
impl PartialEq for CollisionBuiltInShape2DRecord5 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct CollisionBuiltInShape2DRecord6 {
    pub __flight_identity: std::sync::Arc<()>,
    pub x: f64,
    pub y: f64,
    pub radius: f64,
    pub kind: String,
}
impl PartialEq for CollisionBuiltInShape2DRecord6 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Collision.ts:30 (sha256:7db462c203a8326feaa3b6f077253c1486b1160636f9b2ca4703d869a6f4722a)
pub type CollisionShapeKind2D = String;

// Source: upstream/packages/types/src/Collision.ts:33 (sha256:572bc1fab99673a7211ecaffd2f2a4f72974d5eb9e1f2062ff3b44431e6afcc4)
#[derive(Clone, Default)]
pub struct CollisionCircle2D {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub x: f64,
    pub y: f64,
    pub radius: f64,
}
impl PartialEq for CollisionCircle2D {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Collision.ts:41 (sha256:4c63c535423d4c802f4237e5211b2480a0465f414e51b222337c0f4d129db694)
#[derive(Clone, Default)]
pub struct CollisionAabb2D {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub min_x: f64,
    pub min_y: f64,
    pub max_x: f64,
    pub max_y: f64,
}
impl PartialEq for CollisionAabb2D {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Collision.ts:50 (sha256:a575915d72f801d492f02286e72f5c127c85326115f2497eb1b1912baab2f87c)
#[derive(Clone, Default)]
pub struct CollisionObb2D {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub x: f64,
    pub y: f64,
    pub half_w: f64,
    pub half_h: f64,
    pub rotation: f64,
}
impl PartialEq for CollisionObb2D {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Collision.ts:61 (sha256:3c88a2da39fd0c95a7e88ea31dca04c5cfa4b36b90689e4bf2d05546465dece2)
#[derive(Clone, Default)]
pub struct CollisionPolygon2D {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub points: Vec<f64>,
}
impl PartialEq for CollisionPolygon2D {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Collision.ts:67 (sha256:de593bf0091252b67638b568aedcfe522f2e4bc955f147cba25c9d97d7040449)
#[derive(Clone, Default)]
pub struct CollisionSegment2D {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub x0: f64,
    pub y0: f64,
    pub x1: f64,
    pub y1: f64,
}
impl PartialEq for CollisionSegment2D {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Collision.ts:75 (sha256:15530ca0c700ffff8d5c6b411e8bccf705e538c75086919115274e874c3df252)
#[derive(Clone, Default)]
pub struct CollisionPoint2D {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub x: f64,
    pub y: f64,
}
impl PartialEq for CollisionPoint2D {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Collision.ts:83 (sha256:e07cbb0b2cdcd24bbc5c4afc84ac4cd3696b65ff71704b44708cf6e1ff95470f)
pub type CollisionBuiltInShape2D = crate::FlightUnion2<
    CollisionBuiltInShape2DRecord6,
    crate::FlightUnion2<
        CollisionBuiltInShape2DRecord5,
        crate::FlightUnion2<
            CollisionBuiltInShape2DRecord4,
            crate::FlightUnion2<
                CollisionBuiltInShape2DRecord3,
                crate::FlightUnion2<CollisionBuiltInShape2DRecord2, CollisionBuiltInShape2DRecord1>,
            >,
        >,
    >,
>;

// Source: upstream/packages/types/src/Collision.ts:106 (sha256:4878fb53ade5143836a7e1877e9b6afac6536bc81e9faa1baea2d70bbe401802)
#[derive(Clone, Default)]
pub struct CollisionVendorShape2D {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: CollisionVendorKind2D,
}
impl PartialEq for CollisionVendorShape2D {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Collision.ts:111 (sha256:5176c3df5a891975b6b6722a17fe402aa95c948e8890470f60521c4c48ab0775)
pub type CollisionVendorKind2D = String;

// Source: upstream/packages/types/src/Collision.ts:116 (sha256:4be55b3191ecec28dccb82afca9dbad7741b2e80850e9452bd3e84c664666a14)
pub type CollisionShape2D = crate::FlightUnion2<CollisionBuiltInShape2D, CollisionVendorShape2D>;

// Source: upstream/packages/types/src/Collision.ts:124 (sha256:d6aeed28d689880b86690274be5bf1bbae4e6925518f467b381c0a3fb848ba57)
#[derive(Clone, Default)]
pub struct CollisionManifold2D {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub overlapping: bool,
    pub normal_x: f64,
    pub normal_y: f64,
    pub depth: f64,
}
impl PartialEq for CollisionManifold2D {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Collision.ts:134 (sha256:7e697ecfffc8e5104e3e0bc9d2257c08f1f4b162258e53df6c4201f24ee96223)
#[derive(Clone, Default)]
pub struct CollisionRaycastHit2D {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub fraction: f64,
    pub x: f64,
    pub y: f64,
    pub normal_x: f64,
    pub normal_y: f64,
}
impl PartialEq for CollisionRaycastHit2D {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Collision.ts:146 (sha256:c0ed0a556d84d92379c5ceea6f10db4b92255b6633ea4e34a9d102483f40da61)
#[derive(Clone, Default)]
pub struct CollisionTimeOfImpact2D {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub fraction: f64,
    pub x: f64,
    pub y: f64,
    pub normal_x: f64,
    pub normal_y: f64,
}
impl PartialEq for CollisionTimeOfImpact2D {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Collision.ts:156 (sha256:4421b2274708a2926ea619e2ed48e633e97dd2f826f8b8f59b431ffbfecfea16)
#[derive(Clone, Default)]
pub struct CollisionTestExplanation2D {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: Option<CollisionShapeKind2D>,
    pub overlapping: bool,
    pub shape_index: Option<f64>,
    pub status: CollisionTestStatus,
}
impl PartialEq for CollisionTestExplanation2D {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Collision.ts:164 (sha256:c549d89d7eaf43d488c36e5aa6cb4580b9a0f9bcc3c375404d8e5b0daf0719d7)
pub type CollisionTestStatus = String;

// Source: upstream/packages/types/src/Collision.ts:184 (sha256:c0e26b799b9cca4351fc7b7bd786afc6dcbbe50c374efd7cadcd84739f911028)
pub type CollisionSupport2D = std::sync::Arc<
    std::sync::Mutex<Box<dyn FnMut(CollisionShape2D, f64, f64, Vec<f64>) -> () + Send + 'static>>,
>;

// Source: upstream/packages/types/src/Collision.ts:193 (sha256:8df62436be8e066f6b520ef2155aa36cb11392592844a8b3427cdbef2236bdc7)
pub type CollisionPairTest2D = std::sync::Arc<
    std::sync::Mutex<
        Box<
            dyn FnMut(CollisionShape2D, CollisionShape2D, CollisionManifold2D) -> bool
                + Send
                + 'static,
        >,
    >,
>;

// Source: upstream/packages/types/src/Collision.ts:201 (sha256:7cd40d54d6b52c94c1b483afd59b006486bf250f4baca4f3e1f627dc7a2fb6b5)
pub type CollisionTestGuard2D = std::sync::Arc<
    std::sync::Mutex<Box<dyn FnMut(CollisionShape2D, CollisionShape2D) -> () + Send + 'static>>,
>;

// Source: upstream/packages/types/src/Collision.ts:210 (sha256:49d86e4cbb8bd06a2c29ad03a6c6f45088a9596d1f200bb9bb55f07c6842ee10)
#[derive(Clone, Default)]
pub struct CollisionContactPoint2D {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub x: f64,
    pub y: f64,
    pub depth: f64,
    pub feature_id: f64,
}
impl PartialEq for CollisionContactPoint2D {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Collision.ts:236 (sha256:70433e9e6573de517e1e3ff1ea8550a8fbcd0d69578f822f81a548ac128a2cc3)
#[derive(Clone, Default)]
pub struct CollisionContactManifold2D {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub overlapping: bool,
    pub normal_x: f64,
    pub normal_y: f64,
    pub depth: f64,
    pub point_count: f64,
    pub points: Vec<CollisionContactPoint2D>,
}
impl PartialEq for CollisionContactManifold2D {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
