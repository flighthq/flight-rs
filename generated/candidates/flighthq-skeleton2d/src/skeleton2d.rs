// @generated from upstream/packages/skeleton2d/src/skeleton2d.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_entity::create_entity;
use flighthq_geometry::{inverse_matrix, multiply_matrix};
use flighthq_math::DEG_TO_RAD as deg_to_rad_constant;
use flighthq_types::{AttachmentSkin2D, Bone2D, MatrixLike, Skeleton2D, Slot2D};

// Source: upstream/packages/skeleton2d/src/skeleton2d.ts:7 (sha256:43430cb9fcc08b2602c28ac39599e5c4ef0b3c11cc5d023d40daacc2bdcf1160)
const MATRIX_STRIDE: f64 = 6.0_f64;

// Source: upstream/packages/skeleton2d/src/skeleton2d.ts:12 (sha256:fae092d848c1b0a2f88b5e45ce58412fd9d0db24fd8c106ed23a4d4eb51c0808)
pub fn clone_skeleton2_d(skeleton: &Skeleton2D) -> Skeleton2D {
    return create_entity(Some(Skeleton2D {
        __flight_identity: std::sync::Arc::new(()),
        __flight_entity_snapshot: Default::default(),
        __flight_entity_runtime: Default::default(),
        bone_matrices: ((skeleton.bone_matrices).clone()).clone(),
        bones: ((skeleton.bones).clone())
            .iter()
            .cloned()
            .map(|bone: Bone2D| -> Bone2D { (bone).clone() })
            .collect::<Vec<_>>(),
        inverse_bind_matrices: ((skeleton.inverse_bind_matrices).clone()).clone(),
        slots: if (((skeleton.slots).clone()).is_none()) || (((skeleton.slots).clone()).is_none()) {
            (skeleton.slots).clone()
        } else {
            Some(
                (skeleton.slots.as_ref().unwrap())
                    .iter()
                    .cloned()
                    .map(|s: Slot2D| -> Slot2D { (s).clone() })
                    .collect::<Vec<_>>(),
            )
        },
        world_matrices: ((skeleton.world_matrices).clone()).clone(),
        skins: None,
    }));
}

// Source: upstream/packages/skeleton2d/src/skeleton2d.ts:27 (sha256:929c9b1862260ee43c015718015055c4274a975a90309f50fff84f9644382a6b)
pub fn compute_skeleton2_d_bone_matrices(skeleton: &mut Skeleton2D) -> () {
    let count = (skeleton.bones.len() as f64);
    {
        let mut i = 0.0_f64;
        while (i < count) {
            let o = (i * MATRIX_STRIDE);
            read_matrix(
                &mut (*_SCRATCH_A.lock().unwrap()),
                &skeleton.world_matrices,
                o,
            );
            read_matrix(
                &mut (*_SCRATCH_B.lock().unwrap()),
                &skeleton.inverse_bind_matrices,
                o,
            );
            multiply_matrix(
                &mut (*_SCRATCH_C.lock().unwrap()),
                &(*_SCRATCH_A.lock().unwrap()),
                &(*_SCRATCH_B.lock().unwrap()),
            );
            write_matrix(
                &mut skeleton.bone_matrices,
                o,
                &(*_SCRATCH_C.lock().unwrap()),
            );
            {
                i += 1.0;
                i
            };
        }
    }
}

// Source: upstream/packages/skeleton2d/src/skeleton2d.ts:46 (sha256:8474440a822bcb07f472e1f934739b8f6ed31d7bc171d1d92f0c8e6f7c420953)
pub fn compute_skeleton2_d_bone_world_transform(skeleton: &mut Skeleton2D, bone_index: f64) -> () {
    if (bone_index < 0.0_f64) || (bone_index >= (skeleton.bones.len() as f64)) {
        return;
    }
    {
        let i = bone_index;
        let bone = skeleton.bones[i as usize].clone();
        let rot_x = ((bone.rotation + bone.shear_x) * deg_to_rad_constant);
        let rot_y = (((bone.rotation + 90.0_f64) + bone.shear_y) * deg_to_rad_constant);
        let la = ((rot_x).cos() * bone.scale_x);
        let lb = ((rot_x).sin() * bone.scale_x);
        let lc = ((rot_y).cos() * bone.scale_y);
        let ld = ((rot_y).sin() * bone.scale_y);
        let o = (i * MATRIX_STRIDE);
        if (bone.parent_index < 0.0_f64) {
            skeleton.world_matrices[o as usize] = (la) as f32;
            skeleton.world_matrices[(o + 1.0_f64) as usize] = (lb) as f32;
            skeleton.world_matrices[(o + 2.0_f64) as usize] = (lc) as f32;
            skeleton.world_matrices[(o + 3.0_f64) as usize] = (ld) as f32;
            skeleton.world_matrices[(o + 4.0_f64) as usize] = (bone.x) as f32;
            skeleton.world_matrices[(o + 5.0_f64) as usize] = (bone.y) as f32;
            return;
        }
        let p = (bone.parent_index * MATRIX_STRIDE);
        let pa = (skeleton.world_matrices[p as usize] as f64);
        let pb = (skeleton.world_matrices[(p + 1.0_f64) as usize] as f64);
        let pc = (skeleton.world_matrices[(p + 2.0_f64) as usize] as f64);
        let pd = (skeleton.world_matrices[(p + 3.0_f64) as usize] as f64);
        if bone.transform_mode.translation {
            skeleton.world_matrices[(o + 4.0_f64) as usize] = (((pa * bone.x) + (pc * bone.y))
                + (skeleton.world_matrices[(p + 4.0_f64) as usize] as f64))
                as f32;
            skeleton.world_matrices[(o + 5.0_f64) as usize] = (((pb * bone.x) + (pd * bone.y))
                + (skeleton.world_matrices[(p + 5.0_f64) as usize] as f64))
                as f32;
        } else {
            skeleton.world_matrices[(o + 4.0_f64) as usize] = (bone.x) as f32;
            skeleton.world_matrices[(o + 5.0_f64) as usize] = (bone.y) as f32;
        }
        let mut ea: f64;
        let mut eb: f64;
        let mut ec: f64;
        let mut ed: f64;
        if ((bone.transform_mode.rotation) && (bone.transform_mode.scale))
            && (bone.transform_mode.reflection)
        {
            ea = pa;
            eb = pb;
            ec = pc;
            ed = pd;
        } else {
            let psx = if ((((pa).clone()).powi(2) + ((pb).clone()).powi(2)).sqrt()) != 0.0_f64 {
                (((pa).clone()).powi(2) + ((pb).clone()).powi(2)).sqrt()
            } else {
                1.0_f64
            };
            let psy = if ((((pc).clone()).powi(2) + ((pd).clone()).powi(2)).sqrt()) != 0.0_f64 {
                (((pc).clone()).powi(2) + ((pd).clone()).powi(2)).sqrt()
            } else {
                1.0_f64
            };
            let d0x = if bone.transform_mode.rotation {
                (pa / psx)
            } else {
                1.0_f64
            };
            let d0y = if bone.transform_mode.rotation {
                (pb / psx)
            } else {
                0.0_f64
            };
            let mut d1x: f64;
            let mut d1y: f64;
            if (bone.transform_mode.rotation) && (bone.transform_mode.reflection) {
                d1x = (pc / psy);
                d1y = (pd / psy);
            } else {
                if bone.transform_mode.rotation {
                    d1x = (-d0y);
                    d1y = d0x;
                } else {
                    d1x = 0.0_f64;
                    d1y = if (bone.transform_mode.reflection) && (((pa * pd) - (pb * pc)) < 0.0_f64)
                    {
                        (-1.0_f64)
                    } else {
                        1.0_f64
                    };
                }
            }
            let sx = if bone.transform_mode.scale {
                psx
            } else {
                1.0_f64
            };
            let sy = if bone.transform_mode.scale {
                psy
            } else {
                1.0_f64
            };
            ea = (d0x * sx);
            eb = (d0y * sx);
            ec = (d1x * sy);
            ed = (d1y * sy);
        }
        skeleton.world_matrices[o as usize] = ((ea * la) + (ec * lb)) as f32;
        skeleton.world_matrices[(o + 1.0_f64) as usize] = ((eb * la) + (ed * lb)) as f32;
        skeleton.world_matrices[(o + 2.0_f64) as usize] = ((ea * lc) + (ec * ld)) as f32;
        skeleton.world_matrices[(o + 3.0_f64) as usize] = ((eb * lc) + (ed * ld)) as f32;
    }
}

// Source: upstream/packages/skeleton2d/src/skeleton2d.ts:140 (sha256:f440431e737232eada1770427426b23f5f11d3889a700b9800360fe218b8c733)
pub fn compute_skeleton2_d_world_transforms(skeleton: &mut Skeleton2D) -> () {
    let count = (skeleton.bones.len() as f64);
    {
        let mut i = 0.0_f64;
        while (i < count) {
            compute_skeleton2_d_bone_world_transform(skeleton, i);
            {
                i += 1.0;
                i
            };
        }
    }
}

// Source: upstream/packages/skeleton2d/src/skeleton2d.ts:150 (sha256:88457791de120263f202b3d4c78b765e7aeb5838c11c075a61f0ee2f760efa06)
pub fn create_skeleton2_d(bones: &Vec<Bone2D>, slots: Option<Vec<Slot2D>>) -> Skeleton2D {
    let count = (bones.len() as f64);
    return create_entity(Some(Skeleton2D {
        __flight_identity: std::sync::Arc::new(()),
        __flight_entity_snapshot: Default::default(),
        __flight_entity_runtime: Default::default(),
        bone_matrices: vec![0.0_f32; (count * MATRIX_STRIDE) as usize],
        bones: (*bones).clone(),
        inverse_bind_matrices: vec![0.0_f32; (count * MATRIX_STRIDE) as usize],
        slots: (slots).clone(),
        world_matrices: vec![0.0_f32; (count * MATRIX_STRIDE) as usize],
        skins: None,
    }));
}

// Source: upstream/packages/skeleton2d/src/skeleton2d.ts:163 (sha256:d938f876f8f5f2d3c2f11e17ed623c87c8d88ad3759cd7cfd6f696f4058e8000)
pub fn dispose_skeleton2_d(skeleton: &mut Skeleton2D) -> () {
    skeleton.bones = vec![];
    skeleton.slots = None;
}

// Source: upstream/packages/skeleton2d/src/skeleton2d.ts:168 (sha256:5b329a23abd7a887cf13da89ef49997f56ce38f875c50fa36e700dcc4c02ec65)
pub fn equals_skeleton2_d(a: &Skeleton2D, b: &Skeleton2D) -> bool {
    if (a == b) {
        return true;
    }
    if ((a.bones.len() as f64) != (b.bones.len() as f64)) {
        return false;
    }
    {
        let mut i = 0.0_f64;
        while (i < (a.bones.len() as f64)) {
            let x = a.bones[i as usize].clone();
            let y = b.bones[i as usize].clone();
            if ((((((((((((x.parent_index != y.parent_index) || (x.x != y.x))
                || (x.y != y.y))
                || (x.rotation != y.rotation))
                || (x.scale_x != y.scale_x))
                || (x.scale_y != y.scale_y))
                || (x.shear_x != y.shear_x))
                || (x.shear_y != y.shear_y))
                || (x.length != y.length))
                || (x.transform_mode.rotation != y.transform_mode.rotation))
                || (x.transform_mode.scale != y.transform_mode.scale))
                || (x.transform_mode.reflection != y.transform_mode.reflection))
                || (x.transform_mode.translation != y.transform_mode.translation)
            {
                return false;
            }
            {
                i += 1.0;
                i
            };
        }
    }
    return true;
}

// Source: upstream/packages/skeleton2d/src/skeleton2d.ts:196 (sha256:2856145cbd28819f026d6f37130e264f4f85d876092aa1ee92644761b75de7d7)
pub fn get_skeleton2_d_bone_index_by_name(skeleton: &Skeleton2D, name: String) -> f64 {
    {
        let mut i = 0.0_f64;
        while (i < (skeleton.bones.len() as f64)) {
            if ((skeleton.bones[i as usize].name).clone()) == Some((name).clone()) {
                return i;
            }
            {
                i += 1.0;
                i
            };
        }
    }
    return (-1.0_f64);
}

// Source: upstream/packages/skeleton2d/src/skeleton2d.ts:206 (sha256:c7ac21ee164e242e8a16d420154bd35f10b74650f137d393e5568e20ce06770e)
pub fn get_skeleton2_d_bone_world_matrix(
    out: &mut MatrixLike,
    skeleton: &Skeleton2D,
    bone_index: f64,
) -> bool {
    if (bone_index < 0.0_f64) || (bone_index >= (skeleton.bones.len() as f64)) {
        return false;
    }
    read_matrix(out, &skeleton.world_matrices, (bone_index * MATRIX_STRIDE));
    return true;
}

// Source: upstream/packages/skeleton2d/src/skeleton2d.ts:218 (sha256:289af08e4b817f783505fbc800d8a84026e912e79107c02ec1a3313cb9cca9ee)
pub fn get_skeleton2_d_skin(skeleton: &Skeleton2D, name: String) -> Option<AttachmentSkin2D> {
    let skins = (skeleton.skins).clone();
    if ((skins).is_none()) || ((skins).is_none()) {
        return None;
    }
    for skin in (skins.as_ref().unwrap()).iter().cloned() {
        if ((skin.name).clone() == name) {
            return Some((skin).clone());
        }
    }
    return None;
}

// Source: upstream/packages/skeleton2d/src/skeleton2d.ts:228 (sha256:444616f4ade9616cda2a5839e71904ac394be556df90747f2c77551dd76bfe6b)
pub fn set_skeleton2_d_bind_pose(skeleton: &mut Skeleton2D) -> () {
    let count = (skeleton.bones.len() as f64);
    {
        let mut i = 0.0_f64;
        while (i < count) {
            let o = (i * MATRIX_STRIDE);
            read_matrix(
                &mut (*_SCRATCH_A.lock().unwrap()),
                &skeleton.world_matrices,
                o,
            );
            if (!inverse_matrix(
                &mut (*_SCRATCH_B.lock().unwrap()),
                &(*_SCRATCH_A.lock().unwrap()),
            )) {
                set_matrix_identity_local(&mut (*_SCRATCH_B.lock().unwrap()));
            }
            write_matrix(
                &mut skeleton.inverse_bind_matrices,
                o,
                &(*_SCRATCH_B.lock().unwrap()),
            );
            {
                i += 1.0;
                i
            };
        }
    }
}

// Source: upstream/packages/skeleton2d/src/skeleton2d.ts:253 (sha256:1fb1815e51080e97d296b4d47b1e3eccb2ae52e2cd600664cf4dcabbb55fb679)
pub fn set_skeleton2_d_skin(skeleton: &mut Skeleton2D, skin: &AttachmentSkin2D) -> () {
    let mut slots = (skeleton.slots).clone();
    if ((slots).is_none()) || ((slots).is_none()) {
        return;
    }
    for entry in ((skin.attachments).clone()).iter().cloned() {
        if (entry.slot_index >= 0.0_f64)
            && (entry.slot_index < (slots.as_mut().unwrap().len() as f64))
        {
            slots.as_mut().unwrap()[entry.slot_index as usize].attachment =
                Some((entry.attachment).clone());
        }
    }
}

// Source: upstream/packages/skeleton2d/src/skeleton2d.ts:263 (sha256:2dcf83b23df13fecb600cf76bbe4f7b909b8441702476697331194bb794626e9)
pub fn validate_skeleton2_d(skeleton: &Skeleton2D) -> Option<String> {
    let count = (skeleton.bones.len() as f64);
    let expected = (count * MATRIX_STRIDE);
    if ((skeleton.world_matrices.len() as f64) != expected) {
        return Some(format!(
            "worldMatrices length {} != {}",
            (skeleton.world_matrices.len() as f64),
            expected
        ));
    }
    if ((skeleton.inverse_bind_matrices.len() as f64) != expected) {
        return Some(format!(
            "inverseBindMatrices length {} != {}",
            (skeleton.inverse_bind_matrices.len() as f64),
            expected
        ));
    }
    if ((skeleton.bone_matrices.len() as f64) != expected) {
        return Some(format!(
            "boneMatrices length {} != {}",
            (skeleton.bone_matrices.len() as f64),
            expected
        ));
    }
    {
        let mut i = 0.0_f64;
        while (i < count) {
            let parent_index = skeleton.bones[i as usize].parent_index;
            if (parent_index >= i) {
                return Some(format!(
                    "bone {} parentIndex {} is not < its own index (bones must be parent-before-child ordered)",
                    i, parent_index
                ));
            }
            if (parent_index < (-1.0_f64)) {
                return Some(format!("bone {} parentIndex {} < -1", i, parent_index));
            }
            {
                i += 1.0;
                i
            };
        }
    }
    return None;
}

// Source: upstream/packages/skeleton2d/src/skeleton2d.ts:282 (sha256:3b105d57f097c286233a77d507c73c44e91632aa1e220895f95b0e063c3e86b6)
fn read_matrix(out: &mut MatrixLike, buffer: &Vec<f32>, offset: f64) -> () {
    out.a = (buffer[offset as usize] as f64);
    out.b = (buffer[(offset + 1.0_f64) as usize] as f64);
    out.c = (buffer[(offset + 2.0_f64) as usize] as f64);
    out.d = (buffer[(offset + 3.0_f64) as usize] as f64);
    out.tx = (buffer[(offset + 4.0_f64) as usize] as f64);
    out.ty = (buffer[(offset + 5.0_f64) as usize] as f64);
}

// Source: upstream/packages/skeleton2d/src/skeleton2d.ts:291 (sha256:22dbf84fe8b5864d84769ed74f51fc3e352c8c19f6686c5c8010035b7f74166e)
fn set_matrix_identity_local(out: &mut MatrixLike) -> () {
    out.a = 1.0_f64;
    out.b = 0.0_f64;
    out.c = 0.0_f64;
    out.d = 1.0_f64;
    out.tx = 0.0_f64;
    out.ty = 0.0_f64;
}

// Source: upstream/packages/skeleton2d/src/skeleton2d.ts:300 (sha256:5f1dac6713b0c341c2247ac7f4167f2ee61fd9eeff0fa46c7dbfc9eff9d82cbc)
fn write_matrix(buffer: &mut Vec<f32>, offset: f64, source: &MatrixLike) -> () {
    buffer[offset as usize] = (source.a) as f32;
    buffer[(offset + 1.0_f64) as usize] = (source.b) as f32;
    buffer[(offset + 2.0_f64) as usize] = (source.c) as f32;
    buffer[(offset + 3.0_f64) as usize] = (source.d) as f32;
    buffer[(offset + 4.0_f64) as usize] = (source.tx) as f32;
    buffer[(offset + 5.0_f64) as usize] = (source.ty) as f32;
}

// Source: upstream/packages/skeleton2d/src/skeleton2d.ts:309 (sha256:213ea175028ec93386022db867b3fcb9c18756f9088f0898d3ed280e9408ef20)
static _SCRATCH_A: std::sync::LazyLock<std::sync::Mutex<MatrixLike>> =
    std::sync::LazyLock::new(|| {
        std::sync::Mutex::new(MatrixLike {
            __flight_identity: std::sync::Arc::new(()),
            __flight_entity_snapshot: Default::default(),
            __flight_entity_runtime: Default::default(),
            a: 1.0_f64,
            b: 0.0_f64,
            c: 0.0_f64,
            d: 1.0_f64,
            tx: 0.0_f64,
            ty: 0.0_f64,
        })
    });

// Source: upstream/packages/skeleton2d/src/skeleton2d.ts:310 (sha256:181bcb4bdbb1bb6330b5dbd7ba8d094c34a7510b36bd19c70f90b6b84a78d2a2)
static _SCRATCH_B: std::sync::LazyLock<std::sync::Mutex<MatrixLike>> =
    std::sync::LazyLock::new(|| {
        std::sync::Mutex::new(MatrixLike {
            __flight_identity: std::sync::Arc::new(()),
            __flight_entity_snapshot: Default::default(),
            __flight_entity_runtime: Default::default(),
            a: 1.0_f64,
            b: 0.0_f64,
            c: 0.0_f64,
            d: 1.0_f64,
            tx: 0.0_f64,
            ty: 0.0_f64,
        })
    });

// Source: upstream/packages/skeleton2d/src/skeleton2d.ts:311 (sha256:9725e40899945504641471f62047436c31b442984d74cf264c7ea9e0552e2fd2)
static _SCRATCH_C: std::sync::LazyLock<std::sync::Mutex<MatrixLike>> =
    std::sync::LazyLock::new(|| {
        std::sync::Mutex::new(MatrixLike {
            __flight_identity: std::sync::Arc::new(()),
            __flight_entity_snapshot: Default::default(),
            __flight_entity_runtime: Default::default(),
            a: 1.0_f64,
            b: 0.0_f64,
            c: 0.0_f64,
            d: 1.0_f64,
            tx: 0.0_f64,
            ty: 0.0_f64,
        })
    });
