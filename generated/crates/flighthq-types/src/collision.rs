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
    pub points: Vec<f64>,
}
impl PartialEq for SharedStructuralRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct SharedStructuralRecord2 {
    pub __flight_identity: std::sync::Arc<()>,
    pub fraction: f64,
    pub x: f64,
    pub y: f64,
    pub normal_x: f64,
    pub normal_y: f64,
}
impl PartialEq for SharedStructuralRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct SharedStructuralRecord3 {
    pub __flight_identity: std::sync::Arc<()>,
    pub x0: f64,
    pub y0: f64,
    pub z0: f64,
    pub x1: f64,
    pub y1: f64,
    pub z1: f64,
    pub radius: f64,
}
impl PartialEq for SharedStructuralRecord3 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct SharedStructuralRecord4 {
    pub __flight_identity: std::sync::Arc<()>,
    pub fraction: f64,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub normal_x: f64,
    pub normal_y: f64,
    pub normal_z: f64,
}
impl PartialEq for SharedStructuralRecord4 {
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
    pub x0: f64,
    pub y0: f64,
    pub x1: f64,
    pub y1: f64,
    pub radius: f64,
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
    pub x: f64,
    pub y: f64,
    pub half_w: f64,
    pub half_h: f64,
    pub rotation: f64,
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
    pub min_x: f64,
    pub min_y: f64,
    pub max_x: f64,
    pub max_y: f64,
    pub kind: String,
}
impl PartialEq for CollisionBuiltInShape2DRecord6 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct CollisionBuiltInShape2DRecord7 {
    pub __flight_identity: std::sync::Arc<()>,
    pub x: f64,
    pub y: f64,
    pub radius: f64,
    pub kind: String,
}
impl PartialEq for CollisionBuiltInShape2DRecord7 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct CollisionBuiltInShape3DRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub points: Vec<f64>,
    pub kind: String,
}
impl PartialEq for CollisionBuiltInShape3DRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct CollisionBuiltInShape3DRecord2 {
    pub __flight_identity: std::sync::Arc<()>,
    pub apex_x: f64,
    pub apex_y: f64,
    pub apex_z: f64,
    pub base_x: f64,
    pub base_y: f64,
    pub base_z: f64,
    pub radius: f64,
    pub kind: String,
}
impl PartialEq for CollisionBuiltInShape3DRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct CollisionBuiltInShape3DRecord3 {
    pub __flight_identity: std::sync::Arc<()>,
    pub x0: f64,
    pub y0: f64,
    pub z0: f64,
    pub x1: f64,
    pub y1: f64,
    pub z1: f64,
    pub radius: f64,
    pub kind: String,
}
impl PartialEq for CollisionBuiltInShape3DRecord3 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct CollisionBuiltInShape3DRecord4 {
    pub __flight_identity: std::sync::Arc<()>,
    pub x0: f64,
    pub y0: f64,
    pub z0: f64,
    pub x1: f64,
    pub y1: f64,
    pub z1: f64,
    pub radius: f64,
    pub kind: String,
}
impl PartialEq for CollisionBuiltInShape3DRecord4 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct CollisionBuiltInShape3DRecord5 {
    pub __flight_identity: std::sync::Arc<()>,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub half_x: f64,
    pub half_y: f64,
    pub half_z: f64,
    pub rotation_x: f64,
    pub rotation_y: f64,
    pub rotation_z: f64,
    pub rotation_w: f64,
    pub kind: String,
}
impl PartialEq for CollisionBuiltInShape3DRecord5 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct CollisionBuiltInShape3DRecord6 {
    pub __flight_identity: std::sync::Arc<()>,
    pub min_x: f64,
    pub min_y: f64,
    pub min_z: f64,
    pub max_x: f64,
    pub max_y: f64,
    pub max_z: f64,
    pub kind: String,
}
impl PartialEq for CollisionBuiltInShape3DRecord6 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct CollisionBuiltInShape3DRecord7 {
    pub __flight_identity: std::sync::Arc<()>,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub radius: f64,
    pub kind: String,
}
impl PartialEq for CollisionBuiltInShape3DRecord7 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Collision.ts:30 (sha256:40aabd18d52886e6d24b7c1ec32e34fbad13479897122f881f7c85728661e076)
pub type CollisionShapeKind2D = String;

// Source: upstream/packages/types/src/Collision.ts:41 (sha256:572bc1fab99673a7211ecaffd2f2a4f72974d5eb9e1f2062ff3b44431e6afcc4)
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

// Source: upstream/packages/types/src/Collision.ts:49 (sha256:4c63c535423d4c802f4237e5211b2480a0465f414e51b222337c0f4d129db694)
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

// Source: upstream/packages/types/src/Collision.ts:58 (sha256:a575915d72f801d492f02286e72f5c127c85326115f2497eb1b1912baab2f87c)
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

// Source: upstream/packages/types/src/Collision.ts:80 (sha256:18a97268e4b52e0de7a5418680f1d704e486efecd4c69c95d1d66a7fc1ff85d3)
#[derive(Clone, Default)]
pub struct CollisionCapsule2D {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub x0: f64,
    pub y0: f64,
    pub x1: f64,
    pub y1: f64,
    pub radius: f64,
}
impl PartialEq for CollisionCapsule2D {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Collision.ts:91 (sha256:3c88a2da39fd0c95a7e88ea31dca04c5cfa4b36b90689e4bf2d05546465dece2)
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

// Source: upstream/packages/types/src/Collision.ts:97 (sha256:de593bf0091252b67638b568aedcfe522f2e4bc955f147cba25c9d97d7040449)
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

// Source: upstream/packages/types/src/Collision.ts:105 (sha256:15530ca0c700ffff8d5c6b411e8bccf705e538c75086919115274e874c3df252)
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

// Source: upstream/packages/types/src/Collision.ts:113 (sha256:b12d049b429132bbafad7ec375ae59188f6227e12fb12dc0f3b945f7b4bacac7)
pub type CollisionBuiltInShape2D = crate::FlightUnion2<
    CollisionBuiltInShape2DRecord7,
    crate::FlightUnion2<
        CollisionBuiltInShape2DRecord6,
        crate::FlightUnion2<
            CollisionBuiltInShape2DRecord5,
            crate::FlightUnion2<
                CollisionBuiltInShape2DRecord4,
                crate::FlightUnion2<
                    CollisionBuiltInShape2DRecord3,
                    crate::FlightUnion2<
                        CollisionBuiltInShape2DRecord2,
                        CollisionBuiltInShape2DRecord1,
                    >,
                >,
            >,
        >,
    >,
>;

// Source: upstream/packages/types/src/Collision.ts:137 (sha256:4878fb53ade5143836a7e1877e9b6afac6536bc81e9faa1baea2d70bbe401802)
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

// Source: upstream/packages/types/src/Collision.ts:142 (sha256:5176c3df5a891975b6b6722a17fe402aa95c948e8890470f60521c4c48ab0775)
pub type CollisionVendorKind2D = String;

// Source: upstream/packages/types/src/Collision.ts:147 (sha256:4be55b3191ecec28dccb82afca9dbad7741b2e80850e9452bd3e84c664666a14)
pub type CollisionShape2D = crate::FlightUnion2<CollisionBuiltInShape2D, CollisionVendorShape2D>;

// Source: upstream/packages/types/src/Collision.ts:155 (sha256:d6aeed28d689880b86690274be5bf1bbae4e6925518f467b381c0a3fb848ba57)
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

// Source: upstream/packages/types/src/Collision.ts:165 (sha256:7e697ecfffc8e5104e3e0bc9d2257c08f1f4b162258e53df6c4201f24ee96223)
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

// Source: upstream/packages/types/src/Collision.ts:177 (sha256:c0ed0a556d84d92379c5ceea6f10db4b92255b6633ea4e34a9d102483f40da61)
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

// Source: upstream/packages/types/src/Collision.ts:187 (sha256:4421b2274708a2926ea619e2ed48e633e97dd2f826f8b8f59b431ffbfecfea16)
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

// Source: upstream/packages/types/src/Collision.ts:195 (sha256:c549d89d7eaf43d488c36e5aa6cb4580b9a0f9bcc3c375404d8e5b0daf0719d7)
pub type CollisionTestStatus = String;

// Source: upstream/packages/types/src/Collision.ts:215 (sha256:c0e26b799b9cca4351fc7b7bd786afc6dcbbe50c374efd7cadcd84739f911028)
pub type CollisionSupport2D = std::sync::Arc<
    std::sync::Mutex<Box<dyn FnMut(CollisionShape2D, f64, f64, Vec<f64>) -> () + Send + 'static>>,
>;

// Source: upstream/packages/types/src/Collision.ts:224 (sha256:8df62436be8e066f6b520ef2155aa36cb11392592844a8b3427cdbef2236bdc7)
pub type CollisionPairTest2D = std::sync::Arc<
    std::sync::Mutex<
        Box<
            dyn FnMut(CollisionShape2D, CollisionShape2D, CollisionManifold2D) -> bool
                + Send
                + 'static,
        >,
    >,
>;

// Source: upstream/packages/types/src/Collision.ts:232 (sha256:7cd40d54d6b52c94c1b483afd59b006486bf250f4baca4f3e1f627dc7a2fb6b5)
pub type CollisionTestGuard2D = std::sync::Arc<
    std::sync::Mutex<Box<dyn FnMut(CollisionShape2D, CollisionShape2D) -> () + Send + 'static>>,
>;

// Source: upstream/packages/types/src/Collision.ts:241 (sha256:49d86e4cbb8bd06a2c29ad03a6c6f45088a9596d1f200bb9bb55f07c6842ee10)
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

// Source: upstream/packages/types/src/Collision.ts:267 (sha256:70433e9e6573de517e1e3ff1ea8550a8fbcd0d69578f822f81a548ac128a2cc3)
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

// Source: upstream/packages/types/src/Collision.ts:291 (sha256:a82506f72327f2acc694265db397c142e42518b2308ec255dd576648f17fd2d7)
pub type CollisionShapeKind3D = String;

// Source: upstream/packages/types/src/Collision.ts:302 (sha256:97b0d2e737aa9eb435da9ab94b2e9d1c10469b00f6f07aceb8b65d20c5c598e6)
#[derive(Clone, Default)]
pub struct CollisionSphere3D {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub radius: f64,
}
impl PartialEq for CollisionSphere3D {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Collision.ts:312 (sha256:c4a157a5b40e64bf676c0edc6647c2f417f4c9dfc53def35983e7a7f14b9b178)
#[derive(Clone, Default)]
pub struct CollisionAabb3D {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub min_x: f64,
    pub min_y: f64,
    pub min_z: f64,
    pub max_x: f64,
    pub max_y: f64,
    pub max_z: f64,
}
impl PartialEq for CollisionAabb3D {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Collision.ts:327 (sha256:88b3413243e0e44fdb5467ac1d7b47c2276c549c840f5aa92cbc57a5a73a37ce)
#[derive(Clone, Default)]
pub struct CollisionBox3D {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub half_x: f64,
    pub half_y: f64,
    pub half_z: f64,
    pub rotation_x: f64,
    pub rotation_y: f64,
    pub rotation_z: f64,
    pub rotation_w: f64,
}
impl PartialEq for CollisionBox3D {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Collision.ts:347 (sha256:832d7914d91d24b55ebefb6684b6f582eac5698feb36aaf5c07151d179caaadd)
#[derive(Clone, Default)]
pub struct CollisionCapsule3D {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub x0: f64,
    pub y0: f64,
    pub z0: f64,
    pub x1: f64,
    pub y1: f64,
    pub z1: f64,
    pub radius: f64,
}
impl PartialEq for CollisionCapsule3D {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Collision.ts:363 (sha256:5955133cd4b5c4a039af9cc217b5f93cd79efbd1c373993ebeda8e7717b0dfa9)
#[derive(Clone, Default)]
pub struct CollisionConvex3D {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub points: Vec<f64>,
}
impl PartialEq for CollisionConvex3D {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Collision.ts:377 (sha256:5acebd9e0dcb8df17ab872d75ce1f4b15b1941c21fca600596cfc708ab4bdb8c)
#[derive(Clone, Default)]
pub struct CollisionCylinder3D {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub x0: f64,
    pub y0: f64,
    pub z0: f64,
    pub x1: f64,
    pub y1: f64,
    pub z1: f64,
    pub radius: f64,
}
impl PartialEq for CollisionCylinder3D {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Collision.ts:394 (sha256:9ae2af883f8a77c8057939cc5bc7256a64356b0baf27786f29dfbc52a764cb0f)
#[derive(Clone, Default)]
pub struct CollisionCone3D {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub apex_x: f64,
    pub apex_y: f64,
    pub apex_z: f64,
    pub base_x: f64,
    pub base_y: f64,
    pub base_z: f64,
    pub radius: f64,
}
impl PartialEq for CollisionCone3D {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Collision.ts:404 (sha256:a3ffa0733c9392ee0f767a97c3b33518b88bee6c0113286a3cb31491bd8a4e36)
pub type CollisionBuiltInShape3D = crate::FlightUnion2<
    CollisionBuiltInShape3DRecord7,
    crate::FlightUnion2<
        CollisionBuiltInShape3DRecord6,
        crate::FlightUnion2<
            CollisionBuiltInShape3DRecord5,
            crate::FlightUnion2<
                CollisionBuiltInShape3DRecord4,
                crate::FlightUnion2<
                    CollisionBuiltInShape3DRecord3,
                    crate::FlightUnion2<
                        CollisionBuiltInShape3DRecord2,
                        CollisionBuiltInShape3DRecord1,
                    >,
                >,
            >,
        >,
    >,
>;

// Source: upstream/packages/types/src/Collision.ts:415 (sha256:d109b84b24055de84b6945f18e676d835332ce10dda117969f05e385ac7a7192)
pub type CollisionVendorKind3D = String;

// Source: upstream/packages/types/src/Collision.ts:420 (sha256:eb5e1a36a01f4ece89699f2f6b725d8a8299b6fc44fc2996565b4369a53fb36c)
#[derive(Clone, Default)]
pub struct CollisionVendorShape3D {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: CollisionVendorKind3D,
}
impl PartialEq for CollisionVendorShape3D {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Collision.ts:425 (sha256:7da41d36248b098303759b57e3a8f5e8d53b79aab887af68d30d0abc31abe21e)
pub type CollisionShape3D = crate::FlightUnion2<CollisionBuiltInShape3D, CollisionVendorShape3D>;

// Source: upstream/packages/types/src/Collision.ts:439 (sha256:c38afa5f0accde20c376a1f2b073baa411089b95ea2bd9877c07b15bbd3341a4)
#[derive(Clone, Default)]
pub struct CollisionTriangleMesh3D {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: String,
    pub points: Vec<f64>,
    pub indices: Vec<f64>,
    pub version: f64,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub rotation_x: f64,
    pub rotation_y: f64,
    pub rotation_z: f64,
    pub rotation_w: f64,
}
impl PartialEq for CollisionTriangleMesh3D {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Collision.ts:460 (sha256:3aec475f371ae9e9bb3b85a46f936fa9bd570b695f16dd438ffd270d1e7afa20)
#[derive(Clone, Default)]
pub struct CollisionHeightfield3D {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: String,
    pub columns: f64,
    pub rows: f64,
    pub heights: Vec<f64>,
    pub cell_size_x: f64,
    pub cell_size_z: f64,
    pub version: f64,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub rotation_x: f64,
    pub rotation_y: f64,
    pub rotation_z: f64,
    pub rotation_w: f64,
}
impl PartialEq for CollisionHeightfield3D {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Collision.ts:480 (sha256:1d33b8b84b17e8fd6b8aaad19234ca7d4056d11d0bf206a7574ed6ea1261ca6b)
pub type CollisionStaticShape3D =
    crate::FlightUnion2<CollisionTriangleMesh3D, CollisionHeightfield3D>;

// Source: upstream/packages/types/src/Collision.ts:484 (sha256:911f8028ac6cde0130bfecfec409a02afd88450656e3d9631e9b5ced3d8be07c)
pub type CollisionColliderShape3D =
    crate::FlightUnion2<CollisionBuiltInShape3D, CollisionStaticShape3D>;

// Source: upstream/packages/types/src/Collision.ts:492 (sha256:f29d5c20e1613e7f7d57684d7aaf4ff28d3b9804443b461b1513b2501f7806a2)
#[derive(Clone, Default)]
pub struct CollisionManifold3D {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub overlapping: bool,
    pub normal_x: f64,
    pub normal_y: f64,
    pub normal_z: f64,
    pub depth: f64,
}
impl PartialEq for CollisionManifold3D {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Collision.ts:506 (sha256:175973884560fff51c6a7a6a47511cb94fe83eb695b3b9fffe64a0527bb9d996)
pub type CollisionSupport3D = std::sync::Arc<
    std::sync::Mutex<
        Box<dyn FnMut(CollisionShape3D, f64, f64, f64, Vec<f64>) -> () + Send + 'static>,
    >,
>;

// Source: upstream/packages/types/src/Collision.ts:516 (sha256:b2502a98b79836f90560e5155fc679c9ba3e6e5ae0ba22448b86483442581287)
pub type CollisionPairTest3D = std::sync::Arc<
    std::sync::Mutex<
        Box<
            dyn FnMut(CollisionShape3D, CollisionShape3D, CollisionManifold3D) -> bool
                + Send
                + 'static,
        >,
    >,
>;

// Source: upstream/packages/types/src/Collision.ts:524 (sha256:f3f85bfa61bb506f240932e417664f357bf4b3b1d677ac7500d739a84fac911e)
pub type CollisionTestGuard3D = std::sync::Arc<
    std::sync::Mutex<Box<dyn FnMut(CollisionShape3D, CollisionShape3D) -> () + Send + 'static>>,
>;

// Source: upstream/packages/types/src/Collision.ts:536 (sha256:da2d0ac6bf73f44949108d4f245e6bd958d57bd1afcc46b3d8f5da22af47ffd0)
#[derive(Clone, Default)]
pub struct CollisionTestExplanation3D {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: Option<CollisionShapeKind3D>,
    pub overlapping: bool,
    pub shape_index: Option<f64>,
    pub status: CollisionTestStatus,
}
impl PartialEq for CollisionTestExplanation3D {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Collision.ts:547 (sha256:fb80ca8452263fbe9f5da18da9dcb9f39ffd0468fbb2411d41b7bd655fb69364)
#[derive(Clone, Default)]
pub struct CollisionContactPoint3D {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub depth: f64,
    pub feature_id: f64,
}
impl PartialEq for CollisionContactPoint3D {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Collision.ts:568 (sha256:835b8afa8aa3097c32291f768543e48f807fac234b88dab0966cdbeeba724215)
#[derive(Clone, Default)]
pub struct CollisionContactManifold3D {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub overlapping: bool,
    pub normal_x: f64,
    pub normal_y: f64,
    pub normal_z: f64,
    pub point_count: f64,
    pub points: Vec<CollisionContactPoint3D>,
}
impl PartialEq for CollisionContactManifold3D {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Collision.ts:582 (sha256:96edcf154721a3e5b1cbe2c4ff0008d084659e2e606f8ac5d0c78a639c42c0f6)
pub const MAX_COLLISION_CONTACT_POINTS_3_D: f64 = 4.0_f64;

// Source: upstream/packages/types/src/Collision.ts:597 (sha256:f210058e429603b24fbc0cafe7fc00af470e75c34f60f40d09b468cfa01c6c7e)
pub type CollisionFaceQuery3D = std::sync::Arc<
    std::sync::Mutex<
        Box<dyn FnMut(CollisionShape3D, f64, f64, f64, Vec<f64>) -> f64 + Send + 'static>,
    >,
>;

// Source: upstream/packages/types/src/Collision.ts:608 (sha256:313d80d115c24ce475ba9f9e35963c6355834794f5c31dcef6cfa1fecdd54a54)
#[derive(Clone, Default)]
pub struct CollisionRaycastHit3D {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub fraction: f64,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub normal_x: f64,
    pub normal_y: f64,
    pub normal_z: f64,
}
impl PartialEq for CollisionRaycastHit3D {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Collision.ts:638 (sha256:c77fab1ff523919d7796c826dd61caf2b76f7fad29b3621c23e7208fd2aba9b7)
#[derive(Clone, Default)]
pub struct CollisionDistance3D {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub distance: f64,
    pub direction_x: f64,
    pub direction_y: f64,
    pub direction_z: f64,
    pub point_ax: f64,
    pub point_ay: f64,
    pub point_az: f64,
    pub point_bx: f64,
    pub point_by: f64,
    pub point_bz: f64,
    pub overlapping: bool,
}
impl PartialEq for CollisionDistance3D {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Collision.ts:657 (sha256:d62ed7ba60f93a8e6194a467d97ab10cc86fa86b8f4e2f76a91fc2cf610d50d8)
#[derive(Clone, Default)]
pub struct CollisionTimeOfImpact3D {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub fraction: f64,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub normal_x: f64,
    pub normal_y: f64,
    pub normal_z: f64,
}
impl PartialEq for CollisionTimeOfImpact3D {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
