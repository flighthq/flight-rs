// @generated from upstream/packages/animation/src/index.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

pub use crate::{
    accumulate_animation_sample, add_animation_sample, advance_animation_blend_tree,
    advance_animation_crossfade, advance_animation_layer_stack, advance_animation_player,
    advance_animation_state_machine, blend_animation_samples, clone_animation_clip,
    clone_animation_player, clone_animation_track, create_animation_blend_tree,
    create_animation_blend_tree_input, create_animation_blend_tree_layer, create_animation_channel,
    create_animation_clip, create_animation_clip_event, create_animation_crossfade,
    create_animation_layer_stack, create_animation_player, create_animation_root_motion_extractor,
    create_animation_sample_accumulator, create_animation_state_machine,
    create_animation_state_machine_layer, create_animation_state_machine_state,
    create_animation_track, enable_animation_player_signals, extract_animation_root_motion,
    finish_animation_sample, get_animation_clip_duration, get_animation_player_normalized_time,
    get_animation_state_machine_current_state, is_animation_crossfade_complete,
    is_animation_state_machine_transitioning, play_animation_player,
    reset_animation_sample_accumulator, sample_animation_blend_tree,
    sample_animation_blend_tree_channel, sample_animation_clip, sample_animation_crossfade,
    sample_animation_layer_stack, sample_animation_layer_stack_channel,
    sample_animation_state_machine, sample_animation_state_machine_channel, sample_animation_track,
    seek_animation_player, set_animation_blend_tree_input_weight, set_animation_layer_weight,
    stop_animation_player, transition_animation_state_machine, trim_animation_track,
    validate_animation_track,
};
