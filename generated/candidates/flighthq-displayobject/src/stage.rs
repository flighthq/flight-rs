// @generated from upstream/packages/displayobject/src/stage.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::create_display_object;
use flighthq_node::{create_viewport, get_node_root, get_node_runtime};
use flighthq_signals::{create_signal, emit_signal};
use flighthq_types::{
    Adjustment, AdjustmentKind, BlendMode, BoundsNodeAny, ClipRegion, ColorTransform,
    DisplayObject, InteractionSignals, Material, MaterialData, Matrix, Node, NodeInteractionState,
    NodeSignals, NodeTraitsKey, Rectangle, Stage, StageRuntime, StageSignals, ViewportAlign,
    ViewportScaleMode,
};

#[derive(Clone, Default)]
pub struct FlightPartialRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub align: Option<ViewportAlign>,
    pub root: Option<DisplayObject>,
    pub scale_mode: Option<ViewportScaleMode>,
    pub color: Option<f64>,
    pub stage_height: Option<f64>,
    pub stage_width: Option<f64>,
}
impl PartialEq for FlightPartialRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord2 {
    pub __flight_identity: std::sync::Arc<()>,
    pub binding: Option<crate::OpaqueHostValue>,
    pub bounds_rectangle: Option<Rectangle>,
    pub compute_local_bounds_rectangle: Option<
        std::sync::Arc<
            std::sync::Mutex<Box<dyn FnMut(Rectangle, BoundsNodeAny) -> () + Send + 'static>>,
        >,
    >,
    pub local_bounds_rectangle: Option<Rectangle>,
    pub world_bounds_rectangle: Option<Rectangle>,
}
impl PartialEq for FlightPartialRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord3 {
    pub __flight_identity: std::sync::Arc<()>,
    pub alpha_multiplier: Option<f64>,
    pub alpha_offset: Option<f64>,
    pub blue_multiplier: Option<f64>,
    pub blue_offset: Option<f64>,
    pub green_multiplier: Option<f64>,
    pub green_offset: Option<f64>,
    pub red_multiplier: Option<f64>,
    pub red_offset: Option<f64>,
}
impl PartialEq for FlightPartialRecord3 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord4 {
    pub __flight_identity: std::sync::Arc<()>,
    pub binding: Option<crate::OpaqueHostValue>,
    pub appearance_id: Option<f64>,
    pub bounds_using_local_bounds_id: Option<f64>,
    pub bounds_using_local_transform_id: Option<f64>,
    pub can_add_child: Option<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(Node, Node) -> bool + Send + 'static>>>,
    >,
    pub children: Option<Vec<Node>>,
    pub color_adjustments: Option<Vec<Adjustment>>,
    pub resolved_color_transform: Option<ColorTransform>,
    pub color_adjustments_channel_mixing: Option<bool>,
    pub traits: Option<NodeTraitsKey>,
    pub interaction_signals: Option<InteractionSignals>,
    pub local_bounds_id: Option<f64>,
    pub local_bounds_using_local_bounds_id: Option<f64>,
    pub local_content_id: Option<f64>,
    pub local_transform_id: Option<f64>,
    pub local_transform_using_local_transform_id: Option<f64>,
    pub node_signals: Option<NodeSignals>,
    pub interaction_state: Option<NodeInteractionState>,
    pub parent: Option<Node>,
    pub world_bounds_using_local_bounds_id: Option<f64>,
    pub world_bounds_using_world_transform_id: Option<f64>,
    pub world_transform_id: Option<f64>,
    pub world_transform_using_local_transform_id: Option<f64>,
    pub world_transform_using_parent_transform_id: Option<f64>,
    pub local_matrix: Option<Matrix>,
    pub rotation_angle: Option<f64>,
    pub rotation_cosine: Option<f64>,
    pub rotation_sine: Option<f64>,
    pub world_matrix: Option<Matrix>,
    pub bounds_rectangle: Option<Rectangle>,
    pub compute_local_bounds_rectangle: Option<
        std::sync::Arc<
            std::sync::Mutex<Box<dyn FnMut(Rectangle, BoundsNodeAny) -> () + Send + 'static>>,
        >,
    >,
    pub local_bounds_rectangle: Option<Rectangle>,
    pub world_bounds_rectangle: Option<Rectangle>,
    pub stage: Option<Stage>,
}
impl PartialEq for FlightPartialRecord4 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord5 {
    pub __flight_identity: std::sync::Arc<()>,
    pub binding: Option<crate::OpaqueHostValue>,
    pub appearance_id: Option<f64>,
    pub bounds_using_local_bounds_id: Option<f64>,
    pub bounds_using_local_transform_id: Option<f64>,
    pub can_add_child: Option<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(Node, Node) -> bool + Send + 'static>>>,
    >,
    pub children: Option<Vec<Node>>,
    pub color_adjustments: Option<Vec<Adjustment>>,
    pub resolved_color_transform: Option<ColorTransform>,
    pub color_adjustments_channel_mixing: Option<bool>,
    pub traits: Option<NodeTraitsKey>,
    pub interaction_signals: Option<InteractionSignals>,
    pub local_bounds_id: Option<f64>,
    pub local_bounds_using_local_bounds_id: Option<f64>,
    pub local_content_id: Option<f64>,
    pub local_transform_id: Option<f64>,
    pub local_transform_using_local_transform_id: Option<f64>,
    pub node_signals: Option<NodeSignals>,
    pub interaction_state: Option<NodeInteractionState>,
    pub parent: Option<Node>,
    pub world_bounds_using_local_bounds_id: Option<f64>,
    pub world_bounds_using_world_transform_id: Option<f64>,
    pub world_transform_id: Option<f64>,
    pub world_transform_using_local_transform_id: Option<f64>,
    pub world_transform_using_parent_transform_id: Option<f64>,
}
impl PartialEq for FlightPartialRecord5 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord6 {
    pub __flight_identity: std::sync::Arc<()>,
    pub align: Option<ViewportAlign>,
    pub root: Option<Node>,
    pub scale_mode: Option<ViewportScaleMode>,
}
impl PartialEq for FlightPartialRecord6 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord7 {
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: Option<AdjustmentKind>,
    pub color_matrix: Option<Vec<f64>>,
    pub color_transform: Option<ColorTransform>,
}
impl PartialEq for FlightPartialRecord7 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord8 {
    pub __flight_identity: std::sync::Arc<()>,
    pub alpha: Option<f64>,
    pub visible: Option<bool>,
}
impl PartialEq for FlightPartialRecord8 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord9 {
    pub __flight_identity: std::sync::Arc<()>,
    pub blend_mode: Option<BlendMode>,
}
impl PartialEq for FlightPartialRecord9 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord10 {
    pub __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for FlightPartialRecord10 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord11 {
    pub __flight_identity: std::sync::Arc<()>,
    pub clip: Option<ClipRegion>,
}
impl PartialEq for FlightPartialRecord11 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord12 {
    pub __flight_identity: std::sync::Arc<()>,
    pub material: Option<Material>,
    pub material_data: Option<MaterialData>,
}
impl PartialEq for FlightPartialRecord12 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord13 {
    pub __flight_identity: std::sync::Arc<()>,
    pub binding: Option<crate::OpaqueHostValue>,
    pub local_matrix: Option<Matrix>,
    pub rotation_angle: Option<f64>,
    pub rotation_cosine: Option<f64>,
    pub rotation_sine: Option<f64>,
    pub world_matrix: Option<Matrix>,
}
impl PartialEq for FlightPartialRecord13 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord14 {
    pub __flight_identity: std::sync::Arc<()>,
    pub pivot_x: Option<f64>,
    pub pivot_y: Option<f64>,
    pub rotation: Option<f64>,
    pub scale_x: Option<f64>,
    pub scale_y: Option<f64>,
    pub skew_x: Option<f64>,
    pub skew_y: Option<f64>,
    pub x: Option<f64>,
    pub y: Option<f64>,
}
impl PartialEq for FlightPartialRecord14 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/displayobject/src/stage.ts:20 (sha256:da2da113b8caf0d17a4879547f07ddebbab7c2bd57237a666358c0fb0e300e6f)
pub fn create_stage(obj: Option<FlightPartialRecord1>) -> Stage {
    let root = create_display_object(None);
    let mut stage = create_viewport(Some(FlightPartialRecord6 {
        __flight_identity: std::sync::Arc::new(()),
        align: Some((obj.as_ref().and_then(|value| (value.align).clone())).unwrap()),
        root: Some(root),
        scale_mode: Some((obj.as_ref().and_then(|value| (value.scale_mode).clone())).unwrap()),
    }));
    stage.color = obj.as_ref().and_then(|value| value.color);
    stage.stage_height = (obj.as_ref().and_then(|value| value.stage_height)).unwrap_or(550.0_f64);
    stage.stage_width = (obj.as_ref().and_then(|value| value.stage_width)).unwrap_or(400.0_f64);
    get_node_runtime(&root).stage = Some((stage).clone());
    return stage;
}

// Source: upstream/packages/displayobject/src/stage.ts:32 (sha256:d0afe852919bc94716fa0e218a45da19d3a7c50ecc4bfc997edc1c6d3da9b07b)
pub fn create_stage_runtime() -> StageRuntime {
    return StageRuntime {
        __flight_identity: std::sync::Arc::new(()),
        binding: None,
        stage_signals: None,
    };
}

// Source: upstream/packages/displayobject/src/stage.ts:39 (sha256:29e05d8466740288524a3ffc0101563bed174d016b3afbc49e86498748a5e9cb)
pub fn create_stage_signals() -> StageSignals {
    return StageSignals {
        __flight_identity: std::sync::Arc::new(()),
        on_fullscreen_changed: create_signal(),
        on_orientation_changed: create_signal(),
        on_resize: create_signal(),
    };
}

// Source: upstream/packages/displayobject/src/stage.ts:47 (sha256:8f44077bdba5077c06cdffbabfe373825cb9e73273f9b7977a8f024410ffad6a)
pub fn enable_stage_signals(source: &mut Stage) -> StageSignals {
    let mut runtime = ensure_stage_runtime(source);
    return {
        runtime.stage_signals?? = Some(create_stage_signals());
        runtime.stage_signals
    };
}

// Source: upstream/packages/displayobject/src/stage.ts:54 (sha256:17be1b0878e798f0a77f96d645f971f81534b961524296b58053d9c87deb44c3)
pub fn get_display_object_stage(source: &mut DisplayObject) -> Option<Stage> {
    let root = get_node_root(source);
    return (get_node_runtime(&{
        let __flight_source = &(root);
        Node {
            __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
            data: (__flight_source.data).clone(),
            enabled: __flight_source.enabled,
            kind: (__flight_source.kind).clone(),
            name: (__flight_source.name).clone(),
        }
    })
    .stage)
        .clone();
}

// Source: upstream/packages/displayobject/src/stage.ts:61 (sha256:0b9c2d84fc0cb931b337fa571702891c440a22703e738643ebb864e84e452cbf)
pub fn get_stage_runtime(source: &mut Stage) -> StageRuntime {
    return ensure_stage_runtime(&mut (*source).clone());
}

// Source: upstream/packages/displayobject/src/stage.ts:65 (sha256:d73d56adcdeef33c175e91201573449d725352252c6d658aaf412702cfb59586)
pub fn get_stage_signals(source: &Stage) -> Option<StageSignals> {
    let runtime = Some(panic!(
        "entity runtime storage requires the generated native entity trait"
    ));
    return runtime
        .as_ref()
        .and_then(|value| (value.stage_signals).clone());
}

// Source: upstream/packages/displayobject/src/stage.ts:70 (sha256:53e699b0818e59446648ed2b595e699ba69afbf416e121f633142b3ce6fb1270)
pub fn set_stage_size(source: &mut Stage, width: f64, height: f64) -> () {
    if (source.stage_width == width) && (source.stage_height == height) {
        return;
    }
    source.stage_width = width;
    source.stage_height = height;
    let runtime = Some(panic!(
        "entity runtime storage requires the generated native entity trait"
    ));
    if (runtime
        .as_ref()
        .and_then(|value| (value.stage_signals).clone()))
    .is_some()
    {
        emit_signal(
            (runtime
                .as_ref()
                .unwrap()
                .stage_signals
                .as_ref()
                .unwrap()
                .on_resize)
                .clone(),
            (),
        );
    }
}

// Source: upstream/packages/displayobject/src/stage.ts:78 (sha256:1a05e17c0525eeffdb91b14d2c230cc60f78361bc745327ce885abf240ec08ff)
fn ensure_stage_runtime(source: &mut Stage) -> StageRuntime {
    let existing = Some(panic!(
        "entity runtime storage requires the generated native entity trait"
    ));
    if (existing).is_some() {
        return ((existing.as_ref().unwrap()).clone()).clone();
    }
    let runtime = create_stage_runtime();
    ();
    return runtime;
}
