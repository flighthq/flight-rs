// @generated from upstream/packages/types/src/VideoResource.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{EntityRuntime, HostImageSource, Signal};

// Source: upstream/packages/types/src/VideoResource.ts:5 (sha256:c2942df01a5899c8fa06bb098c34069f1c2c62ae9d8d67db1c345b944e3cdaef)
pub type VideoChannelState = String;

// Source: upstream/packages/types/src/VideoResource.ts:7 (sha256:42b446bdc8535444383315e571ffb953e0a5ea68a5548b3819407eafdff6e2ad)
#[derive(Clone)]
pub struct VideoChannel {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub current_time: f64,
    pub gain: f64,
    pub length: f64,
    pub loops: f64,
    pub muted: bool,
    pub playback_rate: f64,
    pub source: Option<VideoResource>,
    pub state: VideoChannelState,
    pub on_complete:
        Signal<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>>,
}
impl PartialEq for VideoChannel {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/VideoResource.ts:20 (sha256:c24ea8ca07cc852b9c31dff22d313919cb474b76f72883496272ba045776c236)
#[derive(Clone, Default)]
pub struct VideoPlayOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub current_time: Option<f64>,
    pub gain: Option<f64>,
    pub loops: Option<f64>,
    pub playback_rate: Option<f64>,
}
impl PartialEq for VideoPlayOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/VideoResource.ts:27 (sha256:d38aa9d51c52b602a5b492a1502a9ef4caeae51a0e3c7779a982764fd1c47e1e)
#[derive(Clone, Default)]
pub struct VideoResource {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub element: Option<HostImageSource>,
    pub object_url: Option<String>,
    pub owns_element: bool,
}
impl PartialEq for VideoResource {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for VideoResource {
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

// Source: upstream/packages/types/src/VideoResource.ts:45 (sha256:4f40c87a867dc2c65bfcd8b5f2b66021d2d15e97b89977e5c19568929bb5da0c)
#[derive(Clone, Default)]
pub struct VideoResourceLoadOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub cross_origin: Option<String>,
    pub muted: Option<bool>,
    pub plays_inline: Option<bool>,
    pub preload: Option<String>,
    pub readiness: Option<String>,
}
impl PartialEq for VideoResourceLoadOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/VideoResource.ts:55 (sha256:041ca805e729322fc47c91f8b88061c91f00961a9aa06f62f8d100014e6e29b3)
#[derive(Clone, Default)]
pub struct VideoResourceUrl {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub url: String,
    pub type_: Option<String>,
}
impl PartialEq for VideoResourceUrl {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
