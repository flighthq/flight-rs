// @generated from upstream/packages/types/src/NodeInteractiveStateBinding.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{EntityRuntime, FlightDocumentFields, NodeAny};

// Source: upstream/packages/types/src/NodeInteractiveStateBinding.ts:7 (sha256:a99683fb69dc750f515abbf2191cd700e081d5315d1771fc5c5f5bfc59e1ea4c)
#[derive(Clone, Default)]
pub struct NodeInteractiveStateBinding {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
}
impl PartialEq for NodeInteractiveStateBinding {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for NodeInteractiveStateBinding {
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

// Source: upstream/packages/types/src/NodeInteractiveStateBinding.ts:11 (sha256:02baa9cf7901f5b0c9e02f621e63592dbfb26a048815d6c7438661f7750e7458)
pub type NodeInteractiveStateBindingRuntime = crate::EntityRuntime;

// Source: upstream/packages/types/src/NodeInteractiveStateBinding.ts:15 (sha256:9f7044cff8804875865b2f31c1eefbb73a546254a59e7690aeaf1a8bf7253a50)
#[derive(Clone, Default)]
pub struct NodeInteractiveStateFlags {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub disabled: bool,
    pub hovered: bool,
    pub pressed: bool,
}
impl PartialEq for NodeInteractiveStateFlags {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/NodeInteractiveStateBinding.ts:21 (sha256:ef275e3f1c84a54f78c2ab9844691d14207c0705485c63a78ef5bfd8ddaa3a1a)
pub type NodeInteractiveStateProperty = String;

// Source: upstream/packages/types/src/NodeInteractiveStateBinding.ts:23 (sha256:5b27db21ae754771c1ed6b94ba21ee01de51e0cedda2e748b1c948c551c89f53)
pub type NodeInteractiveStateTransitionValue = crate::FlightUnion2<bool, f64>;

// Source: upstream/packages/types/src/NodeInteractiveStateBinding.ts:25 (sha256:465a69ab2c64f1ca10fc3c4263c597753e71c79ad0fb93e45b6f103a51d5111d)
#[derive(Clone)]
pub struct NodeInteractiveStateTransitionRequest<N = NodeAny, P = String> {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub apply: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(Option<NodeInteractiveStateTransitionValue>) -> () + Send + 'static>,
        >,
    >,
    pub from: NodeInteractiveStateTransitionValue,
    pub property: P,
    pub target: N,
    pub value: NodeInteractiveStateTransitionValue,
}
impl<N, P> PartialEq for NodeInteractiveStateTransitionRequest<N, P> {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/NodeInteractiveStateBinding.ts:33 (sha256:8dee218f0a79d89db605c74e2b9fd005f2b9c0490d93f2a56c0398ea74bc69fe)
#[derive(Clone)]
pub struct NodeInteractiveStateTransition<N = NodeAny, P = String> {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub run: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(NodeInteractiveStateTransitionRequest<N, P>) -> () + Send + 'static>,
        >,
    >,
}
impl<N, P> PartialEq for NodeInteractiveStateTransition<N, P> {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/NodeInteractiveStateBinding.ts:37 (sha256:fa360f899847fba3705b3601cfcf395689ba9e135fd315f4e5a22c0d67d4d49b)
#[derive(Clone)]
pub struct NodeInteractiveStateExtensionRuntime {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub apply: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        FlightDocumentFields,
                        Option<
                            NodeInteractiveStateTransition<
                                crate::OpaqueHostValue,
                                crate::OpaqueHostValue,
                            >,
                        >,
                    ) -> bool
                    + Send
                    + 'static,
            >,
        >,
    >,
    pub capture: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(FlightDocumentFields) -> bool + Send + 'static>>,
    >,
    pub dispose: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
}
impl PartialEq for NodeInteractiveStateExtensionRuntime {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/NodeInteractiveStateBinding.ts:46 (sha256:df286bdbe68d3fd415c1afd9948ec391cd0f5724e86b4835b329a4edb03bd44d)
#[derive(Clone, Default)]
pub struct NodeInteractiveStateRefusalReasonValues {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub extension_creation_failed: String,
    pub extension_kind_unregistered: String,
    pub extension_target_unsupported: String,
    pub property_target_unsupported: String,
    pub transition_creation_failed: String,
    pub transition_kind_unregistered: String,
}
impl PartialEq for NodeInteractiveStateRefusalReasonValues {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub static NODE_INTERACTIVE_STATE_REFUSAL_REASON: std::sync::LazyLock<
    NodeInteractiveStateRefusalReasonValues,
> = std::sync::LazyLock::new(|| NodeInteractiveStateRefusalReasonValues {
    __flight_identity: std::sync::Arc::new(()),
    extension_creation_failed: "node-interactive-state.extension.creation-failed".to_owned(),
    extension_kind_unregistered: "node-interactive-state.extension-kind.unregistered".to_owned(),
    extension_target_unsupported: "node-interactive-state.extension.target-unsupported".to_owned(),
    property_target_unsupported: "node-interactive-state.property.target-unsupported".to_owned(),
    transition_creation_failed: "node-interactive-state.transition.creation-failed".to_owned(),
    transition_kind_unregistered: "node-interactive-state.transition-kind.unregistered".to_owned(),
});

// Source: upstream/packages/types/src/NodeInteractiveStateBinding.ts:55 (sha256:2a5a9616b7d736c77bf1e73b48b0a5405d6e83d6a2bc41cb3a44e7a78642b799)
pub type NodeInteractiveStateRefusalReason = String;

// Source: upstream/packages/types/src/NodeInteractiveStateBinding.ts:58 (sha256:c8a9afae6f277b1de9c689677e6262f020c8c9d733dfc04d2218b2add2cac560)
#[derive(Clone, Default)]
pub struct NodeInteractiveStateExplanation {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: String,
    pub property: Option<NodeInteractiveStateProperty>,
    pub reason: NodeInteractiveStateRefusalReason,
}
impl PartialEq for NodeInteractiveStateExplanation {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
