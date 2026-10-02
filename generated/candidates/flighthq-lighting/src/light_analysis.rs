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
    AmbientLight, AreaLight, BoundingSphereLike,
    DIRECTIONAL_LIGHT_KIND as directional_light_kind_constant, DirectionalLight,
    ENVIRONMENT_KIND as environment_kind_constant,
    HEMISPHERE_LIGHT_KIND as hemisphere_light_kind_constant, HemisphereLight, Light,
    POINT_LIGHT_KIND as point_light_kind_constant, PointLight,
    SPOT_LIGHT_KIND as spot_light_kind_constant, SpotLight,
};

// Source: upstream/packages/lighting/src/lightAnalysis.ts:29 (sha256:1fd3a149b7280ba8b41b3bbed4fee0ba707f7abea596fb1dfd08816c59ce7038)
pub fn get_light_contribution_at_bounding_sphere(
    light: &crate::FlightUnion2<PointLight, SpotLight>,
    bounds: &BoundingSphereLike,
) -> f64 {
    if (!is_light_enabled(&match ((*light).clone()) {
        crate::FlightUnion2::A(value) => {
            let __flight_source = &(value);
            Light {
                __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
                __flight_entity_runtime: std::sync::Arc::clone(
                    &__flight_source.__flight_entity_runtime,
                ),
                __flight_entity_snapshot: __flight_source
                    .__flight_entity_snapshot
                    .clone()
                    .or_else(|| Some(std::sync::Arc::new((*__flight_source).clone()))),
                kind: (__flight_source.kind).clone(),
                casts_shadow: __flight_source.casts_shadow,
                color: __flight_source.color,
                decay: __flight_source.decay,
                direction: (__flight_source.direction).clone(),
                enabled: __flight_source.enabled,
                inner_cone_cos: __flight_source.inner_cone_cos,
                intensity: __flight_source.intensity,
                intensity_unit: (__flight_source.intensity_unit).clone(),
                layer_mask: __flight_source.layer_mask,
                priority: __flight_source.priority,
                normal_bias: __flight_source.normal_bias,
                outer_cone_cos: __flight_source.outer_cone_cos,
                pcf_radius: __flight_source.pcf_radius,
                position: (__flight_source.position).clone(),
                range: __flight_source.range,
                shadow_bias: __flight_source.shadow_bias,
                shadow_far: __flight_source.shadow_far,
                shadow_map_size: __flight_source.shadow_map_size,
                shadow_near: __flight_source.shadow_near,
                shadow_strength: __flight_source.shadow_strength,
                spot_blend: __flight_source.spot_blend,
                ground_color: __flight_source.ground_color,
                sky_color: __flight_source.sky_color,
                environment: (__flight_source.environment).clone(),
                cascade_count: __flight_source.cascade_count,
                cascade_splits: (__flight_source.cascade_splits).clone(),
                right: (__flight_source.right).clone(),
                up: (__flight_source.up).clone(),
                ..Default::default()
            }
        }
        crate::FlightUnion2::B(value) => {
            let __flight_source = &(value);
            Light {
                __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
                __flight_entity_runtime: std::sync::Arc::clone(
                    &__flight_source.__flight_entity_runtime,
                ),
                __flight_entity_snapshot: __flight_source
                    .__flight_entity_snapshot
                    .clone()
                    .or_else(|| Some(std::sync::Arc::new((*__flight_source).clone()))),
                kind: (__flight_source.kind).clone(),
                casts_shadow: __flight_source.casts_shadow,
                color: __flight_source.color,
                decay: __flight_source.decay,
                direction: (__flight_source.direction).clone(),
                enabled: __flight_source.enabled,
                inner_cone_cos: __flight_source.inner_cone_cos,
                intensity: __flight_source.intensity,
                intensity_unit: (__flight_source.intensity_unit).clone(),
                layer_mask: __flight_source.layer_mask,
                priority: __flight_source.priority,
                normal_bias: __flight_source.normal_bias,
                outer_cone_cos: __flight_source.outer_cone_cos,
                pcf_radius: __flight_source.pcf_radius,
                position: (__flight_source.position).clone(),
                range: __flight_source.range,
                shadow_bias: __flight_source.shadow_bias,
                shadow_far: __flight_source.shadow_far,
                shadow_map_size: __flight_source.shadow_map_size,
                shadow_near: __flight_source.shadow_near,
                shadow_strength: __flight_source.shadow_strength,
                spot_blend: __flight_source.spot_blend,
                ground_color: __flight_source.ground_color,
                sky_color: __flight_source.sky_color,
                environment: (__flight_source.environment).clone(),
                cascade_count: __flight_source.cascade_count,
                cascade_splits: (__flight_source.cascade_splits).clone(),
                right: (__flight_source.right).clone(),
                up: (__flight_source.up).clone(),
                ..Default::default()
            }
        }
    })) || (bounds.radius < 0.0_f64)
    {
        return 0.0_f64;
    }
    let center_dx = (bounds.center.x
        - (match &((*light).clone()) {
            crate::FlightUnion2::A(value) => (value).position.clone(),
            crate::FlightUnion2::B(value) => (value).position.clone(),
        })
        .x);
    let center_dy = (bounds.center.y
        - (match &((*light).clone()) {
            crate::FlightUnion2::A(value) => (value).position.clone(),
            crate::FlightUnion2::B(value) => (value).position.clone(),
        })
        .y);
    let center_dz = (bounds.center.z
        - (match &((*light).clone()) {
            crate::FlightUnion2::A(value) => (value).position.clone(),
            crate::FlightUnion2::B(value) => (value).position.clone(),
        })
        .z);
    let center_distance = ((center_dx).powi(2) + (center_dy).powi(2) + (center_dz).powi(2)).sqrt();
    let distance = (center_distance - bounds.radius).max(0.0_f64);
    let distance_squared = (distance * distance);
    let mut window = 1.0_f64;
    if (match &((*light).clone()) {
        crate::FlightUnion2::A(value) => (value).range.clone(),
        crate::FlightUnion2::B(value) => (value).range.clone(),
    } > 0.0_f64)
    {
        let factor = (distance_squared
            / (match &((*light).clone()) {
                crate::FlightUnion2::A(value) => (value).range.clone(),
                crate::FlightUnion2::B(value) => (value).range.clone(),
            } * match &((*light).clone()) {
                crate::FlightUnion2::A(value) => (value).range.clone(),
                crate::FlightUnion2::B(value) => (value).range.clone(),
            }));
        let windowed = (0.0_f64).max((1.0_f64).min((1.0_f64 - (factor * factor))));
        window = (windowed * windowed);
    }
    let attenuation = ((distance).max(0.01_f64)).powf(match &((*light).clone()) {
        crate::FlightUnion2::A(value) => (value).decay.clone(),
        crate::FlightUnion2::B(value) => (value).decay.clone(),
    });
    let mut contribution = ((get_light_luminance(&match ((*light).clone()) {
        crate::FlightUnion2::A(value) => {
            let __flight_source = &(value);
            Light {
                __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
                __flight_entity_runtime: std::sync::Arc::clone(
                    &__flight_source.__flight_entity_runtime,
                ),
                __flight_entity_snapshot: __flight_source
                    .__flight_entity_snapshot
                    .clone()
                    .or_else(|| Some(std::sync::Arc::new((*__flight_source).clone()))),
                kind: (__flight_source.kind).clone(),
                casts_shadow: __flight_source.casts_shadow,
                color: __flight_source.color,
                decay: __flight_source.decay,
                direction: (__flight_source.direction).clone(),
                enabled: __flight_source.enabled,
                inner_cone_cos: __flight_source.inner_cone_cos,
                intensity: __flight_source.intensity,
                intensity_unit: (__flight_source.intensity_unit).clone(),
                layer_mask: __flight_source.layer_mask,
                priority: __flight_source.priority,
                normal_bias: __flight_source.normal_bias,
                outer_cone_cos: __flight_source.outer_cone_cos,
                pcf_radius: __flight_source.pcf_radius,
                position: (__flight_source.position).clone(),
                range: __flight_source.range,
                shadow_bias: __flight_source.shadow_bias,
                shadow_far: __flight_source.shadow_far,
                shadow_map_size: __flight_source.shadow_map_size,
                shadow_near: __flight_source.shadow_near,
                shadow_strength: __flight_source.shadow_strength,
                spot_blend: __flight_source.spot_blend,
                ground_color: __flight_source.ground_color,
                sky_color: __flight_source.sky_color,
                environment: (__flight_source.environment).clone(),
                cascade_count: __flight_source.cascade_count,
                cascade_splits: (__flight_source.cascade_splits).clone(),
                right: (__flight_source.right).clone(),
                up: (__flight_source.up).clone(),
                ..Default::default()
            }
        }
        crate::FlightUnion2::B(value) => {
            let __flight_source = &(value);
            Light {
                __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
                __flight_entity_runtime: std::sync::Arc::clone(
                    &__flight_source.__flight_entity_runtime,
                ),
                __flight_entity_snapshot: __flight_source
                    .__flight_entity_snapshot
                    .clone()
                    .or_else(|| Some(std::sync::Arc::new((*__flight_source).clone()))),
                kind: (__flight_source.kind).clone(),
                casts_shadow: __flight_source.casts_shadow,
                color: __flight_source.color,
                decay: __flight_source.decay,
                direction: (__flight_source.direction).clone(),
                enabled: __flight_source.enabled,
                inner_cone_cos: __flight_source.inner_cone_cos,
                intensity: __flight_source.intensity,
                intensity_unit: (__flight_source.intensity_unit).clone(),
                layer_mask: __flight_source.layer_mask,
                priority: __flight_source.priority,
                normal_bias: __flight_source.normal_bias,
                outer_cone_cos: __flight_source.outer_cone_cos,
                pcf_radius: __flight_source.pcf_radius,
                position: (__flight_source.position).clone(),
                range: __flight_source.range,
                shadow_bias: __flight_source.shadow_bias,
                shadow_far: __flight_source.shadow_far,
                shadow_map_size: __flight_source.shadow_map_size,
                shadow_near: __flight_source.shadow_near,
                shadow_strength: __flight_source.shadow_strength,
                spot_blend: __flight_source.spot_blend,
                ground_color: __flight_source.ground_color,
                sky_color: __flight_source.sky_color,
                environment: (__flight_source.environment).clone(),
                cascade_count: __flight_source.cascade_count,
                cascade_splits: (__flight_source.cascade_splits).clone(),
                right: (__flight_source.right).clone(),
                up: (__flight_source.up).clone(),
                ..Default::default()
            }
        }
    }) * window)
        / attenuation);
    if (match &((*light).clone()) {
        crate::FlightUnion2::A(value) => (value).kind.clone(),
        crate::FlightUnion2::B(value) => (value).kind.clone(),
    } == spot_light_kind_constant)
    {
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

// Source: upstream/packages/lighting/src/lightAnalysis.ts:71 (sha256:f3284ace50e13a077193da5c92bbb65f54b209e05bdad6e028d15552213e2023)
pub fn get_light_influence_bounds(out: &mut BoundingSphereLike, light: &Light) -> () {
    if (!is_light_enabled(light)) {
        out.center.x = 0.0_f64;
        out.center.y = 0.0_f64;
        out.center.z = 0.0_f64;
        out.radius = 0.0_f64;
        return;
    }
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
                __flight_entity_snapshot: __flight_source.__flight_entity_snapshot.clone(),
                kind: (__flight_source.kind).clone(),
                casts_shadow: __flight_source.casts_shadow,
                color: __flight_source.color,
                decay: __flight_source.decay,
                direction: (__flight_source.direction).clone(),
                enabled: __flight_source.enabled,
                inner_cone_cos: __flight_source.inner_cone_cos,
                intensity: __flight_source.intensity,
                intensity_unit: (__flight_source.intensity_unit).clone(),
                layer_mask: __flight_source.layer_mask,
                priority: __flight_source.priority,
                normal_bias: __flight_source.normal_bias,
                outer_cone_cos: __flight_source.outer_cone_cos,
                pcf_radius: __flight_source.pcf_radius,
                position: (__flight_source.position).clone(),
                range: __flight_source.range,
                shadow_bias: __flight_source.shadow_bias,
                shadow_far: __flight_source.shadow_far,
                shadow_map_size: __flight_source.shadow_map_size,
                shadow_near: __flight_source.shadow_near,
                shadow_strength: __flight_source.shadow_strength,
                spot_blend: __flight_source.spot_blend,
                ground_color: __flight_source.ground_color,
                sky_color: __flight_source.sky_color,
                environment: (__flight_source.environment).clone(),
                cascade_count: __flight_source.cascade_count,
                cascade_splits: (__flight_source.cascade_splits).clone(),
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

// Source: upstream/packages/lighting/src/lightAnalysis.ts:122 (sha256:e60133687e555498e15313d47d265367850da1199294066261bac6c38ba1f7be)
pub fn get_light_luminance(light: &Light) -> f64 {
    if (!is_light_enabled(light)) {
        return 0.0_f64;
    }
    {
        let __switch_value = (light.kind).clone();
        let __flight_case = if __switch_value == ambient_light_kind_constant {
            0_usize
        } else if __switch_value == area_light_kind_constant {
            1_usize
        } else if __switch_value == directional_light_kind_constant {
            2_usize
        } else if __switch_value == environment_kind_constant {
            3_usize
        } else if __switch_value == hemisphere_light_kind_constant {
            4_usize
        } else if __switch_value == point_light_kind_constant {
            5_usize
        } else if __switch_value == spot_light_kind_constant {
            6_usize
        } else {
            7_usize
        };
        '__flight_switch: {
            if __flight_case <= 0_usize {
                {
                    let ambient = {
                        let __flight_source = &((*light).clone());
                        AmbientLight {
                            __flight_identity: std::sync::Arc::clone(
                                &__flight_source.__flight_identity,
                            ),
                            __flight_entity_runtime: std::sync::Arc::clone(
                                &__flight_source.__flight_entity_runtime,
                            ),
                            __flight_entity_snapshot: __flight_source
                                .__flight_entity_snapshot
                                .clone(),
                            kind: (__flight_source.kind).clone(),
                            casts_shadow: __flight_source.casts_shadow,
                            color: __flight_source.color,
                            decay: __flight_source.decay,
                            direction: (__flight_source.direction).clone(),
                            enabled: __flight_source.enabled,
                            inner_cone_cos: __flight_source.inner_cone_cos,
                            intensity: __flight_source.intensity,
                            intensity_unit: (__flight_source.intensity_unit).clone(),
                            layer_mask: __flight_source.layer_mask,
                            priority: __flight_source.priority,
                            normal_bias: __flight_source.normal_bias,
                            outer_cone_cos: __flight_source.outer_cone_cos,
                            pcf_radius: __flight_source.pcf_radius,
                            position: (__flight_source.position).clone(),
                            range: __flight_source.range,
                            shadow_bias: __flight_source.shadow_bias,
                            shadow_far: __flight_source.shadow_far,
                            shadow_map_size: __flight_source.shadow_map_size,
                            shadow_near: __flight_source.shadow_near,
                            shadow_strength: __flight_source.shadow_strength,
                            spot_blend: __flight_source.spot_blend,
                            ground_color: __flight_source.ground_color,
                            sky_color: __flight_source.sky_color,
                            environment: (__flight_source.environment).clone(),
                            cascade_count: __flight_source.cascade_count,
                            cascade_splits: (__flight_source.cascade_splits).clone(),
                            right: (__flight_source.right).clone(),
                            up: (__flight_source.up).clone(),
                            ..Default::default()
                        }
                    };
                    return (get_color_luminance(ambient.color) * ambient.intensity);
                }
            }
            if __flight_case <= 1_usize {
                {
                    let area = {
                        let __flight_source = &((*light).clone());
                        AreaLight {
                            __flight_identity: std::sync::Arc::clone(
                                &__flight_source.__flight_identity,
                            ),
                            __flight_entity_runtime: std::sync::Arc::clone(
                                &__flight_source.__flight_entity_runtime,
                            ),
                            __flight_entity_snapshot: __flight_source
                                .__flight_entity_snapshot
                                .clone(),
                            kind: (__flight_source.kind).clone(),
                            casts_shadow: __flight_source.casts_shadow,
                            color: __flight_source.color,
                            decay: __flight_source.decay,
                            direction: (__flight_source.direction).clone(),
                            enabled: __flight_source.enabled,
                            inner_cone_cos: __flight_source.inner_cone_cos,
                            intensity: __flight_source.intensity,
                            intensity_unit: (__flight_source.intensity_unit).clone(),
                            layer_mask: __flight_source.layer_mask,
                            priority: __flight_source.priority,
                            normal_bias: __flight_source.normal_bias,
                            outer_cone_cos: __flight_source.outer_cone_cos,
                            pcf_radius: __flight_source.pcf_radius,
                            position: (__flight_source.position).clone(),
                            range: __flight_source.range,
                            shadow_bias: __flight_source.shadow_bias,
                            shadow_far: __flight_source.shadow_far,
                            shadow_map_size: __flight_source.shadow_map_size,
                            shadow_near: __flight_source.shadow_near,
                            shadow_strength: __flight_source.shadow_strength,
                            spot_blend: __flight_source.spot_blend,
                            ground_color: __flight_source.ground_color,
                            sky_color: __flight_source.sky_color,
                            environment: (__flight_source.environment).clone(),
                            cascade_count: __flight_source.cascade_count,
                            cascade_splits: (__flight_source.cascade_splits).clone(),
                            right: (__flight_source.right).clone(),
                            up: (__flight_source.up).clone(),
                            ..Default::default()
                        }
                    };
                    return (get_color_luminance(area.color) * area.intensity);
                }
            }
            if __flight_case <= 2_usize {
                {
                    let directional = {
                        let __flight_source = &((*light).clone());
                        DirectionalLight {
                            __flight_identity: std::sync::Arc::clone(
                                &__flight_source.__flight_identity,
                            ),
                            __flight_entity_runtime: std::sync::Arc::clone(
                                &__flight_source.__flight_entity_runtime,
                            ),
                            __flight_entity_snapshot: __flight_source
                                .__flight_entity_snapshot
                                .clone(),
                            kind: (__flight_source.kind).clone(),
                            casts_shadow: __flight_source.casts_shadow,
                            color: __flight_source.color,
                            decay: __flight_source.decay,
                            direction: (__flight_source.direction).clone(),
                            enabled: __flight_source.enabled,
                            inner_cone_cos: __flight_source.inner_cone_cos,
                            intensity: __flight_source.intensity,
                            intensity_unit: (__flight_source.intensity_unit).clone(),
                            layer_mask: __flight_source.layer_mask,
                            priority: __flight_source.priority,
                            normal_bias: __flight_source.normal_bias,
                            outer_cone_cos: __flight_source.outer_cone_cos,
                            pcf_radius: __flight_source.pcf_radius,
                            position: (__flight_source.position).clone(),
                            range: __flight_source.range,
                            shadow_bias: __flight_source.shadow_bias,
                            shadow_far: __flight_source.shadow_far,
                            shadow_map_size: __flight_source.shadow_map_size,
                            shadow_near: __flight_source.shadow_near,
                            shadow_strength: __flight_source.shadow_strength,
                            spot_blend: __flight_source.spot_blend,
                            ground_color: __flight_source.ground_color,
                            sky_color: __flight_source.sky_color,
                            environment: (__flight_source.environment).clone(),
                            cascade_count: __flight_source.cascade_count,
                            cascade_splits: (__flight_source.cascade_splits).clone(),
                            right: (__flight_source.right).clone(),
                            up: (__flight_source.up).clone(),
                            ..Default::default()
                        }
                    };
                    return (get_color_luminance(directional.color) * directional.intensity);
                }
            }
            if __flight_case <= 3_usize {
                return 0.0_f64;
            }
            if __flight_case <= 4_usize {
                {
                    let hemisphere = {
                        let __flight_source = &((*light).clone());
                        HemisphereLight {
                            __flight_identity: std::sync::Arc::clone(
                                &__flight_source.__flight_identity,
                            ),
                            __flight_entity_runtime: std::sync::Arc::clone(
                                &__flight_source.__flight_entity_runtime,
                            ),
                            __flight_entity_snapshot: __flight_source
                                .__flight_entity_snapshot
                                .clone(),
                            kind: (__flight_source.kind).clone(),
                            casts_shadow: __flight_source.casts_shadow,
                            color: __flight_source.color,
                            decay: __flight_source.decay,
                            direction: (__flight_source.direction).clone(),
                            enabled: __flight_source.enabled,
                            inner_cone_cos: __flight_source.inner_cone_cos,
                            intensity: __flight_source.intensity,
                            intensity_unit: (__flight_source.intensity_unit).clone(),
                            layer_mask: __flight_source.layer_mask,
                            priority: __flight_source.priority,
                            normal_bias: __flight_source.normal_bias,
                            outer_cone_cos: __flight_source.outer_cone_cos,
                            pcf_radius: __flight_source.pcf_radius,
                            position: (__flight_source.position).clone(),
                            range: __flight_source.range,
                            shadow_bias: __flight_source.shadow_bias,
                            shadow_far: __flight_source.shadow_far,
                            shadow_map_size: __flight_source.shadow_map_size,
                            shadow_near: __flight_source.shadow_near,
                            shadow_strength: __flight_source.shadow_strength,
                            spot_blend: __flight_source.spot_blend,
                            ground_color: __flight_source.ground_color,
                            sky_color: __flight_source.sky_color,
                            environment: (__flight_source.environment).clone(),
                            cascade_count: __flight_source.cascade_count,
                            cascade_splits: (__flight_source.cascade_splits).clone(),
                            right: (__flight_source.right).clone(),
                            up: (__flight_source.up).clone(),
                            ..Default::default()
                        }
                    };
                    let mean_color_luminance = ((get_color_luminance(hemisphere.ground_color)
                        + get_color_luminance(hemisphere.sky_color))
                        * 0.5_f64);
                    return (mean_color_luminance * hemisphere.intensity);
                }
            }
            if __flight_case <= 5_usize {
                {
                    let point = {
                        let __flight_source = &((*light).clone());
                        PointLight {
                            __flight_identity: std::sync::Arc::clone(
                                &__flight_source.__flight_identity,
                            ),
                            __flight_entity_runtime: std::sync::Arc::clone(
                                &__flight_source.__flight_entity_runtime,
                            ),
                            __flight_entity_snapshot: __flight_source
                                .__flight_entity_snapshot
                                .clone(),
                            kind: (__flight_source.kind).clone(),
                            casts_shadow: __flight_source.casts_shadow,
                            color: __flight_source.color,
                            decay: __flight_source.decay,
                            direction: (__flight_source.direction).clone(),
                            enabled: __flight_source.enabled,
                            inner_cone_cos: __flight_source.inner_cone_cos,
                            intensity: __flight_source.intensity,
                            intensity_unit: (__flight_source.intensity_unit).clone(),
                            layer_mask: __flight_source.layer_mask,
                            priority: __flight_source.priority,
                            normal_bias: __flight_source.normal_bias,
                            outer_cone_cos: __flight_source.outer_cone_cos,
                            pcf_radius: __flight_source.pcf_radius,
                            position: (__flight_source.position).clone(),
                            range: __flight_source.range,
                            shadow_bias: __flight_source.shadow_bias,
                            shadow_far: __flight_source.shadow_far,
                            shadow_map_size: __flight_source.shadow_map_size,
                            shadow_near: __flight_source.shadow_near,
                            shadow_strength: __flight_source.shadow_strength,
                            spot_blend: __flight_source.spot_blend,
                            ground_color: __flight_source.ground_color,
                            sky_color: __flight_source.sky_color,
                            environment: (__flight_source.environment).clone(),
                            cascade_count: __flight_source.cascade_count,
                            cascade_splits: (__flight_source.cascade_splits).clone(),
                            right: (__flight_source.right).clone(),
                            up: (__flight_source.up).clone(),
                            ..Default::default()
                        }
                    };
                    return (get_color_luminance(point.color) * point.intensity);
                }
            }
            if __flight_case <= 6_usize {
                {
                    let spot = {
                        let __flight_source = &((*light).clone());
                        SpotLight {
                            __flight_identity: std::sync::Arc::clone(
                                &__flight_source.__flight_identity,
                            ),
                            __flight_entity_runtime: std::sync::Arc::clone(
                                &__flight_source.__flight_entity_runtime,
                            ),
                            __flight_entity_snapshot: __flight_source
                                .__flight_entity_snapshot
                                .clone(),
                            kind: (__flight_source.kind).clone(),
                            casts_shadow: __flight_source.casts_shadow,
                            color: __flight_source.color,
                            decay: __flight_source.decay,
                            direction: (__flight_source.direction).clone(),
                            enabled: __flight_source.enabled,
                            inner_cone_cos: __flight_source.inner_cone_cos,
                            intensity: __flight_source.intensity,
                            intensity_unit: (__flight_source.intensity_unit).clone(),
                            layer_mask: __flight_source.layer_mask,
                            priority: __flight_source.priority,
                            normal_bias: __flight_source.normal_bias,
                            outer_cone_cos: __flight_source.outer_cone_cos,
                            pcf_radius: __flight_source.pcf_radius,
                            position: (__flight_source.position).clone(),
                            range: __flight_source.range,
                            shadow_bias: __flight_source.shadow_bias,
                            shadow_far: __flight_source.shadow_far,
                            shadow_map_size: __flight_source.shadow_map_size,
                            shadow_near: __flight_source.shadow_near,
                            shadow_strength: __flight_source.shadow_strength,
                            spot_blend: __flight_source.spot_blend,
                            ground_color: __flight_source.ground_color,
                            sky_color: __flight_source.sky_color,
                            environment: (__flight_source.environment).clone(),
                            cascade_count: __flight_source.cascade_count,
                            cascade_splits: (__flight_source.cascade_splits).clone(),
                            right: (__flight_source.right).clone(),
                            up: (__flight_source.up).clone(),
                            ..Default::default()
                        }
                    };
                    return (get_color_luminance(spot.color) * spot.intensity);
                }
            }
            if __flight_case <= 7_usize {
                return 0.0_f64;
            }
            unreachable!("exhaustive TypeScript switch completed without returning");
        }
    }
}

// Source: upstream/packages/lighting/src/lightAnalysis.ts:163 (sha256:e57ab4b0219f16ed73c72185ff32cabba6856d0c2089e5ac5520e47a917f3a86)
pub fn has_light_influence_on_bounds(light: &Light, bounds: &BoundingSphereLike) -> bool {
    if (!is_light_enabled(light)) {
        return false;
    }
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
            __flight_entity_snapshot: __flight_source.__flight_entity_snapshot.clone(),
            kind: (__flight_source.kind).clone(),
            casts_shadow: __flight_source.casts_shadow,
            color: __flight_source.color,
            decay: __flight_source.decay,
            direction: (__flight_source.direction).clone(),
            enabled: __flight_source.enabled,
            inner_cone_cos: __flight_source.inner_cone_cos,
            intensity: __flight_source.intensity,
            intensity_unit: (__flight_source.intensity_unit).clone(),
            layer_mask: __flight_source.layer_mask,
            priority: __flight_source.priority,
            normal_bias: __flight_source.normal_bias,
            outer_cone_cos: __flight_source.outer_cone_cos,
            pcf_radius: __flight_source.pcf_radius,
            position: (__flight_source.position).clone(),
            range: __flight_source.range,
            shadow_bias: __flight_source.shadow_bias,
            shadow_far: __flight_source.shadow_far,
            shadow_map_size: __flight_source.shadow_map_size,
            shadow_near: __flight_source.shadow_near,
            shadow_strength: __flight_source.shadow_strength,
            spot_blend: __flight_source.spot_blend,
            ground_color: __flight_source.ground_color,
            sky_color: __flight_source.sky_color,
            environment: (__flight_source.environment).clone(),
            cascade_count: __flight_source.cascade_count,
            cascade_splits: (__flight_source.cascade_splits).clone(),
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

// Source: upstream/packages/lighting/src/lightAnalysis.ts:199 (sha256:ef1199462263b7ba0b5136e4c0abd3768212cf71989f9993e33d05ca114b339f)
pub fn is_light_casting_shadow(light: &Light) -> bool {
    if (!is_light_enabled(light)) {
        return false;
    }
    let kind = (light.kind).clone();
    if ((kind == ambient_light_kind_constant) || (kind == hemisphere_light_kind_constant))
        || (kind == environment_kind_constant)
    {
        return false;
    }
    return (true) && (light.casts_shadow == true);
}

// Source: upstream/packages/lighting/src/lightAnalysis.ts:210 (sha256:e9b7403d06429f2ff5aa848a51719301deb78022fae3dc6b39e96c48cc2ab7fc)
fn is_light_enabled(light: &Light) -> bool {
    return (!true) || (light.enabled != false);
}

// Source: upstream/packages/lighting/src/lightAnalysis.ts:214 (sha256:e9f211c4258a59ade165bc86e405004b7cc4e74f1771a74d8f92ca749567acc4)
fn smoothstep(edge0: f64, edge1: f64, value: f64) -> f64 {
    if (edge0 == edge1) {
        return if (value < edge0) { 0.0_f64 } else { 1.0_f64 };
    }
    let t = (0.0_f64).max((1.0_f64).min(((value - edge0) / (edge1 - edge0))));
    return ((t * t) * (3.0_f64 - (2.0_f64 * t)));
}
