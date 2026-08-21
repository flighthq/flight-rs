// @generated from upstream/packages/media/src/audioChannel.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_signals::{create_signal, emit_signal};
use flighthq_types::{AudioChannel, AudioPlayOptions, AudioResource};

// Source: upstream/packages/media/src/audioChannel.ts:4 (sha256:f23eb4ba9c870a0e68beabc318c4a6b9b1b19a0ec9ba4fc034688ab76c43ceaa)
pub fn connect_audio_channel_to_node(
    channel: &AudioChannel,
    destination_node: crate::OpaqueHostValue,
) -> () {
    let mut runtime = (*CHANNEL_RUNTIME.lock().unwrap())
        .iter()
        .find(|(entry_key, _)| entry_key == &(*channel).clone())
        .map(|(_, value)| value.clone());
    if (runtime).is_none() {
        return;
    }
    if ((runtime.as_mut().unwrap().gain_node).clone()).is_some() {
        crate::host_value::<()>("host.disconnect");
        crate::host_value::<()>("host.connect");
    }
    runtime.as_mut().unwrap().destination_node = Some((destination_node).clone());
}

// Source: upstream/packages/media/src/audioChannel.ts:14 (sha256:a85d96960a42bd7afd7f3878b0f6da6bfbaa2e9db38c54852762782fb43e62fa)
pub fn fade_audio_channel_gain(
    channel: &mut AudioChannel,
    target_gain: f64,
    duration_ms: f64,
) -> () {
    let runtime = (*CHANNEL_RUNTIME.lock().unwrap())
        .iter()
        .find(|(entry_key, _)| entry_key == &(*channel).clone())
        .map(|(_, value)| value.clone());
    if ((runtime.as_ref().and_then(|value| (value.gain_node).clone())).is_none())
        || ((runtime.as_ref().and_then(|value| (value.gain_node).clone())).is_none())
    {
        channel.gain = target_gain;
        return;
    }
    let ctx = (runtime.as_ref().unwrap().context).clone();
    let now = crate::host_value::<f64>("host.currentTime");
    crate::host_value::<()>("host.cancelScheduledValues");
    crate::host_value::<()>("host.setValueAtTime");
    crate::host_value::<()>("host.linearRampToValueAtTime");
    channel.gain = target_gain;
}

// Source: upstream/packages/media/src/audioChannel.ts:28 (sha256:d5d565fe72f8848ab37a9038dfdef14fee1d3a03a81a2be68e373ae5666c338b)
pub fn get_audio_channel_current_time(channel: &AudioChannel) -> f64 {
    let runtime = (*CHANNEL_RUNTIME.lock().unwrap())
        .iter()
        .find(|(entry_key, _)| entry_key == &(*channel).clone())
        .map(|(_, value)| value.clone());
    if ((runtime).is_none()) || ((channel.state).clone() != "playing") {
        return channel.current_time;
    }
    return ((crate::host_value::<f64>("host.currentTime") - runtime.as_ref().unwrap().started_at)
        * 1000.0_f64)
        .min(channel.length);
}

// Source: upstream/packages/media/src/audioChannel.ts:34 (sha256:6fa095a38a4be72345172e5adcaba5946b7e3390421a7efd0114623ae1ed2657)
pub fn get_audio_channel_duration(channel: &AudioChannel) -> f64 {
    return channel.length;
}

// Source: upstream/packages/media/src/audioChannel.ts:38 (sha256:de1646e007344359b813b81aef14fa073f73673c930e2218554f6d5c39e00e6d)
pub fn get_audio_channel_input_node(channel: &AudioChannel) -> Option<crate::OpaqueHostValue> {
    let runtime = (*CHANNEL_RUNTIME.lock().unwrap())
        .iter()
        .find(|(entry_key, _)| entry_key == &(*channel).clone())
        .map(|(_, value)| value.clone());
    return runtime
        .as_ref()
        .and_then(|value| (value.source_node).clone());
}

// Source: upstream/packages/media/src/audioChannel.ts:43 (sha256:6613f4a69ab2bbf5569b929430dd1c6d7d0cd97d0fb678791cbf0c1c22872a23)
pub fn get_audio_channel_output_node(channel: &AudioChannel) -> Option<crate::OpaqueHostValue> {
    let runtime = (*CHANNEL_RUNTIME.lock().unwrap())
        .iter()
        .find(|(entry_key, _)| entry_key == &(*channel).clone())
        .map(|(_, value)| value.clone());
    return runtime.as_ref().and_then(|value| (value.gain_node).clone());
}

// Source: upstream/packages/media/src/audioChannel.ts:48 (sha256:bf113c05c0d04df83f90e3e20f76c4c97b858591292d5bb24b4a820335430852)
pub fn is_audio_channel_playing(channel: &AudioChannel) -> bool {
    return ((channel.state).clone() == "playing");
}

// Source: upstream/packages/media/src/audioChannel.ts:52 (sha256:610ba002c28529509be1f8cbddb551580b2a446721952cb86a8fa87e7951c503)
pub fn pause_audio_channel(channel: &mut AudioChannel) -> () {
    if ((channel.state).clone() != "playing") {
        return;
    }
    channel.current_time = get_audio_channel_current_time(channel);
    channel.state = "paused".to_owned();
    stop_active_node((channel).clone(), false);
}

// Source: upstream/packages/media/src/audioChannel.ts:59 (sha256:f6cbb96984f5f4e9d19100c5d47ae4c33cef4c89772305e6a85c22bb4e4f6dcd)
pub fn play_audio_resource(
    context: crate::OpaqueHostValue,
    source: &AudioResource,
    options: Option<AudioPlayOptions>,
) -> Option<AudioChannel> {
    if ((source.buffer).clone()).is_none() {
        return None;
    }
    let mut channel: AudioChannel = AudioChannel {
        __flight_identity: std::sync::Arc::new(()),
        current_time: (options.as_ref().and_then(|value| value.current_time))
            .clone()
            .unwrap_or(0.0_f64),
        gain: (options.as_ref().and_then(|value| value.gain))
            .clone()
            .unwrap_or(1.0_f64),
        length: (crate::host_value::<f64>("host.duration") * 1000.0_f64),
        loops: (options.as_ref().and_then(|value| value.loops))
            .clone()
            .unwrap_or(0.0_f64),
        playback_rate: (options.as_ref().and_then(|value| value.playback_rate))
            .clone()
            .unwrap_or(1.0_f64),
        source: (*source).clone(),
        state: "stopped".to_owned(),
        on_complete: create_signal(),
    };
    {
        let __flight_key = (channel).clone();
        let __flight_value = AudioChannelRuntime {
            __flight_identity: std::sync::Arc::new(()),
            context: (context).clone(),
            destination_node: None,
            gain_node: None,
            loops_remaining: channel.loops,
            source_node: None,
            started_at: 0.0_f64,
        };
        if let Some((_, value)) = (*CHANNEL_RUNTIME.lock().unwrap())
            .iter_mut()
            .find(|(key, _)| key == &__flight_key)
        {
            *value = __flight_value;
        } else {
            (*CHANNEL_RUNTIME.lock().unwrap()).push((__flight_key, __flight_value));
        }
    };
    start_audio_channel((channel).clone());
    return Some((channel).clone());
}

// Source: upstream/packages/media/src/audioChannel.ts:90 (sha256:9a335edf4c2e7a9b4d3c143869fdc4992d4fc80e99220c1bc02cfc29fad2f8c6)
pub fn resume_audio_channel(channel: &mut AudioChannel) -> () {
    if ((channel.state).clone() == "playing") || (((channel.source.buffer).clone()).is_none()) {
        return;
    }
    start_audio_channel((channel).clone());
}

// Source: upstream/packages/media/src/audioChannel.ts:95 (sha256:11acfc206c18491d7e632701e9aebc79db6bb0b1552efb859a20e88871003fa0)
pub fn set_audio_channel_current_time(channel: &mut AudioChannel, value: f64) -> f64 {
    channel.current_time = clamp(value, 0.0_f64, channel.length);
    if ((channel.state).clone() == "playing") {
        stop_active_node((channel).clone(), false);
        start_audio_channel((channel).clone());
    }
    return channel.current_time;
}

// Source: upstream/packages/media/src/audioChannel.ts:104 (sha256:f2a9308d7f64cc6fc6b1705ee6fb0ae5db2041b489135948705402de05e2c15a)
pub fn set_audio_channel_gain(channel: &mut AudioChannel, value: f64) -> f64 {
    channel.gain = value;
    let mut runtime = (*CHANNEL_RUNTIME.lock().unwrap())
        .iter()
        .find(|(entry_key, _)| entry_key == &(*channel).clone())
        .map(|(_, value)| value.clone());
    if ((runtime.as_ref().and_then(|value| (value.gain_node).clone())).is_some())
        && ((runtime.as_ref().and_then(|value| (value.gain_node).clone())).is_some())
    {
        crate::host_set("host.value", value);
    }
    return channel.gain;
}

// Source: upstream/packages/media/src/audioChannel.ts:111 (sha256:a1b9125941e2f8f8d5c320b75a688b6698dbbf514995552eb7cfa4a4297f4ffb)
pub fn set_audio_channel_playback_rate(channel: &mut AudioChannel, value: f64) -> f64 {
    channel.playback_rate = value;
    let mut runtime = (*CHANNEL_RUNTIME.lock().unwrap())
        .iter()
        .find(|(entry_key, _)| entry_key == &(*channel).clone())
        .map(|(_, value)| value.clone());
    if ((runtime
        .as_ref()
        .and_then(|value| (value.source_node).clone()))
    .is_some())
        && ((runtime
            .as_ref()
            .and_then(|value| (value.source_node).clone()))
        .is_some())
    {
        crate::host_set("host.value", value);
    }
    return channel.playback_rate;
}

// Source: upstream/packages/media/src/audioChannel.ts:118 (sha256:69231a09eb24698856d74f9d4d70985613cccf15e3159ec9159249e30b161343)
pub fn stop_audio_channel(channel: &mut AudioChannel) -> () {
    stop_active_node((channel).clone(), false);
    channel.current_time = 0.0_f64;
    channel.state = "stopped".to_owned();
}

// Source: upstream/packages/media/src/audioChannel.ts:124 (sha256:c397ad8edeb6bd3d33d363442549a95a27d6117328d9a0a296655467762d40cc)
#[derive(Clone, Default)]
pub(crate) struct AudioChannelRuntime {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub context: crate::OpaqueHostValue,
    pub destination_node: Option<crate::OpaqueHostValue>,
    pub gain_node: Option<crate::OpaqueHostValue>,
    pub loops_remaining: f64,
    pub source_node: Option<crate::OpaqueHostValue>,
    pub started_at: f64,
}
impl PartialEq for AudioChannelRuntime {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/media/src/audioChannel.ts:135 (sha256:d954d9389bd110c7f68afdb4125d952519b02fe8a38474c22a772e0e954e4d5a)
static CHANNEL_RUNTIME: std::sync::LazyLock<
    std::sync::Mutex<Vec<(AudioChannel, AudioChannelRuntime)>>,
> = std::sync::LazyLock::new(|| std::sync::Mutex::new(Vec::new()));

// Source: upstream/packages/media/src/audioChannel.ts:137 (sha256:db06e21bd7457ab9f5d4382d39620a5ebdeedbf2b395623809a5052623043da0)
fn clamp(value: f64, min: f64, max: f64) -> f64 {
    return ((value).max(min)).min(max);
}

// Source: upstream/packages/media/src/audioChannel.ts:141 (sha256:533f2705c1808847fa640c53a943cffc5ed74bfcf9540baf57096f2d1ed10d70)
fn complete_audio_channel(channel: &mut AudioChannel) -> () {
    let mut runtime = (*CHANNEL_RUNTIME.lock().unwrap())
        .iter()
        .find(|(entry_key, _)| entry_key == &(*channel).clone())
        .map(|(_, value)| value.clone());
    if ((runtime).is_none()) || ((channel.state).clone() != "playing") {
        return;
    }
    if (runtime.as_mut().unwrap().loops_remaining != 0.0_f64) {
        if (runtime.as_mut().unwrap().loops_remaining > 0.0_f64) {
            {
                runtime.as_mut().unwrap().loops_remaining -= 1.0;
                runtime.as_mut().unwrap().loops_remaining
            };
        }
        channel.current_time = 0.0_f64;
        start_audio_channel((channel).clone());
        return;
    }
    runtime.as_mut().unwrap().gain_node = None;
    runtime.as_mut().unwrap().source_node = None;
    channel.current_time = channel.length;
    channel.state = "complete".to_owned();
    emit_signal((channel.on_complete).clone(), ());
}

// Source: upstream/packages/media/src/audioChannel.ts:159 (sha256:60845122ca5d20eb1167fbde76f3897cc3e46e765edbdc04a0f11fb79faf27f0)
fn start_audio_channel(mut channel: AudioChannel) -> () {
    let mut runtime = (*CHANNEL_RUNTIME.lock().unwrap())
        .iter()
        .find(|(entry_key, _)| entry_key == &(channel).clone())
        .map(|(_, value)| value.clone());
    let buffer = (channel.source.buffer).clone();
    if ((runtime).is_none()) || ((buffer).is_none()) {
        return;
    }
    let mut source_node = crate::host_value::<crate::OpaqueHostValue>("host.call");
    let mut gain_node = crate::host_value::<crate::OpaqueHostValue>("host.call");
    let current_time = clamp(channel.current_time, 0.0_f64, channel.length);
    crate::host_set("host.buffer", buffer.as_ref().unwrap());
    crate::host_set("host.value", channel.playback_rate);
    crate::host_set("host.value", channel.gain);
    crate::host_value::<()>("host.connect");
    crate::host_value::<()>("host.connect");
    crate::host_set(
        "host.onended",
        std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let mut channel = channel.clone();
            move || -> () { complete_audio_channel(&mut channel) }
        })
            as Box<dyn FnMut() -> () + Send + 'static>)),
    );
    runtime.as_mut().unwrap().gain_node = Some((gain_node).clone());
    runtime.as_mut().unwrap().source_node = Some((source_node).clone());
    runtime.as_mut().unwrap().started_at =
        (crate::host_value::<f64>("host.currentTime") - (current_time / 1000.0_f64));
    channel.current_time = current_time;
    channel.state = "playing".to_owned();
    crate::host_value::<()>("host.start");
    if (crate::host_value::<String>("host.state") == "suspended") {
        crate::host_value::<()>("host.catch");
    }
}

// Source: upstream/packages/media/src/audioChannel.ts:188 (sha256:594c2ae9134981b42b8389044e8490df12db7221946cf49fff5c8cb1f09997da)
fn stop_active_node(mut channel: AudioChannel, complete: bool) -> () {
    let mut runtime = (*CHANNEL_RUNTIME.lock().unwrap())
        .iter()
        .find(|(entry_key, _)| entry_key == &(channel).clone())
        .map(|(_, value)| value.clone());
    let mut source_node = runtime
        .as_ref()
        .and_then(|value| (value.source_node).clone());
    if (((runtime).is_none()) || ((source_node).is_none())) || ((source_node).is_none()) {
        return;
    }
    runtime.as_mut().unwrap().source_node = None;
    runtime.as_mut().unwrap().gain_node = None;
    crate::host_set(
        "host.onended",
        if complete {
            Some(std::sync::Arc::new(std::sync::Mutex::new(Box::new({
                let mut channel = channel.clone();
                move || -> () { complete_audio_channel(&mut channel) }
            })
                as Box<dyn FnMut() -> () + Send + 'static>)))
        } else {
            None
        },
    );
    crate::host_value::<()>("host.stop");
}
