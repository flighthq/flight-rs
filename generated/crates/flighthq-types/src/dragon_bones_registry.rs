// @generated from upstream/packages/types/src/DragonBonesRegistry.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    AnimationChannel, Attachment2D, AttachmentSkin2D, Bone2D, ImportDiagnostic,
    Skeleton2DImportAnimation, Slot2D,
};

// Source: upstream/packages/types/src/DragonBonesRegistry.ts:9 (sha256:4ee627ea7516142a0495161f1ca71c4c780e4d332d1bbe4e76a35b3b79212737)
#[derive(Clone, Default)]
pub struct DragonBonesSectionKindValues {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub animations: String,
    pub bones: String,
    pub ik_constraints: String,
    pub skins: String,
    pub slots: String,
}
impl PartialEq for DragonBonesSectionKindValues {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub static DRAGON_BONES_SECTION_KIND: std::sync::LazyLock<DragonBonesSectionKindValues> =
    std::sync::LazyLock::new(|| DragonBonesSectionKindValues {
        __flight_identity: std::sync::Arc::new(()),
        animations: "animations".to_owned(),
        bones: "bones".to_owned(),
        ik_constraints: "ikConstraints".to_owned(),
        skins: "skins".to_owned(),
        slots: "slots".to_owned(),
    });

// Source: upstream/packages/types/src/DragonBonesRegistry.ts:17 (sha256:bf51c29ccfead4157906c884b2c94bbfdc4056dab59c6f66f72882919f79358b)
pub type DragonBonesSectionKind = String;

// Source: upstream/packages/types/src/DragonBonesRegistry.ts:19 (sha256:313dea0a96710fda0c16cd13d4e15cc78b8b21c9d82867c05b8d6419e04c8ce6)
#[derive(Clone, Default)]
pub struct DragonBonesTimelineKindValues {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub bone: String,
    pub deform: String,
    pub ik: String,
    pub slot: String,
    pub z_order: String,
}
impl PartialEq for DragonBonesTimelineKindValues {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub static DRAGON_BONES_TIMELINE_KIND: std::sync::LazyLock<DragonBonesTimelineKindValues> =
    std::sync::LazyLock::new(|| DragonBonesTimelineKindValues {
        __flight_identity: std::sync::Arc::new(()),
        bone: "bone".to_owned(),
        deform: "deform".to_owned(),
        ik: "ik".to_owned(),
        slot: "slot".to_owned(),
        z_order: "zOrder".to_owned(),
    });

// Source: upstream/packages/types/src/DragonBonesRegistry.ts:27 (sha256:a6d0c188bbdec1e049d70480bd3d942f2543a41da8933ed2eae93755a9019568)
pub type DragonBonesTimelineKind = String;

// Source: upstream/packages/types/src/DragonBonesRegistry.ts:29 (sha256:f210da4e3844373cf960fc7848177ebd0e0c6c32f9d3fd9c88699a490e1da72d)
#[derive(Clone, Default)]
pub struct DragonBonesSectionContext {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub animations: Vec<Skeleton2DImportAnimation>,
    pub armature: Vec<(String, crate::FlightValue)>,
    pub bone_index_by_name: Vec<(String, f64)>,
    pub bones: Vec<Bone2D>,
    pub diagnostics: Option<Vec<ImportDiagnostic>>,
    pub display_table: Vec<(String, Vec<Option<Attachment2D>>)>,
    pub doc: Vec<(String, crate::FlightValue)>,
    pub frame_rate: f64,
    pub raw_index_to_output: Vec<f64>,
    pub registry: DragonBonesRegistry,
    pub skins: Vec<AttachmentSkin2D>,
    pub slot_order: Vec<(String, f64)>,
    pub slots: Vec<Slot2D>,
}
impl PartialEq for DragonBonesSectionContext {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/DragonBonesRegistry.ts:45 (sha256:fb2ecf04c9a32d191239ece17dba9e4c2238b54b13829c54e8f1bdfbbb8b03e9)
pub type DragonBonesSectionHandler = std::sync::Arc<
    std::sync::Mutex<Box<dyn FnMut(DragonBonesSectionContext) -> () + Send + 'static>>,
>;

// Source: upstream/packages/types/src/DragonBonesRegistry.ts:47 (sha256:de0b5b77643c0486168d5d41a590da4f8bb56b85e3c1de9b716a176ba09fb983)
#[derive(Clone)]
pub struct DragonBonesSectionHandlerEntry {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub handle: DragonBonesSectionHandler,
    pub kind: DragonBonesSectionKind,
}
impl PartialEq for DragonBonesSectionHandlerEntry {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/DragonBonesRegistry.ts:52 (sha256:e22506f4dd79ade62c729a51efb15d022045e74f08bcc96296138fe478f1d7b3)
#[derive(Clone, Default)]
pub struct DragonBonesTimelineContext {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub channels: Vec<AnimationChannel>,
    pub section: DragonBonesSectionContext,
    pub unmodeled_timeline_counts: Vec<(String, f64)>,
    pub unresolved_bone_count: f64,
    pub unregistered_timeline_counts: Vec<(DragonBonesTimelineKind, f64)>,
}
impl PartialEq for DragonBonesTimelineContext {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/DragonBonesRegistry.ts:60 (sha256:d713a79821b1770d7e86b0f765010ee1030a78de7884b81119ae9c2a279c4711)
pub type DragonBonesTimelineHandler = std::sync::Arc<
    std::sync::Mutex<
        Box<
            dyn FnMut(DragonBonesTimelineContext, String, Vec<(String, crate::FlightValue)>) -> ()
                + Send
                + 'static,
        >,
    >,
>;

// Source: upstream/packages/types/src/DragonBonesRegistry.ts:66 (sha256:9f6ce53c7df29b90f1382223a8824e6fe0136d4efd6b84cdf558d29f7d58c751)
#[derive(Clone)]
pub struct DragonBonesTimelineHandlerEntry {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub handle: DragonBonesTimelineHandler,
    pub kind: DragonBonesTimelineKind,
}
impl PartialEq for DragonBonesTimelineHandlerEntry {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/DragonBonesRegistry.ts:71 (sha256:b8ab5582b4fa255115a179ebff57e958a581eff80db18b9498cf83d379b48090)
#[derive(Clone, Default)]
pub struct DragonBonesRegistry {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub section_handlers: Vec<DragonBonesSectionHandlerEntry>,
    pub timeline_handlers: Vec<DragonBonesTimelineHandlerEntry>,
}
impl PartialEq for DragonBonesRegistry {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
