// @generated from upstream/packages/types/src/InteractionManager.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    CursorBackend, FocusEventData, InputSignals, InteractionSignals, KeyboardEventData,
    PointerEventData, PointerType, SpatialIndex,
};

// Source: upstream/packages/types/src/InteractionManager.ts:10 (sha256:359ab498ad1854f8c681801a93f133dc17f6d1aecb46524a745ec43f718dff5b)
pub type InteractionSignalName = InteractionSignals;

// Source: upstream/packages/types/src/InteractionManager.ts:11 (sha256:5010c8f00b95ef7a53c829db1535a0a65e7673828a6ddd76a2f6c896d6107276)
pub type AnyInteractionSignalSlot = std::sync::Arc<
    std::sync::Mutex<
        Box<
            dyn FnMut(
                    crate::FlightUnion2<
                        PointerEventData,
                        crate::FlightUnion2<KeyboardEventData, FocusEventData>,
                    >,
                ) -> ()
                + Send
                + 'static,
        >,
    >,
>;

// Source: upstream/packages/types/src/InteractionManager.ts:13 (sha256:862e11110d27a2fae69cc108968d79f6b75dfb8719760df193c2f036752e10e6)
#[derive(Clone)]
pub struct InteractionManager<N> {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub cursor_backend: Option<CursorBackend>,
    pub double_click_delay: f64,
    pub enabled: bool,
    pub pointer_captures: Vec<(f64, N)>,
    pub pointer_states: Vec<(f64, InteractionPointerState<N>)>,
    pub precise: bool,
    pub root: N,
    pub spatial_index: Option<SpatialIndex>,
    pub signal_subscriber_counts: Vec<(InteractionSignalName, f64)>,
    pub tracked_signal_slots: Vec<(
        N,
        Vec<(
            InteractionSignalName,
            Vec<(AnyInteractionSignalSlot, AnyInteractionSignalSlot)>,
        )>,
    )>,
    pub tracked_subscribers_only: bool,
}
impl<N> PartialEq for InteractionManager<N> {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/InteractionManager.ts:35 (sha256:05373e969c761eb766f2817146d279968935d9f6ea2106bae3d7fd29620c361c)
#[derive(Clone, Default)]
pub struct InteractionManagerOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub cursor_backend: Option<CursorBackend>,
    pub enabled: Option<bool>,
    pub precise: Option<bool>,
    pub spatial_index: Option<SpatialIndex>,
    pub tracked_subscribers_only: Option<bool>,
}
impl PartialEq for InteractionManagerOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/InteractionManager.ts:43 (sha256:14bc066b01256092e00d45933fd8d31534950e6aa9879e869eca6e9b3e942018)
pub type InteractionInputSource = InputSignals;

// Source: upstream/packages/types/src/InteractionManager.ts:48 (sha256:034806c53b63b6077ead04323f22478a19925c34cd79a170cacfc8b5e01b4da4)
#[derive(Clone, Default)]
pub struct InteractionPointerOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub alt_key: Option<bool>,
    pub buttons: Option<f64>,
    pub ctrl_key: Option<bool>,
    pub meta_key: Option<bool>,
    pub pointer_id: Option<f64>,
    pub pointer_type: Option<PointerType>,
    pub shift_key: Option<bool>,
}
impl PartialEq for InteractionPointerOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/InteractionManager.ts:58 (sha256:6c846f5649ce0d7c3800c0bf309bebafb3e4bc1f67b56d12bc9e2acce5c9d262)
#[derive(Clone, Default)]
pub struct InteractionPointerState<N> {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub last_click_target: Option<N>,
    pub last_click_time: f64,
    pub pointer_down_target: Option<N>,
    pub pointer_over_target: Option<N>,
}
impl<N> PartialEq for InteractionPointerState<N> {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
