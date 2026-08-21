// @generated from upstream/packages/types/src/Physics3D.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/Physics3D.ts:26 (sha256:f484c4d4fc65e7121ea4ff88c9adb0f7623093ca2872e50787cbb5537230204e)
pub type Physics3DBodyType = String;

// Source: upstream/packages/types/src/Physics3D.ts:35 (sha256:9a89e8b866e39db1683ae559bf348b722b56ddf4ceaca2ffb841f5c34d7d09fd)
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

// Source: upstream/packages/types/src/Physics3D.ts:45 (sha256:1ee81caf276760e1000d88bdb511703a0227b6a15e9054bf44500ff76dcab997)
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

// Source: upstream/packages/types/src/Physics3D.ts:63 (sha256:280e588114daf5dcd9e2597b4995772ffbf8cb4fecc34588b8f09d93669e2ca3)
#[derive(Clone, Default)]
pub struct Physics3DMassData {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
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

// Source: upstream/packages/types/src/Physics3D.ts:87 (sha256:3784ccd03f15909f538cb307f67f20e9c215eec82922b8b9a1a12316253731b0)
#[derive(Clone, Default)]
pub struct RigidBody3D {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
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
    pub material: Physics3DMaterial,
    pub filter: Physics3DCollisionFilter,
}
impl PartialEq for RigidBody3D {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Physics3D.ts:214 (sha256:37098485d1f3c0ab62d7c0e44485809fcf1db020778b9abdc217cc381057e848)
#[derive(Clone, Default)]
pub struct Physics3DContactPoint {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
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

// Source: upstream/packages/types/src/Physics3D.ts:243 (sha256:80fa54977329f20751f7c2bd023c5b0b1b6ece290cfa9bcd5edaf93f43aa9ec5)
#[derive(Clone, Default)]
pub struct Physics3DContact {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub body_a: f64,
    pub body_b: f64,
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

// Source: upstream/packages/types/src/Physics3D.ts:270 (sha256:bc73203c050847a8a91cb49f7a9dc71b915906866c51853b2899cdeb47bf1f7b)
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

// Source: upstream/packages/types/src/Physics3D.ts:280 (sha256:df14fb65f782b9f5aa3355e358ab0e2f1089989c283ee79b53a2a6e2960f42d0)
pub type Physics3DContactCallback = std::sync::Arc<
    std::sync::Mutex<Box<dyn FnMut(Physics3DWorld, Physics3DContact) -> () + Send + 'static>>,
>;

// Source: upstream/packages/types/src/Physics3D.ts:282 (sha256:8aa26c8f4630cb57690992902681fd3d1059268cf511deab5945f5a7b2d9c043)
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

// Source: upstream/packages/types/src/Physics3D.ts:295 (sha256:262ba078e6dc17f030e8b05484f7767f17eec8b6912054f110ba8e1c96ebc41a)
#[derive(Clone, Default)]
pub struct Physics3DContactConstraintPoint {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
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

// Source: upstream/packages/types/src/Physics3D.ts:315 (sha256:6d07842a2d670fb1682fd93b1839bfaa4fe0a59f9deb40eaf02e28f44c143c85)
#[derive(Clone, Default)]
pub struct Physics3DContactConstraint {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
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

// Source: upstream/packages/types/src/Physics3D.ts:337 (sha256:30d66ea2af3409e1b0c2bd65a21eb638acbedf4459c976c4975d470e1a986b3e)
#[derive(Clone, Default)]
pub struct Physics3DSequentialImpulseState {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub constraints: Vec<Physics3DContactConstraint>,
    pub constraint_by_pair: Vec<(f64, Physics3DContactConstraint)>,
}
impl PartialEq for Physics3DSequentialImpulseState {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Physics3D.ts:352 (sha256:0de6797dfe9c8f540e889ae0af3a97e5cd7aff54d43a7d4cc885954bac19f8df)
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

// Source: upstream/packages/types/src/Physics3D.ts:367 (sha256:505d2ba56fc2f74759b113fadc396ec3e642b407eb3177163ae8edfcaae6972c)
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
    pub sequential_impulse: Physics3DSequentialImpulseConfig,
}
impl PartialEq for Physics3DSolverConfig {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Physics3D.ts:398 (sha256:f0152be6c208ba749f2c81a908e2116ef8f9c2e3c3e1aa57950abb0e70ad3b7a)
#[derive(Clone, Default)]
pub struct Physics3DStepExplanation {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub body_state_valid: bool,
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

// Source: upstream/packages/types/src/Physics3D.ts:414 (sha256:4c21e062154663c79db003b34d4bbfcacbe747d4e450b94d58b60b1f45819ce0)
pub type Physics3DStepGuard =
    std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(Physics3DWorld, f64) -> () + Send + 'static>>>;

// Source: upstream/packages/types/src/Physics3D.ts:418 (sha256:c042cce5b85152550c502f3e506881254aaae28f10d57a1fa2172493a13e6fe8)
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

// Source: upstream/packages/types/src/Physics3D.ts:431 (sha256:d9bf6924c96c62b3c26137c72cf20c4138932d592568b0c4bb001a3396efca4e)
pub type Physics3DJointKind = String;

// Source: upstream/packages/types/src/Physics3D.ts:443 (sha256:44db7e07ae0224f9e406590d2a1c2bfa14bc80e5a124b0776c96e340280d5d77)
#[derive(Clone, Default)]
pub struct Physics3DJoint {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
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

// Source: upstream/packages/types/src/Physics3D.ts:488 (sha256:8c1cc0879f76d18ce3d5e4c672cbdd729975bade815068a6aed6fe618c078dd1)
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

// Source: upstream/packages/types/src/Physics3D.ts:529 (sha256:bafab0a5ea70df233986468723f861cc7dff0a5b9eac722d256fe2f19e344f97)
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

// Source: upstream/packages/types/src/Physics3D.ts:542 (sha256:3b0da11468a92a3dd675ec3d6cf7205335cf475811446c07dc627a00a60534f2)
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
}
impl PartialEq for Physics3DJointOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Physics3D.ts:556 (sha256:edac07364f90df64b39c2d1ac50a306e9877d7e8c4ab1c4d6c02dc7e80cb0395)
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

// Source: upstream/packages/types/src/Physics3D.ts:573 (sha256:757088138e3362c0c0cb7259922734c69c19eb0cee7423a4a77895a1de6d83bc)
pub type Physics3DBallAndSocketJoint = Physics3DJoint;

// Source: upstream/packages/types/src/Physics3D.ts:574 (sha256:013b90024d32a84652fb6c5293125aabb56a10f6ad832222040754cb21fe7e45)
pub type Physics3DBallAndSocketJointOptions = Physics3DJointOptions;

// Source: upstream/packages/types/src/Physics3D.ts:580 (sha256:b19829018e650687bb9eb237e86bb2723046d72879d622b2138cf9f433d97ae8)
#[derive(Clone, Default)]
pub struct Physics3DFixedJoint {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
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

// Source: upstream/packages/types/src/Physics3D.ts:581 (sha256:710c2552396be6cb30b1c8e6f45230b3a759d479384a256b7a8cf93e9efcb690)
pub type Physics3DFixedJointOptions = Physics3DJointFrameOptions;

// Source: upstream/packages/types/src/Physics3D.ts:591 (sha256:3617e061adc1153c05c953faf6638b850bdd8a7cf5dc7ea4924c776b3ae87730)
#[derive(Clone, Default)]
pub struct Physics3DHingeJoint {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
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
    pub lower_limit_impulse: f64,
    pub upper_limit_impulse: f64,
}
impl PartialEq for Physics3DHingeJoint {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Physics3D.ts:616 (sha256:f85e886c09a0ae68a8b28e74d19a9516295aef1859f5dac5446ecaeec7f60703)
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
}
impl PartialEq for Physics3DHingeJointOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Physics3D.ts:630 (sha256:2e63948889100e2121b93104f31cca5a88c20e619e5fe18c77d2df4c201b07fd)
#[derive(Clone, Default)]
pub struct Physics3DSliderJoint {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
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
    pub lower_limit_impulse: f64,
    pub upper_limit_impulse: f64,
}
impl PartialEq for Physics3DSliderJoint {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Physics3D.ts:646 (sha256:9958ebade28e19f3a7b5a3edd52859a3f4a17e4005d02e581292b130be0607f8)
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
}
impl PartialEq for Physics3DSliderJointOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Physics3D.ts:667 (sha256:d9850010c5e1c18420f0c418abd2e3c55fff9bd10e447971605d96046db624a5)
#[derive(Clone, Default)]
pub struct Physics3DConeTwistJoint {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
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
    pub swing_limit_impulse: f64,
    pub lower_twist_impulse: f64,
    pub upper_twist_impulse: f64,
}
impl PartialEq for Physics3DConeTwistJoint {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Physics3D.ts:684 (sha256:d9cfb6a6cff69f6fe6b64489d36030ab639f595d5c0db085381fc52ac76b9587)
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
}
impl PartialEq for Physics3DConeTwistJointOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Physics3D.ts:707 (sha256:4ccbdf1e8aedb77ad285e1cbb2fac79c0038c24d9301c0dd3e56c0a9c0e8dd85)
#[derive(Clone, Default)]
pub struct Physics3DGeneric6DofJoint {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
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
    pub lower_limit_impulses: Vec<f64>,
    pub upper_limit_impulses: Vec<f64>,
}
impl PartialEq for Physics3DGeneric6DofJoint {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Physics3D.ts:733 (sha256:40930c997e8c14a8706c563ebe7385d8392f11fcbb0e0ed4ebc0d9f3d1d7c083)
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
}
impl PartialEq for Physics3DGeneric6DofJointOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Physics3D.ts:757 (sha256:478fc67f4c76a013a87b78935ea0183d26e38fcb81c608e340f38357ab1bde6a)
#[derive(Clone, Default)]
pub struct Physics3DWorld {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub version: f64,
    pub bodies: Vec<RigidBody3D>,
    pub body_by_index: Vec<(f64, RigidBody3D)>,
    pub contacts: Vec<Physics3DContact>,
    pub joints: Vec<Physics3DJoint>,
    pub joint_solvers: Vec<(Physics3DJointKind, Physics3DJointSolver)>,
    pub joint_collision_suppressions: Vec<(f64, Vec<(f64, f64)>)>,
    pub events: Physics3DContactEvents,
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
