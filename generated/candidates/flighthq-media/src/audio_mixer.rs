// @generated from upstream/packages/media/src/audioMixer.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    connect_audio_channel_to_node, pause_audio_channel, resume_audio_channel, stop_audio_channel,
};
use flighthq_types::{AudioBus, AudioBusOptions, AudioChannel, AudioMixer, AudioMixerOptions};

// Source: upstream/packages/media/src/audioMixer.ts:5 (sha256:cbc91430c7cc2554baf4fa284f697eddf90c690c609f01c03a576fe72e849012)
pub fn add_audio_bus_to_mixer(mixer: &AudioMixer, bus: &AudioBus) -> () {
    let mut runtime = (*MIXER_RUNTIMES.lock().unwrap())
        .iter()
        .find(|(key, _)| key == &(*mixer).clone())
        .map(|(_, value)| value.clone());
    if (runtime).is_none() {
        return;
    }
    if runtime
        .as_mut()
        .unwrap()
        .bus_gain_nodes
        .iter()
        .any(|(key, _)| key == &(*bus).clone())
    {
        return;
    }
    let mut gain_node = crate::host_value::<()>("host.createGain");
    gain_node.gain.value = if bus.muted { 0.0_f64 } else { bus.gain };
    let mut panner_node: Option<crate::OpaqueHostValue> = None;
    {
        (gain_node.connect)((runtime.as_mut().unwrap().master_gain_node).clone());
    }
    {
        let __flight_key = (*bus).clone();
        let __flight_value = gain_node;
        if let Some((_, value)) = runtime
            .as_mut()
            .unwrap()
            .bus_gain_nodes
            .iter_mut()
            .find(|(key, _)| key == &__flight_key)
        {
            *value = __flight_value;
        } else {
            runtime
                .as_mut()
                .unwrap()
                .bus_gain_nodes
                .push((__flight_key, __flight_value));
        }
    };
    if (panner_node).is_some() {
        {
            let __flight_key = (*bus).clone();
            let __flight_value = (panner_node.as_mut().unwrap()).clone();
            if let Some((_, value)) = runtime
                .as_mut()
                .unwrap()
                .bus_output_nodes
                .iter_mut()
                .find(|(key, _)| key == &__flight_key)
            {
                *value = __flight_value;
            } else {
                runtime
                    .as_mut()
                    .unwrap()
                    .bus_output_nodes
                    .push((__flight_key, __flight_value));
            }
        };
    }
    {
        let __flight_key = (bus.name).clone();
        let __flight_value = (*bus).clone();
        if let Some((_, value)) = runtime
            .as_mut()
            .unwrap()
            .buses
            .iter_mut()
            .find(|(key, _)| key == &__flight_key)
        {
            *value = __flight_value;
        } else {
            runtime
                .as_mut()
                .unwrap()
                .buses
                .push((__flight_key, __flight_value));
        }
    };
    register_bus_in_reverse_map(bus, &runtime.as_mut().unwrap());
}

// Source: upstream/packages/media/src/audioMixer.ts:26 (sha256:f368f046c09b48e8184e8ff3a6dfc144b5d595b25bc662446cdd14b1d489c128)
pub fn create_audio_bus(options: Option<AudioBusOptions>) -> AudioBus {
    return AudioBus {
        __flight_identity: std::sync::Arc::new(()),
        gain: (options.as_ref().and_then(|value| value.gain)).unwrap_or(1.0_f64),
        muted: (options.as_ref().and_then(|value| value.muted)).unwrap_or(false),
        name: (options.as_ref().and_then(|value| (value.name).clone())).unwrap_or("".to_owned()),
        pan: (options.as_ref().and_then(|value| value.pan)).unwrap_or(0.0_f64),
    };
}

// Source: upstream/packages/media/src/audioMixer.ts:35 (sha256:cddba1db7d8332bf41093d3123b64ac9a686c1853d0d3d9f673f5795914c7954)
pub fn create_audio_mixer(
    context: crate::OpaqueHostValue,
    options: Option<AudioMixerOptions>,
) -> AudioMixer {
    let mut master_gain_node = crate::host_value::<()>("host.createGain");
    crate::host_set(
        "host.value",
        (options.as_ref().and_then(|value| value.master_gain)).unwrap_or(1.0_f64),
    );
    crate::host_value::<()>("host.connect");
    let mixer: AudioMixer = AudioMixer {
        __flight_identity: std::sync::Arc::new(()),
        master_gain: (options.as_ref().and_then(|value| value.master_gain)).unwrap_or(1.0_f64),
        master_muted: (options.as_ref().and_then(|value| value.master_muted)).unwrap_or(false),
    };
    {
        let __flight_key = (mixer).clone();
        let __flight_value = AudioMixerRuntime {
            __flight_identity: std::sync::Arc::new(()),
            active_channels: Vec::new(),
            buses: Vec::new(),
            bus_gain_nodes: Vec::new(),
            bus_output_nodes: Vec::new(),
            channel_to_bus: Vec::new(),
            context: (context).clone(),
            master_gain_node: (master_gain_node).clone(),
        };
        if let Some((_, value)) = (*MIXER_RUNTIMES.lock().unwrap())
            .iter_mut()
            .find(|(key, _)| key == &__flight_key)
        {
            *value = __flight_value;
        } else {
            (*MIXER_RUNTIMES.lock().unwrap()).push((__flight_key, __flight_value));
        }
    };
    return mixer;
}

// Source: upstream/packages/media/src/audioMixer.ts:55 (sha256:5c132ab8fc50c6e2118cdcfa1d576ebfcf9603aa63015a42a7197b87f8f506d4)
pub fn destroy_audio_mixer(mixer: &AudioMixer) -> () {
    let mut runtime = (*MIXER_RUNTIMES.lock().unwrap())
        .iter()
        .find(|(key, _)| key == &(*mixer).clone())
        .map(|(_, value)| value.clone());
    if (runtime).is_none() {
        return;
    }
    for channel in ((runtime.as_mut().unwrap().active_channels).clone())
        .iter()
        .cloned()
    {
        stop_audio_channel(&mut channel);
    }
    runtime.as_mut().unwrap().active_channels.clear();
    for panner_node in (runtime
        .as_mut()
        .unwrap()
        .bus_output_nodes
        .iter()
        .map(|(_, value)| value.clone())
        .collect::<Vec<_>>())
    .iter()
    .cloned()
    {
        crate::host_value::<()>("host.disconnect");
    }
    for bus in (runtime
        .as_mut()
        .unwrap()
        .bus_gain_nodes
        .iter()
        .map(|(key, _)| key.clone())
        .collect::<Vec<_>>())
    .iter()
    .cloned()
    {
        unregister_bus_from_reverse_map(&bus, &runtime.as_mut().unwrap());
    }
    for gain_node in (runtime
        .as_mut()
        .unwrap()
        .bus_gain_nodes
        .iter()
        .map(|(_, value)| value.clone())
        .collect::<Vec<_>>())
    .iter()
    .cloned()
    {
        crate::host_value::<()>("host.disconnect");
    }
    crate::host_value::<()>("host.disconnect");
    runtime.as_mut().unwrap().bus_gain_nodes.clear();
    runtime.as_mut().unwrap().bus_output_nodes.clear();
    runtime.as_mut().unwrap().buses.clear();
    {
        let __flight_key = (*mixer).clone();
        if let Some(__flight_index) = (*MIXER_RUNTIMES.lock().unwrap())
            .iter()
            .position(|(key, _)| key == &__flight_key)
        {
            (*MIXER_RUNTIMES.lock().unwrap()).remove(__flight_index);
            true
        } else {
            false
        }
    };
}

// Source: upstream/packages/media/src/audioMixer.ts:72 (sha256:d98c9ab802969b3a35a7c3b69dbbe48dcd2fdea9db12ba93a2f9dd90791ed442)
pub fn fade_audio_bus_gain(
    mixer: &AudioMixer,
    bus: &mut AudioBus,
    target_gain: f64,
    duration_ms: f64,
) -> () {
    let runtime = (*MIXER_RUNTIMES.lock().unwrap())
        .iter()
        .find(|(key, _)| key == &(*mixer).clone())
        .map(|(_, value)| value.clone());
    let gain_node = runtime
        .as_ref()
        .unwrap()
        .bus_gain_nodes
        .as_mut()
        .unwrap()
        .iter()
        .find(|(key, _)| key == &(*bus).clone())
        .map(|(_, value)| value.clone());
    if (gain_node).is_none() {
        bus.gain = target_gain;
        return;
    }
    let now = crate::host_value::<crate::OpaqueHostValue>("host.currentTime");
    crate::host_value::<()>("host.cancelScheduledValues");
    crate::host_value::<()>("host.setValueAtTime");
    crate::host_value::<()>("host.linearRampToValueAtTime");
    bus.gain = target_gain;
}

// Source: upstream/packages/media/src/audioMixer.ts:91 (sha256:6d8358ff910838a3e6fbf516874ef6dfee1231861fd8314c016a2900a0b20e49)
pub fn get_audio_mixer_active_channels(mixer: &AudioMixer) -> Vec<AudioChannel> {
    let runtime = (*MIXER_RUNTIMES.lock().unwrap())
        .iter()
        .find(|(key, _)| key == &(*mixer).clone())
        .map(|(_, value)| value.clone());
    if (runtime).is_none() {
        return vec![];
    }
    return crate::host_value::<Vec<AudioChannel>>("host.Array.from");
}

// Source: upstream/packages/media/src/audioMixer.ts:97 (sha256:73bd1087ef337826d9585d96d7115b7ab94c4f3e9bc6fdc11fd08d8f72082c01)
pub fn pause_all_audio_mixer_channels(mixer: &AudioMixer) -> () {
    let runtime = (*MIXER_RUNTIMES.lock().unwrap())
        .iter()
        .find(|(key, _)| key == &(*mixer).clone())
        .map(|(_, value)| value.clone());
    if (runtime).is_none() {
        return;
    }
    for channel in ((runtime.as_ref().unwrap().active_channels).clone())
        .iter()
        .cloned()
    {
        if (channel.state == "playing") {
            pause_audio_channel(&mut channel);
        }
    }
}

// Source: upstream/packages/media/src/audioMixer.ts:106 (sha256:d83f3a968a8edca6dc41b0b52586f87e5b35f5bb2627c9e0b3577d751bc81fb9)
pub fn resume_all_audio_mixer_channels(mixer: &AudioMixer) -> () {
    let runtime = (*MIXER_RUNTIMES.lock().unwrap())
        .iter()
        .find(|(key, _)| key == &(*mixer).clone())
        .map(|(_, value)| value.clone());
    if (runtime).is_none() {
        return;
    }
    for channel in ((runtime.as_ref().unwrap().active_channels).clone())
        .iter()
        .cloned()
    {
        if (channel.state == "paused") {
            resume_audio_channel(&mut channel);
        }
    }
}

// Source: upstream/packages/media/src/audioMixer.ts:116 (sha256:751d1444f5cd1ffb9611cb6ffef837484f41d3590fe83c1cda2cee93fffe55a7)
pub fn route_audio_channel_to_mixer_bus(
    mixer: &AudioMixer,
    channel: &AudioChannel,
    bus: &AudioBus,
) -> () {
    let mut runtime = (*MIXER_RUNTIMES.lock().unwrap())
        .iter()
        .find(|(key, _)| key == &(*mixer).clone())
        .map(|(_, value)| value.clone());
    if (runtime).is_none() {
        return;
    }
    add_audio_bus_to_mixer(mixer, bus);
    {
        let __flight_value = (*channel).clone();
        if !runtime
            .as_mut()
            .unwrap()
            .active_channels
            .contains(&__flight_value)
        {
            runtime
                .as_mut()
                .unwrap()
                .active_channels
                .push(__flight_value);
        }
    };
    {
        let __flight_key = (*channel).clone();
        let __flight_value = (*bus).clone();
        if let Some((_, value)) = runtime
            .as_mut()
            .unwrap()
            .channel_to_bus
            .iter_mut()
            .find(|(key, _)| key == &__flight_key)
        {
            *value = __flight_value;
        } else {
            runtime
                .as_mut()
                .unwrap()
                .channel_to_bus
                .push((__flight_key, __flight_value));
        }
    };
    let bus_gain_node = runtime
        .as_mut()
        .unwrap()
        .bus_gain_nodes
        .iter()
        .find(|(key, _)| key == &(*bus).clone())
        .map(|(_, value)| value.clone());
    if (bus_gain_node).is_some() {
        connect_audio_channel_to_node(channel, (bus_gain_node.as_ref().unwrap()).clone());
    }
}

// Source: upstream/packages/media/src/audioMixer.ts:130 (sha256:e48c4a387165bb2ce09ebabfb2bf48a7573ce9f37fac1f6e6c8f238a7f49d7f2)
pub fn set_audio_bus_gain(bus: &mut AudioBus, value: f64) -> f64 {
    bus.gain = value;
    update_bus_gain_node(bus);
    return bus.gain;
}

// Source: upstream/packages/media/src/audioMixer.ts:140 (sha256:90ca5ffb1f7f64b833d54ca8ee11d82024a7c9652700671bd7f2db16864ceacf)
pub fn set_audio_bus_muted(bus: &mut AudioBus, muted: bool) -> bool {
    bus.muted = muted;
    update_bus_gain_node(bus);
    return bus.muted;
}

// Source: upstream/packages/media/src/audioMixer.ts:146 (sha256:455ba8a456e80be3b68e9fa895dba6ad69f3298689230d6a7a12bb3c9c9bc83c)
pub fn set_audio_bus_pan(bus: &mut AudioBus, value: f64) -> f64 {
    bus.pan = clamp(value, (-1.0_f64), 1.0_f64);
    update_bus_panner_node(bus);
    return bus.pan;
}

// Source: upstream/packages/media/src/audioMixer.ts:152 (sha256:eeb07a8ad2b156389ee0f75a68c71671e5056a133487e5f901397ea9e9b57f57)
pub fn set_audio_mixer_master_gain(mixer: &mut AudioMixer, value: f64) -> f64 {
    mixer.master_gain = value;
    let mut runtime = (*MIXER_RUNTIMES.lock().unwrap())
        .iter()
        .find(|(key, _)| key == &(*mixer).clone())
        .map(|(_, value)| value.clone());
    if (runtime).is_some() {
        runtime.as_mut().unwrap().master_gain_node.gain.value = if mixer.master_muted {
            crate::OpaqueHostValue::Number(0.0_f64)
        } else {
            value
        };
    }
    return mixer.master_gain;
}

// Source: upstream/packages/media/src/audioMixer.ts:161 (sha256:854650f3dadc2a0fbe432147b180296dd841c3ccd157aa80c69effb0f7d49e0b)
pub fn set_audio_mixer_master_muted(mixer: &mut AudioMixer, muted: bool) -> bool {
    mixer.master_muted = muted;
    let mut runtime = (*MIXER_RUNTIMES.lock().unwrap())
        .iter()
        .find(|(key, _)| key == &(*mixer).clone())
        .map(|(_, value)| value.clone());
    if (runtime).is_some() {
        runtime.as_mut().unwrap().master_gain_node.gain.value = if muted {
            crate::OpaqueHostValue::Number(0.0_f64)
        } else {
            mixer.master_gain
        };
    }
    return mixer.master_muted;
}

// Source: upstream/packages/media/src/audioMixer.ts:170 (sha256:9899b6436cccbf3a8c9a8fcd98cdc139e3fe8aea7c4e9f1e02e31ed876aaa4af)
pub fn stop_all_audio_mixer_channels(mixer: &AudioMixer) -> () {
    let mut runtime = (*MIXER_RUNTIMES.lock().unwrap())
        .iter()
        .find(|(key, _)| key == &(*mixer).clone())
        .map(|(_, value)| value.clone());
    if (runtime).is_none() {
        return;
    }
    for channel in ((runtime.as_mut().unwrap().active_channels).clone())
        .iter()
        .cloned()
    {
        channel.state = "stopped";
        channel.current_time = 0.0_f64;
    }
    runtime.as_mut().unwrap().active_channels.clear();
}

// Source: upstream/packages/media/src/audioMixer.ts:180 (sha256:e64a083af52fcd83cd7cf2f4251857eb59ac9c80d10d9c3bfe308343b7a95ab5)
pub fn unroute_audio_channel_from_mixer_bus(mixer: &AudioMixer, channel: &AudioChannel) -> () {
    let mut runtime = (*MIXER_RUNTIMES.lock().unwrap())
        .iter()
        .find(|(key, _)| key == &(*mixer).clone())
        .map(|(_, value)| value.clone());
    if (runtime).is_none() {
        return;
    }
    {
        let __flight_value = (*channel).clone();
        if let Some(__flight_index) = runtime
            .as_mut()
            .unwrap()
            .active_channels
            .iter()
            .position(|item| item == &__flight_value)
        {
            runtime
                .as_mut()
                .unwrap()
                .active_channels
                .remove(__flight_index);
            true
        } else {
            false
        }
    };
    {
        let __flight_key = (*channel).clone();
        if let Some(__flight_index) = runtime
            .as_mut()
            .unwrap()
            .channel_to_bus
            .iter()
            .position(|(key, _)| key == &__flight_key)
        {
            runtime
                .as_mut()
                .unwrap()
                .channel_to_bus
                .remove(__flight_index);
            true
        } else {
            false
        }
    };
    connect_audio_channel_to_node(
        channel,
        crate::host_value::<crate::OpaqueHostValue>("host.destination"),
    );
}

// Source: upstream/packages/media/src/audioMixer.ts:189 (sha256:09ec725a7aec9b80c6e1a0b3594491e2536780c0f42513c34b6317b2d2a0dc73)
#[derive(Clone, Default)]
struct AudioMixerRuntime {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub active_channels: Vec<AudioChannel>,
    pub buses: Vec<(String, AudioBus)>,
    pub bus_gain_nodes: Vec<(AudioBus, crate::OpaqueHostValue)>,
    pub bus_output_nodes: Vec<(AudioBus, crate::OpaqueHostValue)>,
    pub channel_to_bus: Vec<(AudioChannel, AudioBus)>,
    pub context: crate::OpaqueHostValue,
    pub master_gain_node: crate::OpaqueHostValue,
}
impl PartialEq for AudioMixerRuntime {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/media/src/audioMixer.ts:199 (sha256:d2786a5fc7b0d1c5e85b75905f8d0ed9637dd464ff29a6c9f5bc6003e0fc3b8e)
static MIXER_RUNTIMES: std::sync::LazyLock<std::sync::Mutex<Vec<(AudioMixer, AudioMixerRuntime)>>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(Vec::new()));

// Source: upstream/packages/media/src/audioMixer.ts:204 (sha256:74d426cd4d919324f205d44b54d9f2519db4d02d33a088097e0aeba9dd441a7c)
static BUS_TO_MIXER_RUNTIMES: std::sync::LazyLock<
    std::sync::Mutex<Vec<(AudioBus, Vec<AudioMixerRuntime>)>>,
> = std::sync::LazyLock::new(|| std::sync::Mutex::new(Vec::new()));

// Source: upstream/packages/media/src/audioMixer.ts:206 (sha256:db06e21bd7457ab9f5d4382d39620a5ebdeedbf2b395623809a5052623043da0)
fn clamp(value: f64, min: f64, max: f64) -> f64 {
    return ((value).max(min)).min(max);
}

// Source: upstream/packages/media/src/audioMixer.ts:210 (sha256:5537fda228f9c36201fbde1dcc5df10e1e53ef0592072a3fc8a1be3e22b2274d)
fn register_bus_in_reverse_map(bus: &AudioBus, runtime: &AudioMixerRuntime) -> () {
    let mut runtimes = (*BUS_TO_MIXER_RUNTIMES.lock().unwrap())
        .iter()
        .find(|(key, _)| key == &(*bus).clone())
        .map(|(_, value)| value.clone());
    if (runtimes).is_none() {
        runtimes = Some(Vec::new());
        {
            let __flight_key = (*bus).clone();
            let __flight_value = (runtimes).clone().unwrap();
            if let Some((_, value)) = (*BUS_TO_MIXER_RUNTIMES.lock().unwrap())
                .iter_mut()
                .find(|(key, _)| key == &__flight_key)
            {
                *value = __flight_value;
            } else {
                (*BUS_TO_MIXER_RUNTIMES.lock().unwrap()).push((__flight_key, __flight_value));
            }
        };
    }
    {
        let __flight_value = (*runtime).clone();
        if !runtimes.as_mut().unwrap().contains(&__flight_value) {
            runtimes.as_mut().unwrap().push(__flight_value);
        }
    };
}

// Source: upstream/packages/media/src/audioMixer.ts:221 (sha256:8e917e005449b265a3c0c4cd3d61f24ccbb7d4486b3a155d1c246024db4a0928)
fn unregister_bus_from_reverse_map(bus: &AudioBus, runtime: &AudioMixerRuntime) -> () {
    let mut runtimes = (*BUS_TO_MIXER_RUNTIMES.lock().unwrap())
        .iter()
        .find(|(key, _)| key == &(*bus).clone())
        .map(|(_, value)| value.clone());
    if (runtimes).is_none() {
        return;
    }
    {
        let __flight_value = (*runtime).clone();
        if let Some(__flight_index) = runtimes
            .as_mut()
            .unwrap()
            .iter()
            .position(|item| item == &__flight_value)
        {
            runtimes.as_mut().unwrap().remove(__flight_index);
            true
        } else {
            false
        }
    };
    if ((runtimes.as_mut().unwrap().len() as f64) == 0.0_f64) {
        {
            let __flight_key = (*bus).clone();
            if let Some(__flight_index) = (*BUS_TO_MIXER_RUNTIMES.lock().unwrap())
                .iter()
                .position(|(key, _)| key == &__flight_key)
            {
                (*BUS_TO_MIXER_RUNTIMES.lock().unwrap()).remove(__flight_index);
                true
            } else {
                false
            }
        };
    }
}

// Source: upstream/packages/media/src/audioMixer.ts:228 (sha256:d71827a9581145875e218b203cddd87f3ec7bfc5be468a6ad2ad7f6717576b3d)
fn update_bus_gain_node(bus: &AudioBus) -> () {
    let runtimes = (*BUS_TO_MIXER_RUNTIMES.lock().unwrap())
        .iter()
        .find(|(key, _)| key == &(*bus).clone())
        .map(|(_, value)| value.clone());
    if (runtimes).is_none() {
        return;
    }
    for runtime in (runtimes.as_ref().unwrap()).iter().cloned() {
        let mut gain_node = (runtime.bus_gain_nodes.get)(bus);
        if (gain_node).is_some() {
            gain_node.gain.value = if bus.muted { 0.0_f64 } else { bus.gain };
        }
    }
}

// Source: upstream/packages/media/src/audioMixer.ts:239 (sha256:a56a5fa909a94277e401305efcd30b243741e8395bb403dfdae98f5fe4cd6dca)
fn update_bus_panner_node(bus: &AudioBus) -> () {
    let runtimes = (*BUS_TO_MIXER_RUNTIMES.lock().unwrap())
        .iter()
        .find(|(key, _)| key == &(*bus).clone())
        .map(|(_, value)| value.clone());
    if (runtimes).is_none() {
        return;
    }
    for runtime in (runtimes.as_ref().unwrap()).iter().cloned() {
        let mut panner_node = (runtime.bus_output_nodes.get)(bus);
        if ((panner_node).is_some()) && (false) {
            panner_node.pan.value = bus.pan;
        }
    }
}
