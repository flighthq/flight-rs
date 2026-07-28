// @generated from upstream/packages/audio/src/audioResource.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::AudioResource;

// Source: upstream/packages/audio/src/audioResource.ts:7 (sha256:846daf9ecaa2d3c6f96d61666c83faddd30028677d0ec9a08f32e73f5e492f7f)
pub fn clone_audio_resource(resource: &AudioResource) -> AudioResource {
    return AudioResource {
        __flight_identity: std::sync::Arc::new(()),
        buffer: (resource.buffer).clone(),
    };
}

// Source: upstream/packages/audio/src/audioResource.ts:11 (sha256:bc6351f76d254d74cddb2c6cc86b9a9fec2492e0561486a890d8edc774971932)
pub fn create_audio_resource(buffer: Option<crate::OpaqueHostValue>) -> AudioResource {
    return AudioResource {
        __flight_identity: std::sync::Arc::new(()),
        buffer: buffer,
    };
}

// Source: upstream/packages/audio/src/audioResource.ts:17 (sha256:990bd167c4b3c9397f67e789dca03fc3da442f44a93647ba86708d81f08548a4)
pub fn dispose_audio_resource(resource: &mut AudioResource) -> () {
    resource.buffer = None;
}

// Source: upstream/packages/audio/src/audioResource.ts:23 (sha256:c61e8b3dbf3c7510a3d5c7f9314f075580f301e7db34087b843fb2e3d65ea59f)
pub fn get_audio_resource_byte_size(resource: &AudioResource) -> f64 {
    let buffer = (resource.buffer).clone();
    return if (buffer).is_some() {
        ((crate::host_value::<crate::OpaqueHostValue>("host.numberOfChannels")
            * crate::host_value::<f64>("host.length"))
            * 4.0_f64)
    } else {
        0.0_f64
    };
}

// Source: upstream/packages/audio/src/audioResource.ts:28 (sha256:84dd2f88884a834139240664f3ea905b04aa16099ca356628c13a589f6ae6aeb)
pub fn get_audio_resource_channel_count(resource: &AudioResource) -> f64 {
    return if ((resource.buffer).clone()).is_some() {
        crate::host_value::<f64>("host.numberOfChannels")
    } else {
        0.0_f64
    };
}

// Source: upstream/packages/audio/src/audioResource.ts:35 (sha256:c0772a047c7befd1adb15afe3298cfc0df1dc67c716525e461d1a893242c60ee)
pub fn get_audio_resource_channel_data(resource: &AudioResource, channel: f64) -> Option<Vec<f32>> {
    let buffer = (resource.buffer).clone();
    if (((buffer).is_none()) || (channel < 0.0_f64))
        || (channel >= crate::host_value::<f64>("host.numberOfChannels"))
    {
        return None;
    }
    return Some(crate::host_value::<Vec<f32>>("host.getChannelData"));
}

// Source: upstream/packages/audio/src/audioResource.ts:41 (sha256:ee0d37b727dddbbc80a5fb9d2004e0939361cb313407f03de7eb4848404ee3c4)
pub fn get_audio_resource_duration(resource: &AudioResource) -> f64 {
    return if ((resource.buffer).clone()).is_some() {
        crate::host_value::<f64>("host.duration")
    } else {
        0.0_f64
    };
}

// Source: upstream/packages/audio/src/audioResource.ts:45 (sha256:40d74a5329b8e937acaef7f6ee1b7ab2404f08483a0111a4de8a3b6843aa3d8d)
pub fn get_audio_resource_sample_rate(resource: &AudioResource) -> f64 {
    return if ((resource.buffer).clone()).is_some() {
        crate::host_value::<f64>("host.sampleRate")
    } else {
        0.0_f64
    };
}

// Source: upstream/packages/audio/src/audioResource.ts:49 (sha256:4df993afb7b3bcb52d5b36f2b63225053f7373d0e5afc5259333fe7fe301c3a3)
pub fn has_audio_resource_buffer(resource: &AudioResource) -> bool {
    return ((resource.buffer).clone()).is_some();
}

// Source: upstream/packages/audio/src/audioResource.ts:53 (sha256:d92f2701f943484a1bbb3173538702177c1b6fbeaf59c17aa0d7d5ed4f94b3ac)
pub fn is_audio_resource_empty(resource: &AudioResource) -> bool {
    return (((resource.buffer).clone()).is_none())
        || (crate::host_value::<f64>("host.length") == 0.0_f64);
}
