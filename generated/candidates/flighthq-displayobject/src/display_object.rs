// @generated from upstream/packages/displayobject/src/displayObject.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_adjustments::{
    COLOR_ADJUSTMENT_CHANNEL_MIXING as color_adjustment_channel_mixing_constant,
    COLOR_ADJUSTMENT_NONE as color_adjustment_none_constant, create_color_transform_adjustment,
    resolve_color_adjustments_color_transform,
};
use flighthq_materials::create_color_transform;
use flighthq_node::{
    create_node, create_node_runtime, get_node_runtime, init_appearance_trait,
    init_blend_mode_trait, init_bounds_rectangle_runtime_trait, init_bounds_rectangle_trait,
    init_clip_trait, init_material_trait, init_transform2_d_runtime_trait, init_transform2_d_trait,
    invalidate_node_appearance,
};
use flighthq_types::{
    Adjustment, AdjustmentKind, BlendMode, BoundsNodeAny, ClipRegion, ColorTransform,
    DISPLAY_OBJECT_KIND as display_object_kind_constant,
    DISPLAY_OBJECT_TRAITS_KEY as display_object_traits_key_constant, DisplayObject,
    DisplayObjectDataFactory, DisplayObjectRuntime, DisplayObjectRuntimeFactory,
    HasBoundsRectangle, InteractionSignals, Kind, Material, MaterialData, Matrix, Node, NodeAny,
    NodeInteractionState, NodeSignals, NodeTraitsKey, Rectangle, Stage,
};

#[derive(Clone)]
pub struct FlightPartialRecord1 {
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
impl PartialEq for FlightPartialRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone)]
pub struct FlightPartialRecord2 {
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
impl PartialEq for FlightPartialRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone)]
pub struct FlightPartialRecord3 {
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
impl PartialEq for FlightPartialRecord3 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone)]
pub struct FlightPartialRecord4 {
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: Option<AdjustmentKind>,
    pub color_matrix: Option<Vec<f64>>,
}
impl PartialEq for FlightPartialRecord4 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone)]
pub struct FlightPartialRecord5 {
    pub __flight_identity: std::sync::Arc<()>,
    pub alpha: Option<f64>,
    pub visible: Option<bool>,
}
impl PartialEq for FlightPartialRecord5 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone)]
pub struct FlightPartialRecord6 {
    pub __flight_identity: std::sync::Arc<()>,
    pub blend_mode: Option<BlendMode>,
}
impl PartialEq for FlightPartialRecord6 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone)]
pub struct FlightPartialRecord7 {
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
impl PartialEq for FlightPartialRecord7 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone)]
pub struct FlightPartialRecord8 {
    pub __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for FlightPartialRecord8 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone)]
pub struct FlightPartialRecord9 {
    pub __flight_identity: std::sync::Arc<()>,
    pub clip: Option<ClipRegion>,
}
impl PartialEq for FlightPartialRecord9 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone)]
pub struct FlightPartialRecord10 {
    pub __flight_identity: std::sync::Arc<()>,
    pub material: Option<Material>,
    pub material_data: Option<MaterialData>,
}
impl PartialEq for FlightPartialRecord10 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone)]
pub struct FlightPartialRecord11 {
    pub __flight_identity: std::sync::Arc<()>,
    pub binding: Option<crate::OpaqueHostValue>,
    pub local_matrix: Option<Matrix>,
    pub rotation_angle: Option<f64>,
    pub rotation_cosine: Option<f64>,
    pub rotation_sine: Option<f64>,
    pub world_matrix: Option<Matrix>,
}
impl PartialEq for FlightPartialRecord11 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone)]
pub struct FlightPartialRecord12 {
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
impl PartialEq for FlightPartialRecord12 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/displayobject/src/displayObject.ts:41 (sha256:612c711216e740026141630b02167c8dcf77f92bacf70172250ce875729fc6fb)
pub fn add_display_object_color_adjustment(source: &DisplayObject, adjustment: &Adjustment) -> () {
    let mut runtime = get_node_runtime(source);
    let current = (runtime.color_adjustments).clone();
    runtime.color_adjustments = Some(if (current).is_none() {
        vec![(*adjustment).clone()]
    } else {
        {
            let mut __flight_array = Vec::new();
            __flight_array.extend((current.as_ref().unwrap()).iter().cloned());
            __flight_array.push((*adjustment).clone());
            __flight_array
        }
    });
    resolve_display_object_color_adjustments(&mut runtime);
    invalidate_node_appearance(source);
}

// Source: upstream/packages/displayobject/src/displayObject.ts:49 (sha256:45e3f59a162c6bdee66c20f61ee29bb1041b987c3b5d83b7b8fc9f571c6d40c9)
pub fn create_display_object(obj: Option<DisplayObject>) -> DisplayObject {
    return create_display_object_generic(
        (display_object_kind_constant).to_owned(),
        Some(((obj).clone().unwrap()).clone()),
        None,
        None,
    );
}

// Source: upstream/packages/displayobject/src/displayObject.ts:53 (sha256:e2e1537ab456bd02665f8e4d8e4876d39eb05435c0e1023de4fc294c7d71f02d)
pub fn create_display_object_generic<R: Clone>(
    kind: Kind,
    obj: Option<DisplayObject>,
    create_data: Option<DisplayObjectDataFactory>,
    create_display_object_runtime_factory: Option<DisplayObjectRuntimeFactory<R>>,
) -> DisplayObject {
    let mut out = create_node(
        (kind).clone(),
        Some(((obj).clone().unwrap()).clone()),
        Some(((create_data).clone().unwrap()).clone()),
        Some(
            ((create_display_object_runtime_factory).unwrap_or(create_display_object_runtime))
                .clone(),
        ),
    );
    init_transform2_d_trait(&mut out, Some(((obj).clone().unwrap()).clone()));
    init_bounds_rectangle_trait(
        &HasBoundsRectangle {
            __flight_identity: std::sync::Arc::clone(&(out).__flight_identity),
        },
        Some(((obj).clone().unwrap()).clone()),
    );
    init_appearance_trait(&mut out, Some(((obj).clone().unwrap()).clone()));
    init_blend_mode_trait(&mut out, Some(((obj).clone().unwrap()).clone()));
    init_material_trait(&mut out, Some(((obj).clone().unwrap()).clone()));
    init_clip_trait(&mut out, Some(((obj).clone().unwrap()).clone()));
    return out;
}

// Source: upstream/packages/displayobject/src/displayObject.ts:74 (sha256:0b3aaed33a093799fe4bfd9903f0b2a4166bd7fa6ff38db9852d31e75a056f07)
pub fn create_display_object_runtime(
    methods: Option<FlightPartialRecord1>,
) -> DisplayObjectRuntime {
    let mut out = create_node_runtime(Some(((methods).clone().unwrap()).clone()));
    out.traits = Some(display_object_traits_key_constant);
    out.stage = None;
    init_transform2_d_runtime_trait(&mut out, Some(((methods).clone().unwrap()).clone()));
    init_bounds_rectangle_runtime_trait(&mut out, Some(((methods).clone().unwrap()).clone()));
    return out;
}

// Source: upstream/packages/displayobject/src/displayObject.ts:87 (sha256:7f0ff2a6656f2a1fe27dea97082d4c6e7090eea3c99f2c166482aa852d7b3665)
pub fn get_display_object_color_adjustments(source: &DisplayObject) -> Option<Vec<Adjustment>> {
    return (get_node_runtime(source).color_adjustments).clone();
}

// Source: upstream/packages/displayobject/src/displayObject.ts:91 (sha256:772271827d0f3a9b734b9fdb2f02dda3d1431fb6982f79781651a9e2b17b49fb)
pub fn get_display_object_runtime(source: &DisplayObject) -> DisplayObjectRuntime {
    return get_node_runtime(source);
}

// Source: upstream/packages/displayobject/src/displayObject.ts:95 (sha256:2b791d3d6f013a3c6c937920cc560a118edd4f567f629026944f13605540b654)
pub fn is_display_object(node: &NodeAny) -> bool {
    return ((get_node_runtime(node).traits).clone() == display_object_traits_key_constant);
}

// Source: upstream/packages/displayobject/src/displayObject.ts:99 (sha256:c7a43ea7c71c4292de9bdbedb90aab790bd95c73552155fdbe283cdc1c4c4934)
pub fn set_display_object_clip(source: &mut DisplayObject, value: Option<ClipRegion>) -> () {
    source.clip = (value).clone();
    invalidate_node_appearance(source);
}

// Source: upstream/packages/displayobject/src/displayObject.ts:108 (sha256:a9e36ae07ff2a01c2584eebbb486a3c1ab9f56a69d814104d865184ac3fd87a7)
pub fn set_display_object_color_adjustments(
    source: &DisplayObject,
    value: Option<Vec<Adjustment>>,
) -> () {
    let mut runtime = get_node_runtime(source);
    runtime.color_adjustments = (value).clone();
    resolve_display_object_color_adjustments(&mut runtime);
    invalidate_node_appearance(source);
}

// Source: upstream/packages/displayobject/src/displayObject.ts:118 (sha256:5cc12742001869aab06a9952a61f67f493b5dfeacf8bc9f594c7838ed5ffa30e)
pub fn set_display_object_color_transform(
    source: &DisplayObject,
    color_transform: Option<ColorTransform>,
) -> () {
    set_display_object_color_adjustments(
        source,
        (if (color_transform).is_none() {
            None
        } else {
            Some(vec![create_color_transform_adjustment(
                &color_transform.as_ref().unwrap(),
            )])
        })
        .clone(),
    );
}

// Source: upstream/packages/displayobject/src/displayObject.ts:133 (sha256:ec53c619f284d0880a918ced30cbe9848cbb2cee29faf21f8b2706d8fff34d78)
fn resolve_display_object_color_adjustments(runtime: &mut DisplayObjectRuntime) -> () {
    let adjustments = (runtime.color_adjustments).clone();
    if ((adjustments).is_none()) || ((adjustments.as_ref().unwrap().len() as f64) == 0.0_f64) {
        runtime.resolved_color_transform = None;
        runtime.color_adjustments_channel_mixing = false;
        return;
    }
    let mut out =
        ((runtime.resolved_color_transform).clone()).unwrap_or(create_color_transform(None));
    let status =
        resolve_color_adjustments_color_transform(((adjustments).clone()).clone(), &mut out);
    if (status == color_adjustment_none_constant) {
        runtime.resolved_color_transform = None;
        runtime.color_adjustments_channel_mixing = false;
        return;
    }
    runtime.resolved_color_transform = Some((out).clone());
    runtime.color_adjustments_channel_mixing = (status == color_adjustment_channel_mixing_constant);
}
