// @generated from upstream/packages/skeleton2d/src/transformConstraint2D.ts; do not edit.
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
    Skeleton2DConstraint, Skeleton2DTransformConstraint,
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

// Source: upstream/packages/skeleton2d/src/transformConstraint2D.ts:9 (sha256:43430cb9fcc08b2602c28ac39599e5c4ef0b3c11cc5d023d40daacc2bdcf1160)
const MATRIX_STRIDE: f64 = 6.0_f64;

// Source: upstream/packages/skeleton2d/src/transformConstraint2D.ts:13 (sha256:f04e4b59703ccc03658ca47c29af64302549010752f4f83dad2eb27ab91c188a)
pub fn register_skeleton2_d_transform_constraint_solver() -> () {
    register_skeleton2_d_constraint_solver(
        (skeleton2_d_constraint_kind_constant.transform).clone(),
        std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |mut __flight_argument_0: Skeleton2D,
                  __flight_argument_1: Skeleton2DConstraint|
                  -> () {
                solve_skeleton2_d_transform_constraint(
                    &mut __flight_argument_0,
                    &__flight_argument_1,
                )
            },
        )
            as Box<dyn FnMut(Skeleton2D, Skeleton2DConstraint) -> () + Send + 'static>)),
    );
}

// Source: upstream/packages/skeleton2d/src/transformConstraint2D.ts:26 (sha256:e36b5e8a013f47b6a54cefaee32e66aec2788bc080047f23a21d62779dfaa31e)
pub fn solve_skeleton2_d_transform_constraint(
    skeleton: &mut Skeleton2D,
    constraint: &Skeleton2DConstraint,
) -> () {
    let transform = {
        let __flight_source = &((*constraint).clone());
        Skeleton2DTransformConstraint {
            __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
            kind: (__flight_source.kind).clone(),
            mix: __flight_source.mix,
            bone_indices: (__flight_source.bone_indices).clone(),
            mix_rotate: __flight_source.mix_rotate,
            mix_scale_x: __flight_source.mix_scale_x,
            mix_scale_y: __flight_source.mix_scale_y,
            mix_shear_y: __flight_source.mix_shear_y,
            mix_x: __flight_source.mix_x,
            mix_y: __flight_source.mix_y,
            offset_rotation: __flight_source.offset_rotation,
            offset_scale_x: __flight_source.offset_scale_x,
            offset_scale_y: __flight_source.offset_scale_y,
            offset_shear_y: __flight_source.offset_shear_y,
            offset_x: __flight_source.offset_x,
            offset_y: __flight_source.offset_y,
            target_bone_index: __flight_source.target_bone_index,
            ..Default::default()
        }
    };
    let target = transform.target_bone_index;
    if (target < 0.0_f64) || (target >= (skeleton.bones.len() as f64)) {
        return;
    }
    let t = (target * MATRIX_STRIDE);
    let target_rotation = ((skeleton.world_matrices[(t + 1.0_f64) as usize] as f64)
        .atan2((skeleton.world_matrices[t as usize] as f64))
        * rad_to_deg_constant);
    let target_scale_x = ((skeleton.world_matrices[t as usize] as f64).powi(2)
        + (skeleton.world_matrices[(t + 1.0_f64) as usize] as f64).powi(2))
    .sqrt();
    let target_scale_y = ((skeleton.world_matrices[(t + 2.0_f64) as usize] as f64).powi(2)
        + (skeleton.world_matrices[(t + 3.0_f64) as usize] as f64).powi(2))
    .sqrt();
    let target_shear_y = ((((skeleton.world_matrices[(t + 3.0_f64) as usize] as f64)
        .atan2((skeleton.world_matrices[(t + 2.0_f64) as usize] as f64))
        * rad_to_deg_constant)
        - 90.0_f64)
        - target_rotation);
    let target_x = (skeleton.world_matrices[(t + 4.0_f64) as usize] as f64);
    let target_y = (skeleton.world_matrices[(t + 5.0_f64) as usize] as f64);
    let mix = transform.mix;
    for bone_index in ((transform.bone_indices).clone()).iter().cloned() {
        if (bone_index < 0.0_f64) || (bone_index >= (skeleton.bones.len() as f64)) {
            continue;
        }
        let mut bone = skeleton.bones[bone_index as usize].clone();
        let o = (bone_index * MATRIX_STRIDE);
        let rotate_mix = (transform.mix_rotate * mix);
        let scale_x_mix = (transform.mix_scale_x * mix);
        let scale_y_mix = (transform.mix_scale_y * mix);
        let shear_mix = (transform.mix_shear_y * mix);
        if (rotate_mix != 0.0_f64) {
            let current = ((skeleton.world_matrices[(o + 1.0_f64) as usize] as f64)
                .atan2((skeleton.world_matrices[o as usize] as f64))
                * rad_to_deg_constant);
            let wanted = (target_rotation + transform.offset_rotation);
            bone.rotation += (wrap_skeleton2_d_angle((wanted - current)) * rotate_mix);
        }
        if (scale_x_mix != 0.0_f64) {
            let current = ((skeleton.world_matrices[o as usize] as f64).powi(2)
                + (skeleton.world_matrices[(o + 1.0_f64) as usize] as f64).powi(2))
            .sqrt();
            if (current > 0.0_f64) {
                bone.scale_x *= (1.0_f64
                    + ((((target_scale_x + transform.offset_scale_x) / current) - 1.0_f64)
                        * scale_x_mix));
            }
        }
        if (scale_y_mix != 0.0_f64) {
            let current = ((skeleton.world_matrices[(o + 2.0_f64) as usize] as f64).powi(2)
                + (skeleton.world_matrices[(o + 3.0_f64) as usize] as f64).powi(2))
            .sqrt();
            if (current > 0.0_f64) {
                bone.scale_y *= (1.0_f64
                    + ((((target_scale_y + transform.offset_scale_y) / current) - 1.0_f64)
                        * scale_y_mix));
            }
        }
        if (shear_mix != 0.0_f64) {
            let current_rotation = ((skeleton.world_matrices[(o + 1.0_f64) as usize] as f64)
                .atan2((skeleton.world_matrices[o as usize] as f64))
                * rad_to_deg_constant);
            let current = ((((skeleton.world_matrices[(o + 3.0_f64) as usize] as f64)
                .atan2((skeleton.world_matrices[(o + 2.0_f64) as usize] as f64))
                * rad_to_deg_constant)
                - 90.0_f64)
                - current_rotation);
            bone.shear_y +=
                (wrap_skeleton2_d_angle(((target_shear_y + transform.offset_shear_y) - current))
                    * shear_mix);
        }
        let translate_x_mix = (transform.mix_x * mix);
        let translate_y_mix = (transform.mix_y * mix);
        if (translate_x_mix != 0.0_f64) || (translate_y_mix != 0.0_f64) {
            let wanted_x = ((skeleton.world_matrices[(o + 4.0_f64) as usize] as f64)
                + (((target_x + transform.offset_x)
                    - (skeleton.world_matrices[(o + 4.0_f64) as usize] as f64))
                    * translate_x_mix));
            let wanted_y = ((skeleton.world_matrices[(o + 5.0_f64) as usize] as f64)
                + (((target_y + transform.offset_y)
                    - (skeleton.world_matrices[(o + 5.0_f64) as usize] as f64))
                    * translate_y_mix));
            let local = to_skeleton2_d_parent_space(skeleton, bone_index, wanted_x, wanted_y);
            if (local).is_some() {
                bone.x = local.as_ref().unwrap().x;
                bone.y = local.as_ref().unwrap().y;
            }
        }
        compute_skeleton2_d_bone_world_transform(skeleton, bone_index);
    }
}

// Source: upstream/packages/skeleton2d/src/transformConstraint2D.ts:95 (sha256:a69acac7ceef3946d2617513dab04b3275371c36e3b7f80aff34a9a8efee6a0c)
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

// Source: upstream/packages/skeleton2d/src/transformConstraint2D.ts:118 (sha256:7745159eaceb4afe522e64ce50651041bdc34f5c688bd4265ac5c0c6f4010671)
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

// Source: upstream/packages/skeleton2d/src/transformConstraint2D.ts:125 (sha256:cb81718cab5d0e0c2ff400954746ad3c032b69555e269462c7796bd4fd0b8e3a)
const MINIMUM_DETERMINANT: f64 = 1e-9_f64;
