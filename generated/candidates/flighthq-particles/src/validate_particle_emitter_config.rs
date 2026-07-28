// @generated from upstream/packages/particles/src/validateParticleEmitterConfig.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::create_particle_emitter_config;
pub use flighthq_types::ParticleConfigIssue;
use flighthq_types::{
    ParticleBlendMode, ParticleCurve, ParticleEmitterConfig, ParticleEmitterShape,
};

#[derive(Clone)]
pub struct FlightPartialRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub alpha_end: Option<f64>,
    pub alpha_start: Option<f64>,
    pub blend_mode: Option<ParticleBlendMode>,
    pub color_end_b: Option<f64>,
    pub color_end_g: Option<f64>,
    pub color_end_r: Option<f64>,
    pub color_end_variance_b: Option<f64>,
    pub color_end_variance_g: Option<f64>,
    pub color_end_variance_r: Option<f64>,
    pub color_start_b: Option<f64>,
    pub color_start_g: Option<f64>,
    pub color_start_r: Option<f64>,
    pub color_start_variance_b: Option<f64>,
    pub color_start_variance_g: Option<f64>,
    pub color_start_variance_r: Option<f64>,
    pub direction_x: Option<f64>,
    pub direction_y: Option<f64>,
    pub direction_z: Option<f64>,
    pub gravity_x: Option<f64>,
    pub gravity_y: Option<f64>,
    pub gravity_z: Option<f64>,
    pub emitter_cone_angle: Option<f64>,
    pub emitter_depth: Option<f64>,
    pub emitter_height: Option<f64>,
    pub emitter_radius: Option<f64>,
    pub emitter_shape: Option<ParticleEmitterShape>,
    pub emitter_width: Option<f64>,
    pub burst_count: Option<f64>,
    pub burst_interval: Option<f64>,
    pub duration: Option<f64>,
    pub loop_: Option<bool>,
    pub frame_count: Option<f64>,
    pub frame_rate: Option<f64>,
    pub lifetime_max: Option<f64>,
    pub lifetime_min: Option<f64>,
    pub max_particles: Option<f64>,
    pub region_id_max: Option<f64>,
    pub region_id_min: Option<f64>,
    pub scale_end: Option<f64>,
    pub scale_max: Option<f64>,
    pub scale_min: Option<f64>,
    pub speed_max: Option<f64>,
    pub speed_min: Option<f64>,
    pub spawn_rate: Option<f64>,
    pub spread: Option<f64>,
    pub rotation_speed_max: Option<f64>,
    pub rotation_speed_min: Option<f64>,
    pub velocity_inheritance: Option<f64>,
    pub alpha_curve: Option<ParticleCurve>,
    pub color_curve: Option<ParticleCurve>,
    pub scale_curve: Option<ParticleCurve>,
    pub world_space: Option<bool>,
}
impl PartialEq for FlightPartialRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/particles/src/validateParticleEmitterConfig.ts:9 (sha256:a63f7598f4378b6e361fbcca3aaac29501875667fdce4035b9c61271311bf5ce)
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

// Source: upstream/packages/particles/src/validateParticleEmitterConfig.ts:59 (sha256:882c6aa6341b8ae3425c504eab69c6aa411613da8b8353bd2de8d45d555bb66e)
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

// Source: upstream/packages/particles/src/validateParticleEmitterConfig.ts:85 (sha256:458e7d414d06d6f2468cb586896b90136b8aac0c5adb8a169f38b31fb14c9b22)
pub fn normalize_particle_emitter_config(
    config: Option<FlightPartialRecord1>,
) -> ParticleEmitterConfig {
    let mut out = create_particle_emitter_config(Some(((config).clone().unwrap()).clone()));
    let defaults = create_particle_emitter_config(None);
    let mut mutable = out;
    let defaults_rec = defaults;
    for field in ((*NUMERIC_FIELDS).clone()).iter().cloned() {
        if (!(mutable
            .iter()
            .find(|(key, _)| key == &(field).clone())
            .map(|(_, value)| value)
            .expect("TypeScript Record key was absent")
            .clone())
        .is_finite())
        {
            mutable
                .iter()
                .find(|(key, _)| key == &(field).clone())
                .map(|(_, value)| value)
                .expect("TypeScript Record key was absent") = defaults_rec
                .iter()
                .find(|(key, _)| key == &(field).clone())
                .map(|(_, value)| value)
                .expect("TypeScript Record key was absent")
                .clone();
        }
    }
    return ParticleEmitterConfig {
        alpha_curve: if is_finite_curve(((out.alpha_curve).clone()).clone()) {
            (out.alpha_curve).clone()
        } else {
            None
        },
        color_curve: if is_finite_curve(((out.color_curve).clone()).clone()) {
            (out.color_curve).clone()
        } else {
            None
        },
        scale_curve: if is_finite_curve(((out.scale_curve).clone()).clone()) {
            (out.scale_curve).clone()
        } else {
            None
        },
        max_particles: (0.0_f64).max((out.max_particles).floor()),
        burst_count: (0.0_f64).max((out.burst_count).floor()),
        burst_interval: (0.0_f64).max(out.burst_interval),
        duration: (0.0_f64).max(out.duration),
        frame_count: (1.0_f64).max((out.frame_count).floor()),
        frame_rate: (0.0_f64).max(out.frame_rate),
        region_id_min: (0.0_f64).max((out.region_id_min).floor()),
        region_id_max: ((0.0_f64).max((out.region_id_min).floor()))
            .max((out.region_id_max).floor()),
        spawn_rate: (0.0_f64).max(out.spawn_rate),
        lifetime_min: (0.0_f64).max(out.lifetime_min),
        lifetime_max: (0.0_f64).max(out.lifetime_max),
        speed_min: (0.0_f64).max(out.speed_min),
        speed_max: (0.0_f64).max(out.speed_max),
        scale_min: (0.0_f64).max(out.scale_min),
        scale_max: (0.0_f64).max(out.scale_max),
        emitter_cone_angle: (0.0_f64).max(out.emitter_cone_angle),
        emitter_depth: (0.0_f64).max(out.emitter_depth),
        emitter_radius: (0.0_f64).max(out.emitter_radius),
        emitter_width: (0.0_f64).max(out.emitter_width),
        emitter_height: (0.0_f64).max(out.emitter_height),
        ..((out).clone()).clone()
    };
}

// Source: upstream/packages/particles/src/validateParticleEmitterConfig.ts:130 (sha256:a18a08a4eee98a57f847bc012bb29a248fb41fa6234d854be5820852f66068d0)
#[derive(Clone)]
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
        let value = config[field as usize].clone();
        if (!(value).is_finite()) {
            issues.push(ParticleConfigIssue {
                __flight_identity: std::sync::Arc::new(()),
                field: field,
                message: format!("{} must be a finite number (got {})", field, string(value)),
                severity: "error".to_owned(),
            });
        }
    }
    for field in ((*NON_NEGATIVE_FIELDS).clone()).iter().cloned() {
        let value = config[field as usize].clone();
        if ((value).is_finite() && (value < 0.0_f64)) {
            issues.push(ParticleConfigIssue {
                __flight_identity: std::sync::Arc::new(()),
                field: field,
                message: format!("{} must not be negative (got {})", field, value),
                severity: "warning".to_owned(),
            });
        }
    }
    if ((config.lifetime_max).is_finite() && (config.lifetime_max <= 0.0_f64)) {
        issues.push(ParticleConfigIssue {
            __flight_identity: std::sync::Arc::new(()),
            field: "lifetimeMax",
            message: "lifetimeMax must be > 0 or particles die instantly".to_owned(),
            severity: "warning".to_owned(),
        });
    }
    if ((config.max_particles).is_finite() && (config.max_particles <= 0.0_f64)) {
        issues.push(ParticleConfigIssue {
            __flight_identity: std::sync::Arc::new(()),
            field: "maxParticles",
            message: "maxParticles must be >= 1 or nothing ever spawns".to_owned(),
            severity: "warning".to_owned(),
        });
    }
    if ((config.frame_count).is_finite() && (config.frame_count < 1.0_f64)) {
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
    report_curve(
        &mut issues,
        ((config.alpha_curve).clone()).clone(),
        &"alphaCurve",
        1.0_f64,
    );
    report_curve(
        &mut issues,
        ((config.color_curve).clone()).clone(),
        &"colorCurve",
        3.0_f64,
    );
    report_curve(
        &mut issues,
        ((config.scale_curve).clone()).clone(),
        &"scaleCurve",
        1.0_f64,
    );
    return issues;
}

// Source: upstream/packages/particles/src/validateParticleEmitterConfig.ts:184 (sha256:240fb75be0e4ecb4ddb72582c461f9119053b2a7ae5ebc33752483d6703ed0a0)
fn is_finite_curve(curve: Option<Vec<f64>>) -> bool {
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

// Source: upstream/packages/particles/src/validateParticleEmitterConfig.ts:192 (sha256:ec6018485ce7aa3befb4ddf9c80567fe1a597599f8ec09c769e6aa16b82c0bf7)
fn report_curve(
    issues: &mut Vec<ParticleConfigIssue>,
    curve: Option<Vec<f64>>,
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
            message: format!("{} is empty and will be ignored", field),
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
                field,
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
                    message: format!("{} contains a non-finite sample at index {}", field, i),
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

// Source: upstream/packages/particles/src/validateParticleEmitterConfig.ts:218 (sha256:56764c6b04aae3b47f34da76e9fddd0519876f809968345bdcfaae0de998e7c2)
fn report_inverted_range(
    issues: &mut Vec<ParticleConfigIssue>,
    config: &ParticleEmitterConfig,
    min_field: &ParticleEmitterConfig,
    max_field: &ParticleEmitterConfig,
) -> () {
    let min = config[min_field as usize].clone();
    let max = config[max_field as usize].clone();
    if (((min).is_finite() && (max).is_finite()) && (min > max)) {
        issues.push(ParticleConfigIssue {
            __flight_identity: std::sync::Arc::new(()),
            field: (*min_field).clone(),
            message: format!(
                "{} ({}) is greater than {} ({})",
                min_field, min, max_field, max
            ),
            severity: "warning".to_owned(),
        });
    }
}

// Source: upstream/packages/particles/src/validateParticleEmitterConfig.ts:235 (sha256:fc726b54cf08ed2f9aecdd12527048ee8ef87cbbf792f07dae2d890bc95031d8)
fn report_unit_range(
    issues: &mut Vec<ParticleConfigIssue>,
    config: &ParticleEmitterConfig,
    field: &ParticleEmitterConfig,
) -> () {
    let value = config[field as usize].clone();
    if ((value).is_finite() && (value < 0.0_f64) || (value > 1.0_f64)) {
        issues.push(ParticleConfigIssue {
            __flight_identity: std::sync::Arc::new(()),
            field: (*field).clone(),
            message: format!("{} ({}) is outside the expected 0–1 range", field, value),
            severity: "warning".to_owned(),
        });
    }
}
