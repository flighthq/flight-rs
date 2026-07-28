// @generated from upstream/packages/textinput/src/textInput.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_text::get_rich_text_runtime;
use flighthq_types::{
    Adjustment, AdjustmentKind, BlendMode, BoundsNodeAny, ClipRegion, ColorTransform,
    InteractionSignals, Material, MaterialData, Matrix, Node, NodeInteractionState, NodeSignals,
    NodeTraitsKey, Rectangle, RichText, Stage, TextAutoSize, TextFormat, TextFormatRange,
    TextInputOptions, TextInputState, TextVerticalAlign,
};

#[derive(Clone)]
pub struct FlightPartialRecord1 {
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
impl PartialEq for FlightPartialRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone)]
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
    pub auto_size: Option<TextAutoSize>,
    pub height: Option<f64>,
    pub text: Option<String>,
    pub text_format: Option<TextFormat>,
    pub vertical_align: Option<TextVerticalAlign>,
    pub width: Option<f64>,
    pub background: Option<bool>,
    pub background_color: Option<f64>,
    pub border: Option<bool>,
    pub border_color: Option<f64>,
    pub condense_white: Option<bool>,
    pub default_text_format: Option<TextFormat>,
    pub max_chars: Option<f64>,
    pub mouse_wheel_enabled: Option<bool>,
    pub multiline: Option<bool>,
    pub scroll_h: Option<f64>,
    pub scroll_v: Option<f64>,
    pub selectable: Option<bool>,
    pub text_color: Option<f64>,
    pub text_format_ranges: Option<Vec<TextFormatRange>>,
    pub word_wrap: Option<bool>,
}
impl PartialEq for FlightPartialRecord4 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone)]
pub struct FlightPartialRecord5 {
    pub __flight_identity: std::sync::Arc<()>,
    pub auto_size: Option<TextAutoSize>,
    pub height: Option<f64>,
    pub text: Option<String>,
    pub text_format: Option<TextFormat>,
    pub vertical_align: Option<TextVerticalAlign>,
    pub width: Option<f64>,
}
impl PartialEq for FlightPartialRecord5 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone)]
pub struct FlightPartialRecord6 {
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: Option<AdjustmentKind>,
    pub color_matrix: Option<Vec<f64>>,
}
impl PartialEq for FlightPartialRecord6 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone)]
pub struct FlightPartialRecord7 {
    pub __flight_identity: std::sync::Arc<()>,
    pub alpha: Option<f64>,
    pub visible: Option<bool>,
}
impl PartialEq for FlightPartialRecord7 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone)]
pub struct FlightPartialRecord8 {
    pub __flight_identity: std::sync::Arc<()>,
    pub blend_mode: Option<BlendMode>,
}
impl PartialEq for FlightPartialRecord8 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone)]
pub struct FlightPartialRecord9 {
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
impl PartialEq for FlightPartialRecord9 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone)]
pub struct FlightPartialRecord10 {
    pub __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for FlightPartialRecord10 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone)]
pub struct FlightPartialRecord11 {
    pub __flight_identity: std::sync::Arc<()>,
    pub clip: Option<ClipRegion>,
}
impl PartialEq for FlightPartialRecord11 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone)]
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

#[derive(Clone)]
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

#[derive(Clone)]
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

// Source: upstream/packages/textinput/src/textInput.ts:11 (sha256:76507ce67861b1a1347fd0f6f76ba2aaf8e24ade6e4786c42de05ab486ed74ef)
pub fn disable_text_input(node: &RichText) -> () {
    get_rich_text_runtime(node).input = None;
}

// Source: upstream/packages/textinput/src/textInput.ts:17 (sha256:10757c4ab40395f45a82484fd513ac85646153be488963c65eaa449fc4d69d79)
pub fn enable_text_input(node: &RichText, options: Option<TextInputOptions>) -> TextInputState {
    let mut runtime = get_rich_text_runtime(node);
    let mut state = (runtime.input).clone();
    if (state).is_none() {
        state = Some(create_text_input_state(Some(
            ((options).clone().unwrap()).clone(),
        )));
        runtime.input = (state).clone();
    } else {
        if (options).is_some() {
            apply_text_input_options(&mut state.as_mut().unwrap(), &options.as_ref().unwrap());
        }
    }
    return ((state).clone().unwrap()).clone();
}

// Source: upstream/packages/textinput/src/textInput.ts:29 (sha256:084146c3fd6190c26e0e9272eb6a74f0d6f6fe71ee806877f6f79961cb6453df)
pub fn get_text_input_state(node: &RichText) -> Option<TextInputState> {
    return (get_rich_text_runtime(node).input).clone();
}

// Source: upstream/packages/textinput/src/textInput.ts:33 (sha256:4d09410f18f70184c1df6c5d062ec13892748efccb38eae75fb6ba1a14b365b9)
pub fn has_text_input(node: &RichText) -> bool {
    return ((get_rich_text_runtime(node).input).clone()).is_some();
}

// Source: upstream/packages/textinput/src/textInput.ts:37 (sha256:40a03bb092e4c944038299c20684effb630f70a035c57fdd89d26b0be6c01168)
fn apply_text_input_options(state: &mut TextInputState, options: &TextInputOptions) -> () {
    if (options.always_show_selection).is_some() {
        state.always_show_selection = (options.always_show_selection).unwrap();
    }
    if (options.caret_color).is_some() {
        state.caret_color = (options.caret_color).unwrap();
    }
    if (options.caret_width).is_some() {
        state.caret_width = (options.caret_width).unwrap();
    }
    if (options.display_as_password).is_some() {
        state.display_as_password = (options.display_as_password).unwrap();
    }
    if (options.history_limit).is_some() {
        state.history_limit = (0.0_f64).max(options.history_limit);
    }
    if ((options.password_character).clone()).is_some() {
        state.password_character = ((options.password_character).clone()).unwrap();
    }
    if ((options.restrict).clone()).is_some() {
        state.restrict = ((options.restrict).clone()).unwrap();
    }
    if (options.selection_alpha).is_some() {
        state.selection_alpha = (options.selection_alpha).unwrap();
    }
    if (options.selection_color).is_some() {
        state.selection_color = (options.selection_color).unwrap();
    }
}

// Source: upstream/packages/textinput/src/textInput.ts:52 (sha256:b2557c022962c91d047b2efa411f2ae2e2b5e91fb350e3ead4901608ea3103a8)
fn create_text_input_state(options: Option<TextInputOptions>) -> TextInputState {
    return TextInputState {
        __flight_identity: std::sync::Arc::new(()),
        always_show_selection: (options
            .as_ref()
            .and_then(|value| value.always_show_selection))
        .unwrap_or(false),
        caret_color: (options.as_ref().and_then(|value| value.caret_color)).unwrap_or(0.0_f64),
        caret_index: 0.0_f64,
        caret_width: (options.as_ref().and_then(|value| value.caret_width)).unwrap_or(1.0_f64),
        desired_caret_x: (-1.0_f64),
        display_as_password: (options.as_ref().and_then(|value| value.display_as_password))
            .unwrap_or(false),
        focused: false,
        history: vec![],
        history_index: (-1.0_f64),
        history_limit: if (options.as_ref().and_then(|value| value.history_limit)).is_some() {
            (0.0_f64).max(options.as_ref().unwrap().history_limit)
        } else {
            100.0_f64
        },
        password_character: (options
            .as_ref()
            .and_then(|value| (value.password_character).clone()))
        .unwrap_or("•".to_owned()),
        restrict: (options.as_ref().and_then(|value| (value.restrict).clone()))
            .unwrap_or("".to_owned()),
        selection_alpha: (options.as_ref().and_then(|value| value.selection_alpha))
            .unwrap_or(0.35_f64),
        selection_color: (options.as_ref().and_then(|value| value.selection_color))
            .unwrap_or(30935.0_f64),
        selection_index: 0.0_f64,
    };
}
