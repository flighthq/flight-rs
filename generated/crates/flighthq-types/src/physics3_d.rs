// @generated from upstream/packages/types/src/Physics3D.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{CollisionColliderShape3D, EntityRuntime, SpatialIndexBackend3D};

// Source: upstream/packages/types/src/Physics3D.ts:29 (sha256:f484c4d4fc65e7121ea4ff88c9adb0f7623093ca2872e50787cbb5537230204e)
pub type Physics3DBodyType = String;

// Source: upstream/packages/types/src/Physics3D.ts:38 (sha256:9a89e8b866e39db1683ae559bf348b722b56ddf4ceaca2ffb841f5c34d7d09fd)
#[derive(Clone, Default)]
pub struct Physics3DMaterial {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub density: f64,
    pub friction: f64,
    pub restitution: f64,
}
impl PartialEq for Physics3DMaterial {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Physics3D.ts:48 (sha256:1ee81caf276760e1000d88bdb511703a0227b6a15e9054bf44500ff76dcab997)
#[derive(Clone, Default)]
pub struct Physics3DCollisionFilter {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub category_bits: f64,
    pub mask_bits: f64,
    pub group_index: f64,
}
impl PartialEq for Physics3DCollisionFilter {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Physics3D.ts:70 (sha256:2c7f33bd993f5886ae293a98bd31a52e5a71fee04c390cf479c972db4771e0d1)
#[derive(Clone)]
pub struct Physics3DCollider {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub local: CollisionColliderShape3D,
    pub world: CollisionColliderShape3D,
    pub material: Physics3DMaterial,
    pub filter: Physics3DCollisionFilter,
    pub sensor: bool,
}
impl PartialEq for Physics3DCollider {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for Physics3DCollider {
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

// Source: upstream/packages/types/src/Physics3D.ts:97 (sha256:e8caf481441f38979b55e024587a3729058baa1601a900a19c191de260abe09a)
#[derive(Clone, Default)]
pub struct Physics3DMassData {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub mass: f64,
    pub inertia_xx: f64,
    pub inertia_yy: f64,
    pub inertia_zz: f64,
    pub inertia_xy: f64,
    pub inertia_xz: f64,
    pub inertia_yz: f64,
    pub center_x: f64,
    pub center_y: f64,
    pub center_z: f64,
}
impl PartialEq for Physics3DMassData {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for Physics3DMassData {
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

// Source: upstream/packages/types/src/Physics3D.ts:121 (sha256:18c210d3215d397691d9563449f421ca0bcacaae5ca69c816db2843568b31109)
#[derive(Clone, Default)]
pub struct RigidBody3D {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub index: f64,
    pub type_: Physics3DBodyType,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub orientation_x: f64,
    pub orientation_y: f64,
    pub orientation_z: f64,
    pub orientation_w: f64,
    pub velocity_x: f64,
    pub velocity_y: f64,
    pub velocity_z: f64,
    pub angular_velocity_x: f64,
    pub angular_velocity_y: f64,
    pub angular_velocity_z: f64,
    pub force_x: f64,
    pub force_y: f64,
    pub force_z: f64,
    pub torque_x: f64,
    pub torque_y: f64,
    pub torque_z: f64,
    pub mass: f64,
    pub inverse_mass: f64,
    pub inertia_xx: f64,
    pub inertia_yy: f64,
    pub inertia_zz: f64,
    pub inertia_xy: f64,
    pub inertia_xz: f64,
    pub inertia_yz: f64,
    pub inverse_inertia_xx: f64,
    pub inverse_inertia_yy: f64,
    pub inverse_inertia_zz: f64,
    pub inverse_inertia_xy: f64,
    pub inverse_inertia_xz: f64,
    pub inverse_inertia_yz: f64,
    pub inverse_inertia_world_xx: f64,
    pub inverse_inertia_world_yy: f64,
    pub inverse_inertia_world_zz: f64,
    pub inverse_inertia_world_xy: f64,
    pub inverse_inertia_world_xz: f64,
    pub inverse_inertia_world_yz: f64,
    pub center_x: f64,
    pub center_y: f64,
    pub center_z: f64,
    pub linear_damping: f64,
    pub angular_damping: f64,
    pub gravity_scale: f64,
    pub fixed_rotation: bool,
    pub bullet: bool,
    pub sleeping: bool,
    pub sleep_enabled: bool,
    pub sleep_timer: f64,
    pub colliders: Vec<Physics3DCollider>,
}
impl PartialEq for RigidBody3D {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for RigidBody3D {
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

// Source: upstream/packages/types/src/Physics3D.ts:249 (sha256:2b58699813dc5cf98e9dc283d9b53c6713b29fc05db2cace1d819059643623a8)
#[derive(Clone, Default)]
pub struct Physics3DContactPoint {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub depth: f64,
    pub feature_id: f64,
    pub r_ax: f64,
    pub r_ay: f64,
    pub r_az: f64,
    pub r_bx: f64,
    pub r_by: f64,
    pub r_bz: f64,
}
impl PartialEq for Physics3DContactPoint {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for Physics3DContactPoint {
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

// Source: upstream/packages/types/src/Physics3D.ts:278 (sha256:b229cf49c7a5cd0c04b170569e64d10cb53a7d2afe98b8d98cc54792d973c2a1)
#[derive(Clone, Default)]
pub struct Physics3DContact {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub body_a: f64,
    pub body_b: f64,
    pub collider_a: f64,
    pub collider_b: f64,
    pub normal_x: f64,
    pub normal_y: f64,
    pub normal_z: f64,
    pub point_count: f64,
    pub points: Vec<Physics3DContactPoint>,
    pub friction: f64,
    pub restitution: f64,
    pub enabled: bool,
    pub sensor: bool,
    pub touching: bool,
}
impl PartialEq for Physics3DContact {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for Physics3DContact {
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

// Source: upstream/packages/types/src/Physics3D.ts:313 (sha256:bc73203c050847a8a91cb49f7a9dc71b915906866c51853b2899cdeb47bf1f7b)
#[derive(Clone, Default)]
pub struct Physics3DContactEvents {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub began: Vec<Physics3DContact>,
    pub ended: Vec<Physics3DContact>,
}
impl PartialEq for Physics3DContactEvents {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Physics3D.ts:328 (sha256:df14fb65f782b9f5aa3355e358ab0e2f1089989c283ee79b53a2a6e2960f42d0)
pub type Physics3DContactCallback = std::sync::Arc<
    std::sync::Mutex<Box<dyn FnMut(Physics3DWorld, Physics3DContact) -> () + Send + 'static>>,
>;

// Source: upstream/packages/types/src/Physics3D.ts:330 (sha256:8aa26c8f4630cb57690992902681fd3d1059268cf511deab5945f5a7b2d9c043)
#[derive(Clone, Default)]
pub struct Physics3DContactHooks {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub pre_solve: Option<Physics3DContactCallback>,
    pub post_solve: Option<Physics3DContactCallback>,
}
impl PartialEq for Physics3DContactHooks {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Physics3D.ts:343 (sha256:8506838ab8060e0507427e1e0f7026b0c9667e2436cc4955ee7b15709ad12ce7)
#[derive(Clone, Default)]
pub struct Physics3DContactConstraintPoint {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub feature_id: f64,
    pub normal_impulse: f64,
    pub tangent_impulse0: f64,
    pub tangent_impulse1: f64,
    pub normal_mass: f64,
    pub tangent_mass0: f64,
    pub tangent_mass1: f64,
    pub bias: f64,
}
impl PartialEq for Physics3DContactConstraintPoint {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for Physics3DContactConstraintPoint {
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

// Source: upstream/packages/types/src/Physics3D.ts:363 (sha256:2320ca19f4344a49edcc752793f316c709f0e3c71e706c514052877b99ba92b4)
#[derive(Clone, Default)]
pub struct Physics3DContactConstraint {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub contact: f64,
    pub tangent0_x: f64,
    pub tangent0_y: f64,
    pub tangent0_z: f64,
    pub tangent1_x: f64,
    pub tangent1_y: f64,
    pub tangent1_z: f64,
    pub point_count: f64,
    pub points: Vec<Physics3DContactConstraintPoint>,
}
impl PartialEq for Physics3DContactConstraint {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for Physics3DContactConstraint {
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

// Source: upstream/packages/types/src/Physics3D.ts:385 (sha256:569dea4b7befc1d57bb8cbd31cb5ec7c9f07a375d0381bced28e4ab2636e3352)
#[derive(Clone, Default)]
pub struct Physics3DSequentialImpulseState {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub constraints: Vec<Physics3DContactConstraint>,
    pub constraint_by_contact: Vec<(Physics3DContact, Physics3DContactConstraint)>,
}
impl PartialEq for Physics3DSequentialImpulseState {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Physics3D.ts:401 (sha256:0de6797dfe9c8f540e889ae0af3a97e5cd7aff54d43a7d4cc885954bac19f8df)
#[derive(Clone, Default)]
pub struct Physics3DSequentialImpulseConfig {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub velocity_iterations: f64,
    pub position_iterations: f64,
    pub penetration_slop: f64,
    pub position_correction: f64,
    pub restitution_threshold: f64,
    pub warm_starting: bool,
}
impl PartialEq for Physics3DSequentialImpulseConfig {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Physics3D.ts:416 (sha256:1883c0e833b33891623fec6c21c8c5493571363dfd339c209f86cc16060f2e72)
#[derive(Clone, Default)]
pub struct Physics3DSolverConfig {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub allow_sleeping: bool,
    pub sleep_linear_threshold: f64,
    pub sleep_angular_threshold: f64,
    pub time_to_sleep: f64,
    pub substeps: f64,
    pub continuous_collision: bool,
    pub max_ccd_substeps: f64,
    pub max_ccd_rotation_substeps: f64,
    pub sequential_impulse: Physics3DSequentialImpulseConfig,
}
impl PartialEq for Physics3DSolverConfig {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Physics3D.ts:459 (sha256:187b301955811c3de7115138e2c44a0d076c84a92749130d9bb5050b498bb95d)
#[derive(Clone, Default)]
pub struct Physics3DRotationalCcdEnvelope {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub angular_travel: f64,
    pub sample_count: f64,
    pub max_angular_increment: f64,
    pub max_point_arc_travel: f64,
    pub target_increment_met: bool,
}
impl PartialEq for Physics3DRotationalCcdEnvelope {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Physics3D.ts:479 (sha256:97429f7fe41883e04c005180f871d590ed107631efbd235f942558ebfe66dbf6)
#[derive(Clone, Default)]
pub struct Physics3DCollisionExplanation {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub unsupported_kinds: Vec<String>,
    pub status: String,
}
impl PartialEq for Physics3DCollisionExplanation {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Physics3D.ts:484 (sha256:35ba8a002d72457ca72833a18e5220577ca860696ff0bd474f94da22cefbdf3c)
#[derive(Clone, Default)]
pub struct Physics3DStepExplanation {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub body_state_valid: bool,
    pub collider_state_valid: bool,
    pub contact_state_valid: bool,
    pub gravity_valid: bool,
    pub joint_state_valid: bool,
    pub solver_config_valid: bool,
    pub substeps_valid: bool,
    pub timestep_valid: bool,
    pub velocity_iterations_valid: bool,
    pub position_iterations_valid: bool,
    pub status: String,
}
impl PartialEq for Physics3DStepExplanation {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Physics3D.ts:501 (sha256:4c21e062154663c79db003b34d4bbfcacbe747d4e450b94d58b60b1f45819ce0)
pub type Physics3DStepGuard =
    std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(Physics3DWorld, f64) -> () + Send + 'static>>>;

// Source: upstream/packages/types/src/Physics3D.ts:505 (sha256:c092cdc8419d54b8a2a5d8a3d40fd039e84a4c4e48333274dc6cb4ccfaaa33c6)
pub type Physics3DContactIntakeGuard =
    std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(Physics3DWorld) -> () + Send + 'static>>>;

// Source: upstream/packages/types/src/Physics3D.ts:510 (sha256:3044174f22193198bccd0ab963547bd7509028b2d3dad49d61fc562f5ed579eb)
pub type Physics3DJointResolutionGuard =
    std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(Physics3DWorld) -> () + Send + 'static>>>;

// Source: upstream/packages/types/src/Physics3D.ts:514 (sha256:c042cce5b85152550c502f3e506881254aaae28f10d57a1fa2172493a13e6fe8)
#[derive(Clone, Default)]
pub struct Physics3DJointExplanation {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: Physics3DJointKind,
    pub index: f64,
    pub has_solver: bool,
    pub bodies_resolvable: bool,
    pub status: String,
}
impl PartialEq for Physics3DJointExplanation {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Physics3D.ts:527 (sha256:d9bf6924c96c62b3c26137c72cf20c4138932d592568b0c4bb001a3396efca4e)
pub type Physics3DJointKind = String;

// Source: upstream/packages/types/src/Physics3D.ts:539 (sha256:a14cfa3b7174725877c4ca89836cededa3f8756b683d644c03f87f46c1a31cdf)
#[derive(Clone, Default)]
pub struct Physics3DJoint {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub kind: Physics3DJointKind,
    pub body_a: f64,
    pub body_b: f64,
    pub local_anchor_ax: f64,
    pub local_anchor_ay: f64,
    pub local_anchor_az: f64,
    pub local_anchor_bx: f64,
    pub local_anchor_by: f64,
    pub local_anchor_bz: f64,
    pub collide_connected: bool,
    pub break_force: f64,
    pub break_torque: f64,
    pub broken: bool,
    pub impulse0: f64,
    pub impulse1: f64,
    pub impulse2: f64,
    pub impulse3: f64,
    pub impulse4: f64,
    pub impulse5: f64,
    pub r_ax: f64,
    pub r_ay: f64,
    pub r_az: f64,
    pub r_bx: f64,
    pub r_by: f64,
    pub r_bz: f64,
}
impl PartialEq for Physics3DJoint {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for Physics3DJoint {
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

// Source: upstream/packages/types/src/Physics3D.ts:597 (sha256:aa8e15e5302ee9097177e9c6e7c38808185c13eca993fbf7165e641bd70d6bb2)
#[derive(Clone)]
pub struct Physics3DJointSolver {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub prepare: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(Physics3DWorld, Physics3DJoint, f64) -> () + Send + 'static>,
        >,
    >,
    pub solve: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(Physics3DWorld, Physics3DJoint, f64) -> () + Send + 'static>,
        >,
    >,
    pub uses_body_a: Option<bool>,
    pub keeps_bodies_awake: Option<bool>,
    pub swap_ends: Option<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(Physics3DJoint) -> bool + Send + 'static>>>,
    >,
    pub write_reaction: Option<
        std::sync::Arc<
            std::sync::Mutex<
                Box<
                    dyn FnMut(Physics3DJoint, f64, Physics3DJointReaction) -> bool + Send + 'static,
                >,
            >,
        >,
    >,
    pub warm_start: Option<
        std::sync::Arc<
            std::sync::Mutex<Box<dyn FnMut(Physics3DWorld, Physics3DJoint) -> () + Send + 'static>>,
        >,
    >,
    pub scale_accumulated_impulses: Option<
        std::sync::Arc<
            std::sync::Mutex<Box<dyn FnMut(Physics3DJoint, f64) -> () + Send + 'static>>,
        >,
    >,
    pub clear_accumulated_impulses: Option<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(Physics3DJoint) -> () + Send + 'static>>>,
    >,
}
impl PartialEq for Physics3DJointSolver {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Physics3D.ts:646 (sha256:bafab0a5ea70df233986468723f861cc7dff0a5b9eac722d256fe2f19e344f97)
#[derive(Clone, Default)]
pub struct Physics3DJointFrames {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub local_rotation_ax: f64,
    pub local_rotation_ay: f64,
    pub local_rotation_az: f64,
    pub local_rotation_aw: f64,
    pub local_rotation_bx: f64,
    pub local_rotation_by: f64,
    pub local_rotation_bz: f64,
    pub local_rotation_bw: f64,
}
impl PartialEq for Physics3DJointFrames {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Physics3D.ts:668 (sha256:c8437dff84485cc71a4de839cae384b06f08f3bbc265d716f007333929078442)
#[derive(Clone, Default)]
pub struct Physics3DJointReaction {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub force_x: f64,
    pub force_y: f64,
    pub force_z: f64,
    pub torque_x: f64,
    pub torque_y: f64,
    pub torque_z: f64,
}
impl PartialEq for Physics3DJointReaction {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for Physics3DJointReaction {
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

// Source: upstream/packages/types/src/Physics3D.ts:680 (sha256:23de929fddd2f1c1e92d44f305ee36ad165fef4a9b1163cfa8dff85fc16a8f92)
#[derive(Clone, Default)]
pub struct Physics3DJointEvents {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub broke: Vec<Physics3DJoint>,
}
impl PartialEq for Physics3DJointEvents {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Physics3D.ts:684 (sha256:46ad4888a7ca52b21b8591d90409b267d6433313e236c5bf988ae892dd07b4ff)
#[derive(Clone, Default)]
pub struct Physics3DJointOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub body_a: f64,
    pub body_b: f64,
    pub local_anchor_ax: Option<f64>,
    pub local_anchor_ay: Option<f64>,
    pub local_anchor_az: Option<f64>,
    pub local_anchor_bx: Option<f64>,
    pub local_anchor_by: Option<f64>,
    pub local_anchor_bz: Option<f64>,
    pub collide_connected: Option<bool>,
    pub break_force: Option<f64>,
    pub break_torque: Option<f64>,
}
impl PartialEq for Physics3DJointOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Physics3D.ts:700 (sha256:edac07364f90df64b39c2d1ac50a306e9877d7e8c4ab1c4d6c02dc7e80cb0395)
#[derive(Clone, Default)]
pub struct Physics3DJointFrameOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub body_a: f64,
    pub body_b: f64,
    pub local_anchor_ax: Option<f64>,
    pub local_anchor_ay: Option<f64>,
    pub local_anchor_az: Option<f64>,
    pub local_anchor_bx: Option<f64>,
    pub local_anchor_by: Option<f64>,
    pub local_anchor_bz: Option<f64>,
    pub collide_connected: Option<bool>,
    pub break_force: Option<f64>,
    pub break_torque: Option<f64>,
    pub local_rotation_ax: Option<f64>,
    pub local_rotation_ay: Option<f64>,
    pub local_rotation_az: Option<f64>,
    pub local_rotation_aw: Option<f64>,
    pub local_rotation_bx: Option<f64>,
    pub local_rotation_by: Option<f64>,
    pub local_rotation_bz: Option<f64>,
    pub local_rotation_bw: Option<f64>,
}
impl PartialEq for Physics3DJointFrameOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Physics3D.ts:717 (sha256:757088138e3362c0c0cb7259922734c69c19eb0cee7423a4a77895a1de6d83bc)
pub type Physics3DBallAndSocketJoint = Physics3DJoint;

// Source: upstream/packages/types/src/Physics3D.ts:718 (sha256:013b90024d32a84652fb6c5293125aabb56a10f6ad832222040754cb21fe7e45)
pub type Physics3DBallAndSocketJointOptions = Physics3DJointOptions;

// Source: upstream/packages/types/src/Physics3D.ts:739 (sha256:3d19fe52d4e6e931b5e0406372e92232cba810c05957104ff626c67b68453640)
#[derive(Clone, Default)]
pub struct Physics3DDistanceJoint {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub kind: Physics3DJointKind,
    pub body_a: f64,
    pub body_b: f64,
    pub local_anchor_ax: f64,
    pub local_anchor_ay: f64,
    pub local_anchor_az: f64,
    pub local_anchor_bx: f64,
    pub local_anchor_by: f64,
    pub local_anchor_bz: f64,
    pub collide_connected: bool,
    pub break_force: f64,
    pub break_torque: f64,
    pub broken: bool,
    pub impulse0: f64,
    pub impulse1: f64,
    pub impulse2: f64,
    pub impulse3: f64,
    pub impulse4: f64,
    pub impulse5: f64,
    pub r_ax: f64,
    pub r_ay: f64,
    pub r_az: f64,
    pub r_bx: f64,
    pub r_by: f64,
    pub r_bz: f64,
    pub length: f64,
    pub enable_spring: bool,
    pub frequency_hz: f64,
    pub damping_ratio: f64,
    pub enable_limit: bool,
    pub min_length: f64,
    pub max_length: f64,
    pub lower_limit_impulse: f64,
    pub upper_limit_impulse: f64,
}
impl PartialEq for Physics3DDistanceJoint {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for Physics3DDistanceJoint {
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

// Source: upstream/packages/types/src/Physics3D.ts:762 (sha256:906a6b602bbc6e891f57613c25f8b1c909c30e638ad5956b82a30ac5c774e405)
#[derive(Clone, Default)]
pub struct Physics3DDistanceJointOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub body_a: f64,
    pub body_b: f64,
    pub local_anchor_ax: Option<f64>,
    pub local_anchor_ay: Option<f64>,
    pub local_anchor_az: Option<f64>,
    pub local_anchor_bx: Option<f64>,
    pub local_anchor_by: Option<f64>,
    pub local_anchor_bz: Option<f64>,
    pub collide_connected: Option<bool>,
    pub break_force: Option<f64>,
    pub break_torque: Option<f64>,
    pub length: Option<f64>,
    pub enable_spring: Option<bool>,
    pub frequency_hz: Option<f64>,
    pub damping_ratio: Option<f64>,
    pub enable_limit: Option<bool>,
    pub min_length: Option<f64>,
    pub max_length: Option<f64>,
}
impl PartialEq for Physics3DDistanceJointOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Physics3D.ts:776 (sha256:b19829018e650687bb9eb237e86bb2723046d72879d622b2138cf9f433d97ae8)
#[derive(Clone, Default)]
pub struct Physics3DFixedJoint {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub kind: Physics3DJointKind,
    pub body_a: f64,
    pub body_b: f64,
    pub local_anchor_ax: f64,
    pub local_anchor_ay: f64,
    pub local_anchor_az: f64,
    pub local_anchor_bx: f64,
    pub local_anchor_by: f64,
    pub local_anchor_bz: f64,
    pub collide_connected: bool,
    pub break_force: f64,
    pub break_torque: f64,
    pub broken: bool,
    pub impulse0: f64,
    pub impulse1: f64,
    pub impulse2: f64,
    pub impulse3: f64,
    pub impulse4: f64,
    pub impulse5: f64,
    pub r_ax: f64,
    pub r_ay: f64,
    pub r_az: f64,
    pub r_bx: f64,
    pub r_by: f64,
    pub r_bz: f64,
    pub local_rotation_ax: f64,
    pub local_rotation_ay: f64,
    pub local_rotation_az: f64,
    pub local_rotation_aw: f64,
    pub local_rotation_bx: f64,
    pub local_rotation_by: f64,
    pub local_rotation_bz: f64,
    pub local_rotation_bw: f64,
}
impl PartialEq for Physics3DFixedJoint {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for Physics3DFixedJoint {
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

// Source: upstream/packages/types/src/Physics3D.ts:777 (sha256:710c2552396be6cb30b1c8e6f45230b3a759d479384a256b7a8cf93e9efcb690)
pub type Physics3DFixedJointOptions = Physics3DJointFrameOptions;

// Source: upstream/packages/types/src/Physics3D.ts:787 (sha256:7cdcbd182487a3c0fea4385828ea058a2622c2848a1e377996500b7d691bb753)
#[derive(Clone, Default)]
pub struct Physics3DHingeJoint {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub kind: Physics3DJointKind,
    pub body_a: f64,
    pub body_b: f64,
    pub local_anchor_ax: f64,
    pub local_anchor_ay: f64,
    pub local_anchor_az: f64,
    pub local_anchor_bx: f64,
    pub local_anchor_by: f64,
    pub local_anchor_bz: f64,
    pub collide_connected: bool,
    pub break_force: f64,
    pub break_torque: f64,
    pub broken: bool,
    pub impulse0: f64,
    pub impulse1: f64,
    pub impulse2: f64,
    pub impulse3: f64,
    pub impulse4: f64,
    pub impulse5: f64,
    pub r_ax: f64,
    pub r_ay: f64,
    pub r_az: f64,
    pub r_bx: f64,
    pub r_by: f64,
    pub r_bz: f64,
    pub local_rotation_ax: f64,
    pub local_rotation_ay: f64,
    pub local_rotation_az: f64,
    pub local_rotation_aw: f64,
    pub local_rotation_bx: f64,
    pub local_rotation_by: f64,
    pub local_rotation_bz: f64,
    pub local_rotation_bw: f64,
    pub enable_limit: bool,
    pub lower_angle: f64,
    pub upper_angle: f64,
    pub enable_motor: bool,
    pub motor_speed: f64,
    pub max_motor_torque: f64,
    pub motor_impulse: f64,
    pub enable_limit_spring: bool,
    pub limit_frequency_hz: f64,
    pub limit_damping_ratio: f64,
    pub lower_limit_impulse: f64,
    pub upper_limit_impulse: f64,
}
impl PartialEq for Physics3DHingeJoint {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for Physics3DHingeJoint {
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

// Source: upstream/packages/types/src/Physics3D.ts:825 (sha256:e443913630322194d813f219dddac7f25b140d793dd13e438da46eb006c61767)
#[derive(Clone, Default)]
pub struct Physics3DHingeJointOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub body_a: f64,
    pub body_b: f64,
    pub local_anchor_ax: Option<f64>,
    pub local_anchor_ay: Option<f64>,
    pub local_anchor_az: Option<f64>,
    pub local_anchor_bx: Option<f64>,
    pub local_anchor_by: Option<f64>,
    pub local_anchor_bz: Option<f64>,
    pub collide_connected: Option<bool>,
    pub break_force: Option<f64>,
    pub break_torque: Option<f64>,
    pub local_rotation_ax: Option<f64>,
    pub local_rotation_ay: Option<f64>,
    pub local_rotation_az: Option<f64>,
    pub local_rotation_aw: Option<f64>,
    pub local_rotation_bx: Option<f64>,
    pub local_rotation_by: Option<f64>,
    pub local_rotation_bz: Option<f64>,
    pub local_rotation_bw: Option<f64>,
    pub enable_limit: Option<bool>,
    pub lower_angle: Option<f64>,
    pub upper_angle: Option<f64>,
    pub enable_motor: Option<bool>,
    pub motor_speed: Option<f64>,
    pub max_motor_torque: Option<f64>,
    pub enable_limit_spring: Option<bool>,
    pub limit_frequency_hz: Option<f64>,
    pub limit_damping_ratio: Option<f64>,
}
impl PartialEq for Physics3DHingeJointOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Physics3D.ts:842 (sha256:050a531fd480fabcd1aea7602a27f733b1e2c3ef774e92d8c8679c5810e9c6c9)
#[derive(Clone, Default)]
pub struct Physics3DSliderJoint {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub kind: Physics3DJointKind,
    pub body_a: f64,
    pub body_b: f64,
    pub local_anchor_ax: f64,
    pub local_anchor_ay: f64,
    pub local_anchor_az: f64,
    pub local_anchor_bx: f64,
    pub local_anchor_by: f64,
    pub local_anchor_bz: f64,
    pub collide_connected: bool,
    pub break_force: f64,
    pub break_torque: f64,
    pub broken: bool,
    pub impulse0: f64,
    pub impulse1: f64,
    pub impulse2: f64,
    pub impulse3: f64,
    pub impulse4: f64,
    pub impulse5: f64,
    pub r_ax: f64,
    pub r_ay: f64,
    pub r_az: f64,
    pub r_bx: f64,
    pub r_by: f64,
    pub r_bz: f64,
    pub local_rotation_ax: f64,
    pub local_rotation_ay: f64,
    pub local_rotation_az: f64,
    pub local_rotation_aw: f64,
    pub local_rotation_bx: f64,
    pub local_rotation_by: f64,
    pub local_rotation_bz: f64,
    pub local_rotation_bw: f64,
    pub enable_limit: bool,
    pub lower_translation: f64,
    pub upper_translation: f64,
    pub enable_motor: bool,
    pub motor_speed: f64,
    pub max_motor_force: f64,
    pub motor_impulse: f64,
    pub enable_limit_spring: bool,
    pub limit_frequency_hz: f64,
    pub limit_damping_ratio: f64,
    pub lower_limit_impulse: f64,
    pub upper_limit_impulse: f64,
}
impl PartialEq for Physics3DSliderJoint {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for Physics3DSliderJoint {
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

// Source: upstream/packages/types/src/Physics3D.ts:871 (sha256:b650c4be356e8fb63adfb19a36fed2fcd758153081aa0021a4d10b9d27489e30)
#[derive(Clone, Default)]
pub struct Physics3DSliderJointOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub body_a: f64,
    pub body_b: f64,
    pub local_anchor_ax: Option<f64>,
    pub local_anchor_ay: Option<f64>,
    pub local_anchor_az: Option<f64>,
    pub local_anchor_bx: Option<f64>,
    pub local_anchor_by: Option<f64>,
    pub local_anchor_bz: Option<f64>,
    pub collide_connected: Option<bool>,
    pub break_force: Option<f64>,
    pub break_torque: Option<f64>,
    pub local_rotation_ax: Option<f64>,
    pub local_rotation_ay: Option<f64>,
    pub local_rotation_az: Option<f64>,
    pub local_rotation_aw: Option<f64>,
    pub local_rotation_bx: Option<f64>,
    pub local_rotation_by: Option<f64>,
    pub local_rotation_bz: Option<f64>,
    pub local_rotation_bw: Option<f64>,
    pub enable_limit: Option<bool>,
    pub lower_translation: Option<f64>,
    pub upper_translation: Option<f64>,
    pub enable_motor: Option<bool>,
    pub motor_speed: Option<f64>,
    pub max_motor_force: Option<f64>,
    pub enable_limit_spring: Option<bool>,
    pub limit_frequency_hz: Option<f64>,
    pub limit_damping_ratio: Option<f64>,
}
impl PartialEq for Physics3DSliderJointOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Physics3D.ts:895 (sha256:799f1e572fee32caea94d4bd140ed192e65896eee0d7ea6eaa43a7cc93803755)
#[derive(Clone, Default)]
pub struct Physics3DConeTwistJoint {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub kind: Physics3DJointKind,
    pub body_a: f64,
    pub body_b: f64,
    pub local_anchor_ax: f64,
    pub local_anchor_ay: f64,
    pub local_anchor_az: f64,
    pub local_anchor_bx: f64,
    pub local_anchor_by: f64,
    pub local_anchor_bz: f64,
    pub collide_connected: bool,
    pub break_force: f64,
    pub break_torque: f64,
    pub broken: bool,
    pub impulse0: f64,
    pub impulse1: f64,
    pub impulse2: f64,
    pub impulse3: f64,
    pub impulse4: f64,
    pub impulse5: f64,
    pub r_ax: f64,
    pub r_ay: f64,
    pub r_az: f64,
    pub r_bx: f64,
    pub r_by: f64,
    pub r_bz: f64,
    pub local_rotation_ax: f64,
    pub local_rotation_ay: f64,
    pub local_rotation_az: f64,
    pub local_rotation_aw: f64,
    pub local_rotation_bx: f64,
    pub local_rotation_by: f64,
    pub local_rotation_bz: f64,
    pub local_rotation_bw: f64,
    pub enable_swing_limit: bool,
    pub swing_limit_y: f64,
    pub swing_limit_z: f64,
    pub enable_twist_limit: bool,
    pub lower_twist_angle: f64,
    pub upper_twist_angle: f64,
    pub enable_limit_spring: bool,
    pub limit_frequency_hz: f64,
    pub limit_damping_ratio: f64,
    pub swing_limit_impulse: f64,
    pub lower_twist_impulse: f64,
    pub upper_twist_impulse: f64,
}
impl PartialEq for Physics3DConeTwistJoint {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for Physics3DConeTwistJoint {
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

// Source: upstream/packages/types/src/Physics3D.ts:925 (sha256:72f01d11c5b90da5ff0c8e1af990caa524ecce6691a3ea967a63d94cd9e82a75)
#[derive(Clone, Default)]
pub struct Physics3DConeTwistJointOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub body_a: f64,
    pub body_b: f64,
    pub local_anchor_ax: Option<f64>,
    pub local_anchor_ay: Option<f64>,
    pub local_anchor_az: Option<f64>,
    pub local_anchor_bx: Option<f64>,
    pub local_anchor_by: Option<f64>,
    pub local_anchor_bz: Option<f64>,
    pub collide_connected: Option<bool>,
    pub break_force: Option<f64>,
    pub break_torque: Option<f64>,
    pub local_rotation_ax: Option<f64>,
    pub local_rotation_ay: Option<f64>,
    pub local_rotation_az: Option<f64>,
    pub local_rotation_aw: Option<f64>,
    pub local_rotation_bx: Option<f64>,
    pub local_rotation_by: Option<f64>,
    pub local_rotation_bz: Option<f64>,
    pub local_rotation_bw: Option<f64>,
    pub enable_swing_limit: Option<bool>,
    pub swing_limit_y: Option<f64>,
    pub swing_limit_z: Option<f64>,
    pub enable_twist_limit: Option<bool>,
    pub lower_twist_angle: Option<f64>,
    pub upper_twist_angle: Option<f64>,
    pub enable_limit_spring: Option<bool>,
    pub limit_frequency_hz: Option<f64>,
    pub limit_damping_ratio: Option<f64>,
}
impl PartialEq for Physics3DConeTwistJointOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Physics3D.ts:951 (sha256:5e4c27794d880512671571711e950187f564950a5e70b027e9a1917bc8c16f99)
#[derive(Clone, Default)]
pub struct Physics3DGeneric6DofJoint {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub kind: Physics3DJointKind,
    pub body_a: f64,
    pub body_b: f64,
    pub local_anchor_ax: f64,
    pub local_anchor_ay: f64,
    pub local_anchor_az: f64,
    pub local_anchor_bx: f64,
    pub local_anchor_by: f64,
    pub local_anchor_bz: f64,
    pub collide_connected: bool,
    pub break_force: f64,
    pub break_torque: f64,
    pub broken: bool,
    pub impulse0: f64,
    pub impulse1: f64,
    pub impulse2: f64,
    pub impulse3: f64,
    pub impulse4: f64,
    pub impulse5: f64,
    pub r_ax: f64,
    pub r_ay: f64,
    pub r_az: f64,
    pub r_bx: f64,
    pub r_by: f64,
    pub r_bz: f64,
    pub local_rotation_ax: f64,
    pub local_rotation_ay: f64,
    pub local_rotation_az: f64,
    pub local_rotation_aw: f64,
    pub local_rotation_bx: f64,
    pub local_rotation_by: f64,
    pub local_rotation_bz: f64,
    pub local_rotation_bw: f64,
    pub lower_linear_x: f64,
    pub lower_linear_y: f64,
    pub lower_linear_z: f64,
    pub upper_linear_x: f64,
    pub upper_linear_y: f64,
    pub upper_linear_z: f64,
    pub lower_angular_x: f64,
    pub lower_angular_y: f64,
    pub lower_angular_z: f64,
    pub upper_angular_x: f64,
    pub upper_angular_y: f64,
    pub upper_angular_z: f64,
    pub enable_limit_spring: bool,
    pub limit_frequency_hz: f64,
    pub limit_damping_ratio: f64,
    pub lower_limit_impulses: Vec<f64>,
    pub upper_limit_impulses: Vec<f64>,
}
impl PartialEq for Physics3DGeneric6DofJoint {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for Physics3DGeneric6DofJoint {
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

// Source: upstream/packages/types/src/Physics3D.ts:990 (sha256:1a3a86ea0b1f69e258717f4914660a834f0c7be7d67f4f95d5258fb93a7f3344)
#[derive(Clone, Default)]
pub struct Physics3DGeneric6DofJointOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub body_a: f64,
    pub body_b: f64,
    pub local_anchor_ax: Option<f64>,
    pub local_anchor_ay: Option<f64>,
    pub local_anchor_az: Option<f64>,
    pub local_anchor_bx: Option<f64>,
    pub local_anchor_by: Option<f64>,
    pub local_anchor_bz: Option<f64>,
    pub collide_connected: Option<bool>,
    pub break_force: Option<f64>,
    pub break_torque: Option<f64>,
    pub local_rotation_ax: Option<f64>,
    pub local_rotation_ay: Option<f64>,
    pub local_rotation_az: Option<f64>,
    pub local_rotation_aw: Option<f64>,
    pub local_rotation_bx: Option<f64>,
    pub local_rotation_by: Option<f64>,
    pub local_rotation_bz: Option<f64>,
    pub local_rotation_bw: Option<f64>,
    pub lower_linear_x: Option<f64>,
    pub lower_linear_y: Option<f64>,
    pub lower_linear_z: Option<f64>,
    pub upper_linear_x: Option<f64>,
    pub upper_linear_y: Option<f64>,
    pub upper_linear_z: Option<f64>,
    pub lower_angular_x: Option<f64>,
    pub lower_angular_y: Option<f64>,
    pub lower_angular_z: Option<f64>,
    pub upper_angular_x: Option<f64>,
    pub upper_angular_y: Option<f64>,
    pub upper_angular_z: Option<f64>,
    pub enable_limit_spring: Option<bool>,
    pub limit_frequency_hz: Option<f64>,
    pub limit_damping_ratio: Option<f64>,
}
impl PartialEq for Physics3DGeneric6DofJointOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Physics3D.ts:1017 (sha256:6aac46424c61a4f3a9fbaa82edffb975babcc12407c879a47a6d54ef8c446fa4)
#[derive(Clone)]
pub struct Physics3DWorld {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub version: f64,
    pub bodies: Vec<RigidBody3D>,
    pub body_by_index: Vec<(f64, RigidBody3D)>,
    pub index: SpatialIndexBackend3D,
    pub contacts: Vec<Physics3DContact>,
    pub joints: Vec<Physics3DJoint>,
    pub joint_solvers: Vec<(Physics3DJointKind, Physics3DJointSolver)>,
    pub joint_collision_suppressions: Vec<(f64, Vec<(f64, f64)>)>,
    pub events: Physics3DContactEvents,
    pub joint_events: Physics3DJointEvents,
    pub contact_hooks: Physics3DContactHooks,
    pub solver: Physics3DSequentialImpulseState,
    pub config: Physics3DSolverConfig,
    pub island_parents: Vec<(f64, f64)>,
    pub island_sleep_timers: Vec<(f64, f64)>,
    pub solve_island_by_root: Vec<(f64, f64)>,
    pub solve_island_roots: Vec<f64>,
    pub solve_island_body_starts: Vec<f64>,
    pub solve_island_body_counts: Vec<f64>,
    pub solve_island_contact_starts: Vec<f64>,
    pub solve_island_contact_counts: Vec<f64>,
    pub solve_island_joint_starts: Vec<f64>,
    pub solve_island_joint_counts: Vec<f64>,
    pub solve_island_body_indices: Vec<f64>,
    pub solve_island_contact_indices: Vec<f64>,
    pub solve_island_joint_indices: Vec<f64>,
    pub solve_island_cursors: Vec<f64>,
    pub gravity_x: f64,
    pub gravity_y: f64,
    pub gravity_z: f64,
    pub previous_timestep: f64,
    pub next_body_index: f64,
}
impl PartialEq for Physics3DWorld {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for Physics3DWorld {
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

// Source: upstream/packages/types/src/Physics3D.ts:1088 (sha256:7792e78038f5523c00971cf948144cd3278b69db94a5641e1c86cbc515bc3a05)
#[derive(Clone)]
pub struct Physics3DQueryHit {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub body: RigidBody3D,
    pub collider: Physics3DCollider,
    pub collider_index: f64,
}
impl PartialEq for Physics3DQueryHit {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Physics3D.ts:1097 (sha256:f2f3a94f5ffb4a9c1691bbd3facb23bae77209dd825b9f4a27f2bae7a3d3e6e8)
#[derive(Clone, Default)]
pub struct Physics3DQueryResult {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub hits: Vec<Physics3DQueryHit>,
    pub hit_count: f64,
}
impl PartialEq for Physics3DQueryResult {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for Physics3DQueryResult {
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

// Source: upstream/packages/types/src/Physics3D.ts:1106 (sha256:a3727d7957b822279860980d0b98a80159f61da3a42580d5a0d471267b343796)
#[derive(Clone, Default)]
pub struct Physics3DQueryFilter {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub category_bits: f64,
    pub mask_bits: f64,
    pub include_sensors: bool,
    pub include_dynamic: bool,
    pub include_kinematic: bool,
    pub include_static: bool,
}
impl PartialEq for Physics3DQueryFilter {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Physics3D.ts:1118 (sha256:073464a2cd7e45c2c2c21b0d11300740d7f5def31313b279b2aa4cf45ace64ee)
#[derive(Clone)]
pub struct Physics3DRayHit {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub body: RigidBody3D,
    pub collider: Physics3DCollider,
    pub collider_index: f64,
    pub fraction: f64,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub normal_x: f64,
    pub normal_y: f64,
    pub normal_z: f64,
}
impl PartialEq for Physics3DRayHit {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Physics3D.ts:1128 (sha256:1a3ea320bd4876c75e6ca521eca63262e7a81d07ad62d16c43b4ca158e7c43a9)
#[derive(Clone, Default)]
pub struct Physics3DRayResult {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub hits: Vec<Physics3DRayHit>,
    pub hit_count: f64,
}
impl PartialEq for Physics3DRayResult {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for Physics3DRayResult {
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

// Source: upstream/packages/types/src/Physics3D.ts:1152 (sha256:670a86ab2aabde4db7ee2aef04006ecf11d90b6054bc718b485faad48b6fea63)
#[derive(Clone, Default)]
pub struct Physics3DShapeCastResult {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub body: Option<RigidBody3D>,
    pub collider: Option<Physics3DCollider>,
    pub collider_index: f64,
    pub hit: bool,
    pub fraction: f64,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub normal_x: f64,
    pub normal_y: f64,
    pub normal_z: f64,
}
impl PartialEq for Physics3DShapeCastResult {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for Physics3DShapeCastResult {
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

// Source: upstream/packages/types/src/Physics3D.ts:1166 (sha256:e34c5fc23f41a153d60b952910d7836ca9a8cb27f56182a4b293d67160d80987)
pub type Physics3DDebugFeature = String;

// Source: upstream/packages/types/src/Physics3D.ts:1171 (sha256:014ffe2b00511300ce9adc974a2ea751a7c914cc48f691da871aef43d82ccf55)
#[derive(Clone, Default)]
pub struct Physics3DDebugLine {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub feature: Physics3DDebugFeature,
    pub body_a: f64,
    pub body_b: f64,
    pub x0: f64,
    pub y0: f64,
    pub z0: f64,
    pub x1: f64,
    pub y1: f64,
    pub z1: f64,
}
impl PartialEq for Physics3DDebugLine {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Physics3D.ts:1186 (sha256:0e79c0e577df39815ed88968cac6273ec826f2dc9606d8b09b1b69976d46f665)
#[derive(Clone, Default)]
pub struct Physics3DDebugSphere {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub feature: Physics3DDebugFeature,
    pub body_a: f64,
    pub body_b: f64,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub radius: f64,
}
impl PartialEq for Physics3DDebugSphere {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Physics3D.ts:1198 (sha256:7d395a395dcc05a7c618179bcceb051f7410c6202db767ef40771f838cb4bc1b)
#[derive(Clone, Default)]
pub struct Physics3DDebugGeometry {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    #[doc(hidden)]
    pub __flight_entity_runtime: std::sync::Arc<std::sync::Mutex<Option<crate::EntityRuntime>>>,
    #[doc(hidden)]
    pub __flight_entity_snapshot: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub lines: Vec<Physics3DDebugLine>,
    pub line_count: f64,
    pub spheres: Vec<Physics3DDebugSphere>,
    pub sphere_count: f64,
}
impl PartialEq for Physics3DDebugGeometry {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
impl crate::FlightEntity for Physics3DDebugGeometry {
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

// Source: upstream/packages/types/src/Physics3D.ts:1205 (sha256:8b70baa48bd7c02576b13e37f4b111d6dc40b03e3f6fcbba6137f8676d9c912f)
#[derive(Clone, Default)]
pub struct Physics3DDebugGeometryOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub draw_centers_of_mass: bool,
    pub draw_colliders: bool,
    pub draw_contacts: bool,
    pub draw_joints: bool,
    pub center_of_mass_radius: f64,
    pub contact_normal_length: f64,
}
impl PartialEq for Physics3DDebugGeometryOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
