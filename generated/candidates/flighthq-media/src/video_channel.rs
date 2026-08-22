// @generated from upstream/packages/media/src/videoChannel.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_signals::{create_signal, emit_signal};
use flighthq_types::{VideoChannel, VideoPlayOptions, VideoResource};

// Source: upstream/packages/media/src/videoChannel.ts:4 (sha256:1bc91f11b6699f662170084e957462d0baa20246dc45ddb9fe20f8f553b1f6e8)
pub fn get_video_channel_current_time(channel: &VideoChannel) -> f64 {
    let element = get_video_element(&channel.source);
    if ((element).is_none()) || ((channel.state).clone() != "playing") {
        return channel.current_time;
    }
    return (crate::host_value::<f64>("host.currentTime") * 1000.0_f64);
}

// Source: upstream/packages/media/src/videoChannel.ts:10 (sha256:623ecce17487c660f8b12a5095879f60d9fb747c1ed2c5e25368d5f54e168ea4)
pub fn get_video_channel_duration(channel: &VideoChannel) -> f64 {
    return channel.length;
}

// Source: upstream/packages/media/src/videoChannel.ts:14 (sha256:8fb85dbb461b39d6afa69c10780ec539366b5292db00715e5b093cd9ea5ee358)
pub fn get_video_channel_height(channel: &VideoChannel) -> f64 {
    let element = get_video_element(&channel.source);
    return if (element).is_some() {
        crate::host_value::<f64>("host.videoHeight")
    } else {
        0.0_f64
    };
}

// Source: upstream/packages/media/src/videoChannel.ts:19 (sha256:d7c6bbc9517c3b9141c84f42fee7573971c2a024b9beaef6525e22b2f78613d3)
pub fn get_video_channel_width(channel: &VideoChannel) -> f64 {
    let element = get_video_element(&channel.source);
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

// Source: upstream/packages/media/src/videoChannel.ts:28 (sha256:774bfad7b0e8dc79333003bbb00d9ecd96bb11b398e6d4a88a7912043e04a6a2)
pub fn pause_video_channel(channel: &mut VideoChannel) -> () {
    if ((channel.state).clone() != "playing") {
        return;
    }
    let element = get_video_element(&channel.source);
    if (element).is_none() {
        return;
    }
    channel.current_time = get_video_channel_current_time(channel);
    channel.state = "paused".to_owned();
    crate::host_value::<()>("host.pause");
}

// Source: upstream/packages/media/src/videoChannel.ts:37 (sha256:4be549a95e160d4e9c8fe1fc37eea8482500f1c15c2d1a1dd73257c9e914b44f)
pub fn play_video_resource(
    source: &VideoResource,
    options: Option<VideoPlayOptions>,
) -> Option<VideoChannel> {
    let mut element = get_video_element(source);
    if (element).is_none() {
        return None;
    }
    let runtime = (*VIDEO_CHANNEL_RUNTIMES.lock().unwrap())
        .iter()
        .find(|(entry_key, _)| entry_key == &(element.as_mut().unwrap()).clone())
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
            length: if (crate::host_value::<f64>("host.duration")).is_nan() {
                0.0_f64
            } else {
                (crate::host_value::<f64>("host.duration") * 1000.0_f64)
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
        let __flight_key = (element.as_mut().unwrap()).clone();
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
    start_video_channel((*channel.lock().unwrap()).clone());
    return Some((*channel.lock().unwrap()).clone());
}

// Source: upstream/packages/media/src/videoChannel.ts:70 (sha256:b0e72aec8173e0edcb91bc66cf6ffbd8e666e3a109b8f507b004ad28239853f8)
pub fn resume_video_channel(channel: &mut VideoChannel) -> () {
    if ((channel.state).clone() == "playing") || ((get_video_element(&channel.source)).is_none()) {
        return;
    }
    start_video_channel((channel).clone());
}

// Source: upstream/packages/media/src/videoChannel.ts:75 (sha256:06f37198db2332fed6040f5be4fdbb98c259a149f8edf4a416af7f2fa36d21cd)
pub fn set_video_channel_current_time(channel: &mut VideoChannel, value: f64) -> f64 {
    channel.current_time = clamp(value, 0.0_f64, channel.length);
    let mut element = get_video_element(&channel.source);
    if (element).is_some() {
        crate::host_set("host.currentTime", (channel.current_time / 1000.0_f64));
    }
    return channel.current_time;
}

// Source: upstream/packages/media/src/videoChannel.ts:82 (sha256:8ba5bdeb36b604a1b023bec2c2721b56f834532b21e84518d8a7cdf56a0e88ab)
pub fn set_video_channel_gain(channel: &mut VideoChannel, value: f64) -> f64 {
    channel.gain = value;
    let mut element = get_video_element(&channel.source);
    if (element).is_some() {
        crate::host_set("host.volume", value);
    }
    return channel.gain;
}

// Source: upstream/packages/media/src/videoChannel.ts:89 (sha256:7038d9bbd418026b2f2df54e80a27b2fe3d1d8f1d71dc8e66c984357b8cbb7af)
pub fn set_video_channel_playback_rate(channel: &mut VideoChannel, value: f64) -> f64 {
    channel.playback_rate = value;
    let mut element = get_video_element(&channel.source);
    if (element).is_some() {
        crate::host_set("host.playbackRate", value);
    }
    return channel.playback_rate;
}

// Source: upstream/packages/media/src/videoChannel.ts:96 (sha256:0d23af215f13a6f1bce00ea1a7fc84584c5d2aa056b894425558465501d1ff3e)
pub fn stop_video_channel(channel: &mut VideoChannel) -> () {
    let mut element = get_video_element(&channel.source);
    if (element).is_some() {
        let runtime = (*VIDEO_CHANNEL_RUNTIMES.lock().unwrap())
            .iter()
            .find(|(entry_key, _)| entry_key == &(element.as_mut().unwrap()).clone())
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
pub(crate) struct VideoChannelRuntime {
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

// Source: upstream/packages/media/src/videoChannel.ts:115 (sha256:9cc12b6f226bf885bcfe30622e34271b12dac8665661cde4a6e4ca1f633c0c6f)
fn get_video_element(resource: &VideoResource) -> Option<crate::OpaqueHostValue> {
    return (resource.element).clone();
}

// Source: upstream/packages/media/src/videoChannel.ts:119 (sha256:db06e21bd7457ab9f5d4382d39620a5ebdeedbf2b395623809a5052623043da0)
fn clamp(value: f64, min: f64, max: f64) -> f64 {
    return ((value).max(min)).min(max);
}

// Source: upstream/packages/media/src/videoChannel.ts:123 (sha256:3f15142672660be23dab70413fb49bf8fdb481982441a96ada44727646d90cd7)
fn complete_video_channel(channel: &mut VideoChannel) -> () {
    if ((channel.state).clone() != "playing") {
        return;
    }
    let element = get_video_element(&channel.source);
    let mut runtime = if (element).is_some() {
        (*VIDEO_CHANNEL_RUNTIMES.lock().unwrap())
            .iter()
            .find(|(entry_key, _)| entry_key == &(element.as_ref().unwrap()).clone())
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
        start_video_channel((channel).clone());
        return;
    }
    channel.current_time = channel.length;
    channel.state = "complete".to_owned();
    emit_signal((channel.on_complete).clone(), ());
}

// Source: upstream/packages/media/src/videoChannel.ts:138 (sha256:3b57ad35f820b9c7482cf2d35860db87bfa17eeab1bc553b36ff90a05a80fbab)
fn start_video_channel(mut channel: VideoChannel) -> () {
    let mut element = get_video_element(&channel.source);
    if (element).is_none() {
        return;
    }
    crate::host_set("host.currentTime", (channel.current_time / 1000.0_f64));
    channel.state = "playing".to_owned();
    crate::host_value::<()>("host.catch");
}
