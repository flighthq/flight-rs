// @generated from upstream/packages/media/src/index.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

pub use crate::{
    add_audio_bus_to_mixer, connect_audio_channel_to_node, create_audio_bus, create_audio_mixer,
    destroy_audio_mixer, fade_audio_bus_gain, fade_audio_channel_gain,
    get_audio_channel_current_time, get_audio_channel_duration, get_audio_channel_input_node,
    get_audio_channel_output_node, get_audio_mixer_active_channels, get_video_channel_current_time,
    get_video_channel_duration, get_video_channel_height, get_video_channel_width,
    is_audio_channel_playing, is_video_channel_playing, pause_all_audio_mixer_channels,
    pause_audio_channel, pause_video_channel, play_audio_resource, play_video_resource,
    resume_all_audio_mixer_channels, resume_audio_channel, resume_video_channel,
    route_audio_channel_to_mixer_bus, set_audio_bus_gain, set_audio_bus_muted, set_audio_bus_pan,
    set_audio_channel_current_time, set_audio_channel_gain, set_audio_channel_playback_rate,
    set_audio_mixer_master_gain, set_audio_mixer_master_muted, set_video_channel_current_time,
    set_video_channel_gain, set_video_channel_playback_rate, stop_all_audio_mixer_channels,
    stop_audio_channel, stop_video_channel, unroute_audio_channel_from_mixer_bus,
};
