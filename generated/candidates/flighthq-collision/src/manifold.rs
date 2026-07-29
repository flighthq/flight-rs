// @generated from upstream/packages/collision/src/manifold.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::CollisionManifold;

// Source: upstream/packages/collision/src/manifold.ts:6 (sha256:1400b477cc2f016ed02b8c738faaec82d01184b5833419569ea9cf6193a6d35f)
pub fn clear_collision_manifold(out: &mut CollisionManifold) -> () {
    out.overlapping = false;
    out.normal_x = 0.0_f64;
    out.normal_y = 0.0_f64;
    out.depth = 0.0_f64;
}

// Source: upstream/packages/collision/src/manifold.ts:14 (sha256:de9c25d33785cc2e9bca01a07e1c6bd0f936276ab7c1fba01b63759e73d349b4)
#[derive(Clone, Default)]
struct CreateCollisionManifoldRecord1 {
    __flight_identity: std::sync::Arc<()>,
    overlapping: bool,
    normal_x: f64,
    normal_y: f64,
    depth: f64,
}
impl PartialEq for CreateCollisionManifoldRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_collision_manifold() -> CollisionManifold {
    return CollisionManifold {
        __flight_identity: std::sync::Arc::new(()),
        overlapping: false,
        normal_x: 0.0_f64,
        normal_y: 0.0_f64,
        depth: 0.0_f64,
    };
}
