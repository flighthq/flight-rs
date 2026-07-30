// @generated from upstream/packages/types/src/RenderQueue.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::RenderProxy;

// Source: upstream/packages/types/src/RenderQueue.ts:2 (sha256:41b60b0e39f163a9d167fdc350c63919b2c488d8a7eaa98037125721aab0dad4)
pub type RenderSortKey = f64;

// Source: upstream/packages/types/src/RenderQueue.ts:3 (sha256:a2ce11b93d508a020e1780615c583c770d72c77a9c3f109ac625fdfa54d086a6)
#[derive(Clone)]
pub struct RenderQueueEntry {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub proxy: RenderProxy,
    pub sort_key: RenderSortKey,
}
impl PartialEq for RenderQueueEntry {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/RenderQueue.ts:7 (sha256:3ff8f98c70e258e788235d2d468ab190af55513ae4c7692f0bbf58b1b1de6803)
#[derive(Clone, Default)]
pub struct RenderQueue {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub entries: Vec<RenderQueueEntry>,
    pub entry_count: f64,
}
impl PartialEq for RenderQueue {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
