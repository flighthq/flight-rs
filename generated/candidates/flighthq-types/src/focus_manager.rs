// @generated from upstream/packages/types/src/FocusManager.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/FocusManager.ts:4 (sha256:b6364ae797462e127ef510c7f5ae8e457e58e9220be2670bc2323ea432914101)
pub type FocusDirection = String;

// Source: upstream/packages/types/src/FocusManager.ts:19 (sha256:9243635aa70272176f5cfbfa387824f2d148defb705b2d56775c35a40101d7bd)
#[derive(Clone)]
pub struct FocusManager {
    pub focused: Option<crate::OpaqueHostValue>,
    pub root: crate::OpaqueHostValue,
    pub wrap: bool,
}

// Source: upstream/packages/types/src/FocusManager.ts:25 (sha256:f7ba8b429aae2588af29d1706a7960c1210c571898c5e9aee9b3644104e82811)
#[derive(Clone)]
pub struct FocusManagerOptions {
    pub wrap: Option<bool>,
}

// Source: upstream/packages/types/src/FocusManager.ts:31 (sha256:68ddc2493463eb67824ddeca7e1ac85925757a209b4332deeb6440949341897a)
#[derive(Clone)]
pub struct FocusNavigationOptions {
    pub arrow_keys: Option<bool>,
}

// Source: upstream/packages/types/src/FocusManager.ts:39 (sha256:993a1607a7ad0aae944fc6743ad2357c20aa18b84ba327f36b8446e304586add)
pub type FocusNavigationInput = crate::OpaqueHostValue;
