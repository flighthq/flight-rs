// @generated from upstream/packages/camera/src/camera.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{apply_oblique_near_clip_plane, set_projection_matrix4};
use flighthq_entity::{allocate_entity, finish_entity};
use flighthq_geometry::{
    create_matrix4, create_vector2, inverse_matrix4, multiply_matrix4, set_matrix4_look_at,
};
use flighthq_types::{
    Camera3D, Camera3DOptions, EntityConstruction, Matrix4, Matrix4Like, Projection, Vector3Like,
};

// Source: upstream/packages/camera/src/camera.ts:14 (sha256:1ddb1065752094e8f94fcdb03749251f393b7c6870a52461a1098835f8cf1e4c)
pub fn create_camera3_d(opts: &Camera3DOptions) -> Camera3D {
    let mut out = allocate_entity();
    initialize_camera3_d((out).clone(), opts);
    return finish_entity((out).clone());
}

// Source: upstream/packages/camera/src/camera.ts:27 (sha256:b391dbfbff4df4ff85b3d3a6a40b10b13c0b4479a2c2818679d99587a51ba801)
pub fn get_camera3_d_inverse_view_projection_matrix4(
    out: &mut Matrix4Like,
    camera: &Camera3D,
    aspect: f64,
) -> bool {
    {
        set_projection_matrix4(
            &mut (*__SCRATCH_PROJECTION.lock().unwrap()),
            &camera.projection,
            aspect,
            camera.near,
            camera.far,
        );
        {
            (*__SCRATCH_PROJECTION.lock().unwrap()).m[0.0_f64 as usize] += (camera.jitter.x
                * ((*__SCRATCH_PROJECTION.lock().unwrap()).m[3.0_f64 as usize] as f64))
                as f32;
            (*__SCRATCH_PROJECTION.lock().unwrap()).m[4.0_f64 as usize] += (camera.jitter.x
                * ((*__SCRATCH_PROJECTION.lock().unwrap()).m[7.0_f64 as usize] as f64))
                as f32;
            (*__SCRATCH_PROJECTION.lock().unwrap()).m[8.0_f64 as usize] += (camera.jitter.x
                * ((*__SCRATCH_PROJECTION.lock().unwrap()).m[11.0_f64 as usize] as f64))
                as f32;
            (*__SCRATCH_PROJECTION.lock().unwrap()).m[12.0_f64 as usize] += (camera.jitter.x
                * ((*__SCRATCH_PROJECTION.lock().unwrap()).m[15.0_f64 as usize] as f64))
                as f32;
            (*__SCRATCH_PROJECTION.lock().unwrap()).m[1.0_f64 as usize] += (camera.jitter.y
                * ((*__SCRATCH_PROJECTION.lock().unwrap()).m[3.0_f64 as usize] as f64))
                as f32;
            (*__SCRATCH_PROJECTION.lock().unwrap()).m[5.0_f64 as usize] += (camera.jitter.y
                * ((*__SCRATCH_PROJECTION.lock().unwrap()).m[7.0_f64 as usize] as f64))
                as f32;
            (*__SCRATCH_PROJECTION.lock().unwrap()).m[9.0_f64 as usize] += (camera.jitter.y
                * ((*__SCRATCH_PROJECTION.lock().unwrap()).m[11.0_f64 as usize] as f64))
                as f32;
            (*__SCRATCH_PROJECTION.lock().unwrap()).m[13.0_f64 as usize] += (camera.jitter.y
                * ((*__SCRATCH_PROJECTION.lock().unwrap()).m[15.0_f64 as usize] as f64))
                as f32;
        };
        if ((camera.near_clip_plane).clone()).is_some() {
            apply_oblique_near_clip_plane(
                &mut (*__SCRATCH_PROJECTION.lock().unwrap()),
                camera.near_clip_plane.as_ref().unwrap(),
            );
        }
        multiply_matrix4(
            &mut (*__SCRATCH_VIEW_PROJECTION.lock().unwrap()),
            &{
                let __flight_source = &(*__SCRATCH_PROJECTION.lock().unwrap());
                Matrix4Like {
                    __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
                    __flight_entity_runtime: std::sync::Arc::clone(
                        &__flight_source.__flight_entity_runtime,
                    ),
                    __flight_entity_snapshot: __flight_source.__flight_entity_snapshot.clone(),
                    m: (__flight_source.m).clone(),
                }
            },
            &{
                let __flight_source = &(camera.view);
                Matrix4Like {
                    __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
                    __flight_entity_runtime: std::sync::Arc::clone(
                        &__flight_source.__flight_entity_runtime,
                    ),
                    __flight_entity_snapshot: __flight_source.__flight_entity_snapshot.clone(),
                    m: (__flight_source.m).clone(),
                }
            },
        );
    };
    return inverse_matrix4(out, &{
        let __flight_source = &(*__SCRATCH_VIEW_PROJECTION.lock().unwrap());
        Matrix4Like {
            __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
            __flight_entity_runtime: std::sync::Arc::clone(
                &__flight_source.__flight_entity_runtime,
            ),
            __flight_entity_snapshot: __flight_source.__flight_entity_snapshot.clone(),
            m: (__flight_source.m).clone(),
        }
    });
}

// Source: upstream/packages/camera/src/camera.ts:42 (sha256:5646121a0e21a6a35a55f99c158507218223f70f71a6570bab1746b55c9f08cf)
pub fn get_camera3_d_view_projection_matrix4(
    out: &mut Matrix4Like,
    camera: &Camera3D,
    aspect: f64,
) -> () {
    set_projection_matrix4(
        &mut (*__SCRATCH_PROJECTION.lock().unwrap()),
        &camera.projection,
        aspect,
        camera.near,
        camera.far,
    );
    {
        (*__SCRATCH_PROJECTION.lock().unwrap()).m[0.0_f64 as usize] += (camera.jitter.x
            * ((*__SCRATCH_PROJECTION.lock().unwrap()).m[3.0_f64 as usize] as f64))
            as f32;
        (*__SCRATCH_PROJECTION.lock().unwrap()).m[4.0_f64 as usize] += (camera.jitter.x
            * ((*__SCRATCH_PROJECTION.lock().unwrap()).m[7.0_f64 as usize] as f64))
            as f32;
        (*__SCRATCH_PROJECTION.lock().unwrap()).m[8.0_f64 as usize] += (camera.jitter.x
            * ((*__SCRATCH_PROJECTION.lock().unwrap()).m[11.0_f64 as usize] as f64))
            as f32;
        (*__SCRATCH_PROJECTION.lock().unwrap()).m[12.0_f64 as usize] += (camera.jitter.x
            * ((*__SCRATCH_PROJECTION.lock().unwrap()).m[15.0_f64 as usize] as f64))
            as f32;
        (*__SCRATCH_PROJECTION.lock().unwrap()).m[1.0_f64 as usize] += (camera.jitter.y
            * ((*__SCRATCH_PROJECTION.lock().unwrap()).m[3.0_f64 as usize] as f64))
            as f32;
        (*__SCRATCH_PROJECTION.lock().unwrap()).m[5.0_f64 as usize] += (camera.jitter.y
            * ((*__SCRATCH_PROJECTION.lock().unwrap()).m[7.0_f64 as usize] as f64))
            as f32;
        (*__SCRATCH_PROJECTION.lock().unwrap()).m[9.0_f64 as usize] += (camera.jitter.y
            * ((*__SCRATCH_PROJECTION.lock().unwrap()).m[11.0_f64 as usize] as f64))
            as f32;
        (*__SCRATCH_PROJECTION.lock().unwrap()).m[13.0_f64 as usize] += (camera.jitter.y
            * ((*__SCRATCH_PROJECTION.lock().unwrap()).m[15.0_f64 as usize] as f64))
            as f32;
    };
    if ((camera.near_clip_plane).clone()).is_some() {
        apply_oblique_near_clip_plane(
            &mut (*__SCRATCH_PROJECTION.lock().unwrap()),
            camera.near_clip_plane.as_ref().unwrap(),
        );
    }
    multiply_matrix4(
        out,
        &{
            let __flight_source = &(*__SCRATCH_PROJECTION.lock().unwrap());
            Matrix4Like {
                __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
                __flight_entity_runtime: std::sync::Arc::clone(
                    &__flight_source.__flight_entity_runtime,
                ),
                __flight_entity_snapshot: __flight_source.__flight_entity_snapshot.clone(),
                m: (__flight_source.m).clone(),
            }
        },
        &{
            let __flight_source = &(camera.view);
            Matrix4Like {
                __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
                __flight_entity_runtime: std::sync::Arc::clone(
                    &__flight_source.__flight_entity_runtime,
                ),
                __flight_entity_snapshot: __flight_source.__flight_entity_snapshot.clone(),
                m: (__flight_source.m).clone(),
            }
        },
    );
}

// Source: upstream/packages/camera/src/camera.ts:55 (sha256:20ba32082bb1c054cdbad48f13bf791c283bf23d26f20e293a1cf981fddbfcb8)
pub fn initialize_camera3_d(out: EntityConstruction<Camera3D>, opts: &Camera3DOptions) -> () {
    crate::host_set("host.far", opts.far);
    crate::host_set(
        "host.inverseViewProjection",
        create_matrix4(
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None,
        ),
    );
    crate::host_set("host.jitter", create_vector2(Some(0.0_f64), Some(0.0_f64)));
    crate::host_set("host.near", opts.near);
    crate::host_set("host.nearClipPlane", (opts.near_clip_plane).clone());
    crate::host_set("host.projection", (opts.projection).clone());
    crate::host_set(
        "host.view",
        create_matrix4(
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None,
        ),
    );
}

// Source: upstream/packages/camera/src/camera.ts:71 (sha256:f3d12413ed95bea4cb8d534707ad8e561dcc13d1d36c82b0b278e5d893217732)
pub fn set_camera3_d_aspect(camera: &mut Camera3D, aspect: f64) -> () {
    if matches!(
        &(camera.projection),
        crate::FlightUnion2::B(crate::FlightUnion2::A(_))
    ) {
        (match (camera.projection).clone() {
            flighthq_types::Projection::A(_) => panic!("TypeScript union narrowing failed"),
            flighthq_types::Projection::B(value) => match value {
                crate::FlightUnion2::A(value) => value,
                crate::FlightUnion2::B(_) => panic!("TypeScript union narrowing failed"),
            },
        })
        .aspect = aspect;
        return;
    }
    if matches!(
        &(camera.projection),
        crate::FlightUnion2::B(crate::FlightUnion2::B(_))
    ) {
        return;
    }
    (match (camera.projection).clone() {
        flighthq_types::Projection::A(value) => value,
        flighthq_types::Projection::B(_) => panic!("TypeScript union narrowing failed"),
    })
    .half_width = ((match (camera.projection).clone() {
        flighthq_types::Projection::A(value) => value,
        flighthq_types::Projection::B(_) => panic!("TypeScript union narrowing failed"),
    })
    .half_height
        * aspect);
}

// Source: upstream/packages/camera/src/camera.ts:83 (sha256:bfb927850796b081447fa35ebbbb118b6fc5923dfc60f5e2f704422bd1c5534d)
pub fn set_camera3_d_jitter(camera: &mut Camera3D, x: f64, y: f64) -> () {
    camera.jitter.x = x;
    camera.jitter.y = y;
}

// Source: upstream/packages/camera/src/camera.ts:92 (sha256:2551adfde46b67dbcebbb9af6f8ea7521de1ab26f32b4d6f24782a7e68335501)
pub fn set_camera3_d_view_guard(
    guard: &Option<
        std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(Camera3D) -> () + Send + 'static>>>,
    >,
) -> () {
    (*CAMERA3_D_VIEW_GUARD.lock().unwrap()) = (*guard).clone();
}

// Source: upstream/packages/camera/src/camera.ts:101 (sha256:ad72d8902dbe544369f6f78a21fed72fbd73f8d20cb6d5b1ff37fc11a03992c8)
pub fn set_camera3_d_view_matrix4_from_look_at(
    camera: &mut Camera3D,
    eye: &Vector3Like,
    target: &Vector3Like,
    up: &Vector3Like,
) -> () {
    set_matrix4_look_at(&mut camera.view, eye, target, up);
}

// Source: upstream/packages/camera/src/camera.ts:123 (sha256:bcfe611d5acddd7545bf1e4365077b455b63d2eaba1628635686da35ef3fce38)
pub fn set_camera3_d_view_matrix4_from_matrix4(camera: &mut Camera3D, view: &Matrix4Like) -> () {
    {
        let __flight_offset = (0.0_f64) as usize;
        let __flight_values: Vec<f32> = ((view.m).clone())
            .iter()
            .map(|value| (*value) as f32)
            .collect();
        camera.view.m[__flight_offset..__flight_offset + __flight_values.len()]
            .copy_from_slice(&__flight_values);
    };
    {
        let __flight_callback = (*CAMERA3_D_VIEW_GUARD.lock().unwrap()).clone();
        __flight_callback
            .as_ref()
            .map(|callback| callback.lock().unwrap()((*camera).clone()))
    };
}

// Source: upstream/packages/camera/src/camera.ts:132 (sha256:aba0c8297b05d3db0a1586783d6cd8b5caa3ac4353d3cff7db19632f419cd778)
pub fn update_camera3_d_inverse_view_projection(camera: &mut Camera3D, aspect: f64) -> bool {
    let ok = (|| -> bool {
        {
            set_projection_matrix4(
                &mut (*__SCRATCH_PROJECTION.lock().unwrap()),
                &camera.projection,
                aspect,
                camera.near,
                camera.far,
            );
            {
                (*__SCRATCH_PROJECTION.lock().unwrap()).m[0.0_f64 as usize] += (camera.jitter.x
                    * ((*__SCRATCH_PROJECTION.lock().unwrap()).m[3.0_f64 as usize] as f64))
                    as f32;
                (*__SCRATCH_PROJECTION.lock().unwrap()).m[4.0_f64 as usize] += (camera.jitter.x
                    * ((*__SCRATCH_PROJECTION.lock().unwrap()).m[7.0_f64 as usize] as f64))
                    as f32;
                (*__SCRATCH_PROJECTION.lock().unwrap()).m[8.0_f64 as usize] += (camera.jitter.x
                    * ((*__SCRATCH_PROJECTION.lock().unwrap()).m[11.0_f64 as usize] as f64))
                    as f32;
                (*__SCRATCH_PROJECTION.lock().unwrap()).m[12.0_f64 as usize] += (camera.jitter.x
                    * ((*__SCRATCH_PROJECTION.lock().unwrap()).m[15.0_f64 as usize] as f64))
                    as f32;
                (*__SCRATCH_PROJECTION.lock().unwrap()).m[1.0_f64 as usize] += (camera.jitter.y
                    * ((*__SCRATCH_PROJECTION.lock().unwrap()).m[3.0_f64 as usize] as f64))
                    as f32;
                (*__SCRATCH_PROJECTION.lock().unwrap()).m[5.0_f64 as usize] += (camera.jitter.y
                    * ((*__SCRATCH_PROJECTION.lock().unwrap()).m[7.0_f64 as usize] as f64))
                    as f32;
                (*__SCRATCH_PROJECTION.lock().unwrap()).m[9.0_f64 as usize] += (camera.jitter.y
                    * ((*__SCRATCH_PROJECTION.lock().unwrap()).m[11.0_f64 as usize] as f64))
                    as f32;
                (*__SCRATCH_PROJECTION.lock().unwrap()).m[13.0_f64 as usize] += (camera.jitter.y
                    * ((*__SCRATCH_PROJECTION.lock().unwrap()).m[15.0_f64 as usize] as f64))
                    as f32;
            };
            if ((camera.near_clip_plane).clone()).is_some() {
                apply_oblique_near_clip_plane(
                    &mut (*__SCRATCH_PROJECTION.lock().unwrap()),
                    camera.near_clip_plane.as_ref().unwrap(),
                );
            }
            multiply_matrix4(
                &mut (*__SCRATCH_VIEW_PROJECTION.lock().unwrap()),
                &{
                    let __flight_source = &(*__SCRATCH_PROJECTION.lock().unwrap());
                    Matrix4Like {
                        __flight_identity: std::sync::Arc::clone(
                            &__flight_source.__flight_identity,
                        ),
                        __flight_entity_runtime: std::sync::Arc::clone(
                            &__flight_source.__flight_entity_runtime,
                        ),
                        __flight_entity_snapshot: __flight_source.__flight_entity_snapshot.clone(),
                        m: (__flight_source.m).clone(),
                    }
                },
                &{
                    let __flight_source = &(camera.view);
                    Matrix4Like {
                        __flight_identity: std::sync::Arc::clone(
                            &__flight_source.__flight_identity,
                        ),
                        __flight_entity_runtime: std::sync::Arc::clone(
                            &__flight_source.__flight_entity_runtime,
                        ),
                        __flight_entity_snapshot: __flight_source.__flight_entity_snapshot.clone(),
                        m: (__flight_source.m).clone(),
                    }
                },
            );
        };
        return inverse_matrix4(&mut (*__SCRATCH_INVERSE.lock().unwrap()), &{
            let __flight_source = &(*__SCRATCH_VIEW_PROJECTION.lock().unwrap());
            Matrix4Like {
                __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
                __flight_entity_runtime: std::sync::Arc::clone(
                    &__flight_source.__flight_entity_runtime,
                ),
                __flight_entity_snapshot: __flight_source.__flight_entity_snapshot.clone(),
                m: (__flight_source.m).clone(),
            }
        });
    })();
    if ok {
        {
            let __flight_offset = (0.0_f64) as usize;
            let __flight_values: Vec<f32> = (((*__SCRATCH_INVERSE.lock().unwrap()).m).clone())
                .iter()
                .map(|value| (*value) as f32)
                .collect();
            camera.inverse_view_projection.m
                [__flight_offset..__flight_offset + __flight_values.len()]
                .copy_from_slice(&__flight_values);
        };
    }
    return ok;
}

// Source: upstream/packages/camera/src/camera.ts:141 (sha256:7ec7d78ecc2375afdd5fdace3128662b774baf283c6d578b2e34d225a67cf60b)
fn apply_camera3_d_projection_jitter(out: &mut Matrix4Like, x: f64, y: f64) -> () {
    out.m[0.0_f64 as usize] += (x * (out.m[3.0_f64 as usize] as f64)) as f32;
    out.m[4.0_f64 as usize] += (x * (out.m[7.0_f64 as usize] as f64)) as f32;
    out.m[8.0_f64 as usize] += (x * (out.m[11.0_f64 as usize] as f64)) as f32;
    out.m[12.0_f64 as usize] += (x * (out.m[15.0_f64 as usize] as f64)) as f32;
    out.m[1.0_f64 as usize] += (y * (out.m[3.0_f64 as usize] as f64)) as f32;
    out.m[5.0_f64 as usize] += (y * (out.m[7.0_f64 as usize] as f64)) as f32;
    out.m[9.0_f64 as usize] += (y * (out.m[11.0_f64 as usize] as f64)) as f32;
    out.m[13.0_f64 as usize] += (y * (out.m[15.0_f64 as usize] as f64)) as f32;
}

// Source: upstream/packages/camera/src/camera.ts:154 (sha256:4ad6a35d9104577101cd617015c99a42397b2a539dbed3a36ad1c58c3356133d)
static __SCRATCH_INVERSE: std::sync::LazyLock<std::sync::Mutex<Matrix4>> =
    std::sync::LazyLock::new(|| {
        std::sync::Mutex::new(create_matrix4(
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None,
        ))
    });

// Source: upstream/packages/camera/src/camera.ts:155 (sha256:25f0673e0ee20250bdc4881d41975acf1c9d47c88eec611dac8217aedc5ded65)
static __SCRATCH_PROJECTION: std::sync::LazyLock<std::sync::Mutex<Matrix4>> =
    std::sync::LazyLock::new(|| {
        std::sync::Mutex::new(create_matrix4(
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None,
        ))
    });

// Source: upstream/packages/camera/src/camera.ts:156 (sha256:ea1bce46bff5117486aa66f0bc0c33f5ba239247bd135b339e34f20358b60428)
static __SCRATCH_VIEW_PROJECTION: std::sync::LazyLock<std::sync::Mutex<Matrix4>> =
    std::sync::LazyLock::new(|| {
        std::sync::Mutex::new(create_matrix4(
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None,
        ))
    });

// Source: upstream/packages/camera/src/camera.ts:158 (sha256:da057530e67b2410330af593197368180d7ac9c2f18438132f658fc07f01e231)
static CAMERA3_D_VIEW_GUARD: std::sync::LazyLock<
    std::sync::Mutex<
        Option<std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(Camera3D) -> () + Send + 'static>>>>,
    >,
> = std::sync::LazyLock::new(|| std::sync::Mutex::new(None));
