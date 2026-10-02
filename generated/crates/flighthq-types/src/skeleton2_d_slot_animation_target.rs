// @generated from upstream/packages/types/src/Skeleton2DSlotAnimationTarget.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{Attachment2D, EntityRuntime, Skeleton2DAnimationTargetKind};

// Source: upstream/packages/types/src/Skeleton2DSlotAnimationTarget.ts:18 (sha256:4fb97ae8a80e859190b5e9b5684a18a5f7c5d2da016004ba61a709b4937adafb)
#[derive(Clone, Default)]
pub struct Skeleton2DSlotAnimationPathValues {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub attachment: String,
    pub color: String,
}
impl PartialEq for Skeleton2DSlotAnimationPathValues {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub static SKELETON2_D_SLOT_ANIMATION_PATH: std::sync::LazyLock<Skeleton2DSlotAnimationPathValues> =
    std::sync::LazyLock::new(|| Skeleton2DSlotAnimationPathValues {
        __flight_identity: std::sync::Arc::new(()),
        attachment: "Attachment".to_owned(),
        color: "Color".to_owned(),
    });

// Source: upstream/packages/types/src/Skeleton2DSlotAnimationTarget.ts:23 (sha256:1cd2642a4140fe435d1eaf2b99391f526aff9bdabdb2899083ee4a8ccf24d955)
pub type Skeleton2DSlotAnimationPath = String;

// Source: upstream/packages/types/src/Skeleton2DSlotAnimationTarget.ts:49 (sha256:69516a8fffd65d274d833f0f5741d129c635dbbe27cb9e5a44004117bb892cb8)
#[derive(Clone, Default)]
pub struct Skeleton2DSlotAnimationTarget {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub attachments: Option<Vec<Option<Attachment2D>>>,
    pub kind: Skeleton2DAnimationTargetKind,
    pub path: Skeleton2DSlotAnimationPath,
    pub slot_index: f64,
}
impl PartialEq for Skeleton2DSlotAnimationTarget {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for Skeleton2DSlotAnimationTarget {
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
