// @generated from upstream/packages/lighting/src/lightAnalysis.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_color::get_color_luminance;
use flighthq_types::{
    AMBIENT_LIGHT_KIND as ambient_light_kind_constant, AREA_LIGHT_KIND as area_light_kind_constant,
    BoundingSphereLike, DIRECTIONAL_LIGHT_KIND as directional_light_kind_constant,
    ENVIRONMENT_KIND as environment_kind_constant,
    HEMISPHERE_LIGHT_KIND as hemisphere_light_kind_constant, Light,
    POINT_LIGHT_KIND as point_light_kind_constant, PointLight,
    SPOT_LIGHT_KIND as spot_light_kind_constant, SpotLight,
};

// Source: upstream/packages/lighting/src/lightAnalysis.ts:19 (sha256:5310d50fec0c5c61226ec0c5d5ec564bbfb9f902f83da8faf98b07de38b91e52)
pub fn get_light_contribution_at_bounding_sphere(
    light: &crate::FlightUnion2<PointLight, SpotLight>,
    bounds: &BoundingSphereLike,
) -> f64 {
    if (bounds.radius < 0.0_f64) {
        return 0.0_f64;
    }
    let center_dx = (bounds.center.x - light.position.x);
    let center_dy = (bounds.center.y - light.position.y);
    let center_dz = (bounds.center.z - light.position.z);
    let center_distance = ((center_dx).powi(2) + (center_dy).powi(2) + (center_dz).powi(2)).sqrt();
    let distance = (center_distance - bounds.radius).max(0.0_f64);
    let distance_squared = (distance * distance);
    let mut window = 1.0_f64;
    if (light.range > 0.0_f64) {
        let factor = (distance_squared / (light.range * light.range));
        let windowed = (0.0_f64).max((1.0_f64).min((1.0_f64 - (factor * factor))));
        window = (windowed * windowed);
    }
    let mut contribution =
        ((get_light_luminance(light) * window) / (distance_squared).max(0.0001_f64));
    if (light.kind == spot_light_kind_constant) {
        let spot = match (*light).clone() {
            crate::FlightUnion2::A(_) => panic!("TypeScript union narrowing failed"),
            crate::FlightUnion2::B(value) => value,
        };
        let direction_length =
            ((spot.direction.x).powi(2) + (spot.direction.y).powi(2) + (spot.direction.z).powi(2))
                .sqrt();
        let inverse_ray_length = if (center_distance > 0.0_f64) {
            (1.0_f64 / center_distance)
        } else {
            0.0_f64
        };
        let inverse_direction_length = if (direction_length > 0.0_f64) {
            (1.0_f64 / direction_length)
        } else {
            0.0_f64
        };
        let cosine = (((((spot.direction.x * center_dx) + (spot.direction.y * center_dy))
            + (spot.direction.z * center_dz))
            * inverse_ray_length)
            * inverse_direction_length);
        contribution *= smoothstep(
            spot.outer_cone_cos,
            spot.inner_cone_cos,
            if (center_distance > 0.0_f64) {
                cosine
            } else {
                1.0_f64
            },
        );
    }
    return contribution;
}

// Source: upstream/packages/lighting/src/lightAnalysis.ts:59 (sha256:582322bbc6c5cd67d249036500c4c0b7d89ac0784346fb0d894b617356bda66a)
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
        let spatial = {
            let __flight_source = &((*light).clone());
            PointLight {
                __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
                __flight_entity_runtime: std::sync::Arc::clone(
                    &__flight_source.__flight_entity_runtime,
                ),
                kind: (__flight_source.kind).clone(),
                casts_shadow: __flight_source.casts_shadow,
                color: __flight_source.color,
                direction: (__flight_source.direction).clone(),
                inner_cone_cos: __flight_source.inner_cone_cos,
                intensity: __flight_source.intensity,
                normal_bias: __flight_source.normal_bias,
                outer_cone_cos: __flight_source.outer_cone_cos,
                pcf_radius: __flight_source.pcf_radius,
                position: (__flight_source.position).clone(),
                range: __flight_source.range,
                shadow_bias: __flight_source.shadow_bias,
                ground_color: __flight_source.ground_color,
                sky_color: __flight_source.sky_color,
                environment: (__flight_source.environment).clone(),
                right: (__flight_source.right).clone(),
                up: (__flight_source.up).clone(),
                ..Default::default()
            }
        };
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

// Source: upstream/packages/lighting/src/lightAnalysis.ts:102 (sha256:6d8922f0f8710593981c4b42e773268f20960a03b28145ef4db0872620a61864)
#[derive(Clone, Default)]
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
    let colored = {
        let __flight_source = &((*light).clone());
        GetLightLuminanceRecord1 {
            __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
            color: Some(__flight_source.color),
            intensity: Some(__flight_source.intensity),
        }
    };
    let color = colored.color;
    if (color).is_none() {
        return 0.0_f64;
    }
    let intensity = (colored.intensity).unwrap_or(1.0_f64);
    return (get_color_luminance(*(color.as_ref().unwrap())) * intensity);
}

// Source: upstream/packages/lighting/src/lightAnalysis.ts:114 (sha256:fb2014c1906247332b1e600c567c452cff5a6fc519cf826eb4d08c43419bf326)
pub fn has_light_influence_on_bounds(light: &Light, bounds: &BoundingSphereLike) -> bool {
    let kind = (light.kind).clone();
    if (((kind == ambient_light_kind_constant) || (kind == hemisphere_light_kind_constant))
        || (kind == environment_kind_constant))
        || (kind == directional_light_kind_constant)
    {
        return true;
    }
    if ((kind != point_light_kind_constant) && (kind != spot_light_kind_constant))
        && (kind != area_light_kind_constant)
    {
        return true;
    }
    let spatial = {
        let __flight_source = &((*light).clone());
        PointLight {
            __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
            __flight_entity_runtime: std::sync::Arc::clone(
                &__flight_source.__flight_entity_runtime,
            ),
            kind: (__flight_source.kind).clone(),
            casts_shadow: __flight_source.casts_shadow,
            color: __flight_source.color,
            direction: (__flight_source.direction).clone(),
            inner_cone_cos: __flight_source.inner_cone_cos,
            intensity: __flight_source.intensity,
            normal_bias: __flight_source.normal_bias,
            outer_cone_cos: __flight_source.outer_cone_cos,
            pcf_radius: __flight_source.pcf_radius,
            position: (__flight_source.position).clone(),
            range: __flight_source.range,
            shadow_bias: __flight_source.shadow_bias,
            ground_color: __flight_source.ground_color,
            sky_color: __flight_source.sky_color,
            environment: (__flight_source.environment).clone(),
            right: (__flight_source.right).clone(),
            up: (__flight_source.up).clone(),
            ..Default::default()
        }
    };
    if (spatial.range < 0.0_f64) {
        return true;
    }
    if (bounds.radius < 0.0_f64) {
        return false;
    }
    let dx = (spatial.position.x - bounds.center.x);
    let dy = (spatial.position.y - bounds.center.y);
    let dz = (spatial.position.z - bounds.center.z);
    let dist_sq = (((dx * dx) + (dy * dy)) + (dz * dz));
    let rad_sum = (spatial.range + bounds.radius);
    return (dist_sq <= (rad_sum * rad_sum));
}

// Source: upstream/packages/lighting/src/lightAnalysis.ts:144 (sha256:25cbd62b601a347fb029073aa064a496d8ae54f7db15599017f302ca09989b01)
#[derive(Clone, Default)]
struct IsLightCastingShadowRecord1 {
    __flight_identity: std::sync::Arc<()>,
    casts_shadow: bool,
}
impl PartialEq for IsLightCastingShadowRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn is_light_casting_shadow(light: &Light) -> bool {
    let kind = (light.kind).clone();
    if ((kind == ambient_light_kind_constant) || (kind == hemisphere_light_kind_constant))
        || (kind == environment_kind_constant)
    {
        return false;
    }
    return {
        let __flight_source = &((*light).clone());
        IsLightCastingShadowRecord1 {
            __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
            casts_shadow: __flight_source.casts_shadow,
        }
    }
    .casts_shadow;
}

// Source: upstream/packages/lighting/src/lightAnalysis.ts:152 (sha256:e9f211c4258a59ade165bc86e405004b7cc4e74f1771a74d8f92ca749567acc4)
fn smoothstep(edge0: f64, edge1: f64, value: f64) -> f64 {
    if (edge0 == edge1) {
        return if (value < edge0) { 0.0_f64 } else { 1.0_f64 };
    }
    let t = (0.0_f64).max((1.0_f64).min(((value - edge0) / (edge1 - edge0))));
    return ((t * t) * (3.0_f64 - (2.0_f64 * t)));
}
