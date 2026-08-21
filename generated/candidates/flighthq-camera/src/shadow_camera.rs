// @generated from upstream/packages/camera/src/shadowCamera.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{create_orthographic_projection, set_camera3_d_view_matrix4_from_look_at};
use flighthq_types::{AabbLike, Camera3D, OrthographicProjectionOptions, Projection, Vector3Like};

#[inline]
fn __flight_js_to_u32(value: f64) -> u32 {
    if !value.is_finite() || value == 0.0 {
        return 0;
    }
    value.trunc().rem_euclid(4294967296.0_f64) as u32
}

#[inline]
fn __flight_js_to_i32(value: f64) -> i32 {
    __flight_js_to_u32(value) as i32
}

// Source: upstream/packages/camera/src/shadowCamera.ts:14 (sha256:da6dbbff2fda28c9b002381b137c125b351b42e4882fdf180887a5f33649eb16)
pub fn configure_directional_shadow_camera3_d(
    camera: &mut Camera3D,
    light_direction: &Vector3Like,
    scene_bounds: &AabbLike,
) -> () {
    let cx = ((scene_bounds.min.x + scene_bounds.max.x) * 0.5_f64);
    let cy = ((scene_bounds.min.y + scene_bounds.max.y) * 0.5_f64);
    let cz = ((scene_bounds.min.z + scene_bounds.max.z) * 0.5_f64);
    let mut radius = ((scene_bounds.max.x - cx).powi(2)
        + (scene_bounds.max.y - cy).powi(2)
        + (scene_bounds.max.z - cz).powi(2))
    .sqrt();
    if (radius == 0.0_f64) {
        radius = 1.0_f64;
    }
    let dl = if (((light_direction.x).powi(2)
        + (light_direction.y).powi(2)
        + (light_direction.z).powi(2))
    .sqrt())
        != 0.0_f64
    {
        ((light_direction.x).powi(2) + (light_direction.y).powi(2) + (light_direction.z).powi(2))
            .sqrt()
    } else {
        1.0_f64
    };
    let dx = (light_direction.x / dl);
    let dy = (light_direction.y / dl);
    let dz = (light_direction.z / dl);
    let distance = (radius * 2.0_f64);
    (*_EYE.lock().unwrap()).x = (cx - (dx * distance));
    (*_EYE.lock().unwrap()).y = (cy - (dy * distance));
    (*_EYE.lock().unwrap()).z = (cz - (dz * distance));
    (*_TARGET.lock().unwrap()).x = cx;
    (*_TARGET.lock().unwrap()).y = cy;
    (*_TARGET.lock().unwrap()).z = cz;
    let up = if ((dy).abs() > 0.99_f64) {
        ((*_UP_Z).clone()).clone()
    } else {
        ((*_UP_Y).clone()).clone()
    };
    set_camera3_d_view_matrix4_from_look_at(
        camera,
        &(*_EYE.lock().unwrap()),
        &(*_TARGET.lock().unwrap()),
        &up,
    );
    camera.near = radius;
    camera.far = (radius * 3.0_f64);
    camera.projection = flighthq_types::Projection::A(create_orthographic_projection(
        &OrthographicProjectionOptions {
            __flight_identity: std::sync::Arc::new(()),
            half_height: radius,
            half_width: radius,
        },
    ));
}

// Source: upstream/packages/camera/src/shadowCamera.ts:59 (sha256:aa857769692a1f8afadc207a9a8657ed1881617c11445bb7e55c7a6795eaaaae)
pub fn configure_directional_shadow_camera3_d_tight_fit(
    camera: &mut Camera3D,
    light_direction: &Vector3Like,
    world_bounds: &AabbLike,
    padding: Option<f64>,
) -> () {
    let padding = padding.unwrap_or(1.0_f64);
    let has_bounds = ((world_bounds.min.x <= world_bounds.max.x)
        && (world_bounds.min.y <= world_bounds.max.y))
        && (world_bounds.min.z <= world_bounds.max.z);
    let cx = if has_bounds {
        ((world_bounds.min.x + world_bounds.max.x) * 0.5_f64)
    } else {
        0.0_f64
    };
    let cy = if has_bounds {
        ((world_bounds.min.y + world_bounds.max.y) * 0.5_f64)
    } else {
        0.0_f64
    };
    let cz = if has_bounds {
        ((world_bounds.min.z + world_bounds.max.z) * 0.5_f64)
    } else {
        0.0_f64
    };
    let ex = if has_bounds {
        ((world_bounds.max.x - world_bounds.min.x) * 0.5_f64)
    } else {
        1.0_f64
    };
    let ey = if has_bounds {
        ((world_bounds.max.y - world_bounds.min.y) * 0.5_f64)
    } else {
        1.0_f64
    };
    let ez = if has_bounds {
        ((world_bounds.max.z - world_bounds.min.z) * 0.5_f64)
    } else {
        1.0_f64
    };
    let radius = (((ex).powi(2) + (ey).powi(2) + (ez).powi(2)).sqrt()).max(0.0001_f64);
    let extent_scale = if (padding > 0.0_f64) {
        padding
    } else {
        1.0_f64
    };
    let mut dx = light_direction.x;
    let mut dy = light_direction.y;
    let mut dz = light_direction.z;
    let direction_length = ((dx).powi(2) + (dy).powi(2) + (dz).powi(2)).sqrt();
    if (direction_length > 0.0_f64) {
        dx /= direction_length;
        dy /= direction_length;
        dz /= direction_length;
    } else {
        dx = 0.0_f64;
        dy = (-1.0_f64);
        dz = 0.0_f64;
    }
    let distance = ((radius * 2.0_f64) * extent_scale).max(1.0_f64);
    (*_EYE.lock().unwrap()).x = (cx - (dx * distance));
    (*_EYE.lock().unwrap()).y = (cy - (dy * distance));
    (*_EYE.lock().unwrap()).z = (cz - (dz * distance));
    (*_TARGET.lock().unwrap()).x = cx;
    (*_TARGET.lock().unwrap()).y = cy;
    (*_TARGET.lock().unwrap()).z = cz;
    set_camera3_d_view_matrix4_from_look_at(
        camera,
        &(*_EYE.lock().unwrap()),
        &(*_TARGET.lock().unwrap()),
        &if ((dy).abs() > 0.99_f64) {
            ((*_UP_Z).clone()).clone()
        } else {
            ((*_UP_Y).clone()).clone()
        },
    );
    let mut half_width = 0.0_f64;
    let mut half_height = 0.0_f64;
    let mut min_view_z = f64::INFINITY;
    let mut max_view_z = f64::NEG_INFINITY;
    {
        let mut corner = 0.0_f64;
        while (corner < 8.0_f64) {
            let x =
                if ((__flight_js_to_i32(corner) & __flight_js_to_i32(1.0_f64)) as f64 == 0.0_f64) {
                    (cx - ex)
                } else {
                    (cx + ex)
                };
            let y =
                if ((__flight_js_to_i32(corner) & __flight_js_to_i32(2.0_f64)) as f64 == 0.0_f64) {
                    (cy - ey)
                } else {
                    (cy + ey)
                };
            let z =
                if ((__flight_js_to_i32(corner) & __flight_js_to_i32(4.0_f64)) as f64 == 0.0_f64) {
                    (cz - ez)
                } else {
                    (cz + ez)
                };
            let view_x = (((((camera.view.m[0.0_f64 as usize] as f64) * x)
                + ((camera.view.m[4.0_f64 as usize] as f64) * y))
                + ((camera.view.m[8.0_f64 as usize] as f64) * z))
                + (camera.view.m[12.0_f64 as usize] as f64));
            let view_y = (((((camera.view.m[1.0_f64 as usize] as f64) * x)
                + ((camera.view.m[5.0_f64 as usize] as f64) * y))
                + ((camera.view.m[9.0_f64 as usize] as f64) * z))
                + (camera.view.m[13.0_f64 as usize] as f64));
            let view_z = (((((camera.view.m[2.0_f64 as usize] as f64) * x)
                + ((camera.view.m[6.0_f64 as usize] as f64) * y))
                + ((camera.view.m[10.0_f64 as usize] as f64) * z))
                + (camera.view.m[14.0_f64 as usize] as f64));
            half_width = (half_width).max((view_x).abs());
            half_height = (half_height).max((view_y).abs());
            min_view_z = (min_view_z).min(view_z);
            max_view_z = (max_view_z).max(view_z);
            {
                corner += 1.0;
                corner
            };
        }
    }
    let depth_center = ((min_view_z + max_view_z) * 0.5_f64);
    let half_depth = (((max_view_z - min_view_z) * 0.5_f64) * extent_scale).max(0.0001_f64);
    camera.near = ((-depth_center) - half_depth).max(0.0001_f64);
    camera.far = ((-depth_center) + half_depth).max((camera.near + 0.0001_f64));
    camera.projection = flighthq_types::Projection::A(create_orthographic_projection(
        &OrthographicProjectionOptions {
            __flight_identity: std::sync::Arc::new(()),
            half_height: (half_height * extent_scale).max(0.0001_f64),
            half_width: (half_width * extent_scale).max(0.0001_f64),
        },
    ));
}

// Source: upstream/packages/camera/src/shadowCamera.ts:130 (sha256:f905567836d6d1525a8a8a4c8f583b89fe0b92ac550f2ea16e2ae37619f7b39e)
static _EYE: std::sync::LazyLock<std::sync::Mutex<Vector3Like>> = std::sync::LazyLock::new(|| {
    std::sync::Mutex::new(Vector3Like {
        __flight_identity: std::sync::Arc::new(()),
        __flight_entity_snapshot: Default::default(),
        __flight_entity_runtime: Default::default(),
        x: 0.0_f64,
        y: 0.0_f64,
        z: 0.0_f64,
    })
});

// Source: upstream/packages/camera/src/shadowCamera.ts:131 (sha256:08ea600988417111f59dbabd52f237be39d8058cf35921fdea2f739efe83cc41)
static _TARGET: std::sync::LazyLock<std::sync::Mutex<Vector3Like>> =
    std::sync::LazyLock::new(|| {
        std::sync::Mutex::new(Vector3Like {
            __flight_identity: std::sync::Arc::new(()),
            __flight_entity_snapshot: Default::default(),
            __flight_entity_runtime: Default::default(),
            x: 0.0_f64,
            y: 0.0_f64,
            z: 0.0_f64,
        })
    });

// Source: upstream/packages/camera/src/shadowCamera.ts:132 (sha256:9d8d483e494babc422e1a78566746f1abcff2c7c5f464f53116565950a0f3dde)
static _UP_Y: std::sync::LazyLock<Vector3Like> = std::sync::LazyLock::new(|| Vector3Like {
    __flight_identity: std::sync::Arc::new(()),
    __flight_entity_snapshot: Default::default(),
    __flight_entity_runtime: Default::default(),
    x: 0.0_f64,
    y: 1.0_f64,
    z: 0.0_f64,
});

// Source: upstream/packages/camera/src/shadowCamera.ts:133 (sha256:2e44e7b15a62c7b63d69d3f9edfd7ffae4d503734d7c2460349e0f8b8bc65a4e)
static _UP_Z: std::sync::LazyLock<Vector3Like> = std::sync::LazyLock::new(|| Vector3Like {
    __flight_identity: std::sync::Arc::new(()),
    __flight_entity_snapshot: Default::default(),
    __flight_entity_runtime: Default::default(),
    x: 0.0_f64,
    y: 0.0_f64,
    z: 1.0_f64,
});
