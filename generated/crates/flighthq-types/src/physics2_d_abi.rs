// @generated from upstream/packages/types/src/Physics2DAbi.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{CollisionBuiltInShape2D, EntityRuntime, Physics2DQueryFilter, SpatialAabb2D};

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

// Source: upstream/packages/types/src/Physics2DAbi.ts:13 (sha256:d4e85718f9dab8d27043c193ce4601e8ac8eccc9329ebdae9775648e750d9753)
pub type Physics2DAbiWorldHandle = f64;

// Source: upstream/packages/types/src/Physics2DAbi.ts:14 (sha256:57a668035af950c0d71d30ffa0fc6d44f2461777a8605d2408f70d38f65cfd04)
pub type Physics2DAbiObjectId = f64;

// Source: upstream/packages/types/src/Physics2DAbi.ts:15 (sha256:7e138e00ecf3c592d88b187496efb86118fa33544caec1945dc247b260f5969f)
pub type Physics2DAbiWorldStatus = String;

// Source: upstream/packages/types/src/Physics2DAbi.ts:20 (sha256:5c721d35644da1331fc41a8480d413b6823ef390a505db06a23653d010c8bb09)
pub type Physics2DAbiExecutionStatus = String;

// Source: upstream/packages/types/src/Physics2DAbi.ts:33 (sha256:2ef3853f4145da38282c5794e1bcf1795df67228aa600cb0bcbc249f313e28fd)
#[derive(Clone, Default)]
pub struct Physics2DAbiExecutionResult {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
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
impl crate::FlightEntity for Physics2DAbiExecutionResult {
    fn __flight_entity_runtime(
        &self,
    ) -> &std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>> {
        &self.__flight_entity_runtime
    }
    fn __flight_entity_snapshot(&self) -> &Option<std::sync::Arc<dyn std::any::Any + Send + Sync>> {
        &self.__flight_entity_snapshot
    }
    fn __flight_fresh_clone(&self) -> Self {
        let mut cloned = self.clone();
        cloned.__flight_identity = std::sync::Arc::new(());
        cloned.__flight_entity_runtime = std::sync::Arc::new(std::sync::Mutex::new(
            self.__flight_entity_runtime.lock().unwrap().clone(),
        ));
        cloned
    }
}

// Source: upstream/packages/types/src/Physics2DAbi.ts:43 (sha256:cb0af39432dbcae2f8c4146a92ae4b29f55ab7219f519c96bbb5f18993fa16ca)
#[derive(Clone, Default)]
pub struct Physics2DAbiCommandBuffer {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub data: Vec<u8>,
    pub byte_length: f64,
    pub command_count: f64,
}
impl PartialEq for Physics2DAbiCommandBuffer {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for Physics2DAbiCommandBuffer {
    fn __flight_entity_runtime(
        &self,
    ) -> &std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>> {
        &self.__flight_entity_runtime
    }
    fn __flight_entity_snapshot(&self) -> &Option<std::sync::Arc<dyn std::any::Any + Send + Sync>> {
        &self.__flight_entity_snapshot
    }
    fn __flight_fresh_clone(&self) -> Self {
        let mut cloned = self.clone();
        cloned.__flight_identity = std::sync::Arc::new(());
        cloned.__flight_entity_runtime = std::sync::Arc::new(std::sync::Mutex::new(
            self.__flight_entity_runtime.lock().unwrap().clone(),
        ));
        cloned
    }
}

// Source: upstream/packages/types/src/Physics2DAbi.ts:52 (sha256:cbd4207cdfa5d86e7b72c15e8ae6042f57570025e4d71e5f1b220f7cdc88418a)
#[derive(Clone, Default)]
pub struct Physics2DAbiBodyBuffer {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
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
impl crate::FlightEntity for Physics2DAbiBodyBuffer {
    fn __flight_entity_runtime(
        &self,
    ) -> &std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>> {
        &self.__flight_entity_runtime
    }
    fn __flight_entity_snapshot(&self) -> &Option<std::sync::Arc<dyn std::any::Any + Send + Sync>> {
        &self.__flight_entity_snapshot
    }
    fn __flight_fresh_clone(&self) -> Self {
        let mut cloned = self.clone();
        cloned.__flight_identity = std::sync::Arc::new(());
        cloned.__flight_entity_runtime = std::sync::Arc::new(std::sync::Mutex::new(
            self.__flight_entity_runtime.lock().unwrap().clone(),
        ));
        cloned
    }
}

// Source: upstream/packages/types/src/Physics2DAbi.ts:60 (sha256:837ac63f79d1a06b9d5f1f687c1cc0c15ef34f40128a852cf3b8417a7a102d9d)
pub type Physics2DAbiContactSelection = String;

// Source: upstream/packages/types/src/Physics2DAbi.ts:65 (sha256:7c27442a47831e595f65343541933ea8de1cdda5268fc16c7056c987d021de87)
#[derive(Clone, Default)]
pub struct Physics2DAbiContactBuffer {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
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
impl crate::FlightEntity for Physics2DAbiContactBuffer {
    fn __flight_entity_runtime(
        &self,
    ) -> &std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>> {
        &self.__flight_entity_runtime
    }
    fn __flight_entity_snapshot(&self) -> &Option<std::sync::Arc<dyn std::any::Any + Send + Sync>> {
        &self.__flight_entity_snapshot
    }
    fn __flight_fresh_clone(&self) -> Self {
        let mut cloned = self.clone();
        cloned.__flight_identity = std::sync::Arc::new(());
        cloned.__flight_entity_runtime = std::sync::Arc::new(std::sync::Mutex::new(
            self.__flight_entity_runtime.lock().unwrap().clone(),
        ));
        cloned
    }
}

// Source: upstream/packages/types/src/Physics2DAbi.ts:84 (sha256:8dd0e616ed82cdc04be578219957c3666e2dbe521c8c3b77a6bbaeb267a77c51)
pub type Physics2DAbiContactHook = std::sync::Arc<
    std::sync::Mutex<Box<dyn FnMut(Physics2DAbiContactBuffer) -> () + Send + 'static>>,
>;

// Source: upstream/packages/types/src/Physics2DAbi.ts:86 (sha256:dc43167fc8a8bad5290a292de18f01a642d9266a4e8d4e479ad1d8578e26d757)
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

// Source: upstream/packages/types/src/Physics2DAbi.ts:92 (sha256:de5d6a409e3f8d075b46130ff3d80cc2ea249da7e7c85d4cf98ff9c450a00aa6)
pub type Physics2DAbiStepStatus = String;

// Source: upstream/packages/types/src/Physics2DAbi.ts:97 (sha256:bc71c51010860a2a427ceedfb3dc3c4829a7aa0fc780752140b7924d9a5eb02e)
#[derive(Clone, Default)]
pub struct Physics2DAbiJointBuffer {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
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
impl crate::FlightEntity for Physics2DAbiJointBuffer {
    fn __flight_entity_runtime(
        &self,
    ) -> &std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>> {
        &self.__flight_entity_runtime
    }
    fn __flight_entity_snapshot(&self) -> &Option<std::sync::Arc<dyn std::any::Any + Send + Sync>> {
        &self.__flight_entity_snapshot
    }
    fn __flight_fresh_clone(&self) -> Self {
        let mut cloned = self.clone();
        cloned.__flight_identity = std::sync::Arc::new(());
        cloned.__flight_entity_runtime = std::sync::Arc::new(std::sync::Mutex::new(
            self.__flight_entity_runtime.lock().unwrap().clone(),
        ));
        cloned
    }
}

// Source: upstream/packages/types/src/Physics2DAbi.ts:108 (sha256:9430ea55e4a199d4be705589cadf0cc342f320648c91b4ba12ee2f3741e1cb91)
#[derive(Clone, Default)]
pub struct Physics2DAbiQueryBuffer {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
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
impl crate::FlightEntity for Physics2DAbiQueryBuffer {
    fn __flight_entity_runtime(
        &self,
    ) -> &std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>> {
        &self.__flight_entity_runtime
    }
    fn __flight_entity_snapshot(&self) -> &Option<std::sync::Arc<dyn std::any::Any + Send + Sync>> {
        &self.__flight_entity_snapshot
    }
    fn __flight_fresh_clone(&self) -> Self {
        let mut cloned = self.clone();
        cloned.__flight_identity = std::sync::Arc::new(());
        cloned.__flight_entity_runtime = std::sync::Arc::new(std::sync::Mutex::new(
            self.__flight_entity_runtime.lock().unwrap().clone(),
        ));
        cloned
    }
}

// Source: upstream/packages/types/src/Physics2DAbi.ts:121 (sha256:f21b3d402809e889360f9206ac28abcb33de6ba3bf69eeebc91ed85974e585bb)
#[derive(Clone)]
pub struct Physics2DAbi {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
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
impl crate::FlightEntity for Physics2DAbi {
    fn __flight_entity_runtime(
        &self,
    ) -> &std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>> {
        &self.__flight_entity_runtime
    }
    fn __flight_entity_snapshot(&self) -> &Option<std::sync::Arc<dyn std::any::Any + Send + Sync>> {
        &self.__flight_entity_snapshot
    }
    fn __flight_fresh_clone(&self) -> Self {
        let mut cloned = self.clone();
        cloned.__flight_identity = std::sync::Arc::new(());
        cloned.__flight_entity_runtime = std::sync::Arc::new(std::sync::Mutex::new(
            self.__flight_entity_runtime.lock().unwrap().clone(),
        ));
        cloned
    }
}
