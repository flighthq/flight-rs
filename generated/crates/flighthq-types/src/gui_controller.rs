// @generated from upstream/packages/types/src/GuiController.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    Node2D, NodeInteractiveStateProperty, NodeInteractiveStateTransition,
    NodeInteractiveStateTransitionRequest, NodeInteractiveStateTransitionValue,
};

// Source: upstream/packages/types/src/GuiController.ts:9 (sha256:29b24b1c4504facc36195902f90b805bf0a87d6796080632a96c8efbdd6033e3)
pub type GuiOrientation = String;

// Source: upstream/packages/types/src/GuiController.ts:11 (sha256:abd6d67d1411ab0b5d11c43797d15f4cd4b4fa8f86dbcb0100cf1b56560da28b)
pub type GuiTransitionProperty = NodeInteractiveStateProperty;

// Source: upstream/packages/types/src/GuiController.ts:13 (sha256:27b39f83ac931e87da34bcdf4e3463312cf41403580de5a5bdfb4456378bf250)
pub type GuiTransitionValue = NodeInteractiveStateTransitionValue;

// Source: upstream/packages/types/src/GuiController.ts:19 (sha256:e2139f89b39cc3f86ec0be2f7e751e1fcec11556f8b59832fa02b6b5c07c1cbb)
pub type GuiTransitionRequest =
    NodeInteractiveStateTransitionRequest<Node2D, GuiTransitionProperty>;

// Source: upstream/packages/types/src/GuiController.ts:21 (sha256:c8a1a1106d9e27f7bd91f66ca79474817c939a870a59801b881aaba836b09ece)
pub type GuiTransitionDescriptor = NodeInteractiveStateTransition<Node2D, GuiTransitionProperty>;

// Source: upstream/packages/types/src/GuiController.ts:23 (sha256:df8009b6952d1a3dc5177b22bbc3307df7826d7f76e70198281bf40a98004e0e)
#[derive(Clone, Default)]
pub struct GuiControllerOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub transition: Option<GuiTransitionDescriptor>,
}
impl PartialEq for GuiControllerOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
