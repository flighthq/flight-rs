// @generated from upstream/packages/lighting/src/lightAnalysis.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_geometry::create_bounding_sphere;
use flighthq_types::{
    AMBIENT_LIGHT_KIND as ambient_light_kind_constant, AREA_LIGHT_KIND as area_light_kind_constant,
    BoundingSphere, BoundingSphereLike, DIRECTIONAL_LIGHT_KIND as directional_light_kind_constant,
    ENVIRONMENT_KIND as environment_kind_constant,
    HEMISPHERE_LIGHT_KIND as hemisphere_light_kind_constant, Light,
    POINT_LIGHT_KIND as point_light_kind_constant, SPOT_LIGHT_KIND as spot_light_kind_constant,
};

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

// Source: upstream/packages/lighting/src/lightAnalysis.ts:17 (sha256:582322bbc6c5cd67d249036500c4c0b7d89ac0784346fb0d894b617356bda66a)
pub fn get_light_influence_bounds(out: &mut BoundingSphereLike, light: &Light) -> () {
    let kind = (light.kind).clone();
    if (((kind == ambient_light_kind_constant) || (kind == hemisphere_light_kind_constant))
        || (kind == environment_kind_constant))
        || (kind == directional_light_kind_constant)
    {
        out.center.x = 0.0_f64;
        out.center.y = 0.0_f64;
        out.center.z = 0.0_f64;
        out.radius = (-1.0_f64);
        return;
    }
    if ((kind == point_light_kind_constant) || (kind == spot_light_kind_constant))
        || (kind == area_light_kind_constant)
    {
        let spatial = light;
        let range = spatial.range;
        if (range < 0.0_f64) {
            out.center.x = 0.0_f64;
            out.center.y = 0.0_f64;
            out.center.z = 0.0_f64;
            out.radius = (-1.0_f64);
            return;
        }
        out.center.x = spatial.position.x;
        out.center.y = spatial.position.y;
        out.center.z = spatial.position.z;
        out.radius = range;
        return;
    }
    out.center.x = 0.0_f64;
    out.center.y = 0.0_f64;
    out.center.z = 0.0_f64;
    out.radius = (-1.0_f64);
}

// Source: upstream/packages/lighting/src/lightAnalysis.ts:60 (sha256:d267de3a4faaa737907f95eebabceaa4b992b7a82181095b79291b36c1144123)
#[derive(Clone)]
struct GetLightLuminanceRecord1 {
    __flight_identity: std::sync::Arc<()>,
    color: Option<f64>,
    intensity: Option<f64>,
}
impl PartialEq for GetLightLuminanceRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn get_light_luminance(light: &Light) -> f64 {
    let colored = light;
    let color = colored.color;
    if (color).is_none() {
        return 0.0_f64;
    }
    let r = ((__flight_js_to_i32(
        (__flight_js_to_u32(*(color.as_ref().unwrap())) >> (__flight_js_to_u32(24.0_f64) & 31))
            as f64,
    ) & __flight_js_to_i32(255.0_f64)) as f64
        / 255.0_f64);
    let g = ((__flight_js_to_i32(
        (__flight_js_to_u32(*(color.as_ref().unwrap())) >> (__flight_js_to_u32(16.0_f64) & 31))
            as f64,
    ) & __flight_js_to_i32(255.0_f64)) as f64
        / 255.0_f64);
    let b = ((__flight_js_to_i32(
        (__flight_js_to_u32(*(color.as_ref().unwrap())) >> (__flight_js_to_u32(8.0_f64) & 31))
            as f64,
    ) & __flight_js_to_i32(255.0_f64)) as f64
        / 255.0_f64);
    let luma = (((0.2126_f64 * r) + (0.7152_f64 * g)) + (0.0722_f64 * b));
    let intensity = (colored.intensity).unwrap_or(1.0_f64);
    return (luma * intensity);
}

// Source: upstream/packages/lighting/src/lightAnalysis.ts:76 (sha256:83ce4a25d7f13f6982a1d46f79261e5485a066c4971a2f248de1ba1bec7ffcbe)
pub fn has_light_influence_on_bounds(light: &Light, bounds: &BoundingSphereLike) -> bool {
    get_light_influence_bounds(&mut (*SCRATCH_SPHERE.lock().unwrap()), light);
    if ((*SCRATCH_SPHERE.lock().unwrap()).radius < 0.0_f64) {
        return true;
    }
    if (bounds.radius < 0.0_f64) {
        return false;
    }
    let dx = ((*SCRATCH_SPHERE.lock().unwrap()).center.x - bounds.center.x);
    let dy = ((*SCRATCH_SPHERE.lock().unwrap()).center.y - bounds.center.y);
    let dz = ((*SCRATCH_SPHERE.lock().unwrap()).center.z - bounds.center.z);
    let dist_sq = (((dx * dx) + (dy * dy)) + (dz * dz));
    let rad_sum = ((*SCRATCH_SPHERE.lock().unwrap()).radius + bounds.radius);
    return (dist_sq <= (rad_sum * rad_sum));
}

// Source: upstream/packages/lighting/src/lightAnalysis.ts:94 (sha256:cadd8dbe715f4178ad999a2c4965dc0946bc2459f5afd721e6b5bc84a81f685b)
#[derive(Clone)]
struct IsLightShadowCastingRecord1 {
    __flight_identity: std::sync::Arc<()>,
    casts_shadow: bool,
}
impl PartialEq for IsLightShadowCastingRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn is_light_shadow_casting(light: &Light) -> bool {
    let kind = (light.kind).clone();
    if ((kind == ambient_light_kind_constant) || (kind == hemisphere_light_kind_constant))
        || (kind == environment_kind_constant)
    {
        return false;
    }
    return light.casts_shadow;
}

// Source: upstream/packages/lighting/src/lightAnalysis.ts:103 (sha256:598079d3d933dca6dd38e0c05c819ab7fb9bdfda5fe0f7e31c38909d8f8c1b74)
static SCRATCH_SPHERE: std::sync::LazyLock<std::sync::Mutex<BoundingSphere>> =
    std::sync::LazyLock::new(|| {
        std::sync::Mutex::new(create_bounding_sphere(
            Some(0.0_f64),
            Some(0.0_f64),
            Some(0.0_f64),
            Some((-1.0_f64)),
        ))
    });
