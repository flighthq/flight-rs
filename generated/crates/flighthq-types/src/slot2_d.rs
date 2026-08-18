// @generated from upstream/packages/types/src/Slot2D.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{Attachment2D, Skeleton2DSlotDeform};

// Source: upstream/packages/types/src/Slot2D.ts:11 (sha256:5b524410e81d8490af00fbc1c0e6f4fde47973867e140d4d667c104b89f6c4f6)
#[derive(Clone, Default)]
pub struct Slot2D {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub attachment: Option<Attachment2D>,
    pub deform: Option<Skeleton2DSlotDeform>,
    pub bone_index: f64,
    pub color: Option<f64>,
    pub name: Option<String>,
}
impl PartialEq for Slot2D {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
