// @generated from upstream/packages/movieclip/src/movieClip.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_displayobject::{
    create_display_object_generic, create_display_object_runtime, get_display_object_runtime,
};
use flighthq_timeline::{
    add_timeline_frame_script, create_timeline, dispose_timeline_signals, enable_timeline_signals,
    get_timeline_current_label, get_timeline_frame_script, goto_and_play_timeline,
    goto_and_stop_timeline, next_frame_timeline, play_timeline, prev_frame_timeline,
    remove_timeline_frame_script, stop_timeline, update_timeline,
};
use flighthq_types::{
    Adjustment, AdjustmentKind, BlendMode, BoundsNodeAny, ClipRegion, ColorTransform,
    DisplayObject, FrameScript, InteractionSignals, MOVIE_CLIP_KIND as movie_clip_kind_constant,
    Material, MaterialData, Matrix, MovieClip, MovieClipData, MovieClipRuntime, MovieClipSignals,
    Node, NodeInteractionState, NodeSignals, NodeTraitsKey, Rectangle, Stage, Timeline,
    TimelineLabel, TimelinePlayMode, TimelineSignals, TimelineSource,
};

#[derive(Clone, Default)]
pub struct FlightPartialRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub timeline: Option<Timeline>,
}
impl PartialEq for FlightPartialRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
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

#[derive(Clone, Default)]
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
    pub source: Option<TimelineSource>,
    pub target: Option<DisplayObject>,
    pub current_frame: Option<f64>,
    pub frame_scripts: Option<Vec<(f64, FrameScript)>>,
    pub is_playing: Option<bool>,
    pub time_elapsed: Option<f64>,
    pub last_frame_update: Option<f64>,
    pub play_mode: Option<TimelinePlayMode>,
    pub signals: Option<TimelineSignals>,
}
impl PartialEq for FlightPartialRecord5 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct FlightPartialRecord6 {
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: Option<AdjustmentKind>,
    pub color_matrix: Option<Vec<f64>>,
    pub color_transform: Option<ColorTransform>,
}
impl PartialEq for FlightPartialRecord6 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
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

#[derive(Clone, Default)]
pub struct FlightPartialRecord8 {
    pub __flight_identity: std::sync::Arc<()>,
    pub blend_mode: Option<BlendMode>,
}
impl PartialEq for FlightPartialRecord8 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
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

// Source: upstream/packages/movieclip/src/movieClip.ts:36 (sha256:30642b2634c6dfb5b9a0987f93193511161a3a1e0758751bc7df9f00b5a85077)
pub fn add_movie_clip_frame_script(
    clip: &mut MovieClip,
    frame: &crate::FlightUnion2<f64, String>,
    script: &mut impl FnMut(DisplayObject, f64) -> (),
) -> () {
    if ((clip.data.timeline).clone()).is_none() {
        return;
    }
    add_timeline_frame_script(
        clip.data.timeline.as_mut().unwrap(),
        &((*frame).clone()),
        (script).clone(),
    );
}

// Source: upstream/packages/movieclip/src/movieClip.ts:41 (sha256:2908ac026f087f773d1f7020a2c64405e4d02ca4c7dfadbd6f97dd98e3614dae)
pub fn create_movie_clip(obj: Option<MovieClip>) -> MovieClip {
    return create_display_object_generic(
        (movie_clip_kind_constant).to_owned(),
        Some(((obj).clone().unwrap()).clone()),
        Some(std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |__flight_argument_0: Option<D>| -> D {
                create_movie_clip_data(Some(((__flight_argument_0).clone().unwrap()).clone()))
            },
        )
            as Box<dyn FnMut(Option<D>) -> D + Send + 'static>))),
        Some(std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |__flight_argument_0: Option<R>| -> R { create_movie_clip_runtime() },
        )
            as Box<dyn FnMut(Option<R>) -> R + Send + 'static>))),
    );
}

// Source: upstream/packages/movieclip/src/movieClip.ts:45 (sha256:21eeaaaaba6375563acc188752877a1fe2782ded0047fec49d1042805a0a24d9)
pub fn create_movie_clip_data(data: Option<FlightPartialRecord1>) -> MovieClipData {
    return MovieClipData {
        __flight_identity: std::sync::Arc::new(()),
        timeline: data.as_ref().and_then(|value| (value.timeline).clone()),
    };
}

// Source: upstream/packages/movieclip/src/movieClip.ts:51 (sha256:8d3bde03a3d2a9d1d8a230392243341592faf37d51f549f6e9a4842f94023c79)
pub fn create_movie_clip_runtime() -> MovieClipRuntime {
    let mut out = create_display_object_runtime(None);
    out.movie_clip_signals = None;
    return out;
}

// Source: upstream/packages/movieclip/src/movieClip.ts:57 (sha256:761af96bbad2e1ada29761db35e74e3edd119d9207a09f05e3ee2c83ef900c84)
pub fn dispose_movie_clip_signals(clip: &mut MovieClip) -> () {
    let mut runtime = panic!("entity runtime storage requires the generated native entity trait");
    if ((clip.data.timeline).clone()).is_some() {
        dispose_timeline_signals(clip.data.timeline.as_mut().unwrap());
    }
    runtime.movie_clip_signals = None;
}

// Source: upstream/packages/movieclip/src/movieClip.ts:66 (sha256:f59211c48cd9d41c7d2803014c5e11329d22e1344508c490fa88784d4847742a)
pub fn enable_movie_clip_signals(clip: &mut MovieClip) -> MovieClipSignals {
    let mut runtime = panic!("entity runtime storage requires the generated native entity trait");
    if ((runtime.movie_clip_signals).clone()).is_some() {
        return ((runtime.movie_clip_signals).clone()).unwrap();
    }
    if ((clip.data.timeline).clone()).is_none() {
        clip.data.timeline = Some(create_timeline(None));
    }
    let signals = enable_timeline_signals(clip.data.timeline.as_mut().unwrap());
    runtime.movie_clip_signals = Some(signals);
    return {
        let __flight_source = &(signals);
        MovieClipSignals {
            __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
            on_complete: (__flight_source.on_complete).clone(),
            on_enter_frame: (__flight_source.on_enter_frame).clone(),
            on_exit_frame: (__flight_source.on_exit_frame).clone(),
            on_frame_constructed: (__flight_source.on_frame_constructed).clone(),
            on_loop: (__flight_source.on_loop).clone(),
        }
    };
}

// Source: upstream/packages/movieclip/src/movieClip.ts:76 (sha256:bc292a615868a71d0e5efc95f690936f7ba566bd1c47d8c49fc0e5a58790f818)
pub fn get_movie_clip_current_frame(clip: &MovieClip) -> f64 {
    return (clip.data.timeline.as_ref().map(|value| value.current_frame)).unwrap_or(1.0_f64);
}

// Source: upstream/packages/movieclip/src/movieClip.ts:80 (sha256:2bc724e30714b1dd76c8c51e7130582b217fb5c163d690cdd16a81db0e437d85)
pub fn get_movie_clip_current_label(clip: &MovieClip) -> Option<TimelineLabel> {
    if ((clip.data.timeline).clone()).is_none() {
        return None;
    }
    return get_timeline_current_label(clip.data.timeline.as_ref().unwrap());
}

// Source: upstream/packages/movieclip/src/movieClip.ts:85 (sha256:de6f2570981cc705fa9f4755b4a0e6a458c879aa8797764de5627442dcc067b7)
pub fn get_movie_clip_frame_script(
    clip: &MovieClip,
    frame: &crate::FlightUnion2<f64, String>,
) -> Option<FrameScript> {
    if ((clip.data.timeline).clone()).is_none() {
        return None;
    }
    return get_timeline_frame_script(clip.data.timeline.as_ref().unwrap(), &((*frame).clone()));
}

// Source: upstream/packages/movieclip/src/movieClip.ts:90 (sha256:ffff04fd96bfb54881d7a693c5c88390a43b12c74bcb89de90c7f3a2469865a5)
pub fn get_movie_clip_runtime(source: &MovieClip) -> MovieClipRuntime {
    return get_display_object_runtime(source);
}

// Source: upstream/packages/movieclip/src/movieClip.ts:94 (sha256:7f39470689f58a77594f31320e743c07c3b831d5e44bcb53230c993743d1a180)
pub fn get_movie_clip_signals(clip: &MovieClip) -> Option<MovieClipSignals> {
    let runtime = panic!("entity runtime storage requires the generated native entity trait");
    return (runtime.movie_clip_signals).clone();
}

// Source: upstream/packages/movieclip/src/movieClip.ts:99 (sha256:cb9a0eb30763ecf732e6e7ee598546444a7e8e2acd671e6ecc19fd7aac6bec9f)
pub fn get_movie_clip_total_frames(clip: &MovieClip) -> f64 {
    return (clip
        .data
        .timeline
        .as_ref()
        .unwrap()
        .source
        .as_ref()
        .map(|value| value.total_frames))
    .unwrap_or(1.0_f64);
}

// Source: upstream/packages/movieclip/src/movieClip.ts:103 (sha256:c422a8d084c3895ffa175b50fb4e8a84a85e86ca2bef9cef53d5c5337a8ce241)
pub fn goto_and_play_movie_clip(
    clip: &mut MovieClip,
    frame: &crate::FlightUnion2<f64, String>,
) -> () {
    if ((clip.data.timeline).clone()).is_none() {
        return;
    }
    goto_and_play_timeline(clip.data.timeline.as_mut().unwrap(), &((*frame).clone()));
}

// Source: upstream/packages/movieclip/src/movieClip.ts:108 (sha256:8ce564ccb96a4bdde778c272780514cb4c7a40b2b0d126b28c87d872d3fc184d)
pub fn goto_and_stop_movie_clip(
    clip: &mut MovieClip,
    frame: &crate::FlightUnion2<f64, String>,
) -> () {
    if ((clip.data.timeline).clone()).is_none() {
        return;
    }
    goto_and_stop_timeline(clip.data.timeline.as_mut().unwrap(), &((*frame).clone()));
}

// Source: upstream/packages/movieclip/src/movieClip.ts:113 (sha256:70475fd2e5a10696562a7d688705f047fe4be1fd4c0e8401f010d45f06b97cd0)
pub fn is_movie_clip_playing(clip: &MovieClip) -> bool {
    return (clip.data.timeline.as_ref().map(|value| value.is_playing)).unwrap_or(false);
}

// Source: upstream/packages/movieclip/src/movieClip.ts:117 (sha256:2bce6674dbcd40439c3b76b8000d577c520d4b1b8586864886f75c83c552cfcb)
pub fn next_frame_movie_clip(clip: &mut MovieClip) -> () {
    if ((clip.data.timeline).clone()).is_none() {
        return;
    }
    next_frame_timeline(clip.data.timeline.as_mut().unwrap());
}

// Source: upstream/packages/movieclip/src/movieClip.ts:122 (sha256:59877733565af921358e0684cf589c02f7eeec12dd89d95e5f844b727df4eabe)
pub fn play_movie_clip(clip: &mut MovieClip) -> () {
    if ((clip.data.timeline).clone()).is_none() {
        return;
    }
    play_timeline(clip.data.timeline.as_mut().unwrap());
}

// Source: upstream/packages/movieclip/src/movieClip.ts:127 (sha256:03a243634561fcffeb23f05f0a3da7eefe738b8a3e62fdcf82922635db8954b0)
pub fn prev_frame_movie_clip(clip: &mut MovieClip) -> () {
    if ((clip.data.timeline).clone()).is_none() {
        return;
    }
    prev_frame_timeline(clip.data.timeline.as_mut().unwrap());
}

// Source: upstream/packages/movieclip/src/movieClip.ts:132 (sha256:e086a82a0ff92b731d1689ab50f1e5c78aceada763df9cdeb85b35c39a4c0613)
pub fn remove_movie_clip_frame_script(
    clip: &mut MovieClip,
    frame: &crate::FlightUnion2<f64, String>,
) -> () {
    if ((clip.data.timeline).clone()).is_none() {
        return;
    }
    remove_timeline_frame_script(clip.data.timeline.as_mut().unwrap(), &((*frame).clone()));
}

// Source: upstream/packages/movieclip/src/movieClip.ts:140 (sha256:a50da8e6dea744b5bd03dcb5c07edc91af3ec66074f634dcf5124dd8965a9274)
pub fn set_movie_clip_source(clip: &mut MovieClip, source: &TimelineSource) -> () {
    let mut timeline = ((clip.data.timeline).clone()).unwrap_or(create_timeline(None));
    timeline.source = Some((*source).clone());
    timeline.target = Some(clip);
    clip.data.timeline = Some((timeline).clone());
    {
        let __flight_argument_1 = (timeline.current_frame).clone();
        goto_and_stop_timeline(&mut timeline, &__flight_argument_1)
    };
}

// Source: upstream/packages/movieclip/src/movieClip.ts:151 (sha256:70c82fb7c381309841f3dd0ce9931d8a5e028c81d65e9c4e8a256b1080334bb1)
pub fn stop_movie_clip(clip: &mut MovieClip) -> () {
    if ((clip.data.timeline).clone()).is_none() {
        return;
    }
    stop_timeline(clip.data.timeline.as_mut().unwrap());
}

// Source: upstream/packages/movieclip/src/movieClip.ts:156 (sha256:66e2c3b255a10160ba7acfdcde18b74d98499caf154df20a49cda0392153fcc1)
pub fn update_movie_clip(clip: &mut MovieClip, delta_time: f64) -> () {
    if ((clip.data.timeline).clone()).is_none() {
        return;
    }
    update_timeline(clip.data.timeline.as_mut().unwrap(), delta_time);
}
