// @generated from upstream/packages/types/src/SpineJsonRegistry.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    AnimationChannel, AttachmentSkin2D, Bone2D, ImportDiagnostic, Skeleton2DDrawOrderTimeline,
    Skeleton2DImportAnimation, Slot2D,
};

// Source: upstream/packages/types/src/SpineJsonRegistry.ts:9 (sha256:406cb78b21aa01c77e36226a41034a3727b8d7a06c3700010619c83f261bd632)
#[derive(Clone, Default)]
pub struct SpineJsonSectionKindValues {
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
impl PartialEq for SpineJsonSectionKindValues {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub static SPINE_JSON_SECTION_KIND: std::sync::LazyLock<SpineJsonSectionKindValues> =
    std::sync::LazyLock::new(|| SpineJsonSectionKindValues {
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

// Source: upstream/packages/types/src/SpineJsonRegistry.ts:20 (sha256:4875bf03924118737cd7e40bc6b19c6e593c2f2696df0e1637c12290f2ba569b)
pub type SpineJsonSectionKind = String;

// Source: upstream/packages/types/src/SpineJsonRegistry.ts:22 (sha256:a70909460fbd75374974dde2e9857e07a30096dbcd122a51166075ccfa3b93f5)
#[derive(Clone, Default)]
pub struct SpineJsonTimelineKindValues {
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
impl PartialEq for SpineJsonTimelineKindValues {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub static SPINE_JSON_TIMELINE_KIND: std::sync::LazyLock<SpineJsonTimelineKindValues> =
    std::sync::LazyLock::new(|| SpineJsonTimelineKindValues {
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

// Source: upstream/packages/types/src/SpineJsonRegistry.ts:33 (sha256:651e378e7438fc622c15f4a2989f0cf0fec09a2522118aa0bc5710bf2a7aa667)
pub type SpineJsonTimelineKind = String;

// Source: upstream/packages/types/src/SpineJsonRegistry.ts:35 (sha256:fbaa3cbfd9550c3835e4eff4f979693177d1bf826b5dbfe01237f7988124f16a)
#[derive(Clone, Default)]
pub struct SpineJsonSectionContext {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub animations: Vec<Skeleton2DImportAnimation>,
    pub attachment_names: Vec<Option<String>>,
    pub bones: Vec<Bone2D>,
    pub diagnostics: Option<Vec<ImportDiagnostic>>,
    pub doc: Vec<(String, crate::FlightValue)>,
    pub registry: SpineJsonRegistry,
    pub skins: Vec<AttachmentSkin2D>,
    pub slots: Vec<Slot2D>,
}
impl PartialEq for SpineJsonSectionContext {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/SpineJsonRegistry.ts:46 (sha256:f38f8659370ef385d7bc7541616e5f495999e0b1245752b7acd216983294b0b5)
pub type SpineJsonSectionHandler = std::sync::Arc<
    std::sync::Mutex<Box<dyn FnMut(SpineJsonSectionContext) -> () + Send + 'static>>,
>;

// Source: upstream/packages/types/src/SpineJsonRegistry.ts:48 (sha256:73f1712c53be900afbab45d7cb50cc54ebf8cbec7982b25b9937adfed4f18303)
#[derive(Clone)]
pub struct SpineJsonSectionHandlerEntry {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub handle: SpineJsonSectionHandler,
    pub kind: SpineJsonSectionKind,
}
impl PartialEq for SpineJsonSectionHandlerEntry {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/SpineJsonRegistry.ts:53 (sha256:b13794073d48bdfcc27321c65a396478db36bdd6025d85d41b97857d2c084669)
#[derive(Clone, Default)]
pub struct SpineJsonTimelineContext {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub channels: Vec<AnimationChannel>,
    pub draw_order: Option<Skeleton2DDrawOrderTimeline>,
    pub section: SpineJsonSectionContext,
    pub unregistered_timeline_counts: Vec<(SpineJsonTimelineKind, f64)>,
    pub unmodeled_timeline_counts: Vec<(String, f64)>,
}
impl PartialEq for SpineJsonTimelineContext {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/SpineJsonRegistry.ts:61 (sha256:f1b434dbf93c223ce7aea19eb43b6e2591b5f6c10996d44e67e0674cbf39fe82)
pub type SpineJsonTimelineHandler = std::sync::Arc<
    std::sync::Mutex<
        Box<
            dyn FnMut(SpineJsonTimelineContext, String, Vec<(String, crate::FlightValue)>) -> ()
                + Send
                + 'static,
        >,
    >,
>;

// Source: upstream/packages/types/src/SpineJsonRegistry.ts:67 (sha256:7521518c692e16c51055337169478f641713e3beba038707aa8723997b31972c)
#[derive(Clone)]
pub struct SpineJsonTimelineHandlerEntry {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub handle: SpineJsonTimelineHandler,
    pub kind: SpineJsonTimelineKind,
}
impl PartialEq for SpineJsonTimelineHandlerEntry {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/SpineJsonRegistry.ts:72 (sha256:f162377c3042f503af20ba999bc8fce9ca8cb1fe1303d9a89e48a8c0cd00d87b)
#[derive(Clone, Default)]
pub struct SpineJsonRegistry {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub section_handlers: Vec<SpineJsonSectionHandlerEntry>,
    pub timeline_handlers: Vec<SpineJsonTimelineHandlerEntry>,
}
impl PartialEq for SpineJsonRegistry {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
