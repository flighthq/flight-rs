// @generated from upstream/packages/particles/src/validateParticleEmitterConfig.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::create_particle_emitter_config;
use flighthq_types::{ParticleConfigIssue, ParticleEmitterConfig};

// Source: upstream/packages/particles/src/validateParticleEmitterConfig.ts:7 (sha256:a63f7598f4378b6e361fbcca3aaac29501875667fdce4035b9c61271311bf5ce)
static NUMERIC_FIELDS: std::sync::LazyLock<Vec<String>> = std::sync::LazyLock::new(|| {
    vec![
        "alphaEnd",
        "alphaStart",
        "burstCount",
        "burstInterval",
        "duration",
        "colorEndB",
        "colorEndG",
        "colorEndR",
        "colorEndVarianceB",
        "colorEndVarianceG",
        "colorEndVarianceR",
        "colorStartB",
        "colorStartG",
        "colorStartR",
        "colorStartVarianceB",
        "colorStartVarianceG",
        "colorStartVarianceR",
        "directionX",
        "directionY",
        "directionZ",
        "emitterConeAngle",
        "emitterDepth",
        "emitterHeight",
        "emitterRadius",
        "emitterWidth",
        "frameCount",
        "frameRate",
        "gravityX",
        "gravityY",
        "gravityZ",
        "lifetimeMax",
        "lifetimeMin",
        "maxParticles",
        "regionIdMax",
        "regionIdMin",
        "rotationSpeedMax",
        "rotationSpeedMin",
        "scaleEnd",
        "scaleMax",
        "scaleMin",
        "speedMax",
        "speedMin",
        "spawnRate",
        "spread",
        "velocityInheritance",
    ]
});

// Source: upstream/packages/particles/src/validateParticleEmitterConfig.ts:57 (sha256:882c6aa6341b8ae3425c504eab69c6aa411613da8b8353bd2de8d45d555bb66e)
static NON_NEGATIVE_FIELDS: std::sync::LazyLock<Vec<String>> = std::sync::LazyLock::new(|| {
    vec![
        "burstCount",
        "burstInterval",
        "duration",
        "emitterConeAngle",
        "emitterDepth",
        "emitterHeight",
        "emitterRadius",
        "emitterWidth",
        "frameRate",
        "lifetimeMin",
        "lifetimeMax",
        "maxParticles",
        "scaleMax",
        "scaleMin",
        "speedMax",
        "speedMin",
        "spawnRate",
    ]
});

// Source: upstream/packages/particles/src/validateParticleEmitterConfig.ts:83 (sha256:458e7d414d06d6f2468cb586896b90136b8aac0c5adb8a169f38b31fb14c9b22)
pub fn normalize_particle_emitter_config(
    config: Option<crate::particle_emitter_config::FlightPartialRecord603063107>,
) -> ParticleEmitterConfig {
    let mut out =
        create_particle_emitter_config(((config).clone()).as_ref().map(|__flight_value| {
            let __flight_source = &(__flight_value);
            crate::particle_emitter_config::FlightPartialRecord603063107 {
                __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
                alpha_end: __flight_source.alpha_end,
                alpha_start: __flight_source.alpha_start,
                blend_mode: (__flight_source.blend_mode).clone(),
                color_end_b: __flight_source.color_end_b,
                color_end_g: __flight_source.color_end_g,
                color_end_r: __flight_source.color_end_r,
                color_end_variance_b: __flight_source.color_end_variance_b,
                color_end_variance_g: __flight_source.color_end_variance_g,
                color_end_variance_r: __flight_source.color_end_variance_r,
                color_start_b: __flight_source.color_start_b,
                color_start_g: __flight_source.color_start_g,
                color_start_r: __flight_source.color_start_r,
                color_start_variance_b: __flight_source.color_start_variance_b,
                color_start_variance_g: __flight_source.color_start_variance_g,
                color_start_variance_r: __flight_source.color_start_variance_r,
                direction_x: __flight_source.direction_x,
                direction_y: __flight_source.direction_y,
                direction_z: __flight_source.direction_z,
                gravity_x: __flight_source.gravity_x,
                gravity_y: __flight_source.gravity_y,
                gravity_z: __flight_source.gravity_z,
                emitter_cone_angle: __flight_source.emitter_cone_angle,
                emitter_depth: __flight_source.emitter_depth,
                emitter_height: __flight_source.emitter_height,
                emitter_radius: __flight_source.emitter_radius,
                emitter_shape: (__flight_source.emitter_shape).clone(),
                emitter_width: __flight_source.emitter_width,
                burst_count: __flight_source.burst_count,
                burst_interval: __flight_source.burst_interval,
                duration: __flight_source.duration,
                loop_: __flight_source.loop_,
                frame_count: __flight_source.frame_count,
                frame_rate: __flight_source.frame_rate,
                lifetime_max: __flight_source.lifetime_max,
                lifetime_min: __flight_source.lifetime_min,
                max_particles: __flight_source.max_particles,
                region_id_max: __flight_source.region_id_max,
                region_id_min: __flight_source.region_id_min,
                scale_end: __flight_source.scale_end,
                scale_max: __flight_source.scale_max,
                scale_min: __flight_source.scale_min,
                speed_max: __flight_source.speed_max,
                speed_min: __flight_source.speed_min,
                spawn_rate: __flight_source.spawn_rate,
                spread: __flight_source.spread,
                rotation_speed_max: __flight_source.rotation_speed_max,
                rotation_speed_min: __flight_source.rotation_speed_min,
                velocity_inheritance: __flight_source.velocity_inheritance,
                alpha_curve: (__flight_source.alpha_curve).clone(),
                color_curve: (__flight_source.color_curve).clone(),
                scale_curve: (__flight_source.scale_curve).clone(),
                world_space: __flight_source.world_space,
            }
        }));
    let defaults = create_particle_emitter_config(None);
    let mut mutable = crate::host_value::<Vec<(String, f64)>>("host.cast");
    let defaults_rec = crate::host_value::<Vec<(String, f64)>>("host.cast");
    for field in ((*NUMERIC_FIELDS).clone()).iter().cloned() {
        if (!((mutable
            .iter()
            .find(|(entry_key, _)| entry_key == &(field).clone())
            .map(|(_, value)| value.clone()))
        .expect("TypeScript Record key was absent"))
        .is_finite())
        {
            {
                let __flight_key = (field).clone();
                let __flight_value = (defaults_rec
                    .iter()
                    .find(|(entry_key, _)| entry_key == &(field).clone())
                    .map(|(_, value)| value.clone()))
                .expect("TypeScript Record key was absent");
                if let Some((_, value)) = mutable.iter_mut().find(|(key, _)| key == &__flight_key) {
                    *value = __flight_value;
                } else {
                    mutable.push((__flight_key, __flight_value));
                }
            };
        }
    }
    return {
        let __flight_spread_0 = (out).clone();
        ParticleEmitterConfig {
            __flight_identity: std::sync::Arc::new(()),
            alpha_end: __flight_spread_0.alpha_end,
            alpha_start: __flight_spread_0.alpha_start,
            blend_mode: (__flight_spread_0.blend_mode).clone(),
            color_end_b: __flight_spread_0.color_end_b,
            color_end_g: __flight_spread_0.color_end_g,
            color_end_r: __flight_spread_0.color_end_r,
            color_end_variance_b: __flight_spread_0.color_end_variance_b,
            color_end_variance_g: __flight_spread_0.color_end_variance_g,
            color_end_variance_r: __flight_spread_0.color_end_variance_r,
            color_start_b: __flight_spread_0.color_start_b,
            color_start_g: __flight_spread_0.color_start_g,
            color_start_r: __flight_spread_0.color_start_r,
            color_start_variance_b: __flight_spread_0.color_start_variance_b,
            color_start_variance_g: __flight_spread_0.color_start_variance_g,
            color_start_variance_r: __flight_spread_0.color_start_variance_r,
            direction_x: __flight_spread_0.direction_x,
            direction_y: __flight_spread_0.direction_y,
            direction_z: __flight_spread_0.direction_z,
            gravity_x: __flight_spread_0.gravity_x,
            gravity_y: __flight_spread_0.gravity_y,
            gravity_z: __flight_spread_0.gravity_z,
            emitter_cone_angle: (0.0_f64).max(out.emitter_cone_angle),
            emitter_depth: (0.0_f64).max(out.emitter_depth),
            emitter_height: (0.0_f64).max(out.emitter_height),
            emitter_radius: (0.0_f64).max(out.emitter_radius),
            emitter_shape: (__flight_spread_0.emitter_shape).clone(),
            emitter_width: (0.0_f64).max(out.emitter_width),
            burst_count: (0.0_f64).max((out.burst_count).floor()),
            burst_interval: (0.0_f64).max(out.burst_interval),
            duration: (0.0_f64).max(out.duration),
            loop_: __flight_spread_0.loop_,
            frame_count: (1.0_f64).max((out.frame_count).floor()),
            frame_rate: (0.0_f64).max(out.frame_rate),
            lifetime_max: (0.0_f64).max(out.lifetime_max),
            lifetime_min: (0.0_f64).max(out.lifetime_min),
            max_particles: (0.0_f64).max((out.max_particles).floor()),
            region_id_max: ((0.0_f64).max((out.region_id_min).floor()))
                .max((out.region_id_max).floor()),
            region_id_min: (0.0_f64).max((out.region_id_min).floor()),
            scale_end: __flight_spread_0.scale_end,
            scale_max: (0.0_f64).max(out.scale_max),
            scale_min: (0.0_f64).max(out.scale_min),
            speed_max: (0.0_f64).max(out.speed_max),
            speed_min: (0.0_f64).max(out.speed_min),
            spawn_rate: (0.0_f64).max(out.spawn_rate),
            spread: __flight_spread_0.spread,
            rotation_speed_max: __flight_spread_0.rotation_speed_max,
            rotation_speed_min: __flight_spread_0.rotation_speed_min,
            velocity_inheritance: __flight_spread_0.velocity_inheritance,
            alpha_curve: if is_finite_curve(&(out.alpha_curve)) {
                (out.alpha_curve).clone()
            } else {
                None
            },
            color_curve: if is_finite_curve(&(out.color_curve)) {
                (out.color_curve).clone()
            } else {
                None
            },
            scale_curve: if is_finite_curve(&(out.scale_curve)) {
                (out.scale_curve).clone()
            } else {
                None
            },
            world_space: __flight_spread_0.world_space,
        }
    };
}

// Source: upstream/packages/particles/src/validateParticleEmitterConfig.ts:128 (sha256:a18a08a4eee98a57f847bc012bb29a248fb41fa6234d854be5820852f66068d0)
#[derive(Clone, Default)]
struct ValidateParticleEmitterConfigRecord2 {
    __flight_identity: std::sync::Arc<()>,
    field: String,
    message: String,
    severity: String,
}
impl PartialEq for ValidateParticleEmitterConfigRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn validate_particle_emitter_config(
    config: &ParticleEmitterConfig,
) -> Vec<ParticleConfigIssue> {
    let mut issues: Vec<ParticleConfigIssue> = vec![];
    for field in ((*NUMERIC_FIELDS).clone()).iter().cloned() {
        let value = config[(field).clone() as usize].clone();
        if (!(value).is_finite()) {
            issues.push(ParticleConfigIssue {
                __flight_identity: std::sync::Arc::new(()),
                field: (field).clone(),
                message: format!("{} must be a finite number (got {})", (field).clone(), {
                    let __flight_value = value;
                    crate::flight_value_to_string(&__flight_value)
                }),
                severity: "error".to_owned(),
            });
        }
    }
    for field in ((*NON_NEGATIVE_FIELDS).clone()).iter().cloned() {
        let value = config[(field).clone() as usize].clone();
        if ((value).is_finite()) && (value < 0.0_f64) {
            issues.push(ParticleConfigIssue {
                __flight_identity: std::sync::Arc::new(()),
                field: (field).clone(),
                message: format!("{} must not be negative (got {})", (field).clone(), value),
                severity: "warning".to_owned(),
            });
        }
    }
    if ((config.lifetime_max).is_finite()) && (config.lifetime_max <= 0.0_f64) {
        issues.push(ParticleConfigIssue {
            __flight_identity: std::sync::Arc::new(()),
            field: "lifetimeMax",
            message: "lifetimeMax must be > 0 or particles die instantly".to_owned(),
            severity: "warning".to_owned(),
        });
    }
    if ((config.max_particles).is_finite()) && (config.max_particles <= 0.0_f64) {
        issues.push(ParticleConfigIssue {
            __flight_identity: std::sync::Arc::new(()),
            field: "maxParticles",
            message: "maxParticles must be >= 1 or nothing ever spawns".to_owned(),
            severity: "warning".to_owned(),
        });
    }
    if ((config.frame_count).is_finite()) && (config.frame_count < 1.0_f64) {
        issues.push(ParticleConfigIssue {
            __flight_identity: std::sync::Arc::new(()),
            field: "frameCount",
            message: "frameCount must be >= 1".to_owned(),
            severity: "warning".to_owned(),
        });
    }
    report_inverted_range(&mut issues, config, &"lifetimeMin", &"lifetimeMax");
    report_inverted_range(&mut issues, config, &"speedMin", &"speedMax");
    report_inverted_range(&mut issues, config, &"scaleMin", &"scaleMax");
    report_inverted_range(
        &mut issues,
        config,
        &"rotationSpeedMin",
        &"rotationSpeedMax",
    );
    report_unit_range(&mut issues, config, &"alphaStart");
    report_unit_range(&mut issues, config, &"alphaEnd");
    report_curve(&mut issues, &(config.alpha_curve), &"alphaCurve", 1.0_f64);
    report_curve(&mut issues, &(config.color_curve), &"colorCurve", 3.0_f64);
    report_curve(&mut issues, &(config.scale_curve), &"scaleCurve", 1.0_f64);
    return issues;
}

// Source: upstream/packages/particles/src/validateParticleEmitterConfig.ts:182 (sha256:240fb75be0e4ecb4ddb72582c461f9119053b2a7ae5ebc33752483d6703ed0a0)
fn is_finite_curve(curve: &Option<Vec<f64>>) -> bool {
    if ((curve).is_none()) || ((curve.as_ref().unwrap().len() as f64) == 0.0_f64) {
        return false;
    }
    {
        let mut i = 0.0_f64;
        while (i < (curve.as_ref().unwrap().len() as f64)) {
            if (!(curve.as_ref().unwrap()[i as usize].clone()).is_finite()) {
                return false;
            }
            {
                i += 1.0;
                i
            };
        }
    }
    return true;
}

// Source: upstream/packages/particles/src/validateParticleEmitterConfig.ts:190 (sha256:ec6018485ce7aa3befb4ddf9c80567fe1a597599f8ec09c769e6aa16b82c0bf7)
fn report_curve(
    issues: &mut Vec<ParticleConfigIssue>,
    curve: &Option<Vec<f64>>,
    field: &ParticleEmitterConfig,
    stride: f64,
) -> () {
    if (curve).is_none() {
        return;
    }
    if ((curve.as_ref().unwrap().len() as f64) == 0.0_f64) {
        issues.push(ParticleConfigIssue {
            __flight_identity: std::sync::Arc::new(()),
            field: (*field).clone(),
            message: format!("{} is empty and will be ignored", (*field).clone()),
            severity: "warning".to_owned(),
        });
        return;
    }
    if (((curve.as_ref().unwrap().len() as f64) % stride) != 0.0_f64) {
        issues.push(ParticleConfigIssue {
            __flight_identity: std::sync::Arc::new(()),
            field: (*field).clone(),
            message: format!(
                "{} length ({}) is not a multiple of {}",
                (*field).clone(),
                (curve.as_ref().unwrap().len() as f64),
                stride
            ),
            severity: "warning".to_owned(),
        });
    }
    {
        let mut i = 0.0_f64;
        while (i < (curve.as_ref().unwrap().len() as f64)) {
            if (!(curve.as_ref().unwrap()[i as usize].clone()).is_finite()) {
                issues.push(ParticleConfigIssue {
                    __flight_identity: std::sync::Arc::new(()),
                    field: (*field).clone(),
                    message: format!(
                        "{} contains a non-finite sample at index {}",
                        (*field).clone(),
                        i
                    ),
                    severity: "error".to_owned(),
                });
                break;
            }
            {
                i += 1.0;
                i
            };
        }
    }
}

// Source: upstream/packages/particles/src/validateParticleEmitterConfig.ts:216 (sha256:56764c6b04aae3b47f34da76e9fddd0519876f809968345bdcfaae0de998e7c2)
fn report_inverted_range(
    issues: &mut Vec<ParticleConfigIssue>,
    config: &ParticleEmitterConfig,
    min_field: &ParticleEmitterConfig,
    max_field: &ParticleEmitterConfig,
) -> () {
    let min = config[min_field as usize].clone();
    let max = config[max_field as usize].clone();
    if (((min).is_finite()) && ((max).is_finite())) && (min > max) {
        issues.push(ParticleConfigIssue {
            __flight_identity: std::sync::Arc::new(()),
            field: (*min_field).clone(),
            message: format!(
                "{} ({}) is greater than {} ({})",
                (*min_field).clone(),
                min,
                (*max_field).clone(),
                max
            ),
            severity: "warning".to_owned(),
        });
    }
}

// Source: upstream/packages/particles/src/validateParticleEmitterConfig.ts:233 (sha256:fc726b54cf08ed2f9aecdd12527048ee8ef87cbbf792f07dae2d890bc95031d8)
fn report_unit_range(
    issues: &mut Vec<ParticleConfigIssue>,
    config: &ParticleEmitterConfig,
    field: &ParticleEmitterConfig,
) -> () {
    let value = config[field as usize].clone();
    if ((value).is_finite()) && ((value < 0.0_f64) || (value > 1.0_f64)) {
        issues.push(ParticleConfigIssue {
            __flight_identity: std::sync::Arc::new(()),
            field: (*field).clone(),
            message: format!(
                "{} ({}) is outside the expected 0–1 range",
                (*field).clone(),
                value
            ),
            severity: "warning".to_owned(),
        });
    }
}
