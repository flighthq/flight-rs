// @generated from upstream/packages/types/src/HostVideo.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{HostImageSource, VideoResourceLoadOptions};

// Source: upstream/packages/types/src/HostVideo.ts:4 (sha256:5c740536db22550acdbfdd0738df4158df1c7db26f94dd490d4aa476fef5406a)
#[derive(Clone)]
pub struct HostVideoCapability {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub add_ended_listener: Option<
        std::sync::Arc<
            std::sync::Mutex<
                Box<
                    dyn FnMut(
                            HostImageSource,
                            std::sync::Arc<
                                std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>,
                            >,
                        ) -> ()
                        + Send
                        + 'static,
                >,
            >,
        >,
    >,
    pub attach_stream: Option<
        std::sync::Arc<
            std::sync::Mutex<
                Box<dyn FnMut(crate::FlightValue) -> Option<HostImageSource> + Send + 'static>,
            >,
        >,
    >,
    pub can_play_type:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(String) -> bool + Send + 'static>>>,
    pub create_object_url: Option<
        std::sync::Arc<
            std::sync::Mutex<Box<dyn FnMut(crate::OpaqueHostValue) -> String + Send + 'static>>,
        >,
    >,
    pub create_video_element: Option<
        std::sync::Arc<
            std::sync::Mutex<Box<dyn FnMut() -> Option<HostImageSource> + Send + 'static>>,
        >,
    >,
    pub get_current_time: Option<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(HostImageSource) -> f64 + Send + 'static>>>,
    >,
    pub get_duration: Option<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(HostImageSource) -> f64 + Send + 'static>>>,
    >,
    pub get_height: Option<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(HostImageSource) -> f64 + Send + 'static>>>,
    >,
    pub get_loop: Option<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(HostImageSource) -> bool + Send + 'static>>>,
    >,
    pub get_muted: Option<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(HostImageSource) -> bool + Send + 'static>>>,
    >,
    pub get_playback_rate: Option<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(HostImageSource) -> f64 + Send + 'static>>>,
    >,
    pub get_volume: Option<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(HostImageSource) -> f64 + Send + 'static>>>,
    >,
    pub get_width: Option<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(HostImageSource) -> f64 + Send + 'static>>>,
    >,
    pub is_ready: Option<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(HostImageSource) -> bool + Send + 'static>>>,
    >,
    pub load_url: Option<
        std::sync::Arc<
            std::sync::Mutex<
                Box<
                    dyn FnMut(
                            String,
                            Option<VideoResourceLoadOptions>,
                            Option<crate::OpaqueHostValue>,
                        ) -> crate::FlightTask<HostImageSource>
                        + Send
                        + 'static,
                >,
            >,
        >,
    >,
    pub pause: Option<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(HostImageSource) -> () + Send + 'static>>>,
    >,
    pub play: Option<
        std::sync::Arc<
            std::sync::Mutex<
                Box<dyn FnMut(HostImageSource) -> crate::FlightTask<()> + Send + 'static>,
            >,
        >,
    >,
    pub release_element: Option<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(HostImageSource) -> () + Send + 'static>>>,
    >,
    pub remove_ended_listener: Option<
        std::sync::Arc<
            std::sync::Mutex<
                Box<
                    dyn FnMut(
                            HostImageSource,
                            std::sync::Arc<
                                std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>,
                            >,
                        ) -> ()
                        + Send
                        + 'static,
                >,
            >,
        >,
    >,
    pub revoke_object_url:
        Option<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(String) -> () + Send + 'static>>>>,
    pub set_current_time: Option<
        std::sync::Arc<
            std::sync::Mutex<Box<dyn FnMut(HostImageSource, f64) -> () + Send + 'static>>,
        >,
    >,
    pub set_loop: Option<
        std::sync::Arc<
            std::sync::Mutex<Box<dyn FnMut(HostImageSource, bool) -> () + Send + 'static>>,
        >,
    >,
    pub set_muted: Option<
        std::sync::Arc<
            std::sync::Mutex<Box<dyn FnMut(HostImageSource, bool) -> () + Send + 'static>>,
        >,
    >,
    pub set_playback_rate: Option<
        std::sync::Arc<
            std::sync::Mutex<Box<dyn FnMut(HostImageSource, f64) -> () + Send + 'static>>,
        >,
    >,
    pub set_volume: Option<
        std::sync::Arc<
            std::sync::Mutex<Box<dyn FnMut(HostImageSource, f64) -> () + Send + 'static>>,
        >,
    >,
}
impl PartialEq for HostVideoCapability {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
