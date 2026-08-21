// @generated from upstream/packages/lighting/src/sceneForwardLights.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::get_light_contribution_at_bounding_sphere;
use flighthq_types::{
    BoundingSphereLike, MAX_FORWARD_LIGHTS as max_forward_lights_constant, PointLight,
    Scene3DForwardLightSelection, Scene3DLightsLike, SpotLight,
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

// Source: upstream/packages/lighting/src/sceneForwardLights.ts:20 (sha256:7d1e98287c7aadf7cb5571b4d34ea8c75615e2001f56a6182a8b6654b7258ce4)
pub fn select_scene3_d_forward_lights(
    out: &mut Scene3DForwardLightSelection,
    lights: &Scene3DLightsLike,
    bounds: &BoundingSphereLike,
) -> () {
    let points = (lights.point).clone();
    let spots = (lights.spot).clone();
    let point_count = select_strongest_lights(
        &(((points).clone()).as_ref().map(|__flight_value| {
            (__flight_value)
                .iter()
                .map(|__flight_value| {
                    crate::FlightUnion2::<PointLight, SpotLight>::A((__flight_value).clone())
                })
                .collect::<Vec<_>>()
        })),
        bounds,
        &mut (*SCRATCH_SELECTED_POINT_LIGHTS.lock().unwrap()),
        &mut (*SCRATCH_SELECTED_POINT_INDICES.lock().unwrap()),
        &mut (*SCRATCH_SELECTED_POINT_SCORES.lock().unwrap()),
    );
    let spot_count = select_strongest_lights(
        &(((spots).clone()).as_ref().map(|__flight_value| {
            (__flight_value)
                .iter()
                .map(|__flight_value| {
                    crate::FlightUnion2::<PointLight, SpotLight>::B((__flight_value).clone())
                })
                .collect::<Vec<_>>()
        })),
        bounds,
        &mut (*SCRATCH_SELECTED_SPOT_LIGHTS.lock().unwrap()),
        &mut (*SCRATCH_SELECTED_SPOT_INDICES.lock().unwrap()),
        &mut (*SCRATCH_SELECTED_SPOT_SCORES.lock().unwrap()),
    );
    out.indices.clear();
    out.point.clear();
    out.spot.clear();
    {
        let mut i = 0.0_f64;
        while (i < point_count) {
            out.indices
                .push(((*SCRATCH_SELECTED_POINT_INDICES.lock().unwrap())[i as usize] as f64));
            out.point.push(
                match (*SCRATCH_SELECTED_POINT_LIGHTS.lock().unwrap())[i as usize].clone() {
                    crate::FlightUnion2::A(value) => value,
                    crate::FlightUnion2::B(_) => panic!("TypeScript union narrowing failed"),
                },
            );
            {
                i += 1.0;
                i
            };
        }
    }
    {
        let mut i = 0.0_f64;
        while (i < spot_count) {
            out.indices.push(
                (!__flight_js_to_i32(
                    ((*SCRATCH_SELECTED_SPOT_INDICES.lock().unwrap())[i as usize] as f64),
                )) as f64,
            );
            out.spot.push(
                match (*SCRATCH_SELECTED_SPOT_LIGHTS.lock().unwrap())[i as usize].clone() {
                    crate::FlightUnion2::A(_) => panic!("TypeScript union narrowing failed"),
                    crate::FlightUnion2::B(value) => value,
                },
            );
            {
                i += 1.0;
                i
            };
        }
    }
}

// Source: upstream/packages/lighting/src/sceneForwardLights.ts:60 (sha256:fd97be8ed89a42f395876416904eb7f34f23d5cb72d28a98749e106818a42f44)
fn select_strongest_lights(
    lights: &Option<Vec<crate::FlightUnion2<PointLight, SpotLight>>>,
    bounds: &BoundingSphereLike,
    selected_lights: &mut Vec<crate::FlightUnion2<PointLight, SpotLight>>,
    selected_indices: &mut Vec<i32>,
    selected_scores: &mut Vec<f64>,
) -> f64 {
    let mut selected_count = 0.0_f64;
    if (lights).is_none() {
        return selected_count;
    }
    {
        let mut input_index = 0.0_f64;
        while (input_index < (lights.as_ref().unwrap().len() as f64)) {
            let light = lights.as_ref().unwrap()[input_index as usize].clone();
            let score = get_light_contribution_at_bounding_sphere(&(light), bounds);
            if (!(score > 0.0_f64)) {
                {
                    input_index += 1.0;
                    input_index
                };
                continue;
            }
            let mut insert_at = selected_count;
            while (insert_at > 0.0_f64) {
                let previous = (insert_at - 1.0_f64);
                if (score < (selected_scores[previous as usize] as f64)) {
                    break;
                }
                if (score == (selected_scores[previous as usize] as f64))
                    && (input_index > (selected_indices[previous as usize] as f64))
                {
                    break;
                }
                {
                    insert_at -= 1.0;
                    insert_at
                };
            }
            if (insert_at >= max_forward_lights_constant) {
                {
                    input_index += 1.0;
                    input_index
                };
                continue;
            }
            let next_count = (selected_count + 1.0_f64).min(max_forward_lights_constant);
            {
                let mut i = (next_count - 1.0_f64);
                while (i > insert_at) {
                    {
                        let __flight_index = (i) as usize;
                        let __flight_value = selected_lights[(i - 1.0_f64) as usize].clone();
                        if __flight_index == selected_lights.len() {
                            selected_lights.push(__flight_value);
                        } else {
                            selected_lights[__flight_index] = __flight_value;
                        }
                    };
                    selected_indices[i as usize] =
                        (selected_indices[(i - 1.0_f64) as usize] as f64) as i32;
                    selected_scores[i as usize] = (selected_scores[(i - 1.0_f64) as usize] as f64);
                    {
                        i -= 1.0;
                        i
                    };
                }
            }
            {
                let __flight_index = (insert_at) as usize;
                let __flight_value = (light).clone();
                if __flight_index == selected_lights.len() {
                    selected_lights.push(__flight_value);
                } else {
                    selected_lights[__flight_index] = __flight_value;
                }
            };
            selected_indices[insert_at as usize] = (input_index) as i32;
            selected_scores[insert_at as usize] = score;
            selected_count = next_count;
            {
                input_index += 1.0;
                input_index
            };
        }
    }
    return selected_count;
}

// Source: upstream/packages/lighting/src/sceneForwardLights.ts:98 (sha256:0473f8996d13e3550bf034d640f910ef362b8acc62e26ad59a1e2a665436199a)
static SCRATCH_SELECTED_POINT_INDICES: std::sync::LazyLock<std::sync::Mutex<Vec<i32>>> =
    std::sync::LazyLock::new(|| {
        std::sync::Mutex::new(vec![0_i32; (max_forward_lights_constant) as usize])
    });

// Source: upstream/packages/lighting/src/sceneForwardLights.ts:99 (sha256:5837c4e1310970d81d33cd5e3d34f229d99a50c8d037068195c03abf8a0043ec)
static SCRATCH_SELECTED_POINT_LIGHTS: std::sync::LazyLock<
    std::sync::Mutex<Vec<crate::FlightUnion2<PointLight, SpotLight>>>,
> = std::sync::LazyLock::new(|| {
    std::sync::Mutex::new(vec![
        Default::default();
        (max_forward_lights_constant) as usize
    ])
});

// Source: upstream/packages/lighting/src/sceneForwardLights.ts:100 (sha256:fc8cf506edca0b1037107b8479845fe65395fc7a0bf1657bfafb2420522a498d)
static SCRATCH_SELECTED_POINT_SCORES: std::sync::LazyLock<std::sync::Mutex<Vec<f64>>> =
    std::sync::LazyLock::new(|| {
        std::sync::Mutex::new(vec![0.0_f64; (max_forward_lights_constant) as usize])
    });

// Source: upstream/packages/lighting/src/sceneForwardLights.ts:101 (sha256:3389bb4ee7278c1577f0d1b6c2515868435c5cbe98c79b64b47f9f6103c90f81)
static SCRATCH_SELECTED_SPOT_INDICES: std::sync::LazyLock<std::sync::Mutex<Vec<i32>>> =
    std::sync::LazyLock::new(|| {
        std::sync::Mutex::new(vec![0_i32; (max_forward_lights_constant) as usize])
    });

// Source: upstream/packages/lighting/src/sceneForwardLights.ts:102 (sha256:890c1c70b58e41926b9e32be7a7e63d00bbeb610d957a6899e72618cfa231ae3)
static SCRATCH_SELECTED_SPOT_LIGHTS: std::sync::LazyLock<
    std::sync::Mutex<Vec<crate::FlightUnion2<PointLight, SpotLight>>>,
> = std::sync::LazyLock::new(|| {
    std::sync::Mutex::new(vec![
        Default::default();
        (max_forward_lights_constant) as usize
    ])
});

// Source: upstream/packages/lighting/src/sceneForwardLights.ts:103 (sha256:134d85830c209bf0f6e1998bbd3e82cb06dcbff26bbb579dddfeedea4cd88b9a)
static SCRATCH_SELECTED_SPOT_SCORES: std::sync::LazyLock<std::sync::Mutex<Vec<f64>>> =
    std::sync::LazyLock::new(|| {
        std::sync::Mutex::new(vec![0.0_f64; (max_forward_lights_constant) as usize])
    });
