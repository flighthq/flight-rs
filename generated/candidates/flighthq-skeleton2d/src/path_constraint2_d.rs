// @generated from upstream/packages/skeleton2d/src/pathConstraint2D.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{
    compute_skeleton2_d_bone_world_transform, deform_skeleton2_d_path_attachment,
    register_skeleton2_d_constraint_solver,
};
use flighthq_math::RAD_TO_DEG as rad_to_deg_constant;
use flighthq_path::{get_path_length, get_path_position_at_distance};
use flighthq_types::{
    PATH_ATTACHMENT2_D_KIND as path_attachment2_d_kind_constant, Path, PathAttachment2D,
    SKELETON2_D_CONSTRAINT_KIND as skeleton2_d_constraint_kind_constant,
    SKELETON2_D_PATH_POSITION_MODE as skeleton2_d_path_position_mode_constant,
    SKELETON2_D_PATH_ROTATE_MODE as skeleton2_d_path_rotate_mode_constant,
    SKELETON2_D_PATH_SPACING_MODE as skeleton2_d_path_spacing_mode_constant, Skeleton2D,
    Skeleton2DConstraint, Skeleton2DPathConstraint, Vector2,
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

// Source: upstream/packages/skeleton2d/src/pathConstraint2D.ts:23 (sha256:43430cb9fcc08b2602c28ac39599e5c4ef0b3c11cc5d023d40daacc2bdcf1160)
const MATRIX_STRIDE: f64 = 6.0_f64;

// Source: upstream/packages/skeleton2d/src/pathConstraint2D.ts:31 (sha256:ccdc19e1879d1700a05ebcd580c9537d76722ec040f052281e9c90a90c79291e)
pub fn register_skeleton2_d_path_constraint_solver() -> () {
    register_skeleton2_d_constraint_solver(
        (skeleton2_d_constraint_kind_constant.path).clone(),
        std::sync::Arc::new(std::sync::Mutex::new(Box::new(
            move |mut __flight_argument_0: Skeleton2D,
                  __flight_argument_1: Skeleton2DConstraint|
                  -> () {
                solve_skeleton2_d_path_constraint(&mut __flight_argument_0, &__flight_argument_1)
            },
        )
            as Box<dyn FnMut(Skeleton2D, Skeleton2DConstraint) -> () + Send + 'static>)),
    );
}

// Source: upstream/packages/skeleton2d/src/pathConstraint2D.ts:44 (sha256:beb575457025c1a0014981181d0841e6f852f81d6b61b74daf1c452b7e8543c8)
pub fn solve_skeleton2_d_path_constraint(
    skeleton: &mut Skeleton2D,
    constraint: &Skeleton2DConstraint,
) -> () {
    let path_constraint = {
        let __flight_source = &((*constraint).clone());
        Skeleton2DPathConstraint {
            __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
            kind: (__flight_source.kind).clone(),
            mix: __flight_source.mix,
            bone_indices: (__flight_source.bone_indices).clone(),
            mix_rotate: __flight_source.mix_rotate,
            mix_x: __flight_source.mix_x,
            mix_y: __flight_source.mix_y,
            position: __flight_source.position,
            position_mode: (__flight_source.position_mode).clone(),
            rotate_mode: (__flight_source.rotate_mode).clone(),
            spacing: __flight_source.spacing,
            spacing_mode: (__flight_source.spacing_mode).clone(),
            target_slot_index: __flight_source.target_slot_index,
            ..Default::default()
        }
    };
    let attachment =
        resolve_skeleton2_d_path_attachment(skeleton, path_constraint.target_slot_index);
    if (attachment).is_none() {
        return;
    }
    let slot = skeleton.slots.as_mut().unwrap()[path_constraint.target_slot_index as usize].clone();
    deform_skeleton2_d_path_attachment(
        &mut (*_PATH.lock().unwrap()),
        &attachment.as_ref().unwrap(),
        skeleton,
        slot.bone_index,
        None,
    );
    let total = get_path_length(&(*_PATH.lock().unwrap()), None);
    if (!(total > 0.0_f64)) {
        return;
    }
    let mix = path_constraint.mix;
    let rotate_mix = (path_constraint.mix_rotate * mix);
    let translate_x_mix = (path_constraint.mix_x * mix);
    let translate_y_mix = (path_constraint.mix_y * mix);
    let mut distance = if ((path_constraint.position_mode).clone()
        == skeleton2_d_path_position_mode_constant.percent)
    {
        (path_constraint.position * total)
    } else {
        path_constraint.position
    };
    let count = (path_constraint.bone_indices.len() as f64);
    if (count == 0.0_f64) {
        return;
    }
    ensure_skeleton2_d_path_scratch(count);
    {
        let mut i = 0.0_f64;
        while (i < count) {
            let bone_index = path_constraint.bone_indices[i as usize].clone();
            get_path_position_at_distance(
                &(*_PATH.lock().unwrap()),
                clamp_skeleton2_d_path_distance(distance, total),
                &mut (*_POINT.lock().unwrap()),
                &mut (*_TANGENT.lock().unwrap()),
                None,
            );
            (*_POSITIONS.lock().unwrap())[(i * 2.0_f64) as usize] = (*_POINT.lock().unwrap()).x;
            (*_POSITIONS.lock().unwrap())[((i * 2.0_f64) + 1.0_f64) as usize] =
                (*_POINT.lock().unwrap()).y;
            (*_TANGENTS.lock().unwrap())[(i * 2.0_f64) as usize] = (*_TANGENT.lock().unwrap()).x;
            (*_TANGENTS.lock().unwrap())[((i * 2.0_f64) + 1.0_f64) as usize] =
                (*_TANGENT.lock().unwrap()).y;
            distance += resolve_skeleton2_d_path_spacing(
                &path_constraint,
                if (bone_index >= 0.0_f64) && (bone_index < (skeleton.bones.len() as f64)) {
                    skeleton.bones[bone_index as usize].length
                } else {
                    0.0_f64
                },
                total,
            );
            {
                i += 1.0;
                i
            };
        }
    }
    {
        let mut i = 0.0_f64;
        while (i < count) {
            let bone_index = path_constraint.bone_indices[i as usize].clone();
            if (bone_index < 0.0_f64) || (bone_index >= (skeleton.bones.len() as f64)) {
                {
                    i += 1.0;
                    i
                };
                continue;
            }
            let mut bone = skeleton.bones[bone_index as usize].clone();
            let o = (bone_index * MATRIX_STRIDE);
            if (translate_x_mix != 0.0_f64) || (translate_y_mix != 0.0_f64) {
                let wanted_x = (world(skeleton, (o + 4.0_f64))
                    + ((((*_POSITIONS.lock().unwrap())[(i * 2.0_f64) as usize] as f64)
                        - world(skeleton, (o + 4.0_f64)))
                        * translate_x_mix));
                let wanted_y = (world(skeleton, (o + 5.0_f64))
                    + ((((*_POSITIONS.lock().unwrap())[((i * 2.0_f64) + 1.0_f64) as usize]
                        as f64)
                        - world(skeleton, (o + 5.0_f64)))
                        * translate_y_mix));
                let local = to_skeleton2_d_parent_space(skeleton, bone_index, wanted_x, wanted_y);
                if (local).is_some() {
                    bone.x = local.as_ref().unwrap().x;
                    bone.y = local.as_ref().unwrap().y;
                }
                compute_skeleton2_d_bone_world_transform(skeleton, bone_index);
            }
            if (rotate_mix != 0.0_f64) {
                let mut dir_x: f64;
                let mut dir_y: f64;
                if ((path_constraint.rotate_mode).clone()
                    == skeleton2_d_path_rotate_mode_constant.chain)
                    && ((i + 1.0_f64) < count)
                {
                    dir_x = (((*_POSITIONS.lock().unwrap())[((i + 1.0_f64) * 2.0_f64) as usize]
                        as f64)
                        - ((*_POSITIONS.lock().unwrap())[(i * 2.0_f64) as usize] as f64));
                    dir_y = (((*_POSITIONS.lock().unwrap())
                        [(((i + 1.0_f64) * 2.0_f64) + 1.0_f64) as usize]
                        as f64)
                        - ((*_POSITIONS.lock().unwrap())[((i * 2.0_f64) + 1.0_f64) as usize]
                            as f64));
                    if (dir_x == 0.0_f64) && (dir_y == 0.0_f64) {
                        dir_x = ((*_TANGENTS.lock().unwrap())[(i * 2.0_f64) as usize] as f64);
                        dir_y = ((*_TANGENTS.lock().unwrap())[((i * 2.0_f64) + 1.0_f64) as usize]
                            as f64);
                    }
                } else {
                    dir_x = ((*_TANGENTS.lock().unwrap())[(i * 2.0_f64) as usize] as f64);
                    dir_y =
                        ((*_TANGENTS.lock().unwrap())[((i * 2.0_f64) + 1.0_f64) as usize] as f64);
                }
                let current = ((world(skeleton, (o + 1.0_f64))).atan2(world(skeleton, o))
                    * rad_to_deg_constant);
                let wanted = ((dir_y).atan2(dir_x) * rad_to_deg_constant);
                bone.rotation += (wrap_skeleton2_d_angle((wanted - current)) * rotate_mix);
                compute_skeleton2_d_bone_world_transform(skeleton, bone_index);
            }
            {
                i += 1.0;
                i
            };
        }
    }
}

// Source: upstream/packages/skeleton2d/src/pathConstraint2D.ts:129 (sha256:6289dca71823a593e2943c360f03ffeb0f7ca989984c9ceecf910fdc328f8727)
fn ensure_skeleton2_d_path_scratch(count: f64) -> () {
    if (((*_POSITIONS.lock().unwrap()).len() as f64) >= (count * 2.0_f64)) {
        return;
    }
    (*_POSITIONS.lock().unwrap()) = vec![0.0_f64; (count * 2.0_f64) as usize];
    (*_TANGENTS.lock().unwrap()) = vec![0.0_f64; (count * 2.0_f64) as usize];
}

// Source: upstream/packages/skeleton2d/src/pathConstraint2D.ts:137 (sha256:9749551501992693cfc43bd66efbc6c51eb48e73fe8d8e37cc050c3962b9293f)
fn clamp_skeleton2_d_path_distance(distance: f64, total: f64) -> f64 {
    return if (distance < 0.0_f64) {
        0.0_f64
    } else {
        if (distance > total) { total } else { distance }
    };
}

// Source: upstream/packages/skeleton2d/src/pathConstraint2D.ts:143 (sha256:2fdf96be6831ac7f143daef7d3b00838c37d2760c96db7d01cdef253778cea2e)
fn resolve_skeleton2_d_path_attachment(
    skeleton: &Skeleton2D,
    slot_index: f64,
) -> Option<PathAttachment2D> {
    let slots = (skeleton.slots).clone();
    if ((slots).is_none()) || ((slots).is_none()) {
        return None;
    }
    if (slot_index < 0.0_f64) || (slot_index >= (slots.as_ref().unwrap().len() as f64)) {
        return None;
    }
    let attachment = (slots.as_ref().unwrap()[slot_index as usize].attachment).clone();
    if ((attachment).is_none()) || ((attachment).is_none()) {
        return None;
    }
    return if ((attachment.as_ref().unwrap().kind).clone() == path_attachment2_d_kind_constant) {
        Some({
            let __flight_source = &((attachment.as_ref().unwrap()).clone());
            PathAttachment2D {
                __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
                kind: (__flight_source.kind).clone(),
                name: (__flight_source.name).clone(),
                commands: (__flight_source.commands).clone(),
                point_count: __flight_source.point_count,
                skin: (__flight_source.skin).clone(),
                vertices: (__flight_source.vertices).clone(),
                winding: (__flight_source.winding).clone(),
                ..Default::default()
            }
        })
    } else {
        None
    };
}

// Source: upstream/packages/skeleton2d/src/pathConstraint2D.ts:155 (sha256:7570db402d478ec47aa409ced5073313fa03a3b437f855b8227746b869ed9a50)
fn resolve_skeleton2_d_path_spacing(
    constraint: &Skeleton2DPathConstraint,
    bone_length: f64,
    total: f64,
) -> f64 {
    if ((constraint.spacing_mode).clone() == skeleton2_d_path_spacing_mode_constant.percent) {
        return (constraint.spacing * total);
    }
    if ((constraint.spacing_mode).clone() == skeleton2_d_path_spacing_mode_constant.length) {
        return (constraint.spacing * bone_length);
    }
    return constraint.spacing;
}

// Source: upstream/packages/skeleton2d/src/pathConstraint2D.ts:166 (sha256:d8bf5cf1c4ca69527b62d560a05e88a5de6c19f89d78e5e8cc946be4aa189a64)
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

// Source: upstream/packages/skeleton2d/src/pathConstraint2D.ts:187 (sha256:ebdd914f5aa509053be86ab2d4a220d95de2b17d6406fffe82903827422c2557)
fn world(skeleton: &Skeleton2D, offset: f64) -> f64 {
    return (skeleton.world_matrices[offset as usize] as f64);
}

// Source: upstream/packages/skeleton2d/src/pathConstraint2D.ts:192 (sha256:7745159eaceb4afe522e64ce50651041bdc34f5c688bd4265ac5c0c6f4010671)
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

// Source: upstream/packages/skeleton2d/src/pathConstraint2D.ts:199 (sha256:cb81718cab5d0e0c2ff400954746ad3c032b69555e269462c7796bd4fd0b8e3a)
const MINIMUM_DETERMINANT: f64 = 1e-9_f64;

// Source: upstream/packages/skeleton2d/src/pathConstraint2D.ts:200 (sha256:13f78858f3334d8dab0e13ccc644bf6bfa87e84c2ba0950246e163c218ec5a9f)
static _PATH: std::sync::LazyLock<std::sync::Mutex<Path>> = std::sync::LazyLock::new(|| {
    std::sync::Mutex::new(Path {
        __flight_identity: std::sync::Arc::new(()),
        commands: vec![],
        data: vec![],
        winding: "nonZero".to_owned(),
    })
});

// Source: upstream/packages/skeleton2d/src/pathConstraint2D.ts:201 (sha256:d888f887b52109f9157aa2e0dc441fe8ec729d47110d17cddb3c94d3708a06bc)
static _POINT: std::sync::LazyLock<std::sync::Mutex<Vector2>> = std::sync::LazyLock::new(|| {
    std::sync::Mutex::new(Vector2 {
        __flight_identity: std::sync::Arc::new(()),
        __flight_entity_snapshot: Default::default(),
        __flight_entity_runtime: Default::default(),
        x: 0.0_f64,
        y: 0.0_f64,
    })
});

// Source: upstream/packages/skeleton2d/src/pathConstraint2D.ts:202 (sha256:2f814a40753ee23b185d41661ebf037f0d4d7ff02e488dba3f3f18d30b00da74)
static _TANGENT: std::sync::LazyLock<std::sync::Mutex<Vector2>> = std::sync::LazyLock::new(|| {
    std::sync::Mutex::new(Vector2 {
        __flight_identity: std::sync::Arc::new(()),
        __flight_entity_snapshot: Default::default(),
        __flight_entity_runtime: Default::default(),
        x: 0.0_f64,
        y: 0.0_f64,
    })
});

// Source: upstream/packages/skeleton2d/src/pathConstraint2D.ts:203 (sha256:f23c3358552dca12cf0328e6df2290182672792e3050006b60c1203f2daf5f95)
static _POSITIONS: std::sync::LazyLock<std::sync::Mutex<Vec<f64>>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(vec![0.0_f64; (0.0_f64) as usize]));

// Source: upstream/packages/skeleton2d/src/pathConstraint2D.ts:204 (sha256:0ce60a7129b94fd75b3b5f63f143419375ae47c0d2200655ce6f45f133b08f5e)
static _TANGENTS: std::sync::LazyLock<std::sync::Mutex<Vec<f64>>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(vec![0.0_f64; (0.0_f64) as usize]));
