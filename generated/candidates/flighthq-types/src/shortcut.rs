// @generated from upstream/packages/types/src/Shortcut.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/Shortcut.ts:6 (sha256:b9df7d16c8855374dfa5e55d916aa9c1392625dd0f070ff9421d5d322fbd3160)
#[derive(Clone)]
pub struct ShortcutBackend {
    pub get_registered: crate::OpaqueHostValue,
    pub is_registered: crate::OpaqueHostValue,
    pub register: crate::OpaqueHostValue,
    pub set_all_enabled: crate::OpaqueHostValue,
    pub set_enabled: crate::OpaqueHostValue,
    pub unregister: crate::OpaqueHostValue,
    pub unregister_all: crate::OpaqueHostValue,
}
