// @generated from upstream/packages/types/src/TimelineCue.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{AudioResource, EntityRuntime, Timeline};

// Source: upstream/packages/types/src/TimelineCue.ts:29 (sha256:ebfbc127e19d4af6f58c2f5e1fa38003c9b554aa0b2ae97bdce245bf8e522a28)
pub const TIMELINE_AUDIO_CUE_KIND: &'static str = "Audio";

// Source: upstream/packages/types/src/TimelineCue.ts:30 (sha256:7e271e29cd6b3bba07c7c1c1e743604fdc7f4eb7ae5487630cfa7ddc9d42f236)
pub const TIMELINE_GOTO_CUE_KIND: &'static str = "Goto";

// Source: upstream/packages/types/src/TimelineCue.ts:31 (sha256:5acbbe7f4d8e975a39054b7b7edecd516705e90e822f95ff48927244f95f2c83)
pub const TIMELINE_NEXT_FRAME_CUE_KIND: &'static str = "NextFrame";

// Source: upstream/packages/types/src/TimelineCue.ts:32 (sha256:cf639f3659cdb1432640950e72da8f792923d41c55054dd8724b3d93ffdcee1b)
pub const TIMELINE_PLAY_CUE_KIND: &'static str = "Play";

// Source: upstream/packages/types/src/TimelineCue.ts:33 (sha256:bb082807dabf9b7bc5971fe88b4fa4ab46797eca62eeb4f60aa509a40a0e2665)
pub const TIMELINE_PREVIOUS_FRAME_CUE_KIND: &'static str = "PreviousFrame";

// Source: upstream/packages/types/src/TimelineCue.ts:34 (sha256:784354f71717a72532fa9f47f035bf53a49a29e8e7797441644053efad717b47)
pub const TIMELINE_STOP_CUE_KIND: &'static str = "Stop";

// Source: upstream/packages/types/src/TimelineCue.ts:35 (sha256:fc2f70eb202334cc6a7430695adcbce39786eb095e3da2e60aa08cde723c7ab9)
pub const TIMELINE_STREAM_AUDIO_CUE_KIND: &'static str = "StreamAudio";

// Source: upstream/packages/types/src/TimelineCue.ts:40 (sha256:59d54e34d6d51410b48dc7c16f65b414257da3dc8c9d4b735d00413affce9626)
#[derive(Clone, Default)]
pub struct TimelineFrameEntryCauseValues {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub advance: String,
    pub seek: String,
}
impl PartialEq for TimelineFrameEntryCauseValues {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub static TIMELINE_FRAME_ENTRY_CAUSE: std::sync::LazyLock<TimelineFrameEntryCauseValues> =
    std::sync::LazyLock::new(|| TimelineFrameEntryCauseValues {
        __flight_identity: std::sync::Arc::new(()),
        advance: "Advance".to_owned(),
        seek: "Seek".to_owned(),
    });

// Source: upstream/packages/types/src/TimelineCue.ts:47 (sha256:7e00c6c47a064e097d6f7822dabd384647ed33eb65baf422073c1f80f8d6fe50)
pub type TimelineFrameEntryCause = crate::OpaqueHostValue;

// Source: upstream/packages/types/src/TimelineCue.ts:51 (sha256:93603a283185c76159b1151c97613f337a955272d0e40daa5fe37548434f6384)
#[derive(Clone, Default)]
pub struct TimelineCue {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub frame: f64,
    pub kind: String,
    pub target_frame: Option<f64>,
    pub target_label: Option<String>,
    pub gain: f64,
    pub resource: AudioResource,
    pub duration: Option<f64>,
    pub envelope: Vec<TimelineAudioEnvelopePoint>,
    pub loops: f64,
    pub offset: f64,
    pub skip_if_playing: bool,
    pub stop: bool,
}
impl PartialEq for TimelineCue {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/TimelineCue.ts:59 (sha256:b84b46a46d87d845cad1b6498dc11b3daf261913a30261e710912a5b0b0fdfaa)
#[derive(Clone, Default)]
pub struct TimelineAudioEnvelopePoint {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub left_gain: f64,
    pub right_gain: f64,
    pub time: f64,
}
impl PartialEq for TimelineAudioEnvelopePoint {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/TimelineCue.ts:71 (sha256:377abc726388df0405759cb105b7d9a4595770d0c3b4ce711608981605770862)
#[derive(Clone, Default)]
pub struct TimelineAudioCue {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub frame: f64,
    pub kind: String,
    pub target_frame: Option<f64>,
    pub target_label: Option<String>,
    pub gain: f64,
    pub resource: AudioResource,
    pub duration: Option<f64>,
    pub envelope: Vec<TimelineAudioEnvelopePoint>,
    pub loops: f64,
    pub offset: f64,
    pub skip_if_playing: bool,
    pub stop: bool,
}
impl PartialEq for TimelineAudioCue {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/TimelineCue.ts:100 (sha256:6f27cd47d41311c342a1db9e3bea8632983a996b360adcae7addd918e2233a80)
#[derive(Clone, Default)]
pub struct TimelineStreamAudioCue {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub frame: f64,
    pub kind: String,
    pub target_frame: Option<f64>,
    pub target_label: Option<String>,
    pub gain: f64,
    pub resource: AudioResource,
    pub duration: Option<f64>,
    pub envelope: Vec<TimelineAudioEnvelopePoint>,
    pub loops: f64,
    pub offset: f64,
    pub skip_if_playing: bool,
    pub stop: bool,
}
impl PartialEq for TimelineStreamAudioCue {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/TimelineCue.ts:109 (sha256:8ebf8c31185e649c2058fdd1c46f2f6905167cf85aa73003380f1a5a42aef894)
#[derive(Clone, Default)]
pub struct TimelineGotoCue {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub frame: f64,
    pub kind: String,
    pub target_frame: Option<f64>,
    pub target_label: Option<String>,
    pub gain: f64,
    pub resource: AudioResource,
    pub duration: Option<f64>,
    pub envelope: Vec<TimelineAudioEnvelopePoint>,
    pub loops: f64,
    pub offset: f64,
    pub skip_if_playing: bool,
    pub stop: bool,
}
impl PartialEq for TimelineGotoCue {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/TimelineCue.ts:117 (sha256:1e193ea57a2805e47ba18637afdc02a7d3f4db3b938868d4eede298bde167ee1)
#[derive(Clone, Default)]
pub struct TimelinePlaybackCue {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub frame: f64,
    pub kind: String,
    pub target_frame: Option<f64>,
    pub target_label: Option<String>,
    pub gain: f64,
    pub resource: AudioResource,
    pub duration: Option<f64>,
    pub envelope: Vec<TimelineAudioEnvelopePoint>,
    pub loops: f64,
    pub offset: f64,
    pub skip_if_playing: bool,
    pub stop: bool,
}
impl PartialEq for TimelinePlaybackCue {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/TimelineCue.ts:123 (sha256:46212764f81fd5508b34022bf8942f3c9efa45eea22e05410a677b50c63bd9ef)
pub type TimelineCueHandler = std::sync::Arc<
    std::sync::Mutex<
        Box<dyn FnMut(Timeline, TimelineCue, TimelineFrameEntryCause) -> () + Send + 'static>,
    >,
>;

// Source: upstream/packages/types/src/TimelineCue.ts:129 (sha256:8ecfcbe7169bd9bbfc0cadc75a0f649a87c5836f91730d480bf97e287082a4b7)
#[derive(Clone)]
pub struct TimelineCueHandlerEntry {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub dispatch_on_seek: bool,
    pub handle: TimelineCueHandler,
    pub kind: String,
}
impl PartialEq for TimelineCueHandlerEntry {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/TimelineCue.ts:142 (sha256:6068fe0a753deb71b4da0960d7aa2c6c77f3941f5020fcfc3215974be8025cd0)
#[derive(Clone, Default)]
pub struct TimelineCueRegistry {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    pub entries: Vec<TimelineCueHandlerEntry>,
}
impl PartialEq for TimelineCueRegistry {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for TimelineCueRegistry {
    fn __flight_entity_runtime(
        &self,
    ) -> &std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>> {
        &self.__flight_entity_runtime
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
