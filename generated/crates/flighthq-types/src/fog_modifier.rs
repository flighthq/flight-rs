// @generated from upstream/packages/types/src/FogModifier.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{ModifierKind, ModifierSlot};

// Source: upstream/packages/types/src/FogModifier.ts:8 (sha256:1e1de4c8ca4e3fea22e84f838eef2821dee78f471f3c64c85af74672b9c129b5)
#[derive(Clone, Default)]
pub struct FogModifierModeValues {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub exponential: String,
    pub exponential2: String,
    pub linear: String,
}
impl PartialEq for FogModifierModeValues {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub static FOG_MODIFIER_MODE: std::sync::LazyLock<FogModifierModeValues> =
    std::sync::LazyLock::new(|| FogModifierModeValues {
        __flight_identity: std::sync::Arc::new(()),
        exponential: "Exponential".to_owned(),
        exponential2: "Exponential2".to_owned(),
        linear: "Linear".to_owned(),
    });

// Source: upstream/packages/types/src/FogModifier.ts:14 (sha256:3eee72eb00e1cad648b42b296b15485e221d6fe48b3d9b712e53cf8a398e42e8)
pub type FogModifierMode = crate::OpaqueHostValue;

// Source: upstream/packages/types/src/FogModifier.ts:23 (sha256:0ddef0017cb9786dae56bccef54787182c9ff0a31489f925f8ac31bcf61731a4)
#[derive(Clone, Default)]
pub struct FogModifier {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: ModifierKind,
    pub slot: ModifierSlot,
    pub color: f64,
    pub mode: Option<FogModifierMode>,
    pub near: Option<f64>,
    pub far: Option<f64>,
    pub density: Option<f64>,
}
impl PartialEq for FogModifier {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/FogModifier.ts:33 (sha256:f9be10bf3d22f5b02ac325c59b95d2de376e41eea75769877465c6d6634ddd18)
pub const FOG_MODIFIER_KIND: &'static str = "FogModifier";
