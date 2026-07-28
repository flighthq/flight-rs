// @generated from upstream/packages/types/src/DragForce.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/DragForce.ts:1 (sha256:0c7e83c50022ff2886f0d7f4b6a1dc9bd2abbc11a4087087bf9640f29669e99c)
#[derive(Clone)]
pub struct DragForce {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: String,
    pub strength: f64,
}
impl PartialEq for DragForce {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/DragForce.ts:6 (sha256:3f19241ba951a09c87fdb4314aa9bc5e0dda86478ac672806bf86dc58a466b80)
pub const DRAG_FORCE_KIND: &'static str = "DragForce";
