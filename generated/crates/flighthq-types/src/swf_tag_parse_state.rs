// @generated from upstream/packages/types/src/SwfTagParseState.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    Adjustment, AdvancedBlendMode, AudioResource, BlendMode, Effect, FrameScript,
    GlyphOutlineSource, ImportDiagnostic, MorphShape, RichText, Shape, SwfTagHandlerDispatch,
    Texture2D, TimelineAudioCue, TimelineCue, TimelineLabel,
};

// Source: upstream/packages/types/src/SwfTagParseState.ts:22 (sha256:fcf8a057b428bb363a97a6ad5912c7ac3d47962e690dbfefab26101d7bbe9419)
#[derive(Clone)]
pub struct SwfTagReader {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub bit_position: f64,
    pub end: f64,
    pub pos: f64,
    pub source: Vec<u8>,
    pub valid: bool,
    pub align_to_byte: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>>,
    pub read_encoded_uint32:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> f64 + Send + 'static>>>,
    pub read_fixed8: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> f64 + Send + 'static>>>,
    pub read_signed_bits:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(f64) -> f64 + Send + 'static>>>,
    pub read_string: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> String + Send + 'static>>>,
    pub read_uint8: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> f64 + Send + 'static>>>,
    pub read_uint16: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> f64 + Send + 'static>>>,
    pub read_uint32: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> f64 + Send + 'static>>>,
    pub read_unsigned_bits:
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(f64) -> f64 + Send + 'static>>>,
}
impl PartialEq for SwfTagReader {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/SwfTagParseState.ts:41 (sha256:20530a2ca1b10afbef32a9f74332c48ffbd659f90c5063a8d0f43a0af336c609)
#[derive(Clone, Default)]
pub struct SwfTagRectangle {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub height: f64,
    pub width: f64,
    pub x: f64,
    pub y: f64,
}
impl PartialEq for SwfTagRectangle {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/SwfTagParseState.ts:49 (sha256:e0d6807918edcef940997f4e4301cd2afd3303a37250ad81d664362c828ddbc7)
#[derive(Clone, Default)]
pub struct SwfTagMatrix {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub a: f64,
    pub b: f64,
    pub c: f64,
    pub d: f64,
    pub tx: f64,
    pub ty: f64,
}
impl PartialEq for SwfTagMatrix {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/SwfTagParseState.ts:59 (sha256:bfbef730e755473dab844662ae1d604022c43f6025ec2e755bb96fa0b2969365)
#[derive(Clone, Default)]
pub struct SwfTagPlacement {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub advanced_blend_mode: Option<AdvancedBlendMode>,
    pub alpha: f64,
    pub blend_mode: BlendMode,
    pub character_id: f64,
    pub clip_depth: f64,
    pub color_adjustments: Option<Vec<Adjustment>>,
    pub color_transform_adjustments: Option<Vec<Adjustment>>,
    pub depth: f64,
    pub direct_linkage: Option<String>,
    pub effects: Vec<Effect>,
    pub filter_adjustments: Vec<Adjustment>,
    pub matrix: SwfTagMatrix,
    pub name: Option<String>,
    pub ratio: f64,
}
impl PartialEq for SwfTagPlacement {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/SwfTagParseState.ts:80 (sha256:3ef301b30931e794a433bcb546d5578ededad07495a9953fcffd1de0c7328c14)
#[derive(Clone, Default)]
pub struct SwfTimeline {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub actions: Vec<(f64, FrameScript)>,
    pub cues: Vec<TimelineCue>,
    pub frames: Vec<Vec<(f64, SwfTagPlacement)>>,
    pub labels: Vec<TimelineLabel>,
}
impl PartialEq for SwfTimeline {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/SwfTagParseState.ts:92 (sha256:bb6910ea24b4e1b867e95c29186f2f1d6649f4f9c75cc0114e77d0c46932a089)
#[derive(Clone, Default)]
pub struct SwfTagParseStateRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub codec_id: f64,
    pub deblocking: f64,
    pub frame_count: f64,
    pub height: f64,
    pub smoothing: bool,
    pub width: f64,
}
impl PartialEq for SwfTagParseStateRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct SwfTagParseStateRecord2 {
    pub __flight_identity: std::sync::Arc<()>,
    pub bytes: Vec<u8>,
    pub mime_type: String,
    pub resource: AudioResource,
}
impl PartialEq for SwfTagParseStateRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct SwfTagParseStateRecord3 {
    pub __flight_identity: std::sync::Arc<()>,
    pub bytes: Vec<u8>,
    pub mime_type: Option<String>,
    pub sample_rate: f64,
}
impl PartialEq for SwfTagParseStateRecord3 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct SwfTagParseStateRecord4 {
    pub __flight_identity: std::sync::Arc<()>,
    pub character_id: f64,
    pub cue: TimelineAudioCue,
}
impl PartialEq for SwfTagParseStateRecord4 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct SwfTagParseStateRecord5 {
    pub __flight_identity: std::sync::Arc<()>,
    pub class_name: String,
    pub cue: TimelineAudioCue,
}
impl PartialEq for SwfTagParseStateRecord5 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct SwfTagParseStateRecord6 {
    pub __flight_identity: std::sync::Arc<()>,
    pub character_id: f64,
    pub end: f64,
    pub source: Vec<u8>,
    pub start: f64,
    pub version: f64,
}
impl PartialEq for SwfTagParseStateRecord6 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone)]
pub struct SwfTagParseStateRecord7 {
    pub __flight_identity: std::sync::Arc<()>,
    pub character_id: f64,
    pub script: FrameScript,
}
impl PartialEq for SwfTagParseStateRecord7 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct SwfTagParseStateRecord8 {
    pub __flight_identity: std::sync::Arc<()>,
    pub end: SwfTagRectangle,
    pub start: SwfTagRectangle,
}
impl PartialEq for SwfTagParseStateRecord8 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct SwfTagParseStateRecord9 {
    pub __flight_identity: std::sync::Arc<()>,
    pub character_id: f64,
    pub compressed_alpha_bytes: Vec<u8>,
    pub deblocking_parameter_raw: Option<f64>,
    pub height: f64,
    pub width: f64,
}
impl PartialEq for SwfTagParseStateRecord9 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct SwfTagParseStateRecord10 {
    pub __flight_identity: std::sync::Arc<()>,
    pub bytes: Vec<u8>,
    pub mime_type: String,
}
impl PartialEq for SwfTagParseStateRecord10 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct SwfTagParseStateRecord11 {
    pub __flight_identity: std::sync::Arc<()>,
    pub bytes: Vec<u8>,
    pub named: bool,
}
impl PartialEq for SwfTagParseStateRecord11 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct SwfTagParseState {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub abc_blobs: Vec<SwfTagParseStateRecord11>,
    pub background_color: Option<f64>,
    pub character_bounds: Vec<(f64, SwfTagRectangle)>,
    pub defined_characters: Vec<f64>,
    pub dispatch: SwfTagHandlerDispatch,
    pub diagnostics: Option<Vec<ImportDiagnostic>>,
    pub edit_texts: Vec<(
        f64,
        std::sync::Arc<
            std::sync::Mutex<
                Box<
                    dyn FnMut(
                            std::sync::Arc<
                                std::sync::Mutex<Box<dyn FnMut(f64) -> String + Send + 'static>>,
                            >,
                        ) -> RichText
                        + Send
                        + 'static,
                >,
            >,
        >,
    )>,
    pub font_code_points: Vec<(f64, Vec<f64>)>,
    pub font_names: Vec<(f64, String)>,
    pub font_outline_sources: Vec<(f64, GlyphOutlineSource)>,
    pub images: Vec<(f64, SwfTagParseStateRecord10)>,
    pub image_textures: Vec<(f64, Vec<(String, Texture2D)>)>,
    pub jpeg_alpha_payloads: Vec<(f64, SwfTagParseStateRecord9)>,
    pub jpeg_tables: Option<Vec<u8>>,
    pub linkages: Vec<(f64, String)>,
    pub morph_bounds: Vec<(f64, SwfTagParseStateRecord8)>,
    pub morph_shapes: Vec<(
        f64,
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> Option<MorphShape> + Send + 'static>>>,
    )>,
    pub pending_init_actions: Vec<SwfTagParseStateRecord7>,
    pub pending_texts: Vec<SwfTagParseStateRecord6>,
    pub remaining_frame_entries: f64,
    pub scaling_grids: Vec<(f64, SwfTagRectangle)>,
    pub shapes: Vec<(f64, Shape)>,
    pub sound_cues_awaiting_class: Vec<SwfTagParseStateRecord5>,
    pub sound_cues_awaiting_rate: Vec<SwfTagParseStateRecord4>,
    pub sound_resources: Vec<(f64, AudioResource)>,
    pub sounds: Vec<(f64, SwfTagParseStateRecord3)>,
    pub sprites: Vec<(f64, SwfTimeline)>,
    pub stream_sounds: Vec<SwfTagParseStateRecord2>,
    pub video_textures: Vec<(f64, Texture2D)>,
    pub videos: Vec<(f64, SwfTagParseStateRecord1)>,
}
impl PartialEq for SwfTagParseState {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/SwfTagParseState.ts:153 (sha256:e9f2ae0f6f38f38fa364166fc8ea28d3277f4c5c93b283bc8a51bf2b093d1bca)
#[derive(Clone, Default)]
pub struct SwfTagTimelineState {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub actions: Vec<(f64, FrameScript)>,
    pub cues: Vec<TimelineCue>,
    pub frames: Vec<Vec<(f64, SwfTagPlacement)>>,
    pub labels: Vec<TimelineLabel>,
    pub placements: Vec<(f64, SwfTagPlacement)>,
    pub stream_chunks: Vec<Vec<u8>>,
    pub stream_format: f64,
    pub stream_start_frame: f64,
}
impl PartialEq for SwfTagTimelineState {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/SwfTagParseState.ts:169 (sha256:e7d54f24a5fc11aa479ee91650a7d7ab8b28e8daefcae2e630ba057b52650cdf)
#[derive(Clone, Default)]
pub struct SwfTagParseResultRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub codec_id: f64,
    pub deblocking: f64,
    pub frame_count: f64,
    pub height: f64,
    pub smoothing: bool,
    pub width: f64,
}
impl PartialEq for SwfTagParseResultRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct SwfTagParseResultRecord2 {
    pub __flight_identity: std::sync::Arc<()>,
    pub bytes: Vec<u8>,
    pub mime_type: String,
    pub resource: AudioResource,
}
impl PartialEq for SwfTagParseResultRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct SwfTagParseResultRecord3 {
    pub __flight_identity: std::sync::Arc<()>,
    pub bytes: Vec<u8>,
    pub mime_type: Option<String>,
    pub sample_rate: f64,
}
impl PartialEq for SwfTagParseResultRecord3 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct SwfTagParseResultRecord4 {
    pub __flight_identity: std::sync::Arc<()>,
    pub character_id: f64,
    pub cue: TimelineAudioCue,
}
impl PartialEq for SwfTagParseResultRecord4 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct SwfTagParseResultRecord5 {
    pub __flight_identity: std::sync::Arc<()>,
    pub class_name: String,
    pub cue: TimelineAudioCue,
}
impl PartialEq for SwfTagParseResultRecord5 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct SwfTagParseResultRecord6 {
    pub __flight_identity: std::sync::Arc<()>,
    pub character_id: f64,
    pub end: f64,
    pub source: Vec<u8>,
    pub start: f64,
    pub version: f64,
}
impl PartialEq for SwfTagParseResultRecord6 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone)]
pub struct SwfTagParseResultRecord7 {
    pub __flight_identity: std::sync::Arc<()>,
    pub character_id: f64,
    pub script: FrameScript,
}
impl PartialEq for SwfTagParseResultRecord7 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct SwfTagParseResultRecord8 {
    pub __flight_identity: std::sync::Arc<()>,
    pub end: SwfTagRectangle,
    pub start: SwfTagRectangle,
}
impl PartialEq for SwfTagParseResultRecord8 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct SwfTagParseResultRecord9 {
    pub __flight_identity: std::sync::Arc<()>,
    pub character_id: f64,
    pub compressed_alpha_bytes: Vec<u8>,
    pub deblocking_parameter_raw: Option<f64>,
    pub height: f64,
    pub width: f64,
}
impl PartialEq for SwfTagParseResultRecord9 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct SwfTagParseResultRecord10 {
    pub __flight_identity: std::sync::Arc<()>,
    pub bytes: Vec<u8>,
    pub mime_type: String,
}
impl PartialEq for SwfTagParseResultRecord10 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct SwfTagParseResultRecord11 {
    pub __flight_identity: std::sync::Arc<()>,
    pub bytes: Vec<u8>,
    pub named: bool,
}
impl PartialEq for SwfTagParseResultRecord11 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
pub struct SwfTagParseResult {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub abc_blobs: Vec<SwfTagParseResultRecord11>,
    pub background_color: Option<f64>,
    pub character_bounds: Vec<(f64, SwfTagRectangle)>,
    pub defined_characters: Vec<f64>,
    pub dispatch: SwfTagHandlerDispatch,
    pub diagnostics: Option<Vec<ImportDiagnostic>>,
    pub edit_texts: Vec<(
        f64,
        std::sync::Arc<
            std::sync::Mutex<
                Box<
                    dyn FnMut(
                            std::sync::Arc<
                                std::sync::Mutex<Box<dyn FnMut(f64) -> String + Send + 'static>>,
                            >,
                        ) -> RichText
                        + Send
                        + 'static,
                >,
            >,
        >,
    )>,
    pub font_code_points: Vec<(f64, Vec<f64>)>,
    pub font_names: Vec<(f64, String)>,
    pub font_outline_sources: Vec<(f64, GlyphOutlineSource)>,
    pub images: Vec<(f64, SwfTagParseResultRecord10)>,
    pub image_textures: Vec<(f64, Vec<(String, Texture2D)>)>,
    pub jpeg_alpha_payloads: Vec<(f64, SwfTagParseResultRecord9)>,
    pub jpeg_tables: Option<Vec<u8>>,
    pub linkages: Vec<(f64, String)>,
    pub morph_bounds: Vec<(f64, SwfTagParseResultRecord8)>,
    pub morph_shapes: Vec<(
        f64,
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> Option<MorphShape> + Send + 'static>>>,
    )>,
    pub pending_init_actions: Vec<SwfTagParseResultRecord7>,
    pub pending_texts: Vec<SwfTagParseResultRecord6>,
    pub remaining_frame_entries: f64,
    pub scaling_grids: Vec<(f64, SwfTagRectangle)>,
    pub shapes: Vec<(f64, Shape)>,
    pub sound_cues_awaiting_class: Vec<SwfTagParseResultRecord5>,
    pub sound_cues_awaiting_rate: Vec<SwfTagParseResultRecord4>,
    pub sound_resources: Vec<(f64, AudioResource)>,
    pub sounds: Vec<(f64, SwfTagParseResultRecord3)>,
    pub sprites: Vec<(f64, SwfTimeline)>,
    pub stream_sounds: Vec<SwfTagParseResultRecord2>,
    pub video_textures: Vec<(f64, Texture2D)>,
    pub videos: Vec<(f64, SwfTagParseResultRecord1)>,
    pub timeline: SwfTimeline,
}
impl PartialEq for SwfTagParseResult {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
