// @generated from upstream/packages/types/src/AudioResourceReference.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{AudioResource, ResourceResolutionState};

// Source: upstream/packages/types/src/AudioResourceReference.ts:25 (sha256:cc810c889c3bc5069b98a0511366785d14246682e7847ca5a6aadc761ab82d1b)
#[derive(Clone, Default)]
pub struct AudioResourceReferenceKindValues {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub embedded: String,
    pub external: String,
}
impl PartialEq for AudioResourceReferenceKindValues {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub static AUDIO_RESOURCE_REFERENCE_KIND: std::sync::LazyLock<AudioResourceReferenceKindValues> =
    std::sync::LazyLock::new(|| AudioResourceReferenceKindValues {
        __flight_identity: std::sync::Arc::new(()),
        embedded: "Embedded".to_owned(),
        external: "External".to_owned(),
    });

// Source: upstream/packages/types/src/AudioResourceReference.ts:30 (sha256:6c56e8501073d7f475a25bbca4584d9799304bae65fc188da7b8ccfe4787ad63)
pub type AudioResourceReferenceKind = String;

// Source: upstream/packages/types/src/AudioResourceReference.ts:32 (sha256:42cbb69f81416a95f97163e52582c3d6ff3a430d7fcac9a1555acbdc8c3b753e)
#[derive(Clone, Default)]
pub struct AudioResourceFailureKindValues {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub error: String,
    pub unavailable: String,
}
impl PartialEq for AudioResourceFailureKindValues {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub static AUDIO_RESOURCE_FAILURE_KIND: std::sync::LazyLock<AudioResourceFailureKindValues> =
    std::sync::LazyLock::new(|| AudioResourceFailureKindValues {
        __flight_identity: std::sync::Arc::new(()),
        error: "Error".to_owned(),
        unavailable: "Unavailable".to_owned(),
    });

// Source: upstream/packages/types/src/AudioResourceReference.ts:37 (sha256:6020e15889d3fbca8bfe6dbf22d6082500d08a7886ba2629dd3dea4abdd9c5ee)
pub type AudioResourceFailureKind = String;

// Source: upstream/packages/types/src/AudioResourceReference.ts:41 (sha256:f56c115f23cbca0888a011b52611f386b26e7d3107bda8aac7b9ed073138ffe4)
#[derive(Clone, Default)]
pub struct AudioResourceFailure {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: AudioResourceFailureKind,
    pub message: String,
    pub name: Option<String>,
}
impl PartialEq for AudioResourceFailure {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/AudioResourceReference.ts:47 (sha256:7d5c0bf0e17b2508a4b0f6b32c0d762b37e045b839c912a9b103e8254ae186bd)
#[derive(Clone, Default)]
pub struct AudioResourceReferenceBase {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub failure: Option<AudioResourceFailure>,
    pub mime_type: Option<String>,
    pub name: Option<String>,
    pub resource: AudioResource,
    pub state: ResourceResolutionState,
}
impl PartialEq for AudioResourceReferenceBase {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/AudioResourceReference.ts:69 (sha256:f53d303e14071b87725ebb2c99ea470ca01a53a413735dde4c715ae89d3e5bf3)
#[derive(Clone, Default)]
pub struct EmbeddedAudioResourceReference {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub failure: Option<AudioResourceFailure>,
    pub mime_type: Option<String>,
    pub name: Option<String>,
    pub resource: AudioResource,
    pub state: ResourceResolutionState,
    pub kind: String,
    pub bytes: Vec<u8>,
}
impl PartialEq for EmbeddedAudioResourceReference {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/AudioResourceReference.ts:77 (sha256:d1c93c85b66768070e33c96c080babc3af694bb0428e34adeb2aa7c27e44137b)
#[derive(Clone, Default)]
pub struct ExternalAudioResourceReference {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub failure: Option<AudioResourceFailure>,
    pub mime_type: Option<String>,
    pub name: Option<String>,
    pub resource: AudioResource,
    pub state: ResourceResolutionState,
    pub kind: String,
    pub uri: String,
    pub base_path: Option<String>,
}
impl PartialEq for ExternalAudioResourceReference {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/AudioResourceReference.ts:83 (sha256:e829179f2d0ae5ba3989f9ede6a35dcc5f69137edfe533f4b6d56dad734e0ae9)
pub type AudioResourceReference =
    crate::FlightUnion2<EmbeddedAudioResourceReference, ExternalAudioResourceReference>;

// Source: upstream/packages/types/src/AudioResourceReference.ts:87 (sha256:8715bee4af872c1ea7849d1a7641e1f73b7bbf544f3ea1cca59a6c05e1b526e4)
pub type AudioResourceFetch = std::sync::Arc<
    std::sync::Mutex<
        Box<
            dyn FnMut(
                    ExternalAudioResourceReference,
                    crate::OpaqueHostValue,
                ) -> crate::FlightTask<Option<AudioResource>>
                + Send
                + 'static,
        >,
    >,
>;

// Source: upstream/packages/types/src/AudioResourceReference.ts:97 (sha256:47d2f9f203444fef53b694422142ba10b3d579213aa80733a93a072d786f847a)
pub type AudioDecoder = std::sync::Arc<
    std::sync::Mutex<
        Box<
            dyn FnMut(
                    Vec<u8>,
                    String,
                    crate::OpaqueHostValue,
                ) -> crate::FlightTask<Option<AudioResource>>
                + Send
                + 'static,
        >,
    >,
>;

// Source: upstream/packages/types/src/AudioResourceReference.ts:99 (sha256:2eb993a413e1bfece1f3f7ff265508471b86e183624ffcbc6754bdafac3ca700)
#[derive(Clone, Default)]
pub struct AudioResourceReferenceResolutionExplanation {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub failure: Option<AudioResourceFailure>,
    pub kind: AudioResourceReferenceKind,
    pub retryable: bool,
    pub state: ResourceResolutionState,
}
impl PartialEq for AudioResourceReferenceResolutionExplanation {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
