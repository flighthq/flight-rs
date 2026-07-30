// @generated from upstream/packages/spritesheet/src/spritesheetAnimation.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_entity::create_entity;
use flighthq_types::{Spritesheet, SpritesheetAnimation, SpritesheetAnimationDirection};

#[derive(Clone, Default)]
pub struct FlightPartialRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub frames: Option<Vec<f64>>,
    pub frame_duration: Option<f64>,
    pub frame_durations: Option<Vec<f64>>,
    pub direction: Option<SpritesheetAnimationDirection>,
    pub loop_: Option<bool>,
    pub origin_x: Option<f64>,
    pub origin_y: Option<f64>,
}
impl PartialEq for FlightPartialRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/spritesheet/src/spritesheetAnimation.ts:4 (sha256:1282c8270af722534f5e7642ca2602d0539978fae0a9955e54f2ac9df674771f)
pub fn create_spritesheet_animation(obj: Option<FlightPartialRecord1>) -> SpritesheetAnimation {
    return create_entity(Some(SpritesheetAnimation {
        __flight_identity: std::sync::Arc::new(()),
        __flight_entity_runtime: Default::default(),
        direction: (obj.as_ref().and_then(|value| (value.direction).clone()))
            .unwrap_or("forward".to_owned()),
        frame_duration: (obj.as_ref().and_then(|value| value.frame_duration)).unwrap_or(0.0_f64),
        frame_durations: obj
            .as_ref()
            .and_then(|value| (value.frame_durations).clone()),
        frames: (obj.as_ref().and_then(|value| (value.frames).clone())).unwrap_or(vec![]),
        loop_: (obj.as_ref().and_then(|value| value.loop_)).unwrap_or(false),
        origin_x: (obj.as_ref().and_then(|value| value.origin_x)).unwrap_or(0.0_f64),
        origin_y: (obj.as_ref().and_then(|value| value.origin_y)).unwrap_or(0.0_f64),
    }));
}

// Source: upstream/packages/spritesheet/src/spritesheetAnimation.ts:20 (sha256:5b433336305c7011219ee9d6f25cc2c8d0dd98a56240dfbb02bd90e7c86bac8e)
pub fn create_spritesheet_animation_from_frame_names(
    spritesheet: &Spritesheet,
    pattern: &crate::FlightUnion2<String, crate::OpaqueHostValue>,
    options: Option<FlightPartialRecord1>,
) -> Option<SpritesheetAnimation> {
    let atlas = (spritesheet.atlas).clone();
    if (atlas).is_none() {
        return None;
    }
    let mut matched_indices: Vec<f64> = vec![];
    {
        let mut i = 0.0_f64;
        while (i < (spritesheet.frames.len() as f64)) {
            let region_id = spritesheet.frames[i as usize].id;
            let region = atlas.as_ref().unwrap().regions[region_id as usize].clone();
            if (region).is_none() {
                {
                    i += 1.0;
                    i
                };
                continue;
            }
            let name = (region.name).clone();
            if (name).is_none() {
                {
                    i += 1.0;
                    i
                };
                continue;
            }
            let matches = if (match &(pattern) {
                crate::FlightUnion2::A(_) => "string",
                crate::FlightUnion2::B(value) => "object",
            } == "string")
            {
                ((name)
                    == Some(match (*pattern).clone() {
                        crate::FlightUnion2::A(value) => value,
                        crate::FlightUnion2::B(_) => panic!("TypeScript union narrowing failed"),
                    }))
                    || ((name).starts_with(
                        (match (*pattern).clone() {
                            crate::FlightUnion2::A(value) => value,
                            crate::FlightUnion2::B(_) => {
                                panic!("TypeScript union narrowing failed")
                            }
                        })
                        .as_str(),
                    ))
            } else {
                crate::host_value::<bool>("host.test")
            };
            if matches {
                matched_indices.push(i);
            }
            {
                i += 1.0;
                i
            };
        }
    }
    if ((matched_indices.len() as f64) == 0.0_f64) {
        return None;
    }
    return Some(create_spritesheet_animation(Some(FlightPartialRecord1 {
        __flight_identity: std::sync::Arc::new(()),
        direction: Some((options.as_ref().and_then(|value| (value.direction).clone())).unwrap()),
        frame_duration: Some((options.as_ref().and_then(|value| value.frame_duration)).unwrap()),
        frame_durations: options
            .as_ref()
            .and_then(|value| (value.frame_durations).clone()),
        frames: Some((matched_indices).clone()),
        loop_: Some((options.as_ref().and_then(|value| value.loop_)).unwrap()),
        origin_x: Some((options.as_ref().and_then(|value| value.origin_x)).unwrap()),
        origin_y: Some((options.as_ref().and_then(|value| value.origin_y)).unwrap()),
    })));
}
