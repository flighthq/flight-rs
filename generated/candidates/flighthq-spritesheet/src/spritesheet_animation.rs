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
    pub repeat_count: Option<f64>,
    pub origin_x: Option<f64>,
    pub origin_y: Option<f64>,
}
impl PartialEq for FlightPartialRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/spritesheet/src/spritesheetAnimation.ts:4 (sha256:bc6618f2aaa7f6142dbbb81b978e2ef28b69d40895f56a74c1a4e57ded0228aa)
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
        origin_x: (obj.as_ref().and_then(|value| value.origin_x)).unwrap_or(0.0_f64),
        origin_y: (obj.as_ref().and_then(|value| value.origin_y)).unwrap_or(0.0_f64),
        repeat_count: (obj.as_ref().and_then(|value| value.repeat_count)).unwrap_or(0.0_f64),
    }));
}

// Source: upstream/packages/spritesheet/src/spritesheetAnimation.ts:20 (sha256:f52f8c90eaf0d3e13c56099cfddd6f4e96866d7fe66df201c3d80d724e6064ba)
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
            if ((name).clone()).is_none() {
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
                (((name).clone())
                    == Some(match (*pattern).clone() {
                        crate::FlightUnion2::A(value) => value,
                        crate::FlightUnion2::B(_) => panic!("TypeScript union narrowing failed"),
                    }))
                    || (((name).clone()).starts_with(
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
        origin_x: Some((options.as_ref().and_then(|value| value.origin_x)).unwrap()),
        origin_y: Some((options.as_ref().and_then(|value| value.origin_y)).unwrap()),
        repeat_count: Some((options.as_ref().and_then(|value| value.repeat_count)).unwrap()),
    })));
}
