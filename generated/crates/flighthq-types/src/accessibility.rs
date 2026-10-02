// @generated from upstream/packages/types/src/Accessibility.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::Rectangle;

// Source: upstream/packages/types/src/Accessibility.ts:10 (sha256:f7f46e1ad1154345bbd9252e407c8735cc7131a7208dbc4abe34711155d025a2)
pub type AccessibilityRole = String;

// Source: upstream/packages/types/src/Accessibility.ts:34 (sha256:1b94d96cd414b45711b780ffeb18475bad999008289fe31bdc528c97db6c2d7c)
pub type AccessibilityLiveness = String;

// Source: upstream/packages/types/src/Accessibility.ts:40 (sha256:1f209c4f7d90191f56d8beee8987f5e88fd79dde04fdb55d2259a7ed5061c8e7)
#[derive(Clone, Default)]
pub struct AccessibilityState {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub disabled: Option<bool>,
    pub checked: Option<bool>,
    pub expanded: Option<bool>,
    pub selected: Option<bool>,
    pub pressed: Option<bool>,
    pub busy: Option<bool>,
    pub hidden: Option<bool>,
    pub readonly: Option<bool>,
    pub required: Option<bool>,
    pub level: Option<f64>,
    pub value_min: Option<f64>,
    pub value_max: Option<f64>,
    pub value_now: Option<f64>,
}
impl PartialEq for AccessibilityState {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Accessibility.ts:61 (sha256:0d54531616bd2ab0cae1a50a1978b2e6307e45e6937724a91c4f5dee64f19703)
#[derive(Clone, Default)]
pub struct AccessibilityNode {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub id: String,
    pub role: AccessibilityRole,
    pub label: Option<String>,
    pub description: Option<String>,
    pub value: Option<String>,
    pub parent_id: Option<String>,
    pub bounds: Option<Rectangle>,
    pub states: Option<AccessibilityState>,
}
impl PartialEq for AccessibilityNode {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Accessibility.ts:72 (sha256:9c8a13b2230f11087b78be2f378826f193c21844eadddc049fd7f414da8f4dcb)
pub type AccessibilityOperationBlockReason = String;

// Source: upstream/packages/types/src/Accessibility.ts:76 (sha256:d8e6710de662b89d9fd4af57497c8735aac9d9bf65a5802350dacead582c9b0d)
#[derive(Clone)]
pub struct AccessibilityOperationOutcome<BlockReason = AccessibilityOperationBlockReason> {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub reason: crate::FlightUnion2<String, BlockReason>,
}
impl<BlockReason> PartialEq for AccessibilityOperationOutcome<BlockReason> {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Accessibility.ts:82 (sha256:4ba96b171635a167bbda03a69404675d57989cbddd798c6110ea8e61bbae69d6)
#[derive(Clone)]
pub struct HostAccessibilityCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub announce: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(String, AccessibilityLiveness) -> AccessibilityOperationOutcome<String>
                    + Send
                    + 'static,
            >,
        >,
    >,
    pub clear: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut() -> AccessibilityOperationOutcome<String> + Send + 'static>,
        >,
    >,
    pub destroy: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
    pub remove_node: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(String) -> AccessibilityOperationOutcome<String> + Send + 'static>,
        >,
    >,
    pub set_focus: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(String) -> AccessibilityOperationOutcome<String> + Send + 'static>,
        >,
    >,
    pub set_node: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(AccessibilityNode) -> AccessibilityOperationOutcome<String>
                    + Send
                    + 'static,
            >,
        >,
    >,
}
impl PartialEq for HostAccessibilityCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
