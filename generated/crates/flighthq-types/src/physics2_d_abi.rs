// @generated from upstream/packages/types/src/Physics2DAbi.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{CollisionBuiltInShape2D, Physics2DQueryFilter, SpatialAabb2D};

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

// Source: upstream/packages/types/src/Physics2DAbi.ts:12 (sha256:d4e85718f9dab8d27043c193ce4601e8ac8eccc9329ebdae9775648e750d9753)
pub type Physics2DAbiWorldHandle = f64;

// Source: upstream/packages/types/src/Physics2DAbi.ts:13 (sha256:57a668035af950c0d71d30ffa0fc6d44f2461777a8605d2408f70d38f65cfd04)
pub type Physics2DAbiObjectId = f64;

// Source: upstream/packages/types/src/Physics2DAbi.ts:14 (sha256:7e138e00ecf3c592d88b187496efb86118fa33544caec1945dc247b260f5969f)
pub type Physics2DAbiWorldStatus = String;

// Source: upstream/packages/types/src/Physics2DAbi.ts:19 (sha256:5c721d35644da1331fc41a8480d413b6823ef390a505db06a23653d010c8bb09)
pub type Physics2DAbiExecutionStatus = String;

// Source: upstream/packages/types/src/Physics2DAbi.ts:32 (sha256:694e6e92e6356788cdd9ed664246aacb4da5eb2a4fd63bead18cd2d665aba3b3)
#[derive(Clone, Default)]
pub struct Physics2DAbiExecutionResult {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub status: Physics2DAbiExecutionStatus,
    pub command_index: f64,
    pub byte_offset: f64,
    pub command_kind: f64,
}
impl PartialEq for Physics2DAbiExecutionResult {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Physics2DAbi.ts:42 (sha256:4b5b052c60168ea9fa5348a0db343c47e0bc7c899fe07a51cb9d901b9851bacc)
#[derive(Clone, Default)]
pub struct Physics2DAbiCommandBuffer {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub data: Vec<u8>,
    pub byte_length: f64,
    pub command_count: f64,
}
impl PartialEq for Physics2DAbiCommandBuffer {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Physics2DAbi.ts:51 (sha256:b517633674b610b40f2c8b88d4e0c13eafbc18e480c256ecdd2c0eed6878df32)
#[derive(Clone, Default)]
pub struct Physics2DAbiBodyBuffer {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub ids: Vec<u32>,
    pub flags: Vec<u32>,
    pub values: Vec<f64>,
    pub count: f64,
    pub required_count: f64,
}
impl PartialEq for Physics2DAbiBodyBuffer {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Physics2DAbi.ts:59 (sha256:837ac63f79d1a06b9d5f1f687c1cc0c15ef34f40128a852cf3b8417a7a102d9d)
pub type Physics2DAbiContactSelection = String;

// Source: upstream/packages/types/src/Physics2DAbi.ts:64 (sha256:d61379047830a9de7adb05742792e4d5352c070bb1ee07967f7e31b2b10b395c)
#[derive(Clone, Default)]
pub struct Physics2DAbiContactBuffer {
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
impl PartialEq for Physics2DAbiContactBuffer {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Physics2DAbi.ts:83 (sha256:8dd0e616ed82cdc04be578219957c3666e2dbe521c8c3b77a6bbaeb267a77c51)
pub type Physics2DAbiContactHook = std::sync::Arc<
    std::sync::Mutex<Box<dyn FnMut(Physics2DAbiContactBuffer) -> () + Send + 'static>>,
>;

// Source: upstream/packages/types/src/Physics2DAbi.ts:85 (sha256:dc43167fc8a8bad5290a292de18f01a642d9266a4e8d4e479ad1d8578e26d757)
#[derive(Clone, Default)]
pub struct Physics2DAbiContactHooks {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub buffer: Physics2DAbiContactBuffer,
    pub pre_solve: Option<Physics2DAbiContactHook>,
    pub post_solve: Option<Physics2DAbiContactHook>,
}
impl PartialEq for Physics2DAbiContactHooks {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Physics2DAbi.ts:91 (sha256:de5d6a409e3f8d075b46130ff3d80cc2ea249da7e7c85d4cf98ff9c450a00aa6)
pub type Physics2DAbiStepStatus = String;

// Source: upstream/packages/types/src/Physics2DAbi.ts:96 (sha256:a5b85978cae503eb71dd7e867bc8b33dbc7679ba22e13bda78dba875330ab545)
#[derive(Clone, Default)]
pub struct Physics2DAbiJointBuffer {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub ids: Vec<u32>,
    pub flags: Vec<u32>,
    pub values: Vec<f64>,
    pub count: f64,
    pub required_count: f64,
}
impl PartialEq for Physics2DAbiJointBuffer {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Physics2DAbi.ts:107 (sha256:febf49cb0ba93ee5a5b05c607690977491c4d73b35e09f4d188981d820d4a32b)
#[derive(Clone, Default)]
pub struct Physics2DAbiQueryBuffer {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub body_ids: Vec<u32>,
    pub collider_ids: Vec<u32>,
    pub values: Vec<f64>,
    pub count: f64,
    pub required_count: f64,
}
impl PartialEq for Physics2DAbiQueryBuffer {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Physics2DAbi.ts:120 (sha256:24f0ff42ac4f16ce847e7d3215db8470df7d58f35ed3e22674715c5a14d26409)
#[derive(Clone)]
pub struct Physics2DAbi {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub version: f64,
    pub capabilities: f64,
    pub create_world: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut() -> Physics2DAbiWorldHandle + Send + 'static>>,
    >,
    pub destroy_world: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(Physics2DAbiWorldHandle) -> bool + Send + 'static>>,
    >,
    pub get_world_status: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(Physics2DAbiWorldHandle) -> Physics2DAbiWorldStatus + Send + 'static>,
        >,
    >,
    pub execute: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        Physics2DAbiWorldHandle,
                        Physics2DAbiCommandBuffer,
                        Physics2DAbiExecutionResult,
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
                        Physics2DAbiWorldHandle,
                        f64,
                        Option<Physics2DAbiContactHooks>,
                    ) -> Physics2DAbiStepStatus
                    + Send
                    + 'static,
            >,
        >,
    >,
    pub read_bodies: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(Physics2DAbiWorldHandle, Option<Vec<u32>>, Physics2DAbiBodyBuffer) -> bool
                    + Send
                    + 'static,
            >,
        >,
    >,
    pub read_contacts: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        Physics2DAbiWorldHandle,
                        Physics2DAbiContactSelection,
                        Physics2DAbiContactBuffer,
                    ) -> bool
                    + Send
                    + 'static,
            >,
        >,
    >,
    pub read_joints: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(Physics2DAbiWorldHandle, Physics2DAbiJointBuffer) -> bool
                    + Send
                    + 'static,
            >,
        >,
    >,
    pub query_point: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        Physics2DAbiWorldHandle,
                        f64,
                        f64,
                        Option<Physics2DQueryFilter>,
                        Physics2DAbiQueryBuffer,
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
                        Physics2DAbiWorldHandle,
                        f64,
                        f64,
                        f64,
                        f64,
                        f64,
                        bool,
                        Option<Physics2DQueryFilter>,
                        Physics2DAbiQueryBuffer,
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
                        Physics2DAbiWorldHandle,
                        SpatialAabb2D,
                        Option<Physics2DQueryFilter>,
                        Physics2DAbiQueryBuffer,
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
                        Physics2DAbiWorldHandle,
                        CollisionBuiltInShape2D,
                        f64,
                        f64,
                        f64,
                        Option<Physics2DQueryFilter>,
                        Physics2DAbiQueryBuffer,
                    ) -> bool
                    + Send
                    + 'static,
            >,
        >,
    >,
}
impl PartialEq for Physics2DAbi {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
