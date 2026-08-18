// @generated from upstream/packages/types/src/Physics2D.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{CollisionShape, SpatialIndexBackend};

// Source: upstream/packages/types/src/Physics2D.ts:20 (sha256:56a25100e0e69754e977ebb43bccec1d6fad99d9d7b856c50649af088a0174ab)
pub type Physics2DBodyType = String;

// Source: upstream/packages/types/src/Physics2D.ts:32 (sha256:28fd30f1648e442a5496dba2028bc61e3cf16cd1e7732a4037097077eaf61df9)
#[derive(Clone, Default)]
pub struct Physics2DMaterial {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub density: f64,
    pub friction: f64,
    pub restitution: f64,
}
impl PartialEq for Physics2DMaterial {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Physics2D.ts:42 (sha256:75426e23189c515f9ca0c4b183320489cc0fcb266b98afeecb54710a606110ab)
#[derive(Clone, Default)]
pub struct Physics2DCollisionFilter {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub category_bits: f64,
    pub mask_bits: f64,
    pub group_index: f64,
}
impl PartialEq for Physics2DCollisionFilter {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Physics2D.ts:60 (sha256:61a33980287691a1d2e1de55628a62dc799ca4529f6c87a035683162ee3e72ce)
#[derive(Clone, Default)]
pub struct Physics2DCollider {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub local: CollisionShape,
    pub world: CollisionShape,
    pub material: Physics2DMaterial,
    pub filter: Physics2DCollisionFilter,
    pub sensor: bool,
}
impl PartialEq for Physics2DCollider {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Physics2D.ts:78 (sha256:4db498c8ac68087d55e1489e845ae6c93c321ef8e63c84e2848d03acd2aca853)
#[derive(Clone, Default)]
pub struct Physics2DMassData {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub mass: f64,
    pub inertia: f64,
    pub center_x: f64,
    pub center_y: f64,
}
impl PartialEq for Physics2DMassData {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Physics2D.ts:98 (sha256:a5206341ac2d9702885db0f9ba9da7d8032b3ea3961e7f34536b715e75b467b7)
#[derive(Clone, Default)]
pub struct RigidBody2D {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub index: f64,
    pub type_: Physics2DBodyType,
    pub x: f64,
    pub y: f64,
    pub angle: f64,
    pub velocity_x: f64,
    pub velocity_y: f64,
    pub angular_velocity: f64,
    pub force_x: f64,
    pub force_y: f64,
    pub torque: f64,
    pub mass: f64,
    pub inverse_mass: f64,
    pub inertia: f64,
    pub inverse_inertia: f64,
    pub center_x: f64,
    pub center_y: f64,
    pub linear_damping: f64,
    pub angular_damping: f64,
    pub gravity_scale: f64,
    pub fixed_rotation: bool,
    pub bullet: bool,
    pub sleeping: bool,
    pub sleep_enabled: bool,
    pub sleep_timer: f64,
    pub colliders: Vec<Physics2DCollider>,
}
impl PartialEq for RigidBody2D {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Physics2D.ts:162 (sha256:c001573502aaf804acac17d7c6b00d0c49e53e5c2134ea34ff3761cdba00072b)
#[derive(Clone, Default)]
pub struct Physics2DContactPoint {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub x: f64,
    pub y: f64,
    pub depth: f64,
    pub feature_id: f64,
    pub r_ax: f64,
    pub r_ay: f64,
    pub r_bx: f64,
    pub r_by: f64,
    pub normal_impulse: f64,
    pub tangent_impulse: f64,
    pub normal_mass: f64,
    pub tangent_mass: f64,
    pub bias: f64,
}
impl PartialEq for Physics2DContactPoint {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Physics2D.ts:196 (sha256:3a89f0bc11ff1e68096dbb0499ae192d3abb1cde4962391c47b614b9bc6d616f)
#[derive(Clone, Default)]
pub struct Physics2DContact {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub body_a: f64,
    pub body_b: f64,
    pub collider_a: f64,
    pub collider_b: f64,
    pub normal_x: f64,
    pub normal_y: f64,
    pub point_count: f64,
    pub points: Vec<Physics2DContactPoint>,
    pub friction: f64,
    pub restitution: f64,
    pub enabled: bool,
    pub sensor: bool,
    pub touching: bool,
}
impl PartialEq for Physics2DContact {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Physics2D.ts:227 (sha256:bfd80cbc8eb537da428b2fbfe476a3d02e86ecedf66ee42850da238aea0f6fe2)
pub type Physics2DContactCallback = std::sync::Arc<
    std::sync::Mutex<Box<dyn FnMut(Physics2DWorld, Physics2DContact) -> () + Send + 'static>>,
>;

// Source: upstream/packages/types/src/Physics2D.ts:229 (sha256:5005685a95c0d38e2864c53d20e121efbaf4b16aedac415200af934f8e1f0d0b)
#[derive(Clone, Default)]
pub struct Physics2DContactHooks {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub pre_solve: Option<Physics2DContactCallback>,
    pub post_solve: Option<Physics2DContactCallback>,
}
impl PartialEq for Physics2DContactHooks {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Physics2D.ts:244 (sha256:29644de2ca268e7003a01a34866533f5279d4bc6da62b2de3f2f702b1a5eaaab)
#[derive(Clone, Default)]
pub struct Physics2DSolverConfig {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub allow_sleeping: bool,
    pub sleep_linear_threshold: f64,
    pub sleep_angular_threshold: f64,
    pub time_to_sleep: f64,
    pub velocity_iterations: f64,
    pub position_iterations: f64,
    pub penetration_slop: f64,
    pub position_correction: f64,
    pub restitution_threshold: f64,
    pub warm_starting: bool,
    pub continuous_collision: bool,
    pub max_ccd_substeps: f64,
    pub max_ccd_rotation_substeps: f64,
}
impl PartialEq for Physics2DSolverConfig {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Physics2D.ts:273 (sha256:bf03f6784633fdf67da2ac96c370093dcf9a1989203e9fc784ba2dbf2e975fa1)
#[derive(Clone, Default)]
pub struct Physics2DStepExplanation {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub body_state_valid: bool,
    pub contact_state_valid: bool,
    pub gravity_valid: bool,
    pub joint_state_valid: bool,
    pub previous_timestep_valid: bool,
    pub solver_config_valid: bool,
    pub timestep_valid: bool,
    pub velocity_iterations_valid: bool,
    pub position_iterations_valid: bool,
    pub status: String,
}
impl PartialEq for Physics2DStepExplanation {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Physics2D.ts:292 (sha256:a28a94c95326e5405d33feda957ea8ee57399e1266dae9f9d7c88218d945a9fe)
#[derive(Clone)]
pub struct Physics2DWorld {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub version: f64,
    pub bodies: Vec<RigidBody2D>,
    pub body_by_index: Vec<(f64, RigidBody2D)>,
    pub contacts: Vec<Physics2DContact>,
    pub joints: Vec<Physics2DJoint>,
    pub joint_solvers: Vec<(Physics2DJointKind, Physics2DJointSolver)>,
    pub joint_collision_suppressions: Vec<(f64, Vec<(f64, f64)>)>,
    pub events: Physics2DContactEvents,
    pub contact_hooks: Physics2DContactHooks,
    pub index: SpatialIndexBackend,
    pub config: Physics2DSolverConfig,
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
    pub previous_timestep: f64,
    pub next_body_index: f64,
}
impl PartialEq for Physics2DWorld {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Physics2D.ts:357 (sha256:d9399bc897fd454947f22d5b87a01421946093d4fc4629391a3d6af4e38d0673)
pub type Physics2DJointKind = String;

// Source: upstream/packages/types/src/Physics2D.ts:370 (sha256:41a953a2830748f220897697a487ee6df51a68981a188aa6ce1c3a14cd66efc5)
#[derive(Clone, Default)]
pub struct Physics2DJoint {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: Physics2DJointKind,
    pub body_a: f64,
    pub body_b: f64,
    pub local_anchor_ax: f64,
    pub local_anchor_ay: f64,
    pub local_anchor_bx: f64,
    pub local_anchor_by: f64,
    pub collide_connected: bool,
    pub impulse0: f64,
    pub impulse1: f64,
    pub impulse2: f64,
    pub r_ax: f64,
    pub r_ay: f64,
    pub r_bx: f64,
    pub r_by: f64,
}
impl PartialEq for Physics2DJoint {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Physics2D.ts:401 (sha256:49aef58282491cc209d66ae89145f081fb43a9ae6f7a7abe776a55eba6722255)
#[derive(Clone, Default)]
pub struct Physics2DDistanceJoint {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: Physics2DJointKind,
    pub body_a: f64,
    pub body_b: f64,
    pub local_anchor_ax: f64,
    pub local_anchor_ay: f64,
    pub local_anchor_bx: f64,
    pub local_anchor_by: f64,
    pub collide_connected: bool,
    pub impulse0: f64,
    pub impulse1: f64,
    pub impulse2: f64,
    pub r_ax: f64,
    pub r_ay: f64,
    pub r_bx: f64,
    pub r_by: f64,
    pub length: f64,
    pub frequency_hz: f64,
    pub damping_ratio: f64,
}
impl PartialEq for Physics2DDistanceJoint {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Physics2D.ts:410 (sha256:f45bdd8ec8f78d20609e6978dab03cf413f0cd31305168d66313d7eb273099a1)
#[derive(Clone, Default)]
pub struct Physics2DRevoluteJoint {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: Physics2DJointKind,
    pub body_a: f64,
    pub body_b: f64,
    pub local_anchor_ax: f64,
    pub local_anchor_ay: f64,
    pub local_anchor_bx: f64,
    pub local_anchor_by: f64,
    pub collide_connected: bool,
    pub impulse0: f64,
    pub impulse1: f64,
    pub impulse2: f64,
    pub r_ax: f64,
    pub r_ay: f64,
    pub r_bx: f64,
    pub r_by: f64,
    pub enable_motor: bool,
    pub motor_speed: f64,
    pub max_motor_torque: f64,
    pub motor_impulse: f64,
    pub enable_limit: bool,
    pub lower_angle: f64,
    pub upper_angle: f64,
    pub reference_angle: f64,
}
impl PartialEq for Physics2DRevoluteJoint {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Physics2D.ts:424 (sha256:3cd1f303c8360981be0e1a34e91acf297a5cc5f7cef9e6cc4a610a397f20a51f)
#[derive(Clone, Default)]
pub struct Physics2DWeldJoint {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: Physics2DJointKind,
    pub body_a: f64,
    pub body_b: f64,
    pub local_anchor_ax: f64,
    pub local_anchor_ay: f64,
    pub local_anchor_bx: f64,
    pub local_anchor_by: f64,
    pub collide_connected: bool,
    pub impulse0: f64,
    pub impulse1: f64,
    pub impulse2: f64,
    pub r_ax: f64,
    pub r_ay: f64,
    pub r_bx: f64,
    pub r_by: f64,
    pub reference_angle: f64,
}
impl PartialEq for Physics2DWeldJoint {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Physics2D.ts:431 (sha256:14e73420781716fb117238d19bcdb4f60f2e774649530cfaad6497664f4ec9fd)
#[derive(Clone, Default)]
pub struct Physics2DRopeJoint {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: Physics2DJointKind,
    pub body_a: f64,
    pub body_b: f64,
    pub local_anchor_ax: f64,
    pub local_anchor_ay: f64,
    pub local_anchor_bx: f64,
    pub local_anchor_by: f64,
    pub collide_connected: bool,
    pub impulse0: f64,
    pub impulse1: f64,
    pub impulse2: f64,
    pub r_ax: f64,
    pub r_ay: f64,
    pub r_bx: f64,
    pub r_by: f64,
    pub max_length: f64,
}
impl PartialEq for Physics2DRopeJoint {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Physics2D.ts:438 (sha256:a169f1f5512b2bf35e7587690e6ef634681878d267026c2ab03a3dafd517ed12)
#[derive(Clone, Default)]
pub struct Physics2DPulleyJoint {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: Physics2DJointKind,
    pub body_a: f64,
    pub body_b: f64,
    pub local_anchor_ax: f64,
    pub local_anchor_ay: f64,
    pub local_anchor_bx: f64,
    pub local_anchor_by: f64,
    pub collide_connected: bool,
    pub impulse0: f64,
    pub impulse1: f64,
    pub impulse2: f64,
    pub r_ax: f64,
    pub r_ay: f64,
    pub r_bx: f64,
    pub r_by: f64,
    pub ground_anchor_ax: f64,
    pub ground_anchor_ay: f64,
    pub ground_anchor_bx: f64,
    pub ground_anchor_by: f64,
    pub ratio: f64,
    pub constant: f64,
}
impl PartialEq for Physics2DPulleyJoint {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Physics2D.ts:447 (sha256:18764b4f3d144a8ee09a432e0a3778f8843436502107fa5120e453af89cea4d7)
pub type Physics2DGearCoordinateKind = String;

// Source: upstream/packages/types/src/Physics2D.ts:454 (sha256:7a2a5c30028a7ebe59854b90338612f99a484da9d05bd71eaaa7de448bbb2b7c)
#[derive(Clone, Default)]
pub struct Physics2DGearJoint {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: Physics2DJointKind,
    pub body_a: f64,
    pub body_b: f64,
    pub local_anchor_ax: f64,
    pub local_anchor_ay: f64,
    pub local_anchor_bx: f64,
    pub local_anchor_by: f64,
    pub collide_connected: bool,
    pub impulse0: f64,
    pub impulse1: f64,
    pub impulse2: f64,
    pub r_ax: f64,
    pub r_ay: f64,
    pub r_bx: f64,
    pub r_by: f64,
    pub coordinate_a: Physics2DGearCoordinateKind,
    pub coordinate_b: Physics2DGearCoordinateKind,
    pub axis_ax: f64,
    pub axis_ay: f64,
    pub axis_bx: f64,
    pub axis_by: f64,
    pub ratio: f64,
    pub constant: f64,
}
impl PartialEq for Physics2DGearJoint {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Physics2D.ts:467 (sha256:7047314bcb16b3ebe3626298c25398572a66360e5e872133b33a21415c4e2e88)
#[derive(Clone, Default)]
pub struct Physics2DPrismaticJoint {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: Physics2DJointKind,
    pub body_a: f64,
    pub body_b: f64,
    pub local_anchor_ax: f64,
    pub local_anchor_ay: f64,
    pub local_anchor_bx: f64,
    pub local_anchor_by: f64,
    pub collide_connected: bool,
    pub impulse0: f64,
    pub impulse1: f64,
    pub impulse2: f64,
    pub r_ax: f64,
    pub r_ay: f64,
    pub r_bx: f64,
    pub r_by: f64,
    pub local_axis_ax: f64,
    pub local_axis_ay: f64,
    pub reference_angle: f64,
    pub enable_motor: bool,
    pub motor_speed: f64,
    pub max_motor_force: f64,
    pub motor_impulse: f64,
    pub enable_limit: bool,
    pub lower_translation: f64,
    pub upper_translation: f64,
}
impl PartialEq for Physics2DPrismaticJoint {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Physics2D.ts:484 (sha256:70b93b46c79fe10b1370af8bf3c98f46e51df7082eb8bc53ed35ae7680de8fd4)
#[derive(Clone, Default)]
pub struct Physics2DWheelJoint {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: Physics2DJointKind,
    pub body_a: f64,
    pub body_b: f64,
    pub local_anchor_ax: f64,
    pub local_anchor_ay: f64,
    pub local_anchor_bx: f64,
    pub local_anchor_by: f64,
    pub collide_connected: bool,
    pub impulse0: f64,
    pub impulse1: f64,
    pub impulse2: f64,
    pub r_ax: f64,
    pub r_ay: f64,
    pub r_bx: f64,
    pub r_by: f64,
    pub local_axis_ax: f64,
    pub local_axis_ay: f64,
    pub rest_translation: f64,
    pub frequency_hz: f64,
    pub damping_ratio: f64,
    pub enable_motor: bool,
    pub motor_speed: f64,
    pub max_motor_torque: f64,
    pub motor_impulse: f64,
}
impl PartialEq for Physics2DWheelJoint {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Physics2D.ts:500 (sha256:664e4ea898a800925784de626419296c5db614ca118bb982780fc280589a5c90)
#[derive(Clone, Default)]
pub struct Physics2DMouseJoint {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub kind: Physics2DJointKind,
    pub body_a: f64,
    pub body_b: f64,
    pub local_anchor_ax: f64,
    pub local_anchor_ay: f64,
    pub local_anchor_bx: f64,
    pub local_anchor_by: f64,
    pub collide_connected: bool,
    pub impulse0: f64,
    pub impulse1: f64,
    pub impulse2: f64,
    pub r_ax: f64,
    pub r_ay: f64,
    pub r_bx: f64,
    pub r_by: f64,
    pub target_x: f64,
    pub target_y: f64,
    pub max_force: f64,
    pub frequency_hz: f64,
    pub damping_ratio: f64,
}
impl PartialEq for Physics2DMouseJoint {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Physics2D.ts:511 (sha256:d80659201a446d2c586629608dce3f08746d7cfb0675d9465607bf17f9bae242)
#[derive(Clone, Default)]
pub struct Physics2DJointOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub body_a: f64,
    pub body_b: f64,
    pub local_anchor_ax: Option<f64>,
    pub local_anchor_ay: Option<f64>,
    pub local_anchor_bx: Option<f64>,
    pub local_anchor_by: Option<f64>,
    pub collide_connected: Option<bool>,
}
impl PartialEq for Physics2DJointOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Physics2D.ts:521 (sha256:85755bcc12232b6e020df52315e71d52db17b89510a40769fb2d26fc455357ab)
#[derive(Clone, Default)]
pub struct Physics2DDistanceJointOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub body_a: f64,
    pub body_b: f64,
    pub local_anchor_ax: Option<f64>,
    pub local_anchor_ay: Option<f64>,
    pub local_anchor_bx: Option<f64>,
    pub local_anchor_by: Option<f64>,
    pub collide_connected: Option<bool>,
    pub length: f64,
    pub frequency_hz: Option<f64>,
    pub damping_ratio: Option<f64>,
}
impl PartialEq for Physics2DDistanceJointOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Physics2D.ts:527 (sha256:96dea12d6d63d82bc32a2184f0a4acc14b2931b26642698cd6e87006ef64ccf5)
#[derive(Clone, Default)]
pub struct Physics2DRevoluteJointOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub body_a: f64,
    pub body_b: f64,
    pub local_anchor_ax: Option<f64>,
    pub local_anchor_ay: Option<f64>,
    pub local_anchor_bx: Option<f64>,
    pub local_anchor_by: Option<f64>,
    pub collide_connected: Option<bool>,
    pub enable_motor: Option<bool>,
    pub motor_speed: Option<f64>,
    pub max_motor_torque: Option<f64>,
    pub enable_limit: Option<bool>,
    pub lower_angle: Option<f64>,
    pub upper_angle: Option<f64>,
    pub reference_angle: Option<f64>,
}
impl PartialEq for Physics2DRevoluteJointOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Physics2D.ts:537 (sha256:f9fc6035017a8b5e1f85ecffa76f6db48c2b049a506f952cc547e126002c3c9e)
#[derive(Clone, Default)]
pub struct Physics2DWeldJointOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub body_a: f64,
    pub body_b: f64,
    pub local_anchor_ax: Option<f64>,
    pub local_anchor_ay: Option<f64>,
    pub local_anchor_bx: Option<f64>,
    pub local_anchor_by: Option<f64>,
    pub collide_connected: Option<bool>,
    pub reference_angle: Option<f64>,
}
impl PartialEq for Physics2DWeldJointOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Physics2D.ts:541 (sha256:aa67ce3449b80ff36751dd1496633651a2bfe8f2c014ac49b8d454a622fde60e)
#[derive(Clone, Default)]
pub struct Physics2DRopeJointOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub body_a: f64,
    pub body_b: f64,
    pub local_anchor_ax: Option<f64>,
    pub local_anchor_ay: Option<f64>,
    pub local_anchor_bx: Option<f64>,
    pub local_anchor_by: Option<f64>,
    pub collide_connected: Option<bool>,
    pub max_length: f64,
}
impl PartialEq for Physics2DRopeJointOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Physics2D.ts:545 (sha256:8deac03fff883b2e6da269b0b5ba41242add8a73a608d6259e1bffade21e8cdc)
#[derive(Clone, Default)]
pub struct Physics2DPulleyJointOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub body_a: f64,
    pub body_b: f64,
    pub local_anchor_ax: Option<f64>,
    pub local_anchor_ay: Option<f64>,
    pub local_anchor_bx: Option<f64>,
    pub local_anchor_by: Option<f64>,
    pub collide_connected: Option<bool>,
    pub ground_anchor_ax: f64,
    pub ground_anchor_ay: f64,
    pub ground_anchor_bx: f64,
    pub ground_anchor_by: f64,
    pub constant: f64,
    pub ratio: Option<f64>,
}
impl PartialEq for Physics2DPulleyJointOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Physics2D.ts:554 (sha256:78dcb4caa67b151f54c1f2a426c1264d79135c2d29ddbf220e1f930bbd6fdda1)
#[derive(Clone, Default)]
pub struct Physics2DGearJointOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub body_a: f64,
    pub body_b: f64,
    pub local_anchor_ax: Option<f64>,
    pub local_anchor_ay: Option<f64>,
    pub local_anchor_bx: Option<f64>,
    pub local_anchor_by: Option<f64>,
    pub collide_connected: Option<bool>,
    pub coordinate_a: Physics2DGearCoordinateKind,
    pub coordinate_b: Physics2DGearCoordinateKind,
    pub constant: f64,
    pub axis_ax: Option<f64>,
    pub axis_ay: Option<f64>,
    pub axis_bx: Option<f64>,
    pub axis_by: Option<f64>,
    pub ratio: Option<f64>,
}
impl PartialEq for Physics2DGearJointOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Physics2D.ts:565 (sha256:c96ac7eb691c145a2c2a1c971160cca42ae2382b4cace6917437f2fbe6d98fae)
#[derive(Clone, Default)]
pub struct Physics2DPrismaticJointOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub body_a: f64,
    pub body_b: f64,
    pub local_anchor_ax: Option<f64>,
    pub local_anchor_ay: Option<f64>,
    pub local_anchor_bx: Option<f64>,
    pub local_anchor_by: Option<f64>,
    pub collide_connected: Option<bool>,
    pub local_axis_ax: Option<f64>,
    pub local_axis_ay: Option<f64>,
    pub reference_angle: Option<f64>,
    pub enable_motor: Option<bool>,
    pub motor_speed: Option<f64>,
    pub max_motor_force: Option<f64>,
    pub enable_limit: Option<bool>,
    pub lower_translation: Option<f64>,
    pub upper_translation: Option<f64>,
}
impl PartialEq for Physics2DPrismaticJointOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Physics2D.ts:577 (sha256:9775462cd5ce4402fcc41ff2f91c58cc0fefc7a6b29602a4f44f568e829a5334)
#[derive(Clone, Default)]
pub struct Physics2DWheelJointOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub body_a: f64,
    pub body_b: f64,
    pub local_anchor_ax: Option<f64>,
    pub local_anchor_ay: Option<f64>,
    pub local_anchor_bx: Option<f64>,
    pub local_anchor_by: Option<f64>,
    pub collide_connected: Option<bool>,
    pub local_axis_ax: Option<f64>,
    pub local_axis_ay: Option<f64>,
    pub rest_translation: Option<f64>,
    pub frequency_hz: Option<f64>,
    pub damping_ratio: Option<f64>,
    pub enable_motor: Option<bool>,
    pub motor_speed: Option<f64>,
    pub max_motor_torque: Option<f64>,
}
impl PartialEq for Physics2DWheelJointOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Physics2D.ts:590 (sha256:e35c04cec06deb040e790ba9683b30466109fee8b73fe27022136defde1c7501)
#[derive(Clone, Default)]
pub struct Physics2DMouseJointOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub body: f64,
    pub target_x: f64,
    pub target_y: f64,
    pub max_force: f64,
    pub local_anchor_x: Option<f64>,
    pub local_anchor_y: Option<f64>,
    pub frequency_hz: Option<f64>,
    pub damping_ratio: Option<f64>,
}
impl PartialEq for Physics2DMouseJointOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Physics2D.ts:609 (sha256:05edc17f0609af585d9ef7c796dfbeb00be72621120f88b84b9c91b072153942)
#[derive(Clone)]
pub struct Physics2DJointSolver {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub prepare: std::sync::Arc<
        std::sync::Mutex<
            Box<dyn FnMut(Physics2DWorld, Physics2DJoint, f64) -> () + Send + 'static>,
        >,
    >,
    pub solve: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut(Physics2DWorld, Physics2DJoint) -> () + Send + 'static>>,
    >,
    pub uses_body_a: Option<bool>,
    pub keeps_bodies_awake: Option<bool>,
    pub swap_ends: Option<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(Physics2DJoint) -> bool + Send + 'static>>>,
    >,
    pub warm_start: Option<
        std::sync::Arc<
            std::sync::Mutex<Box<dyn FnMut(Physics2DWorld, Physics2DJoint) -> () + Send + 'static>>,
        >,
    >,
    pub scale_accumulated_impulses: Option<
        std::sync::Arc<
            std::sync::Mutex<Box<dyn FnMut(Physics2DJoint, f64) -> () + Send + 'static>>,
        >,
    >,
    pub clear_accumulated_impulses: Option<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(Physics2DJoint) -> () + Send + 'static>>>,
    >,
}
impl PartialEq for Physics2DJointSolver {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Physics2D.ts:653 (sha256:050084dc75e29590d70cf1bc07935d262935cade3baccadcb460c45834a90c6a)
#[derive(Clone, Default)]
pub struct Physics2DContactEvents {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub began: Vec<Physics2DContact>,
    pub ended: Vec<Physics2DContact>,
}
impl PartialEq for Physics2DContactEvents {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Physics2D.ts:661 (sha256:170b608b1adbe4f5d5adbf9037aae861cd32250fda3110f11bf5dde9eb8b0832)
#[derive(Clone, Default)]
pub struct Physics2DQueryHit {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub body: RigidBody2D,
    pub collider: Physics2DCollider,
    pub collider_index: f64,
}
impl PartialEq for Physics2DQueryHit {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Physics2D.ts:669 (sha256:70501c86dcdcd798ac02822859ebe0836dac6adc52887c67c95a1848ab9405d4)
#[derive(Clone, Default)]
pub struct Physics2DQueryResult {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub hits: Vec<Physics2DQueryHit>,
    pub hit_count: f64,
}
impl PartialEq for Physics2DQueryResult {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Physics2D.ts:677 (sha256:d5fad3e69896ef87b90849b081b585fd0d0ef60cf75dfe8a788516888db17997)
#[derive(Clone, Default)]
pub struct Physics2DQueryFilter {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub category_bits: f64,
    pub mask_bits: f64,
    pub include_sensors: bool,
    pub include_dynamic: bool,
    pub include_kinematic: bool,
    pub include_static: bool,
}
impl PartialEq for Physics2DQueryFilter {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Physics2D.ts:686 (sha256:9094ab4baa041a3973eb2471908827999044b59892109431e6ce46c93436a483)
#[derive(Clone, Default)]
pub struct Physics2DRayHit {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub body: RigidBody2D,
    pub collider: Physics2DCollider,
    pub collider_index: f64,
    pub fraction: f64,
    pub x: f64,
    pub y: f64,
    pub normal_x: f64,
    pub normal_y: f64,
}
impl PartialEq for Physics2DRayHit {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Physics2D.ts:696 (sha256:dcd5f590b1f242ab29d2afd97181bbc6e1cfeb173ca544ef716f2f321130ebd8)
#[derive(Clone, Default)]
pub struct Physics2DRayResult {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub hits: Vec<Physics2DRayHit>,
    pub hit_count: f64,
}
impl PartialEq for Physics2DRayResult {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Physics2D.ts:701 (sha256:3734762e415651440e9db023f04ab4f01e62c648419c7a598fd048ebea3f6a0e)
pub type Physics2DJointResolutionStatus = String;

// Source: upstream/packages/types/src/Physics2D.ts:710 (sha256:66c9b6fd86d8b673b601b333284fa35a8ac7b10390221f667e50d54cfd54e362)
#[derive(Clone, Default)]
pub struct Physics2DJointResolution {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub joint_index: f64,
    pub kind: Physics2DJointKind,
    pub body_a: f64,
    pub body_b: f64,
    pub body_a_found: bool,
    pub body_a_used: bool,
    pub body_b_found: bool,
    pub solver_registered: bool,
    pub status: Physics2DJointResolutionStatus,
}
impl PartialEq for Physics2DJointResolution {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Physics2D.ts:722 (sha256:2b53073ddae56946cb5e7ab5666ba8d35426f7c61941745efd3581fe3dcd4fb7)
#[derive(Clone, Default)]
pub struct Physics2DJointResolutionExplanation {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub joints: Vec<Physics2DJointResolution>,
    pub ready_count: f64,
    pub status: String,
}
impl PartialEq for Physics2DJointResolutionExplanation {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Physics2D.ts:728 (sha256:ceb7de12db62d422c92896660358f66ed2573b4344757c2019eac4198c0baae6)
pub type Physics2DDebugFeature = String;

// Source: upstream/packages/types/src/Physics2D.ts:733 (sha256:40cd280d4b3477b864e91ec95bff9c922335f937200478f6f56d4e542bd25754)
#[derive(Clone, Default)]
pub struct Physics2DDebugLine {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub feature: Physics2DDebugFeature,
    pub body_a: f64,
    pub body_b: f64,
    pub x0: f64,
    pub y0: f64,
    pub x1: f64,
    pub y1: f64,
}
impl PartialEq for Physics2DDebugLine {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Physics2D.ts:743 (sha256:ed38b0c33bd7502b53e2a4922de766310a53c290f77ec3676600a8ee9c56d937)
#[derive(Clone, Default)]
pub struct Physics2DDebugCircle {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub feature: Physics2DDebugFeature,
    pub body_a: f64,
    pub body_b: f64,
    pub x: f64,
    pub y: f64,
    pub radius: f64,
}
impl PartialEq for Physics2DDebugCircle {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Physics2D.ts:754 (sha256:1f8b276b48280ac169c1a2fd693088116385bea498fd8d80746091ed5a42729a)
#[derive(Clone, Default)]
pub struct Physics2DDebugGeometry {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub lines: Vec<Physics2DDebugLine>,
    pub line_count: f64,
    pub circles: Vec<Physics2DDebugCircle>,
    pub circle_count: f64,
}
impl PartialEq for Physics2DDebugGeometry {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/types/src/Physics2D.ts:761 (sha256:47def074a0904f9f25514d36c9de48c415a0d0363de3612860855ad5f0f9f073)
#[derive(Clone, Default)]
pub struct Physics2DDebugGeometryOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub draw_centers_of_mass: bool,
    pub draw_colliders: bool,
    pub draw_contacts: bool,
    pub draw_joints: bool,
    pub center_of_mass_radius: f64,
    pub contact_normal_length: f64,
    pub point_radius: f64,
}
impl PartialEq for Physics2DDebugGeometryOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
