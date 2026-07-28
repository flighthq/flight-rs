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
    DisplayObject, FrameScript, MOVIE_CLIP_KIND as movie_clip_kind_constant, MovieClip,
    MovieClipData, MovieClipRuntime, MovieClipSignals, TimelineLabel, TimelineSource,
};

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
        (*script).clone(),
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
pub fn create_movie_clip_data(data: Option<MovieClipData>) -> MovieClipData {
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
    return signals;
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
