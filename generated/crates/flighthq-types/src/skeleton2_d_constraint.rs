// @generated from upstream/packages/types/src/Skeleton2DConstraint.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::Skeleton2D;

// Source: upstream/packages/types/src/Skeleton2DConstraint.ts:12 (sha256:8002b8eabf3a0c4d9560b76c5a9140b6b019526496afdf85aa3ee0f083196736)
#[derive(Clone, Default)]
pub struct Skeleton2DConstraint {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: Skeleton2DConstraintKind,
    pub mix: f64,
}
impl PartialEq for Skeleton2DConstraint {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Skeleton2DConstraint.ts:27 (sha256:29937c192eef430fc5b763e210e5a527a7d104175182a52d9db1060713e65f5e)
pub type Skeleton2DConstraintSolver = std::sync::Arc<
    std::sync::Mutex<Box<dyn FnMut(Skeleton2D, Skeleton2DConstraint) -> () + Send + 'static>>,
>;

// Source: upstream/packages/types/src/Skeleton2DConstraint.ts:33 (sha256:5d231be4e261cce5bb93e856591656d29e37b14166d4adc6935fa55fc0996546)
#[derive(Clone, Default)]
pub struct Skeleton2DConstraintKindValues {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub ik: String,
    pub path: String,
    pub transform: String,
}
impl PartialEq for Skeleton2DConstraintKindValues {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub static SKELETON2_D_CONSTRAINT_KIND: std::sync::LazyLock<Skeleton2DConstraintKindValues> =
    std::sync::LazyLock::new(|| Skeleton2DConstraintKindValues {
        __flight_identity: std::sync::Arc::new(()),
        ik: "Skeleton2D.IkConstraint".to_owned(),
        path: "Skeleton2D.PathConstraint".to_owned(),
        transform: "Skeleton2D.TransformConstraint".to_owned(),
    });

// Source: upstream/packages/types/src/Skeleton2DConstraint.ts:39 (sha256:5087086c15ba73483edf1052893cd95ad2c8dedbfb9b862d6505c369b2638bdd)
pub type Skeleton2DConstraintKind = String;
