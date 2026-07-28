// @generated from upstream/packages/camera/src/projection.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_geometry::{set_orthographic_matrix4, set_perspective_matrix4};
use flighthq_types::{Matrix4Like, OrthographicProjection, PerspectiveProjection, Projection};

// Source: upstream/packages/camera/src/projection.ts:8 (sha256:5a482d612039727e3571a20822c6f14d18014db81d58dff99dfbfbbcb5ba33dc)
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

// Source: upstream/packages/camera/src/projection.ts:18 (sha256:8bcc0ae8397defb96ff9dfd9c935a480d72b02730de2aa2eaf7155725f33c080)
pub fn create_perspective_projection(opts: &PerspectiveProjectionOptions) -> PerspectiveProjection {
    return PerspectiveProjection {
        __flight_identity: std::sync::Arc::new(()),
        aspect: (opts.aspect).unwrap_or(1.0_f64),
        fov_y: opts.fov_y,
        kind: "perspective".to_owned(),
    };
}

// Source: upstream/packages/camera/src/projection.ts:27 (sha256:de356014920dff8ebff0b838571acbadaa8c8bebfdbfd44753b66342dbb0f576)
pub fn is_orthographic_projection(projection: &Projection) -> bool {
    return matches!(&(projection), flighthq_types::Projection::A(_));
}

// Source: upstream/packages/camera/src/projection.ts:32 (sha256:0e3dff76e488a9d84b8cf2906d22615a143f32eff40039049d4bc64b37d0a20f)
pub fn is_perspective_projection(projection: &Projection) -> bool {
    return matches!(&(projection), flighthq_types::Projection::B(_));
}

// Source: upstream/packages/camera/src/projection.ts:44 (sha256:1c51ac5a6cf7bac13b6f98884a96810c6630e44b29d223eb89b026e2839af983)
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

// Source: upstream/packages/camera/src/projection.ts:64 (sha256:10fca9c7533cef2be0a3c068ffafb9e851966a47e21568c1077b91aa7c8f7e81)
#[derive(Clone, Default)]
pub struct OrthographicProjectionOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub half_height: f64,
    pub half_width: f64,
}
impl PartialEq for OrthographicProjectionOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/camera/src/projection.ts:70 (sha256:34ac68876d041d6ffeaa372aaf4fb1b6dfc40e83a7deda58e9368a6f82ebbb23)
#[derive(Clone, Default)]
pub struct PerspectiveProjectionOptions {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub aspect: Option<f64>,
    pub fov_y: f64,
}
impl PartialEq for PerspectiveProjectionOptions {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
