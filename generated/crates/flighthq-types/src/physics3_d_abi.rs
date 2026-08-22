// @generated from upstream/packages/types/src/Physics3DAbi.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{CollisionBuiltInShape3D, Physics3DQueryFilter, SpatialAabb3D};

#[derive(Clone, Default)]
pub struct SharedStructuralRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub ids: Vec<u32>,
    pub flags: Vec<u32>,
    pub values: Vec<f64>,
    pub count: f64,
    pub required_count: f64,
}
impl PartialEq for SharedStructuralRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Physics3DAbi.ts:12 (sha256:b4ff0e90957ca2842a9d42792c51e7e6254322e7a0967dc3fc650a85333d3476)
pub type Physics3DAbiWorldHandle = f64;

// Source: upstream/packages/types/src/Physics3DAbi.ts:13 (sha256:08d16ffaacd477fee129e301dda7d6155e68a7f168c3ed03e4d1fb2462d631b5)
pub type Physics3DAbiObjectId = f64;

// Source: upstream/packages/types/src/Physics3DAbi.ts:14 (sha256:dace997c1d4195150e08a1aa8b711649144c845390eda1834d2f87dbecb98ae7)
pub type Physics3DAbiWorldStatus = String;

// Source: upstream/packages/types/src/Physics3DAbi.ts:19 (sha256:8a35dc161d0b0b563a22a9816b6e3c724357755d640886f7f21380fc34e3fb12)
pub type Physics3DAbiExecutionStatus = String;

// Source: upstream/packages/types/src/Physics3DAbi.ts:32 (sha256:e232874df593acd78d53e3976d1775517dc020564d8ee4cb42d2c27e4b4c4c36)
#[derive(Clone, Default)]
pub struct Physics3DAbiExecutionResult {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub status: Physics3DAbiExecutionStatus,
    pub command_index: f64,
    pub byte_offset: f64,
    pub command_kind: f64,
}
impl PartialEq for Physics3DAbiExecutionResult {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Physics3DAbi.ts:42 (sha256:344bf545e06f4e1231e04047048a038aa36ce6c322e2dcb7a7952c66f10b1476)
#[derive(Clone, Default)]
pub struct Physics3DAbiCommandBuffer {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub data: Vec<u8>,
    pub byte_length: f64,
    pub command_count: f64,
}
impl PartialEq for Physics3DAbiCommandBuffer {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Physics3DAbi.ts:51 (sha256:12a0644da563b2ee03d54d165a4f82450dc0c2c3f0b91efcf202796d89a86966)
#[derive(Clone, Default)]
pub struct Physics3DAbiBodyBuffer {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub ids: Vec<u32>,
    pub flags: Vec<u32>,
    pub values: Vec<f64>,
    pub count: f64,
    pub required_count: f64,
}
impl PartialEq for Physics3DAbiBodyBuffer {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Physics3DAbi.ts:59 (sha256:7a2bee6b216ffc4a42ce736e4f14a2a3dd5a3ec764e160fb7fbfdd9ef39982cd)
pub type Physics3DAbiContactSelection = String;

// Source: upstream/packages/types/src/Physics3DAbi.ts:64 (sha256:8c8cfb7227ae63c538fd07f3e9812decc564d7e6a15be244c2e15578ebd43c4e)
#[derive(Clone, Default)]
pub struct Physics3DAbiContactBuffer {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub ids: Vec<u32>,
    pub flags: Vec<u32>,
    pub point_starts: Vec<u32>,
    pub point_counts: Vec<u32>,
    pub values: Vec<f64>,
    pub point_feature_ids: Vec<u32>,
    pub point_values: Vec<f64>,
    pub count: f64,
    pub point_count: f64,
    pub required_count: f64,
    pub required_point_count: f64,
}
impl PartialEq for Physics3DAbiContactBuffer {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Physics3DAbi.ts:82 (sha256:30d4834f2a643f5ba36a43b51788ff4522e3ec7c491a352a599d585b0e2a1816)
pub type Physics3DAbiContactHook = std::sync::Arc<
    std::sync::Mutex<Box<dyn FnMut(Physics3DAbiContactBuffer) -> () + Send + 'static>>,
>;

// Source: upstream/packages/types/src/Physics3DAbi.ts:84 (sha256:39fa096a0116e610a50835a085a2034e132dca54573c19203348e8b450e5cdaa)
#[derive(Clone, Default)]
pub struct Physics3DAbiContactHooks {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub buffer: Physics3DAbiContactBuffer,
    pub pre_solve: Option<Physics3DAbiContactHook>,
    pub post_solve: Option<Physics3DAbiContactHook>,
}
impl PartialEq for Physics3DAbiContactHooks {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Physics3DAbi.ts:90 (sha256:9e4330a8ea1482ec1dc1121e6bd6f0400b05c128e2a35e76960a7b3f6246b69d)
pub type Physics3DAbiStepStatus = String;

// Source: upstream/packages/types/src/Physics3DAbi.ts:94 (sha256:fd9990b1ea1d2d95da9a48ffe73617a18a78364b58f571f88d9e86bbaa225d61)
#[derive(Clone, Default)]
pub struct Physics3DAbiJointBuffer {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub ids: Vec<u32>,
    pub flags: Vec<u32>,
    pub values: Vec<f64>,
    pub count: f64,
    pub required_count: f64,
}
impl PartialEq for Physics3DAbiJointBuffer {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Physics3DAbi.ts:105 (sha256:821df23149ae8b3765a926b09e971b64583f169a123723d6d18539453b27ec00)
#[derive(Clone, Default)]
pub struct Physics3DAbiQueryBuffer {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub body_ids: Vec<u32>,
    pub collider_ids: Vec<u32>,
    pub values: Vec<f64>,
    pub count: f64,
    pub required_count: f64,
}
impl PartialEq for Physics3DAbiQueryBuffer {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Physics3DAbi.ts:118 (sha256:2d2ff2058241c552f9fbae4d75e70fdb7cb599cc1758b241875360a016d606bf)
#[derive(Clone)]
pub struct Physics3DAbi {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub version: f64,
    pub capabilities: f64,
    pub create_world: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut() -> Physics3DAbiWorldHandle + Send + 'static>>,
    >,
    pub destroy_world: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(Physics3DAbiWorldHandle) -> bool + Send + 'static>>,
    >,
    pub get_world_status: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(Physics3DAbiWorldHandle) -> Physics3DAbiWorldStatus + Send + 'static>,
        >,
    >,
    pub execute: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        Physics3DAbiWorldHandle,
                        Physics3DAbiCommandBuffer,
                        Physics3DAbiExecutionResult,
                    ) -> bool
                    + Send
                    + 'static,
            >,
        >,
    >,
    pub step: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        Physics3DAbiWorldHandle,
                        f64,
                        Option<Physics3DAbiContactHooks>,
                    ) -> Physics3DAbiStepStatus
                    + Send
                    + 'static,
            >,
        >,
    >,
    pub read_bodies: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(Physics3DAbiWorldHandle, Option<Vec<u32>>, Physics3DAbiBodyBuffer) -> bool
                    + Send
                    + 'static,
            >,
        >,
    >,
    pub read_contacts: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        Physics3DAbiWorldHandle,
                        Physics3DAbiContactSelection,
                        Physics3DAbiContactBuffer,
                    ) -> bool
                    + Send
                    + 'static,
            >,
        >,
    >,
    pub read_joints: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(Physics3DAbiWorldHandle, Physics3DAbiJointBuffer) -> bool
                    + Send
                    + 'static,
            >,
        >,
    >,
    pub query_point: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        Physics3DAbiWorldHandle,
                        f64,
                        f64,
                        f64,
                        Option<Physics3DQueryFilter>,
                        Physics3DAbiQueryBuffer,
                    ) -> bool
                    + Send
                    + 'static,
            >,
        >,
    >,
    pub query_ray: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        Physics3DAbiWorldHandle,
                        f64,
                        f64,
                        f64,
                        f64,
                        f64,
                        f64,
                        f64,
                        bool,
                        Option<Physics3DQueryFilter>,
                        Physics3DAbiQueryBuffer,
                    ) -> bool
                    + Send
                    + 'static,
            >,
        >,
    >,
    pub query_region: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        Physics3DAbiWorldHandle,
                        SpatialAabb3D,
                        Option<Physics3DQueryFilter>,
                        Physics3DAbiQueryBuffer,
                    ) -> bool
                    + Send
                    + 'static,
            >,
        >,
    >,
    pub query_shape_cast: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        Physics3DAbiWorldHandle,
                        CollisionBuiltInShape3D,
                        f64,
                        f64,
                        f64,
                        f64,
                        Option<Physics3DQueryFilter>,
                        Physics3DAbiQueryBuffer,
                    ) -> bool
                    + Send
                    + 'static,
            >,
        >,
    >,
}
impl PartialEq for Physics3DAbi {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
