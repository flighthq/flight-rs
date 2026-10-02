// @generated from upstream/packages/types/src/MediaSession.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{EntityRuntime, Signal};

// Source: upstream/packages/types/src/MediaSession.ts:7 (sha256:165d70f259cafc04182a0625560eaeac80af5cc5825a3f8ea7d7c8aaf2fa89fd)
#[derive(Clone, Default)]
pub struct MediaSessionArtwork {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub src: String,
    pub sizes: Option<String>,
    pub type_: Option<String>,
}
impl PartialEq for MediaSessionArtwork {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/MediaSession.ts:15 (sha256:aee262f51e361dfe8ea521ea2ec6f94a4381baf8678d6ce00fd092f7a3fa562d)
#[derive(Clone, Default)]
pub struct MediaSessionMetadata {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub title: String,
    pub artist: String,
    pub album: String,
    pub artwork: Vec<MediaSessionArtwork>,
}
impl PartialEq for MediaSessionMetadata {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/MediaSession.ts:24 (sha256:b27a26dd5f1d6c5ba85e2d718a4cca89de2eb3575b06c0c04d591c0d0db8c5ed)
pub type MediaSessionAction = String;

// Source: upstream/packages/types/src/MediaSession.ts:38 (sha256:0f48bcd9db61c0a9d9c03f2b69844f40f9a14b13fb84bfeb0b893f02962fdd5d)
#[derive(Clone, Default)]
pub struct MediaSessionActionDetails {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub action: MediaSessionAction,
    pub seek_time: Option<f64>,
    pub seek_offset: Option<f64>,
    pub fast_seek: Option<bool>,
}
impl PartialEq for MediaSessionActionDetails {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/MediaSession.ts:47 (sha256:cc0c479e60d32102532af01cde025911e4d8dc14dc1b96b05e8edb295099ddef)
pub type MediaSessionPlaybackState = String;

// Source: upstream/packages/types/src/MediaSession.ts:51 (sha256:778cd16a82d742b05604332092f1827cab6eb4d6b799756b7c841c1bef9c8cae)
#[derive(Clone, Default)]
pub struct MediaSessionPositionState {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub duration: f64,
    pub playback_rate: f64,
    pub position: f64,
}
impl PartialEq for MediaSessionPositionState {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/MediaSession.ts:59 (sha256:63295dad86b5f55fd055ff987fab8edb449e7038d7e8687ded35651c9b15093b)
pub type MediaSessionOperationBlockReason = String;

// Source: upstream/packages/types/src/MediaSession.ts:69 (sha256:2fdd1562160d0a35d88b6d0ed0470d37189518701cb6909c40f1f5dab1816088)
#[derive(Clone)]
pub struct MediaSessionOperationOutcome<BlockReason = MediaSessionOperationBlockReason> {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub reason: crate::FlightUnion2<String, BlockReason>,
}
impl<BlockReason> PartialEq for MediaSessionOperationOutcome<BlockReason> {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/MediaSession.ts:75 (sha256:dc1a66b539e57d267e453bbc75a404d6132438104a0eb9afbeffa518b7618057)
pub type MediaSessionSetMetadataOutcome = MediaSessionOperationOutcome<String>;

// Source: upstream/packages/types/src/MediaSession.ts:79 (sha256:cc5bd7cb340f724a5477975d36641410146388da756d3be12089043e39c12a88)
pub type MediaSessionClearMetadataOutcome = MediaSessionOperationOutcome<String>;

// Source: upstream/packages/types/src/MediaSession.ts:83 (sha256:58cac66f9e9b5d4e2283e9ac2a2cc345ed6a31da55eb2600453ffeaab7a6fd80)
pub type MediaSessionSetPlaybackStateOutcome = MediaSessionOperationOutcome<String>;

// Source: upstream/packages/types/src/MediaSession.ts:87 (sha256:569f955c1eb7db1b18163643b3e7dbd8164bb73c0a017a40695ce53930f49108)
pub type MediaSessionSetPositionStateOutcome = MediaSessionOperationOutcome<String>;

// Source: upstream/packages/types/src/MediaSession.ts:96 (sha256:b9cb069a51c3928783dd5d984b1bf756724b1cf7a1c9d7e8d9bbad23d798ae0f)
pub type MediaSessionClearPositionStateOutcome = MediaSessionOperationOutcome<String>;

// Source: upstream/packages/types/src/MediaSession.ts:102 (sha256:3f2a918b29c865f468c1de855371e9430f454db40849a9669d6988b053bb7ab3)
#[derive(Clone)]
pub struct HostMediaSessionCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub clear_metadata: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut() -> MediaSessionClearMetadataOutcome + Send + 'static>>,
    >,
    pub clear_position_state: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut() -> MediaSessionClearPositionStateOutcome + Send + 'static>,
        >,
    >,
    pub destroy: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
    pub set_metadata: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(MediaSessionMetadata) -> MediaSessionSetMetadataOutcome + Send + 'static>,
        >,
    >,
    pub set_playback_state: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(MediaSessionPlaybackState) -> MediaSessionSetPlaybackStateOutcome
                    + Send
                    + 'static,
            >,
        >,
    >,
    pub set_position_state: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(MediaSessionPositionState) -> MediaSessionSetPositionStateOutcome
                    + Send
                    + 'static,
            >,
        >,
    >,
}
impl PartialEq for HostMediaSessionCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/MediaSession.ts:115 (sha256:de3d4bc2f15ee67ae49fe7b36bb92692cf589a34be178cb15a85360d8ddf67ab)
#[derive(Clone)]
pub struct HostMediaSessionActionCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub destroy: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
    pub subscribe: std::sync::Arc<
        std::sync::Mutex<
            Box<
                dyn FnMut(
                        MediaSessionAction,
                        std::sync::Arc<
                            std::sync::Mutex<
                                Box<dyn FnMut(MediaSessionActionDetails) -> () + Send + 'static>,
                            >,
                        >,
                    ) -> Option<
                        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
                    > + Send
                    + 'static,
            >,
        >,
    >,
}
impl PartialEq for HostMediaSessionActionCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/MediaSession.ts:129 (sha256:d937aa8423945ae2ed8ddc5bb34765f63aa2d6bcd94723c52a9b6cc3f8809274)
#[derive(Clone)]
pub struct MediaSessionActionSignal {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub action: MediaSessionAction,
    pub on_action: Signal<
        std::sync::Arc<
            std::sync::Mutex<Box<dyn FnMut(MediaSessionActionDetails) -> () + Send + 'static>>,
        >,
    >,
}
impl PartialEq for MediaSessionActionSignal {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for MediaSessionActionSignal {
    fn __flight_entity_runtime(
        &self,
    ) -> &std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>> {
        &self.__flight_entity_runtime
    }
    fn __flight_entity_snapshot(&self) -> &Option<std::sync::Arc<dyn std::any::Any + Send + Sync>> {
        &self.__flight_entity_snapshot
    }
    fn __flight_fresh_clone(&self) -> Self {
        let mut cloned = self.clone();
        cloned.__flight_identity = std::sync::Arc::new(());
        cloned.__flight_entity_runtime = std::sync::Arc::new(std::sync::Mutex::new(
            self.__flight_entity_runtime.lock().unwrap().clone(),
        ));
        cloned
    }
}
