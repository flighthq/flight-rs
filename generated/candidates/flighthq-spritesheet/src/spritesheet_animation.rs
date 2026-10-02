// @generated from upstream/packages/spritesheet/src/spritesheetAnimation.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_entity::{allocate_entity, finish_entity};
use flighthq_types::{
    EntityConstruction, Spritesheet, SpritesheetAnimation, SpritesheetAnimationDirection,
    TextureAtlasRegion,
};

#[derive(Clone, Default)]
pub struct FlightPartialRecord1490253840 {
    pub __flight_identity: std::sync::Arc<()>,
    pub frames: Option<Vec<f64>>,
    pub frame_duration: Option<f64>,
    pub frame_durations: Option<Vec<f64>>,
    pub direction: Option<SpritesheetAnimationDirection>,
    pub repeat_count: Option<f64>,
    pub origin_x: Option<f64>,
    pub origin_y: Option<f64>,
}
impl PartialEq for FlightPartialRecord1490253840 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/spritesheet/src/spritesheetAnimation.ts:4 (sha256:ec5871f3fa12c4778f0ad4b48ea9e3839578b8f4fdc96197f2dc925dfb19ea7d)
pub fn create_spritesheet_animation(
    obj: Option<FlightPartialRecord1490253840>,
) -> SpritesheetAnimation {
    let mut out = allocate_entity();
    initialize_spritesheet_animation(
        (out).clone(),
        ((obj).clone()).as_ref().map(|__flight_value| {
            let __flight_source = &(__flight_value);
            FlightPartialRecord1490253840 {
                __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
                frames: (__flight_source.frames).clone(),
                frame_duration: __flight_source.frame_duration,
                frame_durations: (__flight_source.frame_durations).clone(),
                direction: (__flight_source.direction).clone(),
                repeat_count: __flight_source.repeat_count,
                origin_x: __flight_source.origin_x,
                origin_y: __flight_source.origin_y,
            }
        }),
    );
    return finish_entity((out).clone());
}

// Source: upstream/packages/spritesheet/src/spritesheetAnimation.ts:14 (sha256:f52f8c90eaf0d3e13c56099cfddd6f4e96866d7fe66df201c3d80d724e6064ba)
pub fn create_spritesheet_animation_from_frame_names(
    spritesheet: &Spritesheet,
    pattern: &crate::FlightUnion2<String, crate::OpaqueHostValue>,
    options: Option<FlightPartialRecord1490253840>,
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
            let region: Option<TextureAtlasRegion> = atlas
                .as_ref()
                .unwrap()
                .regions
                .get(region_id as usize)
                .cloned();
            if (region).is_none() {
                {
                    i += 1.0;
                    i
                };
                continue;
            }
            let name = (region.as_ref().unwrap().name).clone();
            if ((name).clone()).is_none() {
                {
                    i += 1.0;
                    i
                };
                continue;
            }
            let matches = if ((match &(pattern) {
                crate::FlightUnion2::A(_) => "string",
                crate::FlightUnion2::B(value) => "object",
            })
            .to_owned()
                == "string")
            {
                (((name).clone())
                    == Some(match (*pattern).clone() {
                        crate::FlightUnion2::A(value) => value,
                        crate::FlightUnion2::B(_) => panic!("TypeScript union narrowing failed"),
                    }))
                    || ((name.as_ref().unwrap()).starts_with(
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
    return Some(create_spritesheet_animation(Some(
        FlightPartialRecord1490253840 {
            __flight_identity: std::sync::Arc::new(()),
            direction: Some(
                (options.as_ref().and_then(|value| (value.direction).clone())).unwrap(),
            ),
            frame_duration: Some(
                (options.as_ref().and_then(|value| value.frame_duration)).unwrap(),
            ),
            frame_durations: options
                .as_ref()
                .and_then(|value| (value.frame_durations).clone()),
            frames: Some((matched_indices).clone()),
            origin_x: Some((options.as_ref().and_then(|value| value.origin_x)).unwrap()),
            origin_y: Some((options.as_ref().and_then(|value| value.origin_y)).unwrap()),
            repeat_count: Some((options.as_ref().and_then(|value| value.repeat_count)).unwrap()),
        },
    )));
}

// Source: upstream/packages/spritesheet/src/spritesheetAnimation.ts:48 (sha256:c22a53272414724b19fe77c54953f24249d36ac50186d200f8744607f71ed25f)
pub fn initialize_spritesheet_animation(
    out: EntityConstruction<SpritesheetAnimation>,
    obj: Option<FlightPartialRecord1490253840>,
) -> () {
    crate::host_set(
        "host.direction",
        (obj.as_ref().and_then(|value| (value.direction).clone())).unwrap_or("forward".to_owned()),
    );
    crate::host_set(
        "host.frameDuration",
        (obj.as_ref().and_then(|value| value.frame_duration)).unwrap_or(0.0_f64),
    );
    crate::host_set(
        "host.frameDurations",
        obj.as_ref()
            .and_then(|value| (value.frame_durations).clone()),
    );
    crate::host_set(
        "host.frames",
        (obj.as_ref().and_then(|value| (value.frames).clone())).unwrap_or(vec![]),
    );
    crate::host_set(
        "host.originX",
        (obj.as_ref().and_then(|value| value.origin_x)).unwrap_or(0.0_f64),
    );
    crate::host_set(
        "host.originY",
        (obj.as_ref().and_then(|value| value.origin_y)).unwrap_or(0.0_f64),
    );
    crate::host_set(
        "host.repeatCount",
        (obj.as_ref().and_then(|value| value.repeat_count)).unwrap_or(0.0_f64),
    );
}
