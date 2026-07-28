// @generated from upstream/packages/media/src/videoChannel.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_signals::{create_signal, emit_signal};
use flighthq_types::{VideoChannel, VideoPlayOptions, VideoResource};

// Source: upstream/packages/media/src/videoChannel.ts:4 (sha256:1dc138a482fd0a2f1cc86cbbf7c5162c15f3072960d7b43125e8ba11b5ceca93)
pub fn get_video_channel_current_time(channel: &VideoChannel) -> f64 {
    let element = (channel.source.element).clone();
    if ((element).is_none()) || ((channel.state).clone() != "playing") {
        return channel.current_time;
    }
    return (crate::host_value::<crate::OpaqueHostValue>("host.currentTime") * 1000.0_f64);
}

// Source: upstream/packages/media/src/videoChannel.ts:10 (sha256:623ecce17487c660f8b12a5095879f60d9fb747c1ed2c5e25368d5f54e168ea4)
pub fn get_video_channel_duration(channel: &VideoChannel) -> f64 {
    return channel.length;
}

// Source: upstream/packages/media/src/videoChannel.ts:14 (sha256:1a74f4f800cff648451a5f8ceb5e22b3e7c86490cddae076428cfc9ec9730484)
pub fn get_video_channel_height(channel: &VideoChannel) -> f64 {
    let element = (channel.source.element).clone();
    return if (element).is_some() {
        crate::host_value::<f64>("host.videoHeight")
    } else {
        0.0_f64
    };
}

// Source: upstream/packages/media/src/videoChannel.ts:19 (sha256:83e1cc370e86974fbb9a67ba36565626c67f3b0031cd9f906c938dd366ac252d)
pub fn get_video_channel_width(channel: &VideoChannel) -> f64 {
    let element = (channel.source.element).clone();
    return if (element).is_some() {
        crate::host_value::<f64>("host.videoWidth")
    } else {
        0.0_f64
    };
}

// Source: upstream/packages/media/src/videoChannel.ts:24 (sha256:91e3e7268ffd1cea272b8d18d16f8dbcaacc34499ed4df68e73fe6456d9d05eb)
pub fn is_video_channel_playing(channel: &VideoChannel) -> bool {
    return ((channel.state).clone() == "playing");
}

// Source: upstream/packages/media/src/videoChannel.ts:28 (sha256:e9a77cd9b5e6391c4ca1758168ed1e94ab629c63fbabab7a5e15d481039b5dac)
pub fn pause_video_channel(channel: &mut VideoChannel) -> () {
    if ((channel.state).clone() != "playing") {
        return;
    }
    let element = (channel.source.element).clone();
    if (element).is_none() {
        return;
    }
    channel.current_time = get_video_channel_current_time(channel);
    channel.state = "paused".to_owned();
    crate::host_value::<()>("host.pause");
}

// Source: upstream/packages/media/src/videoChannel.ts:37 (sha256:9c8951e7fcf82370242470c267c6fb564d594fd061fefc7e422ca84bf45013cf)
pub fn play_video_resource(
    source: &mut VideoResource,
    options: Option<VideoPlayOptions>,
) -> Option<VideoChannel> {
    let mut element = (source.element).clone();
    if (element).is_none() {
        return None;
    }
    let runtime = (*VIDEO_CHANNEL_RUNTIMES.lock().unwrap())
        .iter()
        .find(|(key, _)| key == &(element).clone().unwrap())
        .map(|(_, value)| value.clone());
    if (runtime).is_some() {
        crate::host_value::<()>("host.removeEventListener");
    }
    let channel: std::sync::Arc<std::sync::Mutex<VideoChannel>> =
        std::sync::Arc::new(std::sync::Mutex::new(VideoChannel {
            __flight_identity: std::sync::Arc::new(()),
            current_time: (options.as_ref().and_then(|value| value.current_time))
                .unwrap_or(0.0_f64),
            gain: (options.as_ref().and_then(|value| value.gain)).unwrap_or(1.0_f64),
            length: if crate::host_value::<()>("host.call") {
                0.0_f64
            } else {
                (crate::host_value::<crate::OpaqueHostValue>("host.duration") * 1000.0_f64)
            },
            loops: (options.as_ref().and_then(|value| value.loops)).unwrap_or(0.0_f64),
            playback_rate: (options.as_ref().and_then(|value| value.playback_rate))
                .unwrap_or(1.0_f64),
            source: (*source).clone(),
            state: "stopped".to_owned(),
            on_complete: create_signal(),
        }));
    let mut on_ended: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>> =
        std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let mut channel = channel.clone();
            move || -> () { complete_video_channel(&mut (*channel.lock().unwrap())) }
        })
            as Box<dyn FnMut() -> () + Send + 'static>));
    {
        let __flight_key = (element).clone().unwrap();
        let __flight_value = VideoChannelRuntime {
            __flight_identity: std::sync::Arc::new(()),
            loops_remaining: (*channel.lock().unwrap()).loops,
            on_ended: (on_ended).clone(),
        };
        if let Some((_, value)) = (*VIDEO_CHANNEL_RUNTIMES.lock().unwrap())
            .iter_mut()
            .find(|(key, _)| key == &__flight_key)
        {
            *value = __flight_value;
        } else {
            (*VIDEO_CHANNEL_RUNTIMES.lock().unwrap()).push((__flight_key, __flight_value));
        }
    };
    crate::host_set(
        "host.currentTime",
        ((*channel.lock().unwrap()).current_time / 1000.0_f64),
    );
    crate::host_set("host.volume", (*channel.lock().unwrap()).gain);
    crate::host_set(
        "host.playbackRate",
        (*channel.lock().unwrap()).playback_rate,
    );
    crate::host_set("host.loop", false);
    crate::host_value::<()>("host.addEventListener");
    start_video_channel(&mut (*channel.lock().unwrap()));
    return Some((*channel.lock().unwrap()).clone());
}

// Source: upstream/packages/media/src/videoChannel.ts:70 (sha256:ea25dc8f646fa1de911d7ab858b8405ff7adef8d8877c20800c1e0a92a2cb76b)
pub fn resume_video_channel(channel: &mut VideoChannel) -> () {
    if ((channel.state).clone() == "playing") || (((channel.source.element).clone()).is_none()) {
        return;
    }
    start_video_channel(channel);
}

// Source: upstream/packages/media/src/videoChannel.ts:75 (sha256:ddd2f6665848dbb0002ec1cf43938a9e6ec3a1aa5b59a8e9cbb905073a7b09a3)
pub fn set_video_channel_current_time(channel: &mut VideoChannel, value: f64) -> f64 {
    channel.current_time = clamp(value, 0.0_f64, channel.length);
    let mut element = (channel.source.element).clone();
    if (element).is_some() {
        crate::host_set("host.currentTime", (channel.current_time / 1000.0_f64));
    }
    return channel.current_time;
}

// Source: upstream/packages/media/src/videoChannel.ts:82 (sha256:9af4c06559cbee46340c027f99b5b1a8039f2b31f87bc52be84960a23350c377)
pub fn set_video_channel_gain(channel: &mut VideoChannel, value: f64) -> f64 {
    channel.gain = value;
    let mut element = (channel.source.element).clone();
    if (element).is_some() {
        crate::host_set("host.volume", value);
    }
    return channel.gain;
}

// Source: upstream/packages/media/src/videoChannel.ts:89 (sha256:96a4e228eb6990a954ec5e5d89a8155ff5f072fe7830d928f2adf4a708942f39)
pub fn set_video_channel_playback_rate(channel: &mut VideoChannel, value: f64) -> f64 {
    channel.playback_rate = value;
    let mut element = (channel.source.element).clone();
    if (element).is_some() {
        crate::host_set("host.playbackRate", value);
    }
    return channel.playback_rate;
}

// Source: upstream/packages/media/src/videoChannel.ts:96 (sha256:ccf11c83896eac451ca95d68b9568894970a308e598fe15ab6c7285b8fb4374b)
pub fn stop_video_channel(channel: &mut VideoChannel) -> () {
    let mut element = (channel.source.element).clone();
    if (element).is_some() {
        let runtime = (*VIDEO_CHANNEL_RUNTIMES.lock().unwrap())
            .iter()
            .find(|(key, _)| key == &(element.as_mut().unwrap()).clone())
            .map(|(_, value)| value.clone());
        if (runtime).is_some() {
            crate::host_value::<()>("host.removeEventListener");
        }
        crate::host_value::<()>("host.pause");
        crate::host_set("host.currentTime", 0.0_f64);
    }
    channel.current_time = 0.0_f64;
    channel.state = "stopped".to_owned();
}

// Source: upstream/packages/media/src/videoChannel.ts:108 (sha256:8b7c8ce2bf6e9dca60029213cfa615512ec47de22e65b60ae1d96abccf92725f)
#[derive(Clone)]
struct VideoChannelRuntime {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub loops_remaining: f64,
    pub on_ended: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
}
impl PartialEq for VideoChannelRuntime {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/media/src/videoChannel.ts:113 (sha256:67e5fa4583a9841c855af345cc4add946640ebaa3be27d3a3efcc23a7664423a)
static VIDEO_CHANNEL_RUNTIMES: std::sync::LazyLock<
    std::sync::Mutex<Vec<(crate::OpaqueHostValue, VideoChannelRuntime)>>,
> = std::sync::LazyLock::new(|| std::sync::Mutex::new(Vec::new()));

// Source: upstream/packages/media/src/videoChannel.ts:115 (sha256:db06e21bd7457ab9f5d4382d39620a5ebdeedbf2b395623809a5052623043da0)
fn clamp(value: f64, min: f64, max: f64) -> f64 {
    return ((value).max(min)).min(max);
}

// Source: upstream/packages/media/src/videoChannel.ts:119 (sha256:cf0d6cb9b31f09e876e4468f2e37ce504c2c8e9c712a2e82c5258240ace0b2b0)
fn complete_video_channel(channel: &mut VideoChannel) -> () {
    if ((channel.state).clone() != "playing") {
        return;
    }
    let mut runtime = if ((channel.source.element).clone()).is_some() {
        (*VIDEO_CHANNEL_RUNTIMES.lock().unwrap())
            .iter()
            .find(|(key, _)| key == &((channel.source.element).clone()).unwrap())
            .map(|(_, value)| value.clone())
    } else {
        None
    };
    if ((runtime).is_some()) && (runtime.as_mut().unwrap().loops_remaining != 0.0_f64) {
        if (runtime.as_mut().unwrap().loops_remaining > 0.0_f64) {
            {
                runtime.as_mut().unwrap().loops_remaining -= 1.0;
                runtime.as_mut().unwrap().loops_remaining
            };
        }
        channel.current_time = 0.0_f64;
        start_video_channel(channel);
        return;
    }
    channel.current_time = channel.length;
    channel.state = "complete".to_owned();
    emit_signal((channel.on_complete).clone(), ());
}

// Source: upstream/packages/media/src/videoChannel.ts:133 (sha256:8b2c6f6b01ab71a85d6bc29e7931b93bf12e77785f8e1f583a4558363cd65b43)
fn start_video_channel(mut channel: VideoChannel) -> () {
    let mut element = (channel.source.element).clone();
    if (element).is_none() {
        return;
    }
    crate::host_set("host.currentTime", (channel.current_time / 1000.0_f64));
    channel.state = "playing".to_owned();
    crate::host_value::<()>("host.catch");
}
