// @generated from upstream/packages/camera/src/projection.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_geometry::{set_orthographic_matrix4, set_perspective_matrix4};
use flighthq_types::{
    Matrix4Like, OrthographicProjection, OrthographicProjectionOptions, PerspectiveProjection,
    PerspectiveProjectionOptions, Projection,
};

// Source: upstream/packages/camera/src/projection.ts:14 (sha256:5a482d612039727e3571a20822c6f14d18014db81d58dff99dfbfbbcb5ba33dc)
pub fn create_orthographic_projection(
    opts: &OrthographicProjectionOptions,
) -> OrthographicProjection {
    return OrthographicProjection {
        __flight_identity: std::sync::Arc::new(()),
        half_height: opts.half_height,
        half_width: opts.half_width,
        kind: "orthographic".to_owned(),
    };
}

// Source: upstream/packages/camera/src/projection.ts:24 (sha256:8bcc0ae8397defb96ff9dfd9c935a480d72b02730de2aa2eaf7155725f33c080)
pub fn create_perspective_projection(opts: &PerspectiveProjectionOptions) -> PerspectiveProjection {
    return PerspectiveProjection {
        __flight_identity: std::sync::Arc::new(()),
        aspect: (opts.aspect).unwrap_or(1.0_f64),
        fov_y: opts.fov_y,
        kind: "perspective".to_owned(),
    };
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

// Source: upstream/packages/camera/src/projection.ts:44 (sha256:de356014920dff8ebff0b838571acbadaa8c8bebfdbfd44753b66342dbb0f576)
pub fn is_orthographic_projection(projection: &Projection) -> bool {
    return matches!(&(projection), flighthq_types::Projection::A(_));
}

// Source: upstream/packages/camera/src/projection.ts:49 (sha256:0e3dff76e488a9d84b8cf2906d22615a143f32eff40039049d4bc64b37d0a20f)
pub fn is_perspective_projection(projection: &Projection) -> bool {
    return matches!(&(projection), flighthq_types::Projection::B(_));
}

// Source: upstream/packages/camera/src/projection.ts:61 (sha256:1c51ac5a6cf7bac13b6f98884a96810c6630e44b29d223eb89b026e2839af983)
pub fn set_projection_matrix4(
    out: &mut Matrix4Like,
    projection: &Projection,
    aspect: f64,
    near: f64,
    far: f64,
) -> () {
    if matches!(&(projection), flighthq_types::Projection::B(_)) {
        let tan_half_fov_y = ((match (*projection).clone() {
            flighthq_types::Projection::A(_) => panic!("TypeScript union narrowing failed"),
            flighthq_types::Projection::B(value) => value,
        })
        .fov_y
            * 0.5_f64)
            .tan();
        set_perspective_matrix4(out, tan_half_fov_y, aspect, near, far);
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
