// @generated from upstream/packages/types/src/WgpuModifierSnippet.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{Modifier, ModifierKind, ModifierSlot, Texture};

// Source: upstream/packages/types/src/WgpuModifierSnippet.ts:5 (sha256:192f2fd408ca96515ce77d9fbd358b6690a2337609bd5cf53a527b35a548bf72)
#[derive(Clone)]
pub struct WgpuModifierCompileContext {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub acquire_texture:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(Texture) -> f64 + Send + 'static>>>,
    pub uniform_base: f64,
}
impl PartialEq for WgpuModifierCompileContext {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/WgpuModifierSnippet.ts:10 (sha256:e52a7cf8c427ff9a90d8e0c32208bd8b5d8c65ae4be7651dfbd8c9e46103b350)
#[derive(Clone, Default)]
pub struct WgpuModifierContribution {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub declarations: Option<String>,
    pub source: String,
}
impl PartialEq for WgpuModifierContribution {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/WgpuModifierSnippet.ts:18 (sha256:98955639ed9320f97def9dfe5884eea880e700e47eb8fe26c8d692053b92a0dd)
#[derive(Clone)]
pub struct WgpuModifierSnippet {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: ModifierKind,
    pub slot: ModifierSlot,
    pub get_define_signature: Option<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(Modifier) -> String + Send + 'static>>>,
    >,
    pub bind: Option<
        std::sync::Arc<
            std::sync::Mutex<Box<dyn FnMut(Modifier, Vec<f32>, f64) -> () + Send + 'static>>,
        >,
    >,
    pub contribution: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(Modifier, f64, WgpuModifierCompileContext) -> WgpuModifierContribution
                    + Send
                    + 'static,
            >,
        >,
    >,
    pub textures: Option<
        std::sync::Arc<
            std::sync::Mutex<
                Box<dyn FnMut(Modifier, Vec<Option<Texture>>, f64) -> f64 + Send + 'static>,
            >,
        >,
    >,
}
impl PartialEq for WgpuModifierSnippet {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
