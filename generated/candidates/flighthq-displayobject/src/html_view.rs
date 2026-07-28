// @generated from upstream/packages/displayobject/src/htmlView.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    create_display_object_generic, create_display_object_runtime, get_display_object_runtime,
};
use flighthq_node::invalidate_node_local_bounds;
use flighthq_types::{
    Adjustment, AdjustmentKind, BlendMode, BoundsNodeAny, ClipRegion, ColorTransform,
    HTML_VIEW_KIND as html_view_kind_constant, HtmlView, HtmlViewData, HtmlViewRuntime,
    InteractionSignals, Material, MaterialData, Matrix, Node, NodeInteractionState, NodeSignals,
    NodeTraitsKey, Rectangle, Stage,
};

#[derive(Clone, Default)]
pub struct FlightPartialRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub element: Option<crate::OpaqueHostValue>,
    pub height: Option<f64>,
    pub width: Option<f64>,
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
}
impl PartialEq for FlightPartialRecord4 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord5 {
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: Option<AdjustmentKind>,
    pub color_matrix: Option<Vec<f64>>,
    pub color_transform: Option<ColorTransform>,
}
impl PartialEq for FlightPartialRecord5 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord6 {
    pub __flight_identity: std::sync::Arc<()>,
    pub alpha: Option<f64>,
    pub visible: Option<bool>,
}
impl PartialEq for FlightPartialRecord6 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord7 {
    pub __flight_identity: std::sync::Arc<()>,
    pub blend_mode: Option<BlendMode>,
}
impl PartialEq for FlightPartialRecord7 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord8 {
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
impl PartialEq for FlightPartialRecord8 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord9 {
    pub __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for FlightPartialRecord9 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord10 {
    pub __flight_identity: std::sync::Arc<()>,
    pub clip: Option<ClipRegion>,
}
impl PartialEq for FlightPartialRecord10 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord11 {
    pub __flight_identity: std::sync::Arc<()>,
    pub material: Option<Material>,
    pub material_data: Option<MaterialData>,
}
impl PartialEq for FlightPartialRecord11 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord12 {
    pub __flight_identity: std::sync::Arc<()>,
    pub binding: Option<crate::OpaqueHostValue>,
    pub local_matrix: Option<Matrix>,
    pub rotation_angle: Option<f64>,
    pub rotation_cosine: Option<f64>,
    pub rotation_sine: Option<f64>,
    pub world_matrix: Option<Matrix>,
}
impl PartialEq for FlightPartialRecord12 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord13 {
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
impl PartialEq for FlightPartialRecord13 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/displayobject/src/htmlView.ts:7 (sha256:7dc87ec911bb21de9c8b0e30c8a51123e6625110f4e23005a6075be68296dfb8)
pub fn compute_html_view_local_bounds_rectangle(out: &mut Rectangle, source: &Node) -> () {
    out.width = source.data.width;
    out.height = source.data.height;
}

// Source: upstream/packages/displayobject/src/htmlView.ts:13 (sha256:456ee90b8bb4067523a77d4b57581c0171a30d3705481e1795fb4592f2b2220a)
pub fn create_html_view(obj: Option<HtmlView>) -> HtmlView {
    return create_display_object_generic(
        (html_view_kind_constant).to_owned(),
        Some(((obj).clone().unwrap()).clone()),
        Some(std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |__flight_argument_0: Option<D>| -> D {
                create_html_view_data(Some(((__flight_argument_0).clone().unwrap()).clone()))
            },
        )
            as Box<dyn FnMut(Option<D>) -> D + Send + 'static>))),
        Some(std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |__flight_argument_0: Option<R>| -> R { create_html_view_runtime() },
        )
            as Box<dyn FnMut(Option<R>) -> R + Send + 'static>))),
    );
}

// Source: upstream/packages/displayobject/src/htmlView.ts:17 (sha256:1d146ce6ff8398ea85cd40df01f0316124a94f51fa891b4470a251d5688efe84)
pub fn create_html_view_data(data: Option<FlightPartialRecord1>) -> HtmlViewData {
    return HtmlViewData {
        __flight_identity: std::sync::Arc::new(()),
        element: data.as_ref().and_then(|value| (value.element).clone()),
        height: (data.as_ref().and_then(|value| value.height)).unwrap_or(100.0_f64),
        width: (data.as_ref().and_then(|value| value.width)).unwrap_or(100.0_f64),
    };
}

// Source: upstream/packages/displayobject/src/htmlView.ts:25 (sha256:6201bbd0c3f0f4b89fe375124dbbeb6242f1c1bda1770f99677370b09c845cde)
pub fn create_html_view_runtime() -> HtmlViewRuntime {
    return {
        let __flight_source =
            &(create_display_object_runtime(Some(((*DEFAULT_METHODS).clone()).clone())));
        HtmlViewRuntime {
            __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
            binding: (__flight_source.binding).clone(),
            appearance_id: __flight_source.appearance_id,
            bounds_using_local_bounds_id: __flight_source.bounds_using_local_bounds_id,
            bounds_using_local_transform_id: __flight_source.bounds_using_local_transform_id,
            can_add_child: (__flight_source.can_add_child).clone(),
            children: (__flight_source.children).clone(),
            color_adjustments: (__flight_source.color_adjustments).clone(),
            resolved_color_transform: (__flight_source.resolved_color_transform).clone(),
            color_adjustments_channel_mixing: __flight_source.color_adjustments_channel_mixing,
            traits: (__flight_source.traits).clone(),
            interaction_signals: (__flight_source.interaction_signals).clone(),
            local_bounds_id: __flight_source.local_bounds_id,
            local_bounds_using_local_bounds_id: __flight_source.local_bounds_using_local_bounds_id,
            local_content_id: __flight_source.local_content_id,
            local_transform_id: __flight_source.local_transform_id,
            local_transform_using_local_transform_id: __flight_source
                .local_transform_using_local_transform_id,
            node_signals: (__flight_source.node_signals).clone(),
            interaction_state: (__flight_source.interaction_state).clone(),
            parent: (__flight_source.parent).clone(),
            world_bounds_using_local_bounds_id: __flight_source.world_bounds_using_local_bounds_id,
            world_bounds_using_world_transform_id: __flight_source
                .world_bounds_using_world_transform_id,
            world_transform_id: __flight_source.world_transform_id,
            world_transform_using_local_transform_id: __flight_source
                .world_transform_using_local_transform_id,
            world_transform_using_parent_transform_id: __flight_source
                .world_transform_using_parent_transform_id,
            local_matrix: (__flight_source.local_matrix).clone(),
            rotation_angle: __flight_source.rotation_angle,
            rotation_cosine: __flight_source.rotation_cosine,
            rotation_sine: __flight_source.rotation_sine,
            world_matrix: (__flight_source.world_matrix).clone(),
            bounds_rectangle: (__flight_source.bounds_rectangle).clone(),
            compute_local_bounds_rectangle: (__flight_source.compute_local_bounds_rectangle)
                .clone(),
            local_bounds_rectangle: (__flight_source.local_bounds_rectangle).clone(),
            world_bounds_rectangle: (__flight_source.world_bounds_rectangle).clone(),
            stage: (__flight_source.stage).clone(),
        }
    };
}

// Source: upstream/packages/displayobject/src/htmlView.ts:29 (sha256:c6c4563b2f6992502ec83f2ae5bc3cd74dae972b4171b3d254f854d399a066ed)
pub fn get_html_view_runtime(source: &HtmlView) -> HtmlViewRuntime {
    return {
        let __flight_source = &(get_display_object_runtime(source));
        HtmlViewRuntime {
            __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
            binding: (__flight_source.binding).clone(),
            appearance_id: __flight_source.appearance_id,
            bounds_using_local_bounds_id: __flight_source.bounds_using_local_bounds_id,
            bounds_using_local_transform_id: __flight_source.bounds_using_local_transform_id,
            can_add_child: (__flight_source.can_add_child).clone(),
            children: (__flight_source.children).clone(),
            color_adjustments: (__flight_source.color_adjustments).clone(),
            resolved_color_transform: (__flight_source.resolved_color_transform).clone(),
            color_adjustments_channel_mixing: __flight_source.color_adjustments_channel_mixing,
            traits: (__flight_source.traits).clone(),
            interaction_signals: (__flight_source.interaction_signals).clone(),
            local_bounds_id: __flight_source.local_bounds_id,
            local_bounds_using_local_bounds_id: __flight_source.local_bounds_using_local_bounds_id,
            local_content_id: __flight_source.local_content_id,
            local_transform_id: __flight_source.local_transform_id,
            local_transform_using_local_transform_id: __flight_source
                .local_transform_using_local_transform_id,
            node_signals: (__flight_source.node_signals).clone(),
            interaction_state: (__flight_source.interaction_state).clone(),
            parent: (__flight_source.parent).clone(),
            world_bounds_using_local_bounds_id: __flight_source.world_bounds_using_local_bounds_id,
            world_bounds_using_world_transform_id: __flight_source
                .world_bounds_using_world_transform_id,
            world_transform_id: __flight_source.world_transform_id,
            world_transform_using_local_transform_id: __flight_source
                .world_transform_using_local_transform_id,
            world_transform_using_parent_transform_id: __flight_source
                .world_transform_using_parent_transform_id,
            local_matrix: (__flight_source.local_matrix).clone(),
            rotation_angle: __flight_source.rotation_angle,
            rotation_cosine: __flight_source.rotation_cosine,
            rotation_sine: __flight_source.rotation_sine,
            world_matrix: (__flight_source.world_matrix).clone(),
            bounds_rectangle: (__flight_source.bounds_rectangle).clone(),
            compute_local_bounds_rectangle: (__flight_source.compute_local_bounds_rectangle)
                .clone(),
            local_bounds_rectangle: (__flight_source.local_bounds_rectangle).clone(),
            world_bounds_rectangle: (__flight_source.world_bounds_rectangle).clone(),
            stage: (__flight_source.stage).clone(),
        }
    };
}

// Source: upstream/packages/displayobject/src/htmlView.ts:33 (sha256:f2543255c3a7982453ff94464b0646cd9ef4d1b1bbf0094a79ff36bbf1315944)
pub fn set_html_view_size(source: &mut HtmlView, width: f64, height: f64) -> () {
    if (source.data.width == width) && (source.data.height == height) {
        return;
    }
    source.data.width = width;
    source.data.height = height;
    invalidate_node_local_bounds(source);
}

// Source: upstream/packages/displayobject/src/htmlView.ts:40 (sha256:598e2a451d040f8337cc8e0834e84ad6db057fe0830b3497d7280393e70b22f3)
static DEFAULT_METHODS: std::sync::LazyLock<FlightPartialRecord2> =
    std::sync::LazyLock::new(|| FlightPartialRecord2 {
        __flight_identity: std::sync::Arc::new(()),
        compute_local_bounds_rectangle: Some(std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |mut __flight_argument_0: Rectangle, __flight_argument_1: BoundsNodeAny| -> () {
                compute_html_view_local_bounds_rectangle(
                    &mut __flight_argument_0,
                    &__flight_argument_1,
                )
            },
        )
            as Box<dyn FnMut(Rectangle, BoundsNodeAny) -> () + Send + 'static>))),
        binding: None,
        appearance_id: None,
        bounds_using_local_bounds_id: None,
        bounds_using_local_transform_id: None,
        can_add_child: None,
        children: None,
        color_adjustments: None,
        resolved_color_transform: None,
        color_adjustments_channel_mixing: None,
        traits: None,
        interaction_signals: None,
        local_bounds_id: None,
        local_bounds_using_local_bounds_id: None,
        local_content_id: None,
        local_transform_id: None,
        local_transform_using_local_transform_id: None,
        node_signals: None,
        interaction_state: None,
        parent: None,
        world_bounds_using_local_bounds_id: None,
        world_bounds_using_world_transform_id: None,
        world_transform_id: None,
        world_transform_using_local_transform_id: None,
        world_transform_using_parent_transform_id: None,
        local_matrix: None,
        rotation_angle: None,
        rotation_cosine: None,
        rotation_sine: None,
        world_matrix: None,
        bounds_rectangle: None,
        local_bounds_rectangle: None,
        world_bounds_rectangle: None,
        stage: None,
    });
