// @generated from upstream/packages/types/src/Haptics.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/Haptics.ts:4 (sha256:657eafd12e02526dbf22fd415c3a46ae863808492273f739783467e6bdf9dd5a)
pub type HapticImpactStyle = String;

// Source: upstream/packages/types/src/Haptics.ts:5 (sha256:576a412ea6080a8e1a6817d714b944af20ea35af738d84855a4b91f4914dcf7f)
pub type HapticNotificationType = String;

// Source: upstream/packages/types/src/Haptics.ts:9 (sha256:493ca4196d2dc3374693378a6d9043ca983c1eaafd0ffb8973b1edd62f841d28)
#[derive(Clone)]
pub struct HapticsCapabilities {
    pub amplitude_control: bool,
    pub custom_events: bool,
    pub intensity: bool,
    pub patterns: bool,
    pub supported: bool,
}

// Source: upstream/packages/types/src/Haptics.ts:17 (sha256:7451aefe9a23805b4a47234fa02bd11c635b0188891dbcb413899544b2e44573)
#[derive(Clone)]
pub struct HapticsBackend {
    pub cancel: crate::OpaqueHostValue,
    pub capabilities: crate::OpaqueHostValue,
    pub impact: crate::OpaqueHostValue,
    pub is_supported: crate::OpaqueHostValue,
    pub notification: crate::OpaqueHostValue,
    pub prepare: Option<crate::OpaqueHostValue>,
    pub selection: crate::OpaqueHostValue,
    pub vibrate: crate::OpaqueHostValue,
    pub vibrate_pattern: crate::OpaqueHostValue,
    pub vibrate_waveform: Option<crate::OpaqueHostValue>,
}
