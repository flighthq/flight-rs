// @generated from upstream/packages/types/src/GizmoState.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    Camera2D, EntityRuntime, HierarchyNodeAny, Rectangle, Scene2D, SelectionState, Signal,
    Vector2Like,
};

// Source: upstream/packages/types/src/GizmoState.ts:12 (sha256:12be252051e0db3079e8fc73e88278da8400023085ef43c7e625cf0a685d8687)
pub type GizmoHandleKind = String;

// Source: upstream/packages/types/src/GizmoState.ts:21 (sha256:8dada7ce519686006a9ef256d812be61911bfcbb4ec6521dd5ce52c8440b8388)
pub type GizmoMode = crate::FlightUnion2<String, GizmoTransformMode>;

// Source: upstream/packages/types/src/GizmoState.ts:23 (sha256:9f2c823593a0d8b4d3ae527352c08aae923510f6a87a35982e9c374eb1c86e9d)
pub type GizmoPivot = String;

// Source: upstream/packages/types/src/GizmoState.ts:25 (sha256:89aed2ed9e3d62878862f8ed06ecba05516c7774f8c0d97219b038c564debaa3)
pub type GizmoSpace = String;

// Source: upstream/packages/types/src/GizmoState.ts:27 (sha256:f1d10050090a889672d879ad540cbb5429fa3f5b45a43c3afbba5acf4deb347a)
pub type GizmoTransformMode = String;

// Source: upstream/packages/types/src/GizmoState.ts:33 (sha256:9347d3ac1c08a2e7971e7df64efd42eed82b5ef1b5080b61f98c1d5a6d3215e6)
#[derive(Clone)]
pub struct GizmoNode2DFeatures<NodeType = HierarchyNodeAny> {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub get_world_bounds_rectangle: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(Rectangle, NodeType) -> bool + Send + 'static>>,
    >,
    pub get_world_origin: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(Vector2Like, NodeType) -> () + Send + 'static>>,
    >,
    pub get_world_rotation:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(NodeType) -> f64 + Send + 'static>>>,
}
impl<NodeType> PartialEq for GizmoNode2DFeatures<NodeType> {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/GizmoState.ts:39 (sha256:059b4da2c6cd4f91a9c6d76c64a80c3bad3606aec0bbe41257688c118fd0b112)
#[derive(Clone)]
pub struct GizmoCreateOptions<NodeType = HierarchyNodeAny> {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub camera: Camera2D,
    pub features: GizmoNode2DFeatures<NodeType>,
    pub overlay_scene: Scene2D,
    pub selection: SelectionState,
    pub viewport_height: f64,
    pub viewport_width: f64,
}
impl<NodeType> PartialEq for GizmoCreateOptions<NodeType> {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/GizmoState.ts:48 (sha256:487dd20b6933595bf566ec0d9fa673dce02a51e07dd2470e6f1cef869ebca7ca)
#[derive(Clone)]
pub struct GizmoSignals {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub on_rotate:
        Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(f64) -> () + Send + 'static>>>>,
    pub on_scale:
        Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(f64, f64) -> () + Send + 'static>>>>,
    pub on_transform_begin:
        Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>>,
    pub on_transform_end:
        Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>>,
    pub on_translate:
        Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(f64, f64) -> () + Send + 'static>>>>,
}
impl PartialEq for GizmoSignals {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/GizmoState.ts:62 (sha256:8c00c0d020f886a9ad031e8c493c5fce029130344e723ee5feac43c7923b52ee)
#[derive(Clone, Default)]
pub struct GizmoState {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
}
impl PartialEq for GizmoState {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for GizmoState {
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
