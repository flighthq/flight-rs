// @generated from upstream/packages/skeleton2d/src/ikConstraint2D.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{compute_skeleton2_d_bone_world_transform, register_skeleton2_d_constraint_solver};
use flighthq_math::RAD_TO_DEG as rad_to_deg_constant;
use flighthq_types::{
    SKELETON2_D_CONSTRAINT_KIND as skeleton2_d_constraint_kind_constant, Skeleton2D,
    Skeleton2DConstraint, Skeleton2DIkConstraint,
};

#[derive(Clone, Default)]
pub struct SharedStructuralRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub x: f64,
    pub y: f64,
}
impl PartialEq for SharedStructuralRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/skeleton2d/src/ikConstraint2D.ts:9 (sha256:43430cb9fcc08b2602c28ac39599e5c4ef0b3c11cc5d023d40daacc2bdcf1160)
const MATRIX_STRIDE: f64 = 6.0_f64;

// Source: upstream/packages/skeleton2d/src/ikConstraint2D.ts:13 (sha256:d6a5dbdfb9f4c6bcdc1ba9f4aee369f164db9b980ed39d36dccaf2b2f89b0ec0)
pub fn register_skeleton2_d_ik_constraint_solver() -> () {
    register_skeleton2_d_constraint_solver(
        (skeleton2_d_constraint_kind_constant.ik).clone(),
        std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |mut __flight_argument_0: Skeleton2D,
                  __flight_argument_1: Skeleton2DConstraint|
                  -> () {
                solve_skeleton2_d_ik_constraint(&mut __flight_argument_0, &__flight_argument_1)
            },
        )
            as Box<dyn FnMut(Skeleton2D, Skeleton2DConstraint) -> () + Send + 'static>)),
    );
}

// Source: upstream/packages/skeleton2d/src/ikConstraint2D.ts:23 (sha256:fd6cbbecf70b4bb1af7273c6a1e7f32a8bfbf3a7bdb3b32120ba63fe5b92128b)
pub fn solve_skeleton2_d_ik_constraint(
    skeleton: &mut Skeleton2D,
    constraint: &Skeleton2DConstraint,
) -> () {
    let ik = {
        let __flight_source = &((*constraint).clone());
        Skeleton2DIkConstraint {
            __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
            kind: (__flight_source.kind).clone(),
            mix: __flight_source.mix,
            bend_positive: __flight_source.bend_positive,
            bone_indices: (__flight_source.bone_indices).clone(),
            compress: __flight_source.compress,
            stretch: __flight_source.stretch,
            target_bone_index: __flight_source.target_bone_index,
            ..Default::default()
        }
    };
    let target = ik.target_bone_index;
    if (target < 0.0_f64) || (target >= (skeleton.bones.len() as f64)) {
        return;
    }
    let target_x = (skeleton.world_matrices[((target * MATRIX_STRIDE) + 4.0_f64) as usize] as f64);
    let target_y = (skeleton.world_matrices[((target * MATRIX_STRIDE) + 5.0_f64) as usize] as f64);
    if ((ik.bone_indices.len() as f64) == 1.0_f64) {
        solve_skeleton2_d_ik_chain1(
            skeleton,
            ik.bone_indices[0.0_f64 as usize].clone(),
            (target_x).clone(),
            (target_y).clone(),
            &ik,
        );
        return;
    }
    if ((ik.bone_indices.len() as f64) == 2.0_f64) {
        solve_skeleton2_d_ik_chain2(
            skeleton,
            ik.bone_indices[0.0_f64 as usize].clone(),
            ik.bone_indices[1.0_f64 as usize].clone(),
            (target_x).clone(),
            (target_y).clone(),
            &ik,
        );
    }
}

// Source: upstream/packages/skeleton2d/src/ikConstraint2D.ts:43 (sha256:3fc6d4c166e5f57ef74906be9eaed4f23a17389c7eede29e37704f6341da32dc)
fn solve_skeleton2_d_ik_chain1(
    skeleton: &mut Skeleton2D,
    bone_index: f64,
    target_x: f64,
    target_y: f64,
    ik: &Skeleton2DIkConstraint,
) -> () {
    if (bone_index < 0.0_f64) || (bone_index >= (skeleton.bones.len() as f64)) {
        return;
    }
    let mut bone = skeleton.bones[bone_index as usize].clone();
    let local = to_skeleton2_d_parent_space(skeleton, bone_index, target_x, target_y);
    if (local).is_none() {
        return;
    }
    let dx = (local.as_ref().unwrap().x - bone.x);
    let dy = (local.as_ref().unwrap().y - bone.y);
    let rotation = (((dy).atan2(dx) * rad_to_deg_constant) - bone.shear_x);
    bone.rotation += (wrap_skeleton2_d_angle((rotation - bone.rotation)) * ik.mix);
    if (ik.stretch) || (ik.compress) {
        let reach = ((dx).powi(2) + (dy).powi(2)).sqrt();
        let length = (bone.length * (bone.scale_x).abs());
        if (length > 0.0_f64) {
            let wanted = (reach / length);
            if ((wanted > 1.0_f64) && (ik.stretch)) || ((wanted < 1.0_f64) && (ik.compress)) {
                let scale = (1.0_f64 + ((wanted - 1.0_f64) * ik.mix));
                bone.scale_x *= scale;
                bone.scale_y *= scale;
            }
        }
    }
    compute_skeleton2_d_bone_world_transform(skeleton, bone_index);
}

// Source: upstream/packages/skeleton2d/src/ikConstraint2D.ts:84 (sha256:47cbe85105e8d55f9893c66093bfeb74a3b9af7df2d0e47771107b312e452662)
fn solve_skeleton2_d_ik_chain2(
    skeleton: &mut Skeleton2D,
    parent_index: f64,
    child_index: f64,
    target_x: f64,
    target_y: f64,
    ik: &Skeleton2DIkConstraint,
) -> () {
    if (((parent_index < 0.0_f64) || (parent_index >= (skeleton.bones.len() as f64)))
        || (child_index < 0.0_f64))
        || (child_index >= (skeleton.bones.len() as f64))
    {
        return;
    }
    let mut parent = skeleton.bones[parent_index as usize].clone();
    let mut child = skeleton.bones[child_index as usize].clone();
    let parent_length = (parent.length * (parent.scale_x).abs());
    let child_length = (child.length * (child.scale_x).abs());
    if (parent_length <= 0.0_f64) || (child_length <= 0.0_f64) {
        return;
    }
    let local = to_skeleton2_d_parent_space(skeleton, parent_index, target_x, target_y);
    if (local).is_none() {
        return;
    }
    let dx = (local.as_ref().unwrap().x - parent.x);
    let dy = (local.as_ref().unwrap().y - parent.y);
    let reach = ((dx).powi(2) + (dy).powi(2)).sqrt();
    if (reach <= 0.0_f64) {
        return;
    }
    let span = (parent_length + child_length);
    let mut bend_angle: f64;
    if (reach >= span) {
        bend_angle = 0.0_f64;
        if ik.stretch {
            let scale = (1.0_f64 + (((reach / span) - 1.0_f64) * ik.mix));
            parent.scale_x *= scale;
            parent.scale_y *= scale;
        }
    } else {
        let cos_joint = ((((parent_length * parent_length) + (child_length * child_length))
            - (reach * reach))
            / ((2.0_f64 * parent_length) * child_length));
        bend_angle = (std::f64::consts::PI - ((1.0_f64).min((-1.0_f64).max(cos_joint))).acos());
    }
    let cos_parent = ((((parent_length * parent_length) + (reach * reach))
        - (child_length * child_length))
        / ((2.0_f64 * parent_length) * reach));
    let parent_offset = if (reach >= span) {
        0.0_f64
    } else {
        ((1.0_f64).min((-1.0_f64).max(cos_parent))).acos()
    };
    let direction = if ik.bend_positive {
        1.0_f64
    } else {
        (-1.0_f64)
    };
    let aim = (dy).atan2(dx);
    let parent_rotation =
        (((aim + (parent_offset * direction)) * rad_to_deg_constant) - parent.shear_x);
    parent.rotation += (wrap_skeleton2_d_angle((parent_rotation - parent.rotation)) * ik.mix);
    compute_skeleton2_d_bone_world_transform(skeleton, parent_index);
    let child_rotation = ((((-bend_angle) * direction) * rad_to_deg_constant) - child.shear_x);
    child.rotation += (wrap_skeleton2_d_angle((child_rotation - child.rotation)) * ik.mix);
    compute_skeleton2_d_bone_world_transform(skeleton, child_index);
}

// Source: upstream/packages/skeleton2d/src/ikConstraint2D.ts:157 (sha256:a69acac7ceef3946d2617513dab04b3275371c36e3b7f80aff34a9a8efee6a0c)
fn to_skeleton2_d_parent_space(
    skeleton: &Skeleton2D,
    bone_index: f64,
    x: f64,
    y: f64,
) -> Option<SharedStructuralRecord1> {
    let parent_index = skeleton.bones[bone_index as usize].parent_index;
    if (parent_index < 0.0_f64) {
        return Some(SharedStructuralRecord1 {
            __flight_identity: std::sync::Arc::new(()),
            x: x,
            y: y,
        });
    }
    let p = (parent_index * MATRIX_STRIDE);
    let a = (skeleton.world_matrices[p as usize] as f64);
    let b = (skeleton.world_matrices[(p + 1.0_f64) as usize] as f64);
    let c = (skeleton.world_matrices[(p + 2.0_f64) as usize] as f64);
    let d = (skeleton.world_matrices[(p + 3.0_f64) as usize] as f64);
    let determinant = ((a * d) - (c * b));
    if ((determinant).abs() < MINIMUM_DETERMINANT) {
        return None;
    }
    let wx = (x - (skeleton.world_matrices[(p + 4.0_f64) as usize] as f64));
    let wy = (y - (skeleton.world_matrices[(p + 5.0_f64) as usize] as f64));
    return Some(SharedStructuralRecord1 {
        __flight_identity: std::sync::Arc::new(()),
        x: (((d * wx) - (c * wy)) / determinant),
        y: (((a * wy) - (b * wx)) / determinant),
    });
}

// Source: upstream/packages/skeleton2d/src/ikConstraint2D.ts:181 (sha256:7745159eaceb4afe522e64ce50651041bdc34f5c688bd4265ac5c0c6f4010671)
fn wrap_skeleton2_d_angle(degrees: f64) -> f64 {
    let mut value = (degrees % 360.0_f64);
    if (value > 180.0_f64) {
        value -= 360.0_f64;
    } else {
        if (value < (-180.0_f64)) {
            value += 360.0_f64;
        }
    }
    return value;
}

// Source: upstream/packages/skeleton2d/src/ikConstraint2D.ts:188 (sha256:cb81718cab5d0e0c2ff400954746ad3c032b69555e269462c7796bd4fd0b8e3a)
const MINIMUM_DETERMINANT: f64 = 1e-9_f64;
