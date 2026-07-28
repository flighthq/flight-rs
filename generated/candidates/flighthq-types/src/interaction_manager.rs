// @generated from upstream/packages/types/src/InteractionManager.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{CursorBackend, InteractionSignals, PointerType, SpatialIndex};

// Source: upstream/packages/types/src/InteractionManager.ts:10 (sha256:359ab498ad1854f8c681801a93f133dc17f6d1aecb46524a745ec43f718dff5b)
pub type InteractionSignalName = InteractionSignals;

// Source: upstream/packages/types/src/InteractionManager.ts:11 (sha256:5010c8f00b95ef7a53c829db1535a0a65e7673828a6ddd76a2f6c896d6107276)
pub type AnyInteractionSignalSlot =
    std::sync::Arc<dyn Fn(crate::OpaqueHostValue) -> () + Send + Sync + 'static>;

// Source: upstream/packages/types/src/InteractionManager.ts:13 (sha256:862e11110d27a2fae69cc108968d79f6b75dfb8719760df193c2f036752e10e6)
#[derive(Clone)]
pub struct InteractionManager {
    pub cursor_backend: Option<CursorBackend>,
    pub double_click_delay: f64,
    pub enabled: bool,
    pub pointer_captures: crate::OpaqueHostValue,
    pub pointer_states: crate::OpaqueHostValue,
    pub precise: bool,
    pub root: crate::OpaqueHostValue,
    pub spatial_index: Option<SpatialIndex>,
    pub signal_subscriber_counts: crate::OpaqueHostValue,
    pub tracked_signal_slots: crate::OpaqueHostValue,
    pub tracked_subscribers_only: bool,
}

// Source: upstream/packages/types/src/InteractionManager.ts:35 (sha256:05373e969c761eb766f2817146d279968935d9f6ea2106bae3d7fd29620c361c)
#[derive(Clone)]
pub struct InteractionManagerOptions {
    pub cursor_backend: Option<Option<CursorBackend>>,
    pub enabled: Option<bool>,
    pub precise: Option<bool>,
    pub spatial_index: Option<Option<SpatialIndex>>,
    pub tracked_subscribers_only: Option<bool>,
}

// Source: upstream/packages/types/src/InteractionManager.ts:43 (sha256:14bc066b01256092e00d45933fd8d31534950e6aa9879e869eca6e9b3e942018)
pub type InteractionInputSource = crate::OpaqueHostValue;

// Source: upstream/packages/types/src/InteractionManager.ts:48 (sha256:034806c53b63b6077ead04323f22478a19925c34cd79a170cacfc8b5e01b4da4)
#[derive(Clone)]
pub struct InteractionPointerOptions {
    pub alt_key: Option<bool>,
    pub buttons: Option<f64>,
    pub ctrl_key: Option<bool>,
    pub meta_key: Option<bool>,
    pub pointer_id: Option<f64>,
    pub pointer_type: Option<PointerType>,
    pub shift_key: Option<bool>,
}

// Source: upstream/packages/types/src/InteractionManager.ts:58 (sha256:6c846f5649ce0d7c3800c0bf309bebafb3e4bc1f67b56d12bc9e2acce5c9d262)
#[derive(Clone)]
pub struct InteractionPointerState {
    pub last_click_target: Option<crate::OpaqueHostValue>,
    pub last_click_time: f64,
    pub pointer_down_target: Option<crate::OpaqueHostValue>,
    pub pointer_over_target: Option<crate::OpaqueHostValue>,
}
