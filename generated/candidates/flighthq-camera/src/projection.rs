// @generated from upstream/packages/camera/src/projection.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_entity::{allocate_entity, finish_entity};
use flighthq_geometry::{create_matrix4, set_orthographic_matrix4, set_perspective_matrix4};
use flighthq_types::{
    EntityConstruction, Matrix4Like, OrthographicProjection, OrthographicProjectionOptions,
    PerspectiveProjection, PerspectiveProjectionOptions, Projection, RawProjection,
    RawProjectionOptions,
};

// Source: upstream/packages/camera/src/projection.ts:14 (sha256:171d523413e079b2a73061c138bc4bc9dd97e181f888b21a95059c4ffc1ceaa1)
pub fn create_orthographic_projection(
    opts: &OrthographicProjectionOptions,
) -> OrthographicProjection {
    let mut out = allocate_entity();
    initialize_orthographic_projection((out).clone(), opts);
    return finish_entity((out).clone());
}

// Source: upstream/packages/camera/src/projection.ts:20 (sha256:3bf5d12263282e5fc34c0c7bb35285b705c71ebbf9635d24d021ed52ad317eda)
pub fn create_perspective_projection(opts: &PerspectiveProjectionOptions) -> PerspectiveProjection {
    let mut out = allocate_entity();
    initialize_perspective_projection((out).clone(), opts);
    return finish_entity((out).clone());
}

// Source: upstream/packages/camera/src/projection.ts:26 (sha256:972fbcd51b9f4d3ce950bced25e93317e914fdfeacf41dc3ba979f8c2bde3e01)
pub fn create_raw_projection(opts: &RawProjectionOptions) -> RawProjection {
    let mut out = allocate_entity();
    initialize_raw_projection((out).clone(), opts);
    return finish_entity((out).clone());
}

// Source: upstream/packages/camera/src/projection.ts:35 (sha256:8eec5a9cc2f49d6fde6986c19f44eabb3b6ea18f88291aba860fd4562ea13c0d)
pub fn get_orthographic_projection_texel_size(
    projection: &OrthographicProjection,
    pixel_width: f64,
    pixel_height: f64,
) -> f64 {
    return ((projection.half_width * 2.0_f64) / pixel_width)
        .max(((projection.half_height * 2.0_f64) / pixel_height));
}

// Source: upstream/packages/camera/src/projection.ts:46 (sha256:8839f958b30ae78a88639eedabf451438a21338e83a3d649793096bd761250ee)
pub fn initialize_orthographic_projection(
    out: EntityConstruction<OrthographicProjection>,
    opts: &OrthographicProjectionOptions,
) -> () {
    crate::host_set("host.halfHeight", opts.half_height);
    crate::host_set("host.halfWidth", opts.half_width);
    crate::host_set("host.kind", "orthographic");
}

// Source: upstream/packages/camera/src/projection.ts:57 (sha256:218f33c4b1a6afe1c854d46de53c1b6e9d60ee1b197369fc824b592280a23774)
pub fn initialize_perspective_projection(
    out: EntityConstruction<PerspectiveProjection>,
    opts: &PerspectiveProjectionOptions,
) -> () {
    crate::host_set("host.aspect", (opts.aspect).unwrap_or(1.0_f64));
    crate::host_set("host.fovY", opts.fov_y);
    crate::host_set("host.kind", "perspective");
}

// Source: upstream/packages/camera/src/projection.ts:66 (sha256:520e7336827dad0b7de4ddf992b0d05a63e84c69b3fabbfe4868340a6cb572c1)
pub fn initialize_raw_projection(
    out: EntityConstruction<RawProjection>,
    opts: &RawProjectionOptions,
) -> () {
    crate::host_set("host.kind", "raw");
    crate::host_set(
        "host.matrix",
        create_matrix4(
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None,
        ),
    );
    crate::host_value::<()>("host.set");
}

// Source: upstream/packages/camera/src/projection.ts:76 (sha256:de356014920dff8ebff0b838571acbadaa8c8bebfdbfd44753b66342dbb0f576)
pub fn is_orthographic_projection(projection: &Projection) -> bool {
    return matches!(&(projection), flighthq_types::Projection::A(_));
}

// Source: upstream/packages/camera/src/projection.ts:81 (sha256:0e3dff76e488a9d84b8cf2906d22615a143f32eff40039049d4bc64b37d0a20f)
pub fn is_perspective_projection(projection: &Projection) -> bool {
    return matches!(
        &(projection),
        crate::FlightUnion2::B(crate::FlightUnion2::A(_))
    );
}

// Source: upstream/packages/camera/src/projection.ts:86 (sha256:d3c144e0e586b034acb8e38d32000a303a6d576bf437956d1e3bafad3b57ffea)
pub fn is_raw_projection(projection: &Projection) -> bool {
    return matches!(
        &(projection),
        crate::FlightUnion2::B(crate::FlightUnion2::B(_))
    );
}

// Source: upstream/packages/camera/src/projection.ts:98 (sha256:4d37b5c73a9970d9291473a5d43f64a85f6687f66ec7d9f6c27dca332a46f778)
pub fn set_projection_matrix4(
    out: &mut Matrix4Like,
    projection: &Projection,
    aspect: f64,
    near: f64,
    far: f64,
) -> () {
    if matches!(
        &(projection),
        crate::FlightUnion2::B(crate::FlightUnion2::A(_))
    ) {
        set_perspective_matrix4(
            out,
            ((match (*projection).clone() {
                flighthq_types::Projection::A(_) => panic!("TypeScript union narrowing failed"),
                flighthq_types::Projection::B(value) => match value {
                    crate::FlightUnion2::A(value) => value,
                    crate::FlightUnion2::B(_) => panic!("TypeScript union narrowing failed"),
                },
            })
            .fov_y
                * 0.5_f64)
                .tan(),
            aspect,
            near,
            far,
        );
        return;
    }
    if matches!(
        &(projection),
        crate::FlightUnion2::B(crate::FlightUnion2::B(_))
    ) {
        {
            let __flight_offset = (0.0_f64) as usize;
            let __flight_values: Vec<f32> = (((match (*projection).clone() {
                flighthq_types::Projection::A(_) => panic!("TypeScript union narrowing failed"),
                flighthq_types::Projection::B(value) => match value {
                    crate::FlightUnion2::A(_) => panic!("TypeScript union narrowing failed"),
                    crate::FlightUnion2::B(value) => value,
                },
            })
            .matrix
            .m)
                .clone())
            .iter()
            .map(|value| (*value) as f32)
            .collect();
            out.m[__flight_offset..__flight_offset + __flight_values.len()]
                .copy_from_slice(&__flight_values);
        };
        return;
    }
    let half_width = (match (*projection).clone() {
        flighthq_types::Projection::A(value) => value,
        flighthq_types::Projection::B(_) => panic!("TypeScript union narrowing failed"),
    })
    .half_width;
    let half_height = (match (*projection).clone() {
        flighthq_types::Projection::A(value) => value,
        flighthq_types::Projection::B(_) => panic!("TypeScript union narrowing failed"),
    })
    .half_height;
    set_orthographic_matrix4(
        out,
        (-half_width),
        half_width,
        (-half_height),
        half_height,
        near,
        far,
    );
}
