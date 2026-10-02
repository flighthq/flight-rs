// @generated from upstream/packages/types/src/FlightDocument.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    Camera3D, EntityRuntime, FlightDocumentFields, FlightDocumentInteractiveStateBinding,
    FlightDocumentInteractiveStateTransitionDescriptor, FlightDocumentInteractiveStates,
    FlightDocumentLayoutBinding, FlightDocumentLayoutDescriptor, FlightDocumentResourceDescriptor,
    FlightDocumentToken, Kind, Node2D, Node3D, Scene2D, Scene3D, Scene3DDocumentCamera,
    Scene3DDocumentLight, Scene3DLights,
};

// Source: upstream/packages/types/src/FlightDocument.ts:19 (sha256:c023ecf41d3f76c6d56fe7eb15748e9a92cbaaf07c217da473d8cda51df8b015)
#[derive(Clone, Default)]
pub struct FlightDocumentNode {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub children: Vec<FlightDocumentNode>,
    pub fields: FlightDocumentFields,
    pub interactive_states: Option<FlightDocumentInteractiveStates>,
    pub kind: Kind,
    pub transition: Option<FlightDocumentInteractiveStateTransitionDescriptor>,
}
impl PartialEq for FlightDocumentNode {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/FlightDocument.ts:30 (sha256:e345ad33b3aa539c2b0bfc68f0f8a6cd1b62ebe84ad9681a487d3ad78098f943)
#[derive(Clone, Default)]
pub struct FlightDocument {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub default_scene: f64,
    pub resources: Vec<FlightDocumentResourceDescriptor>,
    pub scenes: Vec<FlightDocumentScene>,
    pub version: f64,
}
impl PartialEq for FlightDocument {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/FlightDocument.ts:39 (sha256:42a79c131ab169deca980ad4f21464983c5279c8ab9664a5dae34dead2428655)
pub type FlightDocumentScene = crate::FlightUnion2<FlightDocumentScene2D, FlightDocumentScene3D>;

// Source: upstream/packages/types/src/FlightDocument.ts:41 (sha256:97cad4b12b6d8cc001fcc6351a0217a4382b8b21faac404807a38f5bc5d87a8b)
#[derive(Clone, Default)]
pub struct FlightDocumentScene2D {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub background_color: Option<f64>,
    pub kind: String,
    pub layouts: Vec<FlightDocumentLayoutDescriptor>,
    pub scene: FlightDocumentNode,
    pub tokens: Vec<FlightDocumentToken>,
}
impl PartialEq for FlightDocumentScene2D {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/FlightDocument.ts:49 (sha256:e3d2c03394ee972677431f1658f1be22e331065bb736bcfba1d2f4b0b6ea5e30)
#[derive(Clone, Default)]
pub struct FlightDocumentScene3D {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub cameras: Vec<Scene3DDocumentCamera>,
    pub kind: String,
    pub layouts: Vec<FlightDocumentLayoutDescriptor>,
    pub lights: Vec<Scene3DDocumentLight>,
    pub scene: FlightDocumentNode,
    pub tokens: Vec<FlightDocumentToken>,
}
impl PartialEq for FlightDocumentScene3D {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/FlightDocument.ts:58 (sha256:d3182897b701651b866541ca36896bdbf2add70be5cd4223d74cd557e4a4bc77)
#[derive(Clone, Default)]
pub struct FlightDocumentScene2DMaterialization {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub interactive_state_bindings: Vec<FlightDocumentInteractiveStateBinding<Node2D>>,
    pub layout_bindings: Vec<FlightDocumentLayoutBinding<Node2D>>,
    pub scene: Scene2D,
}
impl PartialEq for FlightDocumentScene2DMaterialization {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for FlightDocumentScene2DMaterialization {
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

// Source: upstream/packages/types/src/FlightDocument.ts:64 (sha256:5fdbb8299238fe2bbca416401d06adf84a384345d4195ac674e077c67ff39c46)
#[derive(Clone, Default)]
pub struct FlightDocumentScene3DMaterialization {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub cameras: Vec<Camera3D>,
    pub interactive_state_bindings: Vec<FlightDocumentInteractiveStateBinding<Node3D>>,
    pub layout_bindings: Vec<FlightDocumentLayoutBinding<Node3D>>,
    pub lights: Scene3DLights,
    pub scene: Scene3D,
}
impl PartialEq for FlightDocumentScene3DMaterialization {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for FlightDocumentScene3DMaterialization {
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
