// @generated from upstream/packages/types/src/Skeleton2DAnimationTargetKind.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/Skeleton2DAnimationTargetKind.ts:13 (sha256:e23310b3a8670bf53c5953f4daa3dae52cfa6d36d876ea4c35fe693cd0c0e299)
#[derive(Clone, Default)]
pub struct Skeleton2DAnimationTargetKindValues {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub bone: String,
    pub constraint: String,
    pub deform: String,
    pub draw_order: String,
    pub slot: String,
}
impl PartialEq for Skeleton2DAnimationTargetKindValues {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub static SKELETON2_D_ANIMATION_TARGET_KIND: std::sync::LazyLock<
    Skeleton2DAnimationTargetKindValues,
> = std::sync::LazyLock::new(|| Skeleton2DAnimationTargetKindValues {
    __flight_identity: std::sync::Arc::new(()),
    bone: "Skeleton2D.BoneTarget".to_owned(),
    constraint: "Skeleton2D.ConstraintTarget".to_owned(),
    deform: "Skeleton2D.DeformTarget".to_owned(),
    draw_order: "Skeleton2D.DrawOrderTarget".to_owned(),
    slot: "Skeleton2D.SlotTarget".to_owned(),
});

// Source: upstream/packages/types/src/Skeleton2DAnimationTargetKind.ts:21 (sha256:81c17b1e25c72a8c7cf3eef89cbeb7ac4f69f7a9d1dc8eb107eee2e6cb9b4db9)
pub type Skeleton2DAnimationTargetKind = String;
