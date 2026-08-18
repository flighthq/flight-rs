// @generated from upstream/packages/types/src/GlModifierSnippet.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{GlRenderState, Modifier, ModifierKind, ModifierSlot};

// Source: upstream/packages/types/src/GlModifierSnippet.ts:13 (sha256:c55c0fbae15cde96e2ce51f2d43151c3936ac8108b82ff046a55ee7c33ff316a)
#[derive(Clone)]
pub struct GlModifierBindContext {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub acquire_modifier_texture_unit:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> f64 + Send + 'static>>>,
    pub index: f64,
    pub program: crate::OpaqueHostValue,
    pub state: GlRenderState,
}
impl PartialEq for GlModifierBindContext {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/GlModifierSnippet.ts:33 (sha256:5b8fa686689304cc7c5bd81ab2388af3da8b5d13c0ad9e562b57b35d54c4a8a9)
#[derive(Clone)]
pub struct GlModifierSnippet {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: ModifierKind,
    pub slot: ModifierSlot,
    pub get_define_signature: Option<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(Modifier) -> String + Send + 'static>>>,
    >,
    pub bind: Option<
        std::sync::Arc<
            std::sync::Mutex<
                Box<dyn FnMut(Modifier, GlModifierBindContext) -> () + Send + 'static>,
            >,
        >,
    >,
    pub contribution:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(Modifier, f64) -> String + Send + 'static>>>,
    pub declarations: Option<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(Modifier, f64) -> String + Send + 'static>>>,
    >,
}
impl PartialEq for GlModifierSnippet {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
