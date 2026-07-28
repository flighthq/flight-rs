// @generated from upstream/packages/scene/src/billboardCamera.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{get_scene_node_runtime, is_billboard};
use flighthq_geometry::{
    copy_matrix4, create_matrix4, create_quaternion, create_vector3, decompose_matrix4,
    inverse_matrix4, multiply_matrix4,
};
use flighthq_node::{get_node_parent, get_node_world_matrix4, set_node_local_matrix4};
use flighthq_types::{Billboard, BillboardMode, Camera, Matrix4, Quaternion, SceneNode, Vector3};

// Source: upstream/packages/scene/src/billboardCamera.ts:25 (sha256:825e965464f88bc9d6945819ac3e5ea39a654c76d57f7d79896c3b5ffb938646)
pub fn orient_billboard_to_camera(billboard: &Billboard, camera: &Camera) -> () {
    set_billboard_camera_basis(camera);
    apply_billboard_facing(billboard);
}

// Source: upstream/packages/scene/src/billboardCamera.ts:35 (sha256:aee73956717f29f0097a8f3fedc009e910b9b7a8a946b23805dd3ea8a6a78e71)
pub fn orient_scene_billboards_to_camera(scene: &SceneNode, camera: &Camera) -> () {
    set_billboard_camera_basis(camera);
    orient_billboard_subtree(scene);
}

// Source: upstream/packages/scene/src/billboardCamera.ts:44 (sha256:e0e1cf56a6834188df69070d1f7e3b9d97534d2595507dd46b68ec67e3c2a189)
fn apply_billboard_facing(billboard: &Billboard) -> () {
    let world = get_node_world_matrix4(billboard);
    decompose_matrix4(
        &mut (*_POSITION.lock().unwrap()),
        &mut (*_ROTATION_SCRATCH.lock().unwrap()),
        &mut (*_SCALE.lock().unwrap()),
        &world,
    );
    write_billboard_facing_matrix(
        &mut (*_FACING_WORLD.lock().unwrap()),
        (billboard.mode).clone(),
    );
    let parent = get_node_parent(&billboard);
    if (parent).is_none() {
        copy_matrix4(
            &mut (*_LOCAL_SCRATCH.lock().unwrap()),
            &(*_FACING_WORLD.lock().unwrap()),
        );
    } else {
        let parent_world = get_node_world_matrix4(&parent.as_ref().unwrap());
        if inverse_matrix4(&mut (*_INVERSE_PARENT_WORLD.lock().unwrap()), &parent_world) {
            multiply_matrix4(
                &mut (*_LOCAL_SCRATCH.lock().unwrap()),
                &(*_INVERSE_PARENT_WORLD.lock().unwrap()),
                &(*_FACING_WORLD.lock().unwrap()),
            );
        } else {
            copy_matrix4(
                &mut (*_LOCAL_SCRATCH.lock().unwrap()),
                &(*_FACING_WORLD.lock().unwrap()),
            );
        }
    }
    set_node_local_matrix4(billboard, &(*_LOCAL_SCRATCH.lock().unwrap()));
}

// Source: upstream/packages/scene/src/billboardCamera.ts:66 (sha256:383eda49db8c7bd0f1a8eed77902e919f4e45f95076b65f76c77918329188d0a)
fn orient_billboard_subtree(node: &SceneNode) -> () {
    if is_billboard(node) {
        apply_billboard_facing(node);
    }
    let children = (get_scene_node_runtime(node).children).clone();
    if (children).is_some() {
        {
            let mut i = 0.0_f64;
            while (i < (children.as_ref().unwrap().len() as f64)) {
                orient_billboard_subtree(&children.as_ref().unwrap()[i as usize].clone());
                {
                    i += 1.0;
                    i
                };
            }
        }
    }
}

// Source: upstream/packages/scene/src/billboardCamera.ts:83 (sha256:c833d71802f43edc7987b05711ed4787c968a2c34770502241e0161d7657fb32)
fn set_billboard_camera_basis(camera: &Camera) -> () {
    inverse_matrix4(&mut (*_CAMERA_WORLD.lock().unwrap()), &camera.view);
    (*_CAMERA_EYE_X.lock().unwrap()) =
        ((*_CAMERA_WORLD.lock().unwrap()).m[12.0_f64 as usize] as f64);
    (*_CAMERA_EYE_Y.lock().unwrap()) =
        ((*_CAMERA_WORLD.lock().unwrap()).m[13.0_f64 as usize] as f64);
    (*_CAMERA_EYE_Z.lock().unwrap()) =
        ((*_CAMERA_WORLD.lock().unwrap()).m[14.0_f64 as usize] as f64);
    (*_CAMERA_RIGHT_X.lock().unwrap()) =
        ((*_CAMERA_WORLD.lock().unwrap()).m[0.0_f64 as usize] as f64);
    (*_CAMERA_RIGHT_Y.lock().unwrap()) =
        ((*_CAMERA_WORLD.lock().unwrap()).m[1.0_f64 as usize] as f64);
    (*_CAMERA_RIGHT_Z.lock().unwrap()) =
        ((*_CAMERA_WORLD.lock().unwrap()).m[2.0_f64 as usize] as f64);
    (*_CAMERA_UP_X.lock().unwrap()) = ((*_CAMERA_WORLD.lock().unwrap()).m[4.0_f64 as usize] as f64);
    (*_CAMERA_UP_Y.lock().unwrap()) = ((*_CAMERA_WORLD.lock().unwrap()).m[5.0_f64 as usize] as f64);
    (*_CAMERA_UP_Z.lock().unwrap()) = ((*_CAMERA_WORLD.lock().unwrap()).m[6.0_f64 as usize] as f64);
    (*_CAMERA_BACK_X.lock().unwrap()) =
        ((*_CAMERA_WORLD.lock().unwrap()).m[8.0_f64 as usize] as f64);
    (*_CAMERA_BACK_Y.lock().unwrap()) =
        ((*_CAMERA_WORLD.lock().unwrap()).m[9.0_f64 as usize] as f64);
    (*_CAMERA_BACK_Z.lock().unwrap()) =
        ((*_CAMERA_WORLD.lock().unwrap()).m[10.0_f64 as usize] as f64);
    let rl = ((((*_CAMERA_RIGHT_X.lock().unwrap()).clone()).powi(2)
        + ((*_CAMERA_RIGHT_Y.lock().unwrap()).clone()).powi(2)
        + ((*_CAMERA_RIGHT_Z.lock().unwrap()).clone()).powi(2))
    .sqrt()
        || 1.0_f64);
    (*_CAMERA_RIGHT_X.lock().unwrap()) /= rl;
    (*_CAMERA_RIGHT_Y.lock().unwrap()) /= rl;
    (*_CAMERA_RIGHT_Z.lock().unwrap()) /= rl;
    let ul = ((((*_CAMERA_UP_X.lock().unwrap()).clone()).powi(2)
        + ((*_CAMERA_UP_Y.lock().unwrap()).clone()).powi(2)
        + ((*_CAMERA_UP_Z.lock().unwrap()).clone()).powi(2))
    .sqrt()
        || 1.0_f64);
    (*_CAMERA_UP_X.lock().unwrap()) /= ul;
    (*_CAMERA_UP_Y.lock().unwrap()) /= ul;
    (*_CAMERA_UP_Z.lock().unwrap()) /= ul;
    let bl = ((((*_CAMERA_BACK_X.lock().unwrap()).clone()).powi(2)
        + ((*_CAMERA_BACK_Y.lock().unwrap()).clone()).powi(2)
        + ((*_CAMERA_BACK_Z.lock().unwrap()).clone()).powi(2))
    .sqrt()
        || 1.0_f64);
    (*_CAMERA_BACK_X.lock().unwrap()) /= bl;
    (*_CAMERA_BACK_Y.lock().unwrap()) /= bl;
    (*_CAMERA_BACK_Z.lock().unwrap()) /= bl;
}

// Source: upstream/packages/scene/src/billboardCamera.ts:120 (sha256:ece5f42c9e33902e644a4e12e8ce20c3e42058f50a75fb474baf152a31032f55)
fn write_billboard_facing_matrix(out: &mut Matrix4, mode: BillboardMode) -> () {
    let px = (*_POSITION.lock().unwrap()).x;
    let py = (*_POSITION.lock().unwrap()).y;
    let pz = (*_POSITION.lock().unwrap()).z;
    let sx = (*_SCALE.lock().unwrap()).x;
    let sy = (*_SCALE.lock().unwrap()).y;
    let sz = (*_SCALE.lock().unwrap()).z;
    let mut rx: f64;
    let mut ry: f64;
    let mut rz: f64;
    let mut ux: f64;
    let mut uy: f64;
    let mut uz: f64;
    let mut nx: f64;
    let mut ny: f64;
    let mut nz: f64;
    if (mode == "screenAligned") {
        rx = (*_CAMERA_RIGHT_X.lock().unwrap()).clone();
        ry = (*_CAMERA_RIGHT_Y.lock().unwrap()).clone();
        rz = (*_CAMERA_RIGHT_Z.lock().unwrap()).clone();
        ux = (*_CAMERA_UP_X.lock().unwrap()).clone();
        uy = (*_CAMERA_UP_Y.lock().unwrap()).clone();
        uz = (*_CAMERA_UP_Z.lock().unwrap()).clone();
        nx = (*_CAMERA_BACK_X.lock().unwrap()).clone();
        ny = (*_CAMERA_BACK_Y.lock().unwrap()).clone();
        nz = (*_CAMERA_BACK_Z.lock().unwrap()).clone();
    } else {
        if (mode == "axisY") {
            let mut dx = ((*_CAMERA_EYE_X.lock().unwrap()).clone() - px);
            let mut dz = ((*_CAMERA_EYE_Z.lock().unwrap()).clone() - pz);
            let mut dl = ((dx).powi(2) + (dz).powi(2)).sqrt();
            if (dl < FACING_EPSILON) {
                dx = (*_CAMERA_BACK_X.lock().unwrap()).clone();
                dz = (*_CAMERA_BACK_Z.lock().unwrap()).clone();
                dl = ((dx).powi(2) + (dz).powi(2)).sqrt();
                if (dl < FACING_EPSILON) {
                    dx = 0.0_f64;
                    dz = 1.0_f64;
                    dl = 1.0_f64;
                }
            }
            nx = (dx / dl);
            ny = 0.0_f64;
            nz = (dz / dl);
            rx = nz;
            ry = 0.0_f64;
            rz = (-nx);
            ux = 0.0_f64;
            uy = 1.0_f64;
            uz = 0.0_f64;
        } else {
            let mut dnx = ((*_CAMERA_EYE_X.lock().unwrap()).clone() - px);
            let mut dny = ((*_CAMERA_EYE_Y.lock().unwrap()).clone() - py);
            let mut dnz = ((*_CAMERA_EYE_Z.lock().unwrap()).clone() - pz);
            let mut dnl = ((dnx).powi(2) + (dny).powi(2) + (dnz).powi(2)).sqrt();
            if (dnl < FACING_EPSILON) {
                dnx = (*_CAMERA_BACK_X.lock().unwrap()).clone();
                dny = (*_CAMERA_BACK_Y.lock().unwrap()).clone();
                dnz = (*_CAMERA_BACK_Z.lock().unwrap()).clone();
                dnl = (((dnx).powi(2) + (dny).powi(2) + (dnz).powi(2)).sqrt() || 1.0_f64);
            }
            nx = (dnx / dnl);
            ny = (dny / dnl);
            nz = (dnz / dnl);
            rx = (((*_CAMERA_UP_Y.lock().unwrap()).clone() * nz)
                - ((*_CAMERA_UP_Z.lock().unwrap()).clone() * ny));
            ry = (((*_CAMERA_UP_Z.lock().unwrap()).clone() * nx)
                - ((*_CAMERA_UP_X.lock().unwrap()).clone() * nz));
            rz = (((*_CAMERA_UP_X.lock().unwrap()).clone() * ny)
                - ((*_CAMERA_UP_Y.lock().unwrap()).clone() * nx));
            let mut rl = ((rx).powi(2) + (ry).powi(2) + (rz).powi(2)).sqrt();
            if (rl < FACING_EPSILON) {
                let d = ((((*_CAMERA_RIGHT_X.lock().unwrap()).clone() * nx)
                    + ((*_CAMERA_RIGHT_Y.lock().unwrap()).clone() * ny))
                    + ((*_CAMERA_RIGHT_Z.lock().unwrap()).clone() * nz));
                rx = ((*_CAMERA_RIGHT_X.lock().unwrap()).clone() - (d * nx));
                ry = ((*_CAMERA_RIGHT_Y.lock().unwrap()).clone() - (d * ny));
                rz = ((*_CAMERA_RIGHT_Z.lock().unwrap()).clone() - (d * nz));
                rl = ((rx).powi(2) + (ry).powi(2) + (rz).powi(2)).sqrt();
                if (rl < FACING_EPSILON) {
                    rx = 1.0_f64;
                    ry = 0.0_f64;
                    rz = 0.0_f64;
                    rl = 1.0_f64;
                }
            }
            rx /= rl;
            ry /= rl;
            rz /= rl;
            ux = ((ny * rz) - (nz * ry));
            uy = ((nz * rx) - (nx * rz));
            uz = ((nx * ry) - (ny * rx));
        }
    }
    out.m[0.0_f64 as usize] = (rx * sx) as f32;
    out.m[1.0_f64 as usize] = (ry * sx) as f32;
    out.m[2.0_f64 as usize] = (rz * sx) as f32;
    out.m[3.0_f64 as usize] = (0.0_f64) as f32;
    out.m[4.0_f64 as usize] = (ux * sy) as f32;
    out.m[5.0_f64 as usize] = (uy * sy) as f32;
    out.m[6.0_f64 as usize] = (uz * sy) as f32;
    out.m[7.0_f64 as usize] = (0.0_f64) as f32;
    out.m[8.0_f64 as usize] = (nx * sz) as f32;
    out.m[9.0_f64 as usize] = (ny * sz) as f32;
    out.m[10.0_f64 as usize] = (nz * sz) as f32;
    out.m[11.0_f64 as usize] = (0.0_f64) as f32;
    out.m[12.0_f64 as usize] = (px) as f32;
    out.m[13.0_f64 as usize] = (py) as f32;
    out.m[14.0_f64 as usize] = (pz) as f32;
    out.m[15.0_f64 as usize] = (1.0_f64) as f32;
}

// Source: upstream/packages/scene/src/billboardCamera.ts:237 (sha256:25897101007b080c84136ea8b89fac02009b5e56e1698cebe3aab1279c107b8a)
const FACING_EPSILON: f64 = 0.000001_f64;

// Source: upstream/packages/scene/src/billboardCamera.ts:239 (sha256:46d0a7bdd79f01d63e291741bcc6b82e16bd42894971d03763619bcf2c47efdb)
static _CAMERA_WORLD: std::sync::LazyLock<std::sync::Mutex<Matrix4>> =
    std::sync::LazyLock::new(|| {
        std::sync::Mutex::new(create_matrix4(
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None,
        ))
    });

// Source: upstream/packages/scene/src/billboardCamera.ts:240 (sha256:de8e779b31edcef789d3640ecec78584f38d348958d497d474e88312c3e0102f)
static _INVERSE_PARENT_WORLD: std::sync::LazyLock<std::sync::Mutex<Matrix4>> =
    std::sync::LazyLock::new(|| {
        std::sync::Mutex::new(create_matrix4(
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None,
        ))
    });

// Source: upstream/packages/scene/src/billboardCamera.ts:241 (sha256:fc9554b707a5c70dd929a47481584abc64589f7b3fc9fc2e0063fe79b62f4f6f)
static _FACING_WORLD: std::sync::LazyLock<std::sync::Mutex<Matrix4>> =
    std::sync::LazyLock::new(|| {
        std::sync::Mutex::new(create_matrix4(
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None,
        ))
    });

// Source: upstream/packages/scene/src/billboardCamera.ts:242 (sha256:43c2051ec91c25e1d3bc8d86b44777a99bc1017d4be3127ecb4c0d382ecf72cc)
static _LOCAL_SCRATCH: std::sync::LazyLock<std::sync::Mutex<Matrix4>> =
    std::sync::LazyLock::new(|| {
        std::sync::Mutex::new(create_matrix4(
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None,
        ))
    });

// Source: upstream/packages/scene/src/billboardCamera.ts:243 (sha256:463d2c82b602045f0ea386ffb52718887a318b01a8ac336c38ab097b299c5607)
static _POSITION: std::sync::LazyLock<std::sync::Mutex<Vector3>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(create_vector3(None, None, None)));

// Source: upstream/packages/scene/src/billboardCamera.ts:244 (sha256:e0e6713e8654b0d783dc9d39bb93eeb597cd417832192f439f154fd5e26fef94)
static _SCALE: std::sync::LazyLock<std::sync::Mutex<Vector3>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(create_vector3(None, None, None)));

// Source: upstream/packages/scene/src/billboardCamera.ts:245 (sha256:335712920ad5eba3e5931bd876a96961e10f398e4fcaa37c4d45402ec4935290)
static _ROTATION_SCRATCH: std::sync::LazyLock<std::sync::Mutex<Quaternion>> =
    std::sync::LazyLock::new(|| {
        std::sync::Mutex::new(create_quaternion(
            Some(0.0_f64),
            Some(0.0_f64),
            Some(0.0_f64),
            Some(1.0_f64),
        ))
    });

// Source: upstream/packages/scene/src/billboardCamera.ts:247 (sha256:6ab1895f60c09de8571817c16f10f0da07dffd70892ec71bf4f616be940b098c)
static _CAMERA_EYE_X: std::sync::LazyLock<std::sync::Mutex<f64>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(0.0_f64));

// Source: upstream/packages/scene/src/billboardCamera.ts:248 (sha256:d8ed4b9ab13aca421bbdb9c48e6eefe57e96fbda9db9d49e6649426ee1f22c56)
static _CAMERA_EYE_Y: std::sync::LazyLock<std::sync::Mutex<f64>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(0.0_f64));

// Source: upstream/packages/scene/src/billboardCamera.ts:249 (sha256:96e059a774ddc102ac60a9ce3dfc33bbd3291a5b22f5e5f1dac0262e712a074d)
static _CAMERA_EYE_Z: std::sync::LazyLock<std::sync::Mutex<f64>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(0.0_f64));

// Source: upstream/packages/scene/src/billboardCamera.ts:250 (sha256:2d6d4a68acf00dcc04741f49cddd9bbe1ca04538cca8cb1262ca8662fa806ce9)
static _CAMERA_RIGHT_X: std::sync::LazyLock<std::sync::Mutex<f64>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(1.0_f64));

// Source: upstream/packages/scene/src/billboardCamera.ts:251 (sha256:1b049c59cea390a05a022115f7877e663148108fcac1e962f310ee50a4887736)
static _CAMERA_RIGHT_Y: std::sync::LazyLock<std::sync::Mutex<f64>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(0.0_f64));

// Source: upstream/packages/scene/src/billboardCamera.ts:252 (sha256:6dddc7e1f782570ebbf7cf47e6eeb806e42c4054594bfe0a1cd78055fe788242)
static _CAMERA_RIGHT_Z: std::sync::LazyLock<std::sync::Mutex<f64>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(0.0_f64));

// Source: upstream/packages/scene/src/billboardCamera.ts:253 (sha256:5cb53125599ac33fc7477c54f2e8af9157740ed6e65829e41a35a59424942cd7)
static _CAMERA_UP_X: std::sync::LazyLock<std::sync::Mutex<f64>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(0.0_f64));

// Source: upstream/packages/scene/src/billboardCamera.ts:254 (sha256:23bc1ec279acdce4cab5ef30ae2c85df8101adbda2f313d08749634949940ec8)
static _CAMERA_UP_Y: std::sync::LazyLock<std::sync::Mutex<f64>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(1.0_f64));

// Source: upstream/packages/scene/src/billboardCamera.ts:255 (sha256:1da166d94768963c2702ae423bc6d1a060523af8965ac39cc404005bc1bb8c5c)
static _CAMERA_UP_Z: std::sync::LazyLock<std::sync::Mutex<f64>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(0.0_f64));

// Source: upstream/packages/scene/src/billboardCamera.ts:256 (sha256:1775f4b65f4e43c2fff5e43532a951354e25a99acee28c639d09821f2a063916)
static _CAMERA_BACK_X: std::sync::LazyLock<std::sync::Mutex<f64>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(0.0_f64));

// Source: upstream/packages/scene/src/billboardCamera.ts:257 (sha256:e80c0064ecb80128fb1e2137a11040510c5fcace662c5216025c0e354341b1f1)
static _CAMERA_BACK_Y: std::sync::LazyLock<std::sync::Mutex<f64>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(0.0_f64));

// Source: upstream/packages/scene/src/billboardCamera.ts:258 (sha256:c9ed060474909b9c34494202e3572390b23b82f1480736bca120783b34f945f3)
static _CAMERA_BACK_Z: std::sync::LazyLock<std::sync::Mutex<f64>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(1.0_f64));
