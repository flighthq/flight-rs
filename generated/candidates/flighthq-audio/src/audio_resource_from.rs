// @generated from upstream/packages/audio/src/audioResourceFrom.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{can_play_audio_type, create_audio_resource, infer_audio_mime_type};
use flighthq_types::{AudioResource, AudioResourceUrl};

// Source: upstream/packages/audio/src/audioResourceFrom.ts:10 (sha256:04fa60e9cedbb2f9d6767b67d9ef02e6719b334d90d4ab636924f9627813ea30)
pub fn create_audio_resource_from_samples(
    channels: &Vec<Vec<f32>>,
    sample_rate: f64,
) -> AudioResource {
    let number_of_channels = (channels.len() as f64);
    let length = if (number_of_channels > 0.0_f64) {
        (channels[0.0_f64 as usize].len() as f64)
    } else {
        0.0_f64
    };
    if (number_of_channels == 0.0_f64) || (length == 0.0_f64) {
        return create_audio_resource(None);
    }
    let buffer = crate::OpaqueHostValue::Object;
    {
        let mut channel = 0.0_f64;
        while (channel < number_of_channels) {
            crate::host_value::<()>("host.copyToChannel");
            {
                channel += 1.0;
                channel
            };
        }
    }
    return create_audio_resource(Some(((buffer).clone()).clone()));
}

// Source: upstream/packages/audio/src/audioResourceFrom.ts:24 (sha256:f62565721d1676f6b065becc6e2abdd9e41b9d327d26c08664159eaf20e4e9fc)
pub fn load_audio_resource_from_base64(
    context: crate::OpaqueHostValue,
    base64: String,
    mime_type: String,
    signal: Option<crate::OpaqueHostValue>,
) -> crate::Promise<AudioResource> {
    Default::default()
}

// Source: upstream/packages/audio/src/audioResourceFrom.ts:36 (sha256:6f31ecce9820d5f8d705caee039a2579bafeb3dca51a82e47de6c043e688642e)
pub fn load_audio_resource_from_blob(
    context: crate::OpaqueHostValue,
    blob: crate::OpaqueHostValue,
    signal: Option<crate::OpaqueHostValue>,
) -> crate::Promise<AudioResource> {
    Default::default()
}

// Source: upstream/packages/audio/src/audioResourceFrom.ts:48 (sha256:d54dd5fbcc30d298fda7b249601018a99e88cb2048c29e17efd9bea7eb5110d5)
pub fn load_audio_resource_from_bytes(
    context: crate::OpaqueHostValue,
    bytes: &Vec<u8>,
    mime_type: Option<String>,
    signal: Option<crate::OpaqueHostValue>,
) -> crate::Promise<AudioResource> {
    Default::default()
}

// Source: upstream/packages/audio/src/audioResourceFrom.ts:60 (sha256:0d18bb89086b9b42f5252d194365bd5f6bfd83020372e1b87f43a80ed4ce8cc4)
pub fn load_audio_resource_from_url(
    context: crate::OpaqueHostValue,
    url: String,
    signal: Option<crate::OpaqueHostValue>,
) -> crate::Promise<AudioResource> {
    Default::default()
}

// Source: upstream/packages/audio/src/audioResourceFrom.ts:75 (sha256:f704e57788fe00ece2e8f7e0e8f89db8a63e3a17ca93c092fb6d1adf17df4c64)
pub fn load_audio_resource_from_urls(
    context: crate::OpaqueHostValue,
    sources: &Vec<AudioResourceUrl>,
    signal: Option<crate::OpaqueHostValue>,
) -> crate::Promise<AudioResource> {
    Default::default()
}

// Source: upstream/packages/audio/src/audioResourceFrom.ts:87 (sha256:5bc9b9e8c05c400d3efed46985f191201409e6071f5577e892e8615d6d862c31)
pub fn select_audio_resource_url(sources: &Vec<AudioResourceUrl>) -> Option<String> {
    for source in (sources).iter().cloned() {
        let type_ = ((source.type_).clone()).unwrap_or(infer_audio_mime_type((source.url).clone()));
        if can_play_audio_type((type_).clone()) {
            return Some((source.url).clone());
        }
    }
    return None;
}
