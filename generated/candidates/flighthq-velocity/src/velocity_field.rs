// @generated from upstream/packages/velocity/src/velocityField.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::{Velocity2D, VelocityField, VelocitySample};

// Source: upstream/packages/velocity/src/velocityField.ts:10 (sha256:60c5b07f464d1017977a0c9d2fd1058a34d49eb5a8668345652ca9cb4e2fb643)
pub fn add_velocity(out: &mut Velocity2D, a: &Velocity2D, b: &Velocity2D) -> Velocity2D {
    let ax = a.x;
    let ay = a.y;
    out.x = (ax + b.x);
    out.y = (ay + b.y);
    return out.clone();
}

// Source: upstream/packages/velocity/src/velocityField.ts:18 (sha256:f9aab20472fafacae3833914926225255da5632a375b23b546d91fc7f7804487)
pub fn begin_velocity_frame(field: &mut VelocityField) -> () {
    {
        field.frame_id += 1.0;
        field.frame_id
    };
}

// Source: upstream/packages/velocity/src/velocityField.ts:24 (sha256:f92a10ec2bc4d70a0174ffbcbb48f0fe417f8ac64bc53a9cf0c8057077bee7f3)
pub fn clamp_velocity(out: &mut Velocity2D, velocity: &Velocity2D, max_length: f64) -> Velocity2D {
    let vx = velocity.x;
    let vy = velocity.y;
    let len_sq = ((vx * vx) + (vy * vy));
    let max_sq = (max_length * max_length);
    if ((len_sq > max_sq) && (len_sq > 0.0_f64)) {
        let scale = (max_length / (len_sq).sqrt());
        out.x = (vx * scale);
        out.y = (vy * scale);
    } else {
        out.x = vx;
        out.y = vy;
    }
    return out.clone();
}

// Source: upstream/packages/velocity/src/velocityField.ts:40 (sha256:a61b7dc95e858cbab0ca6e1cb22b08e256a5b2e4746addef01862f181898279d)
pub fn contribute_velocity(
    field: &mut VelocityField,
    source: crate::OpaqueHostValue,
    x: f64,
    y: f64,
) -> () {
    let mut sample = ensure_velocity_sample(field, (source).clone());
    sample.velocity.x = x;
    sample.velocity.y = y;
    sample.last_frame_id = field.frame_id;
    sample.explicit_frame_id = field.frame_id;
}

// Source: upstream/packages/velocity/src/velocityField.ts:49 (sha256:46361e0eec257b1be2b148a8c4d74f7644e2dba1c907279cb03ce53178f53f87)
pub fn copy_velocity(out: &mut Velocity2D, source: &Velocity2D) -> Velocity2D {
    let sx = source.x;
    let sy = source.y;
    out.x = sx;
    out.y = sy;
    return out.clone();
}

// Source: upstream/packages/velocity/src/velocityField.ts:57 (sha256:fe7d4e4e231d2c435b69b5ae51d9a15bf63e0efbef2488a6c970617ef4bb5c8e)
#[derive(Clone)]
struct CreateVelocityFieldRecord1 {
    __flight_identity: std::sync::Arc<()>,
    samples: Vec<(crate::OpaqueHostValue, crate::OpaqueHostValue)>,
    frame_id: f64,
}
impl PartialEq for CreateVelocityFieldRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_velocity_field() -> VelocityField {
    return VelocityField {
        __flight_identity: std::sync::Arc::new(()),
        samples: Vec::new(),
        frame_id: 0.0_f64,
    };
}

// Source: upstream/packages/velocity/src/velocityField.ts:64 (sha256:e39999ff55acbf1cafff8579e2e85b26ffd7eec710ba5b0e728bd69e517e3b7e)
pub fn damp_velocity(
    out: &mut Velocity2D,
    current: &Velocity2D,
    previous: &Velocity2D,
    factor: f64,
) -> Velocity2D {
    let cx = current.x;
    let cy = current.y;
    let px = previous.x;
    let py = previous.y;
    out.x = ((cx * factor) + (px * (1.0_f64 - factor)));
    out.y = ((cy * factor) + (py * (1.0_f64 - factor)));
    return out.clone();
}

// Source: upstream/packages/velocity/src/velocityField.ts:80 (sha256:8f596053da0af3d96dcee722eb5814c68847a9f9d75db45e8a18edc0cc778029)
#[derive(Clone)]
struct EnsureVelocitySampleRecord1 {
    __flight_identity: std::sync::Arc<()>,
    x: f64,
    y: f64,
}
impl PartialEq for EnsureVelocitySampleRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn ensure_velocity_sample(
    field: &mut VelocityField,
    source: crate::OpaqueHostValue,
) -> VelocitySample {
    let mut sample = field
        .samples
        .iter()
        .find(|(key, _)| key == &(source).clone())
        .map(|(_, value)| value.clone());
    if (sample).is_none() {
        sample = Some(VelocitySample {
            __flight_identity: std::sync::Arc::new(()),
            previous_world_transform: None,
            velocity: Velocity2D {
                __flight_identity: std::sync::Arc::new(()),
                x: 0.0_f64,
                y: 0.0_f64,
            },
            last_frame_id: (-1.0_f64),
            explicit_frame_id: (-1.0_f64),
        });
        {
            let __flight_key = (source).clone();
            let __flight_value = (sample).clone().unwrap();
            if let Some((_, value)) = field
                .samples
                .iter_mut()
                .find(|(key, _)| key == &__flight_key)
            {
                *value = __flight_value;
            } else {
                field.samples.push((__flight_key, __flight_value));
            }
        };
    }
    return (sample).clone().unwrap();
}

// Source: upstream/packages/velocity/src/velocityField.ts:91 (sha256:2101e8a86d7a71ae1aab896617db881fdf9b731e27a9a333ea495987179e1894)
pub fn get_velocity(
    field: &VelocityField,
    source: crate::OpaqueHostValue,
    out: &mut Velocity2D,
) -> Velocity2D {
    let sample = field
        .samples
        .iter()
        .find(|(key, _)| key == &(source).clone())
        .map(|(_, value)| value.clone());
    if ((sample).is_none() || (sample.as_ref().unwrap().last_frame_id != field.frame_id)) {
        out.x = 0.0_f64;
        out.y = 0.0_f64;
        return out.clone();
    }
    out.x = sample.as_ref().unwrap().velocity.x;
    out.y = sample.as_ref().unwrap().velocity.y;
    return out.clone();
}

// Source: upstream/packages/velocity/src/velocityField.ts:103 (sha256:d08a11438a08c9aa720f63bfb059a752162cc9b98884156ad8ec39f7a456bbf8)
pub fn has_velocity(field: &VelocityField, source: crate::OpaqueHostValue) -> bool {
    let sample = field
        .samples
        .iter()
        .find(|(key, _)| key == &(source).clone())
        .map(|(_, value)| value.clone());
    return (((sample).is_some() && (sample.as_ref().unwrap().last_frame_id == field.frame_id))
        && ((sample.as_ref().unwrap().velocity.x != 0.0_f64)
            || (sample.as_ref().unwrap().velocity.y != 0.0_f64)));
}

// Source: upstream/packages/velocity/src/velocityField.ts:112 (sha256:2c406cd053fec2c78104de5760c0f0c13fbc9140caaef30a2484d4b14c87ea6f)
pub fn is_velocity_zero(velocity: &Velocity2D, epsilon: Option<f64>) -> bool {
    let e = (epsilon).unwrap_or(0.0_f64);
    return (((velocity.x).abs() <= e) && ((velocity.y).abs() <= e));
}

// Source: upstream/packages/velocity/src/velocityField.ts:118 (sha256:9eeecc456d811005b724d64c80b5823717a201ead6cc5e66e8c25fc21336e603)
pub fn length_of_velocity(velocity: &Velocity2D) -> f64 {
    return ((velocity.x * velocity.x) + (velocity.y * velocity.y)).sqrt();
}

// Source: upstream/packages/velocity/src/velocityField.ts:123 (sha256:242f9691c2f08b7e93956389ed9a431990d4dbe23ded0e9d60017a804e927555)
pub fn lerp_velocity(out: &mut Velocity2D, a: &Velocity2D, b: &Velocity2D, t: f64) -> Velocity2D {
    let ax = a.x;
    let ay = a.y;
    out.x = (ax + ((b.x - ax) * t));
    out.y = (ay + ((b.y - ay) * t));
    return out.clone();
}

// Source: upstream/packages/velocity/src/velocityField.ts:133 (sha256:ab6ea9b8f0a7ff0691458f49f714a59785f3617a6728ea95112fc92e7336aa14)
pub fn normalize_velocity(out: &mut Velocity2D, source: &Velocity2D) -> Velocity2D {
    let sx = source.x;
    let sy = source.y;
    let len = ((sx * sx) + (sy * sy)).sqrt();
    if (len > 0.0_f64) {
        let inv = (1.0_f64 / len);
        out.x = (sx * inv);
        out.y = (sy * inv);
    } else {
        out.x = 0.0_f64;
        out.y = 0.0_f64;
    }
    return out.clone();
}

// Source: upstream/packages/velocity/src/velocityField.ts:150 (sha256:11fa1963c2d7b7adb03b528cd63519e95f6bf33f2db7c64b13b0533a743bd9b3)
pub fn scale_velocity(out: &mut Velocity2D, velocity: &Velocity2D, scale: f64) -> Velocity2D {
    let vx = velocity.x;
    let vy = velocity.y;
    out.x = (vx * scale);
    out.y = (vy * scale);
    return out.clone();
}

// Source: upstream/packages/velocity/src/velocityField.ts:159 (sha256:afee1e933320b55b190bc0d0abc9d61f568ced3f19368ef5b797e7e459d6ce42)
pub fn subtract_velocity(out: &mut Velocity2D, a: &Velocity2D, b: &Velocity2D) -> Velocity2D {
    let ax = a.x;
    let ay = a.y;
    out.x = (ax - b.x);
    out.y = (ay - b.y);
    return out.clone();
}

// Source: upstream/packages/velocity/src/velocityField.ts:168 (sha256:712a0808bf93d58aff86500ce1d80d3be3b729d18911112b33d70c94a81f3010)
pub fn suppress_velocity(field: &mut VelocityField, source: crate::OpaqueHostValue) -> () {
    contribute_velocity(field, (source).clone(), 0.0_f64, 0.0_f64);
}

// Source: upstream/packages/velocity/src/velocityField.ts:173 (sha256:7d263be7000323545c3ec565bf524b7e90ace3f73a0f23fa616d75a45eb54954)
pub fn zero_velocity(out: &mut Velocity2D) -> Velocity2D {
    out.x = 0.0_f64;
    out.y = 0.0_f64;
    return out.clone();
}
