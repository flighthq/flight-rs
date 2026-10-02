// @generated from upstream/packages/types/src/FlightDocumentInteractiveState.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{FlightDocumentFields, Kind, NodeAny, NodeInteractiveStateProperty};

#[derive(Clone, Default)]
pub struct SharedStructuralRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub fields: FlightDocumentFields,
    pub kind: Kind,
}
impl PartialEq for SharedStructuralRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/FlightDocumentInteractiveState.ts:6 (sha256:4c2979e867c3ecf19002a819b284df0f8adb9c17c151e3edb6cf1e6258e201d5)
pub type FlightDocumentInteractiveStateProperty = NodeInteractiveStateProperty;

// Source: upstream/packages/types/src/FlightDocumentInteractiveState.ts:8 (sha256:c27afed198233d352d2c0afff74c43307ea6af7e8e180b85e426f4939fb71f70)
pub type FlightDocumentInteractiveStateValue = crate::FlightUnion2<bool, f64>;

// Source: upstream/packages/types/src/FlightDocumentInteractiveState.ts:10 (sha256:1bb8b99b933a2720970661005a1f8dfd33f5f9cd0acdf37716c4346d84185c0b)
#[derive(Clone, Default)]
pub struct FlightDocumentInteractiveStateExtensionDescriptor {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub fields: FlightDocumentFields,
    pub kind: Kind,
}
impl PartialEq for FlightDocumentInteractiveStateExtensionDescriptor {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/FlightDocumentInteractiveState.ts:15 (sha256:0ad3417dfbe09c39d2d81688a05a43676f53bb8e12ba2d5cfd68da42781bd6ce)
#[derive(Clone, Default)]
pub struct FlightDocumentInteractiveState {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub alpha: Option<f64>,
    pub extensions: Vec<FlightDocumentInteractiveStateExtensionDescriptor>,
    pub scale_x: Option<f64>,
    pub scale_y: Option<f64>,
    pub visible: Option<bool>,
    pub x: Option<f64>,
    pub y: Option<f64>,
}
impl PartialEq for FlightDocumentInteractiveState {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/FlightDocumentInteractiveState.ts:25 (sha256:7d1468b1aa9a60ccc11dcc955b95027b0fcc3b661ccd32a431e87b96cdcdfbb8)
#[derive(Clone, Default)]
pub struct FlightDocumentInteractiveStates {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub disabled: Option<FlightDocumentInteractiveState>,
    pub hover: Option<FlightDocumentInteractiveState>,
    pub pressed: Option<FlightDocumentInteractiveState>,
}
impl PartialEq for FlightDocumentInteractiveStates {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/FlightDocumentInteractiveState.ts:31 (sha256:5d8e653174ffcfc78fc55092af63d28f58294fc7c40969ee8deea16e745bb895)
#[derive(Clone, Default)]
pub struct FlightDocumentInteractiveStateTransitionDescriptor {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub fields: FlightDocumentFields,
    pub kind: Kind,
}
impl PartialEq for FlightDocumentInteractiveStateTransitionDescriptor {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/FlightDocumentInteractiveState.ts:38 (sha256:46f96fe08f2ea7eda9f205edf095021145647824f6d68cc7b8576286df5b8643)
#[derive(Clone)]
pub struct FlightDocumentInteractiveStateBinding<N = NodeAny> {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub interactive_states: FlightDocumentInteractiveStates,
    pub node: N,
    pub transition: Option<FlightDocumentInteractiveStateTransitionDescriptor>,
}
impl<N> PartialEq for FlightDocumentInteractiveStateBinding<N> {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
