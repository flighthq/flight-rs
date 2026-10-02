// @generated from upstream/packages/types/src/Physics3DAbi.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{CollisionBuiltInShape3D, EntityRuntime, Physics3DQueryFilter, SpatialAabb3D};

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

// Source: upstream/packages/types/src/Physics3DAbi.ts:13 (sha256:b4ff0e90957ca2842a9d42792c51e7e6254322e7a0967dc3fc650a85333d3476)
pub type Physics3DAbiWorldHandle = f64;

// Source: upstream/packages/types/src/Physics3DAbi.ts:14 (sha256:08d16ffaacd477fee129e301dda7d6155e68a7f168c3ed03e4d1fb2462d631b5)
pub type Physics3DAbiObjectId = f64;

// Source: upstream/packages/types/src/Physics3DAbi.ts:15 (sha256:dace997c1d4195150e08a1aa8b711649144c845390eda1834d2f87dbecb98ae7)
pub type Physics3DAbiWorldStatus = String;

// Source: upstream/packages/types/src/Physics3DAbi.ts:20 (sha256:8a35dc161d0b0b563a22a9816b6e3c724357755d640886f7f21380fc34e3fb12)
pub type Physics3DAbiExecutionStatus = String;

// Source: upstream/packages/types/src/Physics3DAbi.ts:33 (sha256:96a1d22e37ef013d3c6dcb38547990d4ebd3878986a3f6311cacc11f33dad81b)
#[derive(Clone, Default)]
pub struct Physics3DAbiExecutionResult {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
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
impl crate::FlightEntity for Physics3DAbiExecutionResult {
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

// Source: upstream/packages/types/src/Physics3DAbi.ts:43 (sha256:d0b3a1f3da907b07f9659ede088f6325bc54693623ab241f0853ea421edecbf4)
#[derive(Clone, Default)]
pub struct Physics3DAbiCommandBuffer {
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
impl PartialEq for Physics3DAbiCommandBuffer {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for Physics3DAbiCommandBuffer {
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

// Source: upstream/packages/types/src/Physics3DAbi.ts:52 (sha256:1799b5b55cf2756ac41cce2764df0e01e7c830c62edf31bd35865c070171bd5c)
#[derive(Clone, Default)]
pub struct Physics3DAbiBodyBuffer {
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
impl PartialEq for Physics3DAbiBodyBuffer {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for Physics3DAbiBodyBuffer {
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

// Source: upstream/packages/types/src/Physics3DAbi.ts:60 (sha256:7a2bee6b216ffc4a42ce736e4f14a2a3dd5a3ec764e160fb7fbfdd9ef39982cd)
pub type Physics3DAbiContactSelection = String;

// Source: upstream/packages/types/src/Physics3DAbi.ts:65 (sha256:c6db38779fbb13c76e063635d469b9f96d98fa2608e43e7fb67307553f96f97c)
#[derive(Clone, Default)]
pub struct Physics3DAbiContactBuffer {
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
impl PartialEq for Physics3DAbiContactBuffer {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for Physics3DAbiContactBuffer {
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

// Source: upstream/packages/types/src/Physics3DAbi.ts:83 (sha256:30d4834f2a643f5ba36a43b51788ff4522e3ec7c491a352a599d585b0e2a1816)
pub type Physics3DAbiContactHook = std::sync::Arc<
    std::sync::Mutex<Box<dyn FnMut(Physics3DAbiContactBuffer) -> () + Send + 'static>>,
>;

// Source: upstream/packages/types/src/Physics3DAbi.ts:85 (sha256:39fa096a0116e610a50835a085a2034e132dca54573c19203348e8b450e5cdaa)
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

// Source: upstream/packages/types/src/Physics3DAbi.ts:91 (sha256:9e4330a8ea1482ec1dc1121e6bd6f0400b05c128e2a35e76960a7b3f6246b69d)
pub type Physics3DAbiStepStatus = String;

// Source: upstream/packages/types/src/Physics3DAbi.ts:95 (sha256:f1f6e80e54e076566cba9628e5ca3d6e4e24156a364ced7ae4a2bd4c1d799e28)
#[derive(Clone, Default)]
pub struct Physics3DAbiJointBuffer {
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
impl PartialEq for Physics3DAbiJointBuffer {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for Physics3DAbiJointBuffer {
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

// Source: upstream/packages/types/src/Physics3DAbi.ts:106 (sha256:42a6467ee59dd773445e10845df24bb60a73867bbfea2913782c8696a6881bdf)
#[derive(Clone, Default)]
pub struct Physics3DAbiQueryBuffer {
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
impl PartialEq for Physics3DAbiQueryBuffer {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for Physics3DAbiQueryBuffer {
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

// Source: upstream/packages/types/src/Physics3DAbi.ts:119 (sha256:f2c6c94060fe6dd977589671d21bfc3ebf0f966033eb6ff91adfced06b5a03d9)
#[derive(Clone)]
pub struct Physics3DAbi {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
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
impl crate::FlightEntity for Physics3DAbi {
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
