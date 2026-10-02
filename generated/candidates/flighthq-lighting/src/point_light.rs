// @generated from upstream/packages/lighting/src/pointLight.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_entity::{allocate_entity, finish_entity};
use flighthq_geometry::{clone_vector3, create_vector3};
use flighthq_types::{
    ALL_LIGHT_LAYERS as all_light_layers_constant, EntityConstruction,
    POINT_LIGHT_KIND as point_light_kind_constant, PointLight, PointLightOptions,
    UNITLESS_LIGHT_UNIT as unitless_light_unit_constant, Vector3Like,
};

// Source: upstream/packages/lighting/src/pointLight.ts:7 (sha256:c070224fb7b8495447d912afee311233b3514d6996753ca53e24f409598ce573)
pub fn clone_point_light(source: &PointLight) -> PointLight {
    let mut out = allocate_entity();
    crate::host_set("host.castsShadow", source.casts_shadow);
    crate::host_set("host.color", source.color);
    crate::host_set("host.decay", source.decay);
    crate::host_set("host.enabled", source.enabled);
    crate::host_set("host.intensity", source.intensity);
    crate::host_set("host.intensityUnit", (source.intensity_unit).clone());
    crate::host_set("host.layerMask", source.layer_mask);
    crate::host_set("host.priority", source.priority);
    crate::host_set("host.kind", point_light_kind_constant);
    crate::host_set("host.normalBias", source.normal_bias);
    crate::host_set("host.pcfRadius", source.pcf_radius);
    crate::host_set(
        "host.position",
        clone_vector3(&{
            let __flight_source = &(source.position);
            Vector3Like {
                __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
                __flight_entity_runtime: std::sync::Arc::clone(
                    &__flight_source.__flight_entity_runtime,
                ),
                __flight_entity_snapshot: __flight_source.__flight_entity_snapshot.clone(),
                x: __flight_source.x,
                y: __flight_source.y,
                z: __flight_source.z,
            }
        }),
    );
    crate::host_set("host.range", source.range);
    crate::host_set("host.shadowBias", source.shadow_bias);
    crate::host_set("host.shadowFar", source.shadow_far);
    crate::host_set("host.shadowMapSize", source.shadow_map_size);
    crate::host_set("host.shadowNear", source.shadow_near);
    crate::host_set("host.shadowStrength", source.shadow_strength);
    return finish_entity((out).clone());
}

// Source: upstream/packages/lighting/src/pointLight.ts:30 (sha256:247a5e229c6578310f7ff6bbe20eb2c8be317d9d2b852523ecc6be86c8815f45)
pub fn create_point_light(options: Option<PointLightOptions>) -> PointLight {
    let mut out = allocate_entity();
    initialize_point_light((out).clone(), ((options).clone()).clone());
    return finish_entity((out).clone());
}

// Source: upstream/packages/lighting/src/pointLight.ts:39 (sha256:5a31f959cc27693fc55e35d3275765fb6ef250f138af2f5dd389ef90cce50abc)
pub fn initialize_point_light(
    out: EntityConstruction<PointLight>,
    options: Option<PointLightOptions>,
) -> () {
    let position = options.as_ref().and_then(|value| (value.position).clone());
    crate::host_set(
        "host.castsShadow",
        (options.as_ref().and_then(|value| value.casts_shadow)).unwrap_or(false),
    );
    crate::host_set(
        "host.color",
        (options.as_ref().and_then(|value| value.color)).unwrap_or(4294967295.0_f64),
    );
    crate::host_set(
        "host.decay",
        (options.as_ref().and_then(|value| value.decay)).unwrap_or(2.0_f64),
    );
    crate::host_set(
        "host.enabled",
        (options.as_ref().and_then(|value| value.enabled)).unwrap_or(true),
    );
    crate::host_set(
        "host.intensity",
        (options.as_ref().and_then(|value| value.intensity)).unwrap_or(1.0_f64),
    );
    crate::host_set(
        "host.intensityUnit",
        (options
            .as_ref()
            .and_then(|value| (value.intensity_unit).clone()))
        .unwrap_or((unitless_light_unit_constant).to_owned()),
    );
    crate::host_set(
        "host.layerMask",
        (options.as_ref().and_then(|value| value.layer_mask)).unwrap_or(all_light_layers_constant),
    );
    crate::host_set(
        "host.priority",
        (options.as_ref().and_then(|value| value.priority)).unwrap_or(0.0_f64),
    );
    crate::host_set("host.kind", point_light_kind_constant);
    crate::host_set(
        "host.normalBias",
        (options.as_ref().and_then(|value| value.normal_bias)).unwrap_or(0.0_f64),
    );
    crate::host_set(
        "host.pcfRadius",
        (options.as_ref().and_then(|value| value.pcf_radius)).unwrap_or(0.0_f64),
    );
    crate::host_set(
        "host.position",
        if (position).is_some() {
            clone_vector3(&position.as_ref().unwrap())
        } else {
            create_vector3(Some(0.0_f64), Some(0.0_f64), Some(0.0_f64))
        },
    );
    crate::host_set(
        "host.range",
        (options.as_ref().and_then(|value| value.range)).unwrap_or((-1.0_f64)),
    );
    crate::host_set(
        "host.shadowBias",
        (options.as_ref().and_then(|value| value.shadow_bias)).unwrap_or(0.0_f64),
    );
    crate::host_set(
        "host.shadowFar",
        (options.as_ref().and_then(|value| value.shadow_far)).unwrap_or(500.0_f64),
    );
    crate::host_set(
        "host.shadowMapSize",
        (options.as_ref().and_then(|value| value.shadow_map_size)).unwrap_or(1024.0_f64),
    );
    crate::host_set(
        "host.shadowNear",
        (options.as_ref().and_then(|value| value.shadow_near)).unwrap_or(0.5_f64),
    );
    crate::host_set(
        "host.shadowStrength",
        (options.as_ref().and_then(|value| value.shadow_strength)).unwrap_or(1.0_f64),
    );
}
