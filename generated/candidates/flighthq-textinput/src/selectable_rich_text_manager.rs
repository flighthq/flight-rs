// @generated from upstream/packages/textinput/src/selectableRichTextManager.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_text::{get_rich_text_runtime, set_rich_text_scroll_v};
use flighthq_textlayout::compute_rich_text_char_index_at_point;
use flighthq_types::{
    Adjustment, AdjustmentKind, BlendMode, BoundsNodeAny, ClipRegion, ColorTransform,
    InputKeyboardData, InteractionSignals, KeyCode, Material, MaterialData, Matrix, Node,
    NodeInteractionState, NodeSignals, NodeTraitsKey, Rectangle, RichText, RichTextRuntime,
    SelectableRichTextManager, Stage, TextAutoSize, TextFormat, TextFormatRange, TextVerticalAlign,
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

// Source: upstream/packages/textinput/src/selectableRichTextManager.ts:6 (sha256:dc0cfcd96ce6e431b84590d05050cb5b21117b511f5aaa41b5950090906a8ffe)
pub fn blur_selectable_rich_text(manager: &mut SelectableRichTextManager) -> () {
    if ((manager.focused).clone()).is_some() {
        let mut runtime = get_mutable_runtime(manager.focused.as_ref().unwrap());
        runtime.selection_begin_index = 0.0_f64;
        runtime.selection_end_index = 0.0_f64;
    }
    manager.focused = None;
}

// Source: upstream/packages/textinput/src/selectableRichTextManager.ts:15 (sha256:b036116604472a36d40849b18c2d80a3b12fd8884d12553fc3301d15461afa57)
pub fn create_selectable_rich_text_manager() -> SelectableRichTextManager {
    return SelectableRichTextManager {
        __flight_identity: std::sync::Arc::new(()),
        focused: None,
    };
}

// Source: upstream/packages/textinput/src/selectableRichTextManager.ts:19 (sha256:2c301c56502f40a2a6cfc48af4ed32e6a294b1fc26f3f4e92059e1fbc27eda4c)
pub fn dispatch_selectable_rich_text_key_down(
    manager: &SelectableRichTextManager,
    data: &InputKeyboardData,
    on_copy: Option<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(String) -> () + Send + 'static>>>,
    >,
) -> bool {
    let target = (manager.focused).clone();
    if (target).is_none() {
        return false;
    }
    if ((data.ctrl_key) || (data.meta_key))
        && ((((data.key).clone()).to_lowercase() == "a") || (data.key_code == KeyCode::A))
    {
        let mut runtime = get_mutable_runtime(&target.as_ref().unwrap());
        runtime.selection_begin_index = 0.0_f64;
        runtime.selection_end_index =
            (target.as_ref().unwrap().data.text.encode_utf16().count() as f64);
        return true;
    }
    if ((data.ctrl_key) || (data.meta_key))
        && ((((data.key).clone()).to_lowercase() == "c") || (data.key_code == KeyCode::C))
    {
        let mut runtime = get_mutable_runtime(&target.as_ref().unwrap());
        let start = (runtime.selection_begin_index).min(runtime.selection_end_index);
        let end = (runtime.selection_begin_index).max(runtime.selection_end_index);
        let selected = (target.as_ref().unwrap().data.text.slice)(start, end);
        if (selected.length > 0.0_f64) {
            {
                let __flight_callback = on_copy;
                __flight_callback
                    .as_ref()
                    .map(|callback| callback.lock().unwrap()(selected))
            };
        }
        return true;
    }
    return false;
}

// Source: upstream/packages/textinput/src/selectableRichTextManager.ts:43 (sha256:17858d41ee8a97578a5354aacea20b206f7f6857151967f3a44d82e7eeb600ac)
pub fn dispatch_selectable_rich_text_pointer_down(
    manager: &mut SelectableRichTextManager,
    target: &RichText,
    x: f64,
    y: f64,
    extend: Option<bool>,
) -> () {
    let extend = extend.unwrap_or(false);
    manager.focused = Some((*target).clone());
    let mut runtime = get_mutable_runtime(target);
    let layout = (runtime.text_layout).clone();
    if (layout).is_none() {
        if (!extend) {
            runtime.selection_begin_index = 0.0_f64;
            runtime.selection_end_index = 0.0_f64;
        }
        return;
    }
    let index = compute_rich_text_char_index_at_point(&layout.as_ref().unwrap(), x, y);
    if extend {
        runtime.selection_end_index = index;
    } else {
        runtime.selection_begin_index = index;
        runtime.selection_end_index = index;
    }
}

// Source: upstream/packages/textinput/src/selectableRichTextManager.ts:69 (sha256:925955e6cded27a704a7ccaa61f1b831647530e6dcb4262c8eccdc86e0a59140)
pub fn dispatch_selectable_rich_text_pointer_move(
    manager: &SelectableRichTextManager,
    x: f64,
    y: f64,
) -> () {
    let target = (manager.focused).clone();
    if (target).is_none() {
        return;
    }
    let mut runtime = get_mutable_runtime(&target.as_ref().unwrap());
    let layout = (runtime.text_layout).clone();
    if (layout).is_none() {
        return;
    }
    runtime.selection_end_index =
        compute_rich_text_char_index_at_point(&layout.as_ref().unwrap(), x, y);
}

// Source: upstream/packages/textinput/src/selectableRichTextManager.ts:78 (sha256:d80e14dfbb500694e6db9f91e6bd26d8389c8660dbf4aac9583e49b096dd3f09)
pub fn dispatch_selectable_rich_text_wheel(
    manager: &mut SelectableRichTextManager,
    delta_lines: f64,
) -> () {
    let mut target = (manager.focused).clone();
    if (target).is_none() {
        return;
    }
    set_rich_text_scroll_v(
        &mut target.as_mut().unwrap(),
        (target.as_mut().unwrap().data.scroll_v + (delta_lines).round()),
        None,
    );
}

// Source: upstream/packages/textinput/src/selectableRichTextManager.ts:84 (sha256:3e078a74724577332e280401524f14b2c6933d282bb12963a07776987cd3d6a8)
pub fn focus_selectable_rich_text(
    manager: &mut SelectableRichTextManager,
    target: &RichText,
) -> () {
    manager.focused = Some((*target).clone());
}

// Source: upstream/packages/textinput/src/selectableRichTextManager.ts:88 (sha256:1e8629eb066c102b653229a10c178f41d234f81659d87fd1aa6a98bbefcc1180)
pub fn get_selectable_rich_text_selection_text(manager: &SelectableRichTextManager) -> String {
    let target = (manager.focused).clone();
    if (target).is_none() {
        return "".to_owned();
    }
    let runtime = get_mutable_runtime(&target.as_ref().unwrap());
    let start = (runtime.selection_begin_index).min(runtime.selection_end_index);
    let end = (runtime.selection_begin_index).max(runtime.selection_end_index);
    return (target.as_ref().unwrap().data.text.slice)(start, end);
}

// Source: upstream/packages/textinput/src/selectableRichTextManager.ts:97 (sha256:733e60d658d9c7706305e948bd891d19dd23aaca31961e89d0f62f726b7ddd45)
fn get_mutable_runtime(source: &RichText) -> RichTextRuntime {
    return get_rich_text_runtime(source);
}
