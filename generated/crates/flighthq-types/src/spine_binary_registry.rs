// @generated from upstream/packages/types/src/SpineBinaryRegistry.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    AnimationChannel, AttachmentSkin2D, Bone2D, ByteReader, ImportDiagnostic,
    Skeleton2DDrawOrderTimeline, Skeleton2DImportAnimation, Slot2D,
};

// Source: upstream/packages/types/src/SpineBinaryRegistry.ts:13 (sha256:4ead0f39dec10140a69115b769d90363466bdc4af0039e259be57fe6fa45e693)
#[derive(Clone, Default)]
pub struct SpineBinarySectionKindValues {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub animations: String,
    pub bones: String,
    pub events: String,
    pub ik_constraints: String,
    pub path_constraints: String,
    pub skins: String,
    pub slots: String,
    pub transform_constraints: String,
}
impl PartialEq for SpineBinarySectionKindValues {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub static SPINE_BINARY_SECTION_KIND: std::sync::LazyLock<SpineBinarySectionKindValues> =
    std::sync::LazyLock::new(|| SpineBinarySectionKindValues {
        __flight_identity: std::sync::Arc::new(()),
        animations: "animations".to_owned(),
        bones: "bones".to_owned(),
        events: "events".to_owned(),
        ik_constraints: "ikConstraints".to_owned(),
        path_constraints: "pathConstraints".to_owned(),
        skins: "skins".to_owned(),
        slots: "slots".to_owned(),
        transform_constraints: "transformConstraints".to_owned(),
    });

// Source: upstream/packages/types/src/SpineBinaryRegistry.ts:24 (sha256:34d1d2a02d55a406d81a745aaa354b220fc2217fcaa2f20b9677b9d122be91b0)
pub type SpineBinarySectionKind = String;

// Source: upstream/packages/types/src/SpineBinaryRegistry.ts:28 (sha256:d77fdbf00daafe8ca38eca3c041551d4b2208f3750d042733f0c5a99b10da3a2)
#[derive(Clone, Default)]
pub struct SpineBinaryTimelineKindValues {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub bone: String,
    pub deform: String,
    pub draw_order: String,
    pub event: String,
    pub ik: String,
    pub path: String,
    pub slot: String,
    pub transform: String,
}
impl PartialEq for SpineBinaryTimelineKindValues {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub static SPINE_BINARY_TIMELINE_KIND: std::sync::LazyLock<SpineBinaryTimelineKindValues> =
    std::sync::LazyLock::new(|| SpineBinaryTimelineKindValues {
        __flight_identity: std::sync::Arc::new(()),
        bone: "bone".to_owned(),
        deform: "deform".to_owned(),
        draw_order: "drawOrder".to_owned(),
        event: "event".to_owned(),
        ik: "ik".to_owned(),
        path: "path".to_owned(),
        slot: "slot".to_owned(),
        transform: "transform".to_owned(),
    });

// Source: upstream/packages/types/src/SpineBinaryRegistry.ts:39 (sha256:97fdc6177d6ff5a6cb247c884c48f0fdca639af1c7419fb8de11b2ab9b21635b)
pub type SpineBinaryTimelineKind = String;

// Source: upstream/packages/types/src/SpineBinaryRegistry.ts:44 (sha256:64ce3823527568b7884a5e6876b5311973415bef6469e4491382b9a64d751cbb)
#[derive(Clone, Default)]
pub struct SpineBinarySectionContext {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub animations: Vec<Skeleton2DImportAnimation>,
    pub attachment_names: Vec<Option<String>>,
    pub bones: Vec<Bone2D>,
    pub diagnostics: Option<Vec<ImportDiagnostic>>,
    pub nonessential: bool,
    pub reader: ByteReader,
    pub registry: SpineBinaryRegistry,
    pub skins: Vec<AttachmentSkin2D>,
    pub slots: Vec<Slot2D>,
    pub strings: Vec<Option<String>>,
}
impl PartialEq for SpineBinarySectionContext {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/SpineBinaryRegistry.ts:57 (sha256:3def17d1f190b6c762244df94f4dc1c71308ecca97a6474942cb9b33f5d5030d)
pub type SpineBinarySectionHandler = std::sync::Arc<
    std::sync::Mutex<Box<dyn FnMut(SpineBinarySectionContext) -> () + Send + 'static>>,
>;

// Source: upstream/packages/types/src/SpineBinaryRegistry.ts:59 (sha256:2577505c55787344ccc2e26d519989347dc11e136dd7ac190d5f32ff69c5b980)
#[derive(Clone)]
pub struct SpineBinarySectionHandlerEntry {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub handle: SpineBinarySectionHandler,
    pub kind: SpineBinarySectionKind,
}
impl PartialEq for SpineBinarySectionHandlerEntry {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/SpineBinaryRegistry.ts:66 (sha256:22f1c5466d4b2e0d0d72d8efb032bd93b684612cc75db52d973bb5fd44eb8cd1)
#[derive(Clone, Default)]
pub struct SpineBinaryTimelineContext {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub channels: Vec<AnimationChannel>,
    pub draw_order: Option<Skeleton2DDrawOrderTimeline>,
    pub section: SpineBinarySectionContext,
    pub unregistered_timeline_counts: Vec<(SpineBinaryTimelineKind, f64)>,
    pub unmodeled_timeline_counts: Vec<(String, f64)>,
}
impl PartialEq for SpineBinaryTimelineContext {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/SpineBinaryRegistry.ts:74 (sha256:77003e40133abff7a670873f37e3168dd978435c6540c99d4c58b4f25b397e2d)
pub type SpineBinaryTimelineHandler = std::sync::Arc<
    std::sync::Mutex<Box<dyn FnMut(SpineBinaryTimelineContext) -> () + Send + 'static>>,
>;

// Source: upstream/packages/types/src/SpineBinaryRegistry.ts:76 (sha256:b3ed8993a2402cbe7810aac652bdca42cba362d41715e89031fd068616329762)
#[derive(Clone)]
pub struct SpineBinaryTimelineHandlerEntry {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub handle: SpineBinaryTimelineHandler,
    pub kind: SpineBinaryTimelineKind,
}
impl PartialEq for SpineBinaryTimelineHandlerEntry {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/SpineBinaryRegistry.ts:83 (sha256:e12f43386d9fc7367bf1bf9b685217d2aa5804d03f665ed269a29855ba3c5eac)
#[derive(Clone, Default)]
pub struct SpineBinaryRegistry {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub section_handlers: Vec<SpineBinarySectionHandlerEntry>,
    pub timeline_handlers: Vec<SpineBinaryTimelineHandlerEntry>,
}
impl PartialEq for SpineBinaryRegistry {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
