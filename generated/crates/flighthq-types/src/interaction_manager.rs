// @generated from upstream/packages/types/src/InteractionManager.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    CursorBackend, EntityRuntime, FocusEventData, InputSignals, InteractionSignals,
    KeyboardEventData, Node, NodeInteractionState, PointerEventData, PointerType, SpatialIndex2D,
};

#[derive(Clone)]
pub struct InteractionManagerRecord1<N> {
    pub __flight_identity: std::sync::Arc<()>,
    pub layer: InteractionDispatchLayer<N>,
    pub priority: f64,
}
impl<N> PartialEq for InteractionManagerRecord1<N> {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/InteractionManager.ts:15 (sha256:6aceffb76e2558ac1868f3b544b165f4b5c2b1fd8d2233877b5c801be447c637)
pub type InteractionSignalName = InteractionSignals;

// Source: upstream/packages/types/src/InteractionManager.ts:16 (sha256:5010c8f00b95ef7a53c829db1535a0a65e7673828a6ddd76a2f6c896d6107276)
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

// Source: upstream/packages/types/src/InteractionManager.ts:17 (sha256:a6ba4dcff654010e6970912de1c9dfece733a1abf906f7a62bf7a106b3896459)
pub type InteractionDispatchLayer<N = Node> = std::sync::Arc<
    std::sync::Mutex<
        Box<
            dyn FnMut(
                    N,
                    InteractionSignalName,
                    crate::FlightUnion2<
                        PointerEventData,
                        crate::FlightUnion2<KeyboardEventData, FocusEventData>,
                    >,
                ) -> bool
                + Send
                + 'static,
        >,
    >,
>;

// Source: upstream/packages/types/src/InteractionManager.ts:23 (sha256:247c42d99f252c9324156940389ea1158f95ec556646bc37c93ea5309d045c13)
#[derive(Clone, Default)]
pub struct InteractionDispatchLayerOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub priority: Option<f64>,
}
impl PartialEq for InteractionDispatchLayerOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/InteractionManager.ts:27 (sha256:dfad89a5b24e4323976592ed0ed1938340383368da44fc7f0571432f58b23a61)
#[derive(Clone)]
pub struct InteractionManager<N = Node> {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub cursor_backend: Option<CursorBackend>,
    pub cursor_target: Option<N>,
    pub dispatch_layers: Option<Vec<InteractionManagerRecord1<N>>>,
    pub double_click_delay: f64,
    pub double_click_distance: f64,
    pub enabled: bool,
    pub pointer_captures: Vec<(f64, N)>,
    pub pointer_states: Vec<(f64, InteractionPointerState<N>)>,
    pub precise: bool,
    pub root: N,
    pub spatial_index: Option<SpatialIndex2D>,
    pub signal_subscriber_counts: Vec<(InteractionSignalName, f64)>,
    pub suppress_touch_hover: bool,
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
impl<N: Clone + Send + Sync + 'static> crate::FlightEntity for InteractionManager<N> {
    fn __flight_entity_runtime(
        &self,
    ) -> &std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>> {
        &self.__flight_entity_runtime
    }
    fn __flight_entity_snapshot(&self) -> &Option<std::sync::Arc<dyn std::any::Any + Send + Sync>> {
        &self.__flight_entity_snapshot
    }
    fn __flight_fresh_clone(&self) -> Self {
        let mut cloned = self.clone();
        cloned.__flight_identity = std::sync::Arc::new(());
        cloned.__flight_entity_runtime = std::sync::Arc::new(std::sync::Mutex::new(
            self.__flight_entity_runtime.lock().unwrap().clone(),
        ));
        cloned
    }
}

// Source: upstream/packages/types/src/InteractionManager.ts:63 (sha256:f7c5e7a3609a62f0fbf523ba1e5da6c85eeb890e177184d660954967fa1de68d)
#[derive(Clone, Default)]
pub struct InteractionManagerOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub cursor_backend: Option<CursorBackend>,
    pub double_click_delay: Option<f64>,
    pub double_click_distance: Option<f64>,
    pub enabled: Option<bool>,
    pub precise: Option<bool>,
    pub spatial_index: Option<SpatialIndex2D>,
    pub suppress_touch_hover: Option<bool>,
    pub tracked_subscribers_only: Option<bool>,
}
impl PartialEq for InteractionManagerOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/InteractionManager.ts:74 (sha256:14bc066b01256092e00d45933fd8d31534950e6aa9879e869eca6e9b3e942018)
pub type InteractionInputSource = InputSignals;

// Source: upstream/packages/types/src/InteractionManager.ts:79 (sha256:81772acc1a43859e1f6b52083a0f5b7f1a0ba02f46f61c4636bd1e5bc63163aa)
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
    pub time_stamp: Option<f64>,
}
impl PartialEq for InteractionPointerOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/InteractionManager.ts:90 (sha256:2c60e479926c797a20ea429b5a01bfba2c2b8d18bb72b90d1cad2d33d7bf9d60)
#[derive(Clone)]
pub struct InteractionPointerState<N = Node> {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub last_click_target: Option<N>,
    pub last_click_time: f64,
    pub last_pointer_click_button: f64,
    pub last_pointer_click_interaction_state: Option<NodeInteractionState>,
    pub last_pointer_click_target: Option<N>,
    pub last_pointer_click_time: f64,
    pub last_pointer_click_x: f64,
    pub last_pointer_click_y: f64,
    pub pointer_down_target: Option<N>,
    pub pointer_over_target: Option<N>,
}
impl<N> Default for InteractionPointerState<N> {
    fn default() -> Self {
        Self {
            __flight_identity: Default::default(),
            last_click_target: Default::default(),
            last_click_time: Default::default(),
            last_pointer_click_button: Default::default(),
            last_pointer_click_interaction_state: Default::default(),
            last_pointer_click_target: Default::default(),
            last_pointer_click_time: Default::default(),
            last_pointer_click_x: Default::default(),
            last_pointer_click_y: Default::default(),
            pointer_down_target: Default::default(),
            pointer_over_target: Default::default(),
        }
    }
}
impl<N> PartialEq for InteractionPointerState<N> {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
