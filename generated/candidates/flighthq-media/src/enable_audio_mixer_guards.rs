// @generated from upstream/packages/media/src/enableAudioMixerGuards.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::set_audio_bus_mixer_guard;
use flighthq_log::log_once;
use flighthq_types::{AudioBus, AudioBusMixerOperation, LogData, LogDataProvider, LogLevel};

// Source: upstream/packages/media/src/enableAudioMixerGuards.ts:8 (sha256:1fb9bffa5a319bfaa31f94857cfaae8944791bc6ae9c4142ee5881906c9c6ab7)
pub fn disable_audio_mixer_guards() -> () {
    set_audio_bus_mixer_guard(&(None));
}

// Source: upstream/packages/media/src/enableAudioMixerGuards.ts:22 (sha256:6739d4bbc6ce114f9929604623c8ac62194d62ec8d0784acdd08de011002118c)
pub fn enable_audio_mixer_guards() -> () {
    set_audio_bus_mixer_guard(&(warn_on_unmixed_bus));
}

// Source: upstream/packages/media/src/enableAudioMixerGuards.ts:26 (sha256:f0034b7040b4808fb21b12cc43ef83476da7961886bc908cddc1adfc6413d6a3)
#[derive(Clone, Default)]
struct WarnOnUnmixedBusRecord1 {
    __flight_identity: std::sync::Arc<()>,
    message: String,
}
impl PartialEq for WarnOnUnmixedBusRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

fn warn_on_unmixed_bus(operation: AudioBusMixerOperation, bus: &AudioBus) -> () {
    let setter = if (operation == "gain") {
        "setAudioBusGain".to_owned()
    } else {
        if (operation == "mute") {
            "setAudioBusMuted".to_owned()
        } else {
            "setAudioBusPan".to_owned()
        }
    };
    log_once(
        format!("media:unmixed-bus-{}", (operation).clone()),
        LogLevel::Warn,
        &(crate::FlightUnion2::<LogData, LogDataProvider>::A(crate::FlightUnion2::<
            String,
            Vec<(String, crate::FlightValue)>,
        >::B({
            let mut __flight_record = Vec::new();
            __flight_record.push(("message".to_owned(), { let __flight_portable_source = format!("{}: bus \"{}\" belongs to no mixer, so the value was stored but reached no audio node and nothing changed audibly. Add the bus with addAudioBusToMixer(mixer, bus) — or route a channel through it with routeAudioChannelToMixerBus — before setting its properties.", (setter).clone(), (bus.name).clone()); crate::FlightValue::String((&__flight_portable_source).clone()) }));
            __flight_record
        }))),
        Some(("media".to_owned()).clone()),
    );
}
