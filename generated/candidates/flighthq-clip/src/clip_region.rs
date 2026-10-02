// @generated from upstream/packages/clip/src/clipRegion.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_entity::{allocate_entity, finish_entity};
use flighthq_geometry::{
    clone_rectangle, contains_rectangle_point_xy, copy_rectangle, create_rectangle,
    encloses_rectangle, intersects_rectangle, is_empty_rectangle, matrix_transform_rectangle,
    merge_rectangle,
};
use flighthq_math::CIRCLE_KAPPA as circle_kappa_constant;
use flighthq_path::{
    append_path_cubic_curve_to, append_path_line_to, append_path_move_to, create_path, flatten_path,
};
use flighthq_types::{
    ClipRegion, ClipRegionContoursExplanation, ClipRegionContoursGuard, ClipRegionExplanation,
    ClipRegionReleaseGuard, ClipRegionUseGuard, EntityConstruction, MatrixLike, Path, PathWinding,
    RectangleLike,
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

// Source: upstream/packages/clip/src/clipRegion.ts:38 (sha256:9bd8a98dcb93e1f3dafd093011d6275b352395814339f972aff4b903dff5fe15)
pub fn acquire_clip_region() -> ClipRegion {
    let mut region = CLIP_REGION_POOL.lock().unwrap().pop();
    if (region).is_some() {
        region.as_mut().unwrap().rect.x = 0.0_f64;
        region.as_mut().unwrap().rect.y = 0.0_f64;
        region.as_mut().unwrap().rect.width = 0.0_f64;
        region.as_mut().unwrap().rect.height = 0.0_f64;
        region.as_mut().unwrap().contours = None;
        region.as_mut().unwrap().winding = "nonZero".to_owned();
        region.as_mut().unwrap().version = 0.0_f64;
        return ((region.as_mut().unwrap()).clone()).clone();
    }
    return make_empty_clip_region();
}

// Source: upstream/packages/clip/src/clipRegion.ts:55 (sha256:37efcb34759af9ad15116d3826f75e110f7f0783bcf3cbb18733a079f6f376af)
pub fn clip_region_contains_clip_region(a: &ClipRegion, b: &ClipRegion) -> bool {
    guard_clip_region_use(a);
    guard_clip_region_use(b);
    return encloses_rectangle(
        &{
            let __flight_source = &(a.rect);
            RectangleLike {
                __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
                __flight_entity_runtime: std::sync::Arc::clone(
                    &__flight_source.__flight_entity_runtime,
                ),
                __flight_entity_snapshot: __flight_source.__flight_entity_snapshot.clone(),
                height: __flight_source.height,
                width: __flight_source.width,
                x: __flight_source.x,
                y: __flight_source.y,
            }
        },
        &{
            let __flight_source = &(b.rect);
            RectangleLike {
                __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
                __flight_entity_runtime: std::sync::Arc::clone(
                    &__flight_source.__flight_entity_runtime,
                ),
                __flight_entity_snapshot: __flight_source.__flight_entity_snapshot.clone(),
                height: __flight_source.height,
                width: __flight_source.width,
                x: __flight_source.x,
                y: __flight_source.y,
            }
        },
    );
}

// Source: upstream/packages/clip/src/clipRegion.ts:64 (sha256:597f50dc209a5ae77ec598879c15d6b0999b9e70e8316a94b2777b2cf0ec66be)
pub fn clip_region_contains_point(clip: &ClipRegion, x: f64, y: f64) -> bool {
    guard_clip_region_use(clip);
    if (!contains_rectangle_point_xy(
        &{
            let __flight_source = &(clip.rect);
            RectangleLike {
                __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
                __flight_entity_runtime: std::sync::Arc::clone(
                    &__flight_source.__flight_entity_runtime,
                ),
                __flight_entity_snapshot: __flight_source.__flight_entity_snapshot.clone(),
                height: __flight_source.height,
                width: __flight_source.width,
                x: __flight_source.x,
                y: __flight_source.y,
            }
        },
        x,
        y,
    )) {
        return false;
    }
    if ((clip.contours).clone()).is_none() {
        return true;
    }
    return point_in_contours(
        clip.contours.as_ref().unwrap(),
        (clip.winding).clone(),
        x,
        y,
    );
}

// Source: upstream/packages/clip/src/clipRegion.ts:73 (sha256:96c2ada5ca6aacc60e33e7a80f750878b0ab523ce427893f6444afffc0d5a2cf)
pub fn clip_region_contains_rectangle(clip: &ClipRegion, rectangle: &RectangleLike) -> bool {
    guard_clip_region_use(clip);
    return encloses_rectangle(
        &{
            let __flight_source = &(clip.rect);
            RectangleLike {
                __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
                __flight_entity_runtime: std::sync::Arc::clone(
                    &__flight_source.__flight_entity_runtime,
                ),
                __flight_entity_snapshot: __flight_source.__flight_entity_snapshot.clone(),
                height: __flight_source.height,
                width: __flight_source.width,
                x: __flight_source.x,
                y: __flight_source.y,
            }
        },
        rectangle,
    );
}

// Source: upstream/packages/clip/src/clipRegion.ts:80 (sha256:a07e0b4f095ff2b95035ae1f173aac0192334e6fb1737f9982b3454206b907d8)
pub fn clip_region_intersects_clip_region(a: &ClipRegion, b: &ClipRegion) -> bool {
    guard_clip_region_use(a);
    guard_clip_region_use(b);
    return intersects_rectangle(
        &{
            let __flight_source = &(a.rect);
            RectangleLike {
                __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
                __flight_entity_runtime: std::sync::Arc::clone(
                    &__flight_source.__flight_entity_runtime,
                ),
                __flight_entity_snapshot: __flight_source.__flight_entity_snapshot.clone(),
                height: __flight_source.height,
                width: __flight_source.width,
                x: __flight_source.x,
                y: __flight_source.y,
            }
        },
        &{
            let __flight_source = &(b.rect);
            RectangleLike {
                __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
                __flight_entity_runtime: std::sync::Arc::clone(
                    &__flight_source.__flight_entity_runtime,
                ),
                __flight_entity_snapshot: __flight_source.__flight_entity_snapshot.clone(),
                height: __flight_source.height,
                width: __flight_source.width,
                x: __flight_source.x,
                y: __flight_source.y,
            }
        },
    );
}

// Source: upstream/packages/clip/src/clipRegion.ts:88 (sha256:dc137977055cf0d69eb543b43555861783f1b4fbb0f48a6c4f6a627fddf187aa)
pub fn clip_region_intersects_rectangle(clip: &ClipRegion, rectangle: &RectangleLike) -> bool {
    guard_clip_region_use(clip);
    return intersects_rectangle(
        &{
            let __flight_source = &(clip.rect);
            RectangleLike {
                __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
                __flight_entity_runtime: std::sync::Arc::clone(
                    &__flight_source.__flight_entity_runtime,
                ),
                __flight_entity_snapshot: __flight_source.__flight_entity_snapshot.clone(),
                height: __flight_source.height,
                width: __flight_source.width,
                x: __flight_source.x,
                y: __flight_source.y,
            }
        },
        rectangle,
    );
}

// Source: upstream/packages/clip/src/clipRegion.ts:95 (sha256:5502b2d5a32725d0c6950c7eddc72ee9bdfeb3ba4d6895ed9f2cc5648228c06a)
pub fn clone_clip_region(clip: &ClipRegion) -> ClipRegion {
    guard_clip_region_use(clip);
    let rect = clone_rectangle(&{
        let __flight_source = &(clip.rect);
        RectangleLike {
            __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
            __flight_entity_runtime: std::sync::Arc::clone(
                &__flight_source.__flight_entity_runtime,
            ),
            __flight_entity_snapshot: __flight_source.__flight_entity_snapshot.clone(),
            height: __flight_source.height,
            width: __flight_source.width,
            x: __flight_source.x,
            y: __flight_source.y,
        }
    });
    let contours = if ((clip.contours).clone()).is_none() {
        None
    } else {
        Some(
            (clip.contours.as_ref().unwrap())
                .iter()
                .cloned()
                .map(|c: Vec<f64>| -> Vec<f64> { (c).clone() })
                .collect::<Vec<_>>(),
        )
    };
    let mut out = allocate_entity();
    crate::host_set("host.contours", contours);
    crate::host_set("host.rect", rect);
    crate::host_set("host.version", clip.version);
    crate::host_set("host.winding", (clip.winding).clone());
    return finish_entity((out).clone());
}

// Source: upstream/packages/clip/src/clipRegion.ts:109 (sha256:278ff24c24ab628b6f5d6c2d3b9907ee5acaccc22de0cb20e8157e19563d0b3e)
pub fn copy_clip_region(out: &mut ClipRegion, source: &ClipRegion) -> () {
    guard_clip_region_use(out);
    guard_clip_region_use(source);
    if ({
        let __flight_portable_source = (*out).clone();
        crate::FlightValue::Record({
            let mut __flight_record = Vec::new();
            __flight_record.push((
                "rect".to_owned(),
                crate::FlightValue::Record({
                    let mut __flight_record = Vec::new();
                    __flight_record.push((
                        "height".to_owned(),
                        crate::FlightValue::Number(
                            *(&((&((&__flight_portable_source).rect)).height)) as f64,
                        ),
                    ));
                    __flight_record.push((
                        "width".to_owned(),
                        crate::FlightValue::Number(
                            *(&((&((&__flight_portable_source).rect)).width)) as f64,
                        ),
                    ));
                    __flight_record.push((
                        "x".to_owned(),
                        crate::FlightValue::Number(
                            *(&((&((&__flight_portable_source).rect)).x)) as f64,
                        ),
                    ));
                    __flight_record.push((
                        "y".to_owned(),
                        crate::FlightValue::Number(
                            *(&((&((&__flight_portable_source).rect)).y)) as f64,
                        ),
                    ));
                    __flight_record
                }),
            ));
            __flight_record.push((
                "contours".to_owned(),
                match (&((&__flight_portable_source).contours)).as_ref() {
                    Some(value) => crate::FlightValue::Array(
                        (value)
                            .iter()
                            .map(|value| {
                                crate::FlightValue::Array(
                                    (value)
                                        .iter()
                                        .map(|value| crate::FlightValue::Number(*(value) as f64))
                                        .collect(),
                                )
                            })
                            .collect(),
                    ),
                    None => crate::FlightValue::Null,
                },
            ));
            __flight_record.push((
                "winding".to_owned(),
                crate::FlightValue::String((&((&__flight_portable_source).winding)).clone()),
            ));
            __flight_record.push((
                "version".to_owned(),
                crate::FlightValue::Number(*(&((&__flight_portable_source).version)) as f64),
            ));
            __flight_record
        })
    } == {
        let __flight_portable_source = (*source).clone();
        crate::FlightValue::Record({
            let mut __flight_record = Vec::new();
            __flight_record.push((
                "rect".to_owned(),
                crate::FlightValue::Record({
                    let mut __flight_record = Vec::new();
                    __flight_record.push((
                        "height".to_owned(),
                        crate::FlightValue::Number(
                            *(&((&((&__flight_portable_source).rect)).height)) as f64,
                        ),
                    ));
                    __flight_record.push((
                        "width".to_owned(),
                        crate::FlightValue::Number(
                            *(&((&((&__flight_portable_source).rect)).width)) as f64,
                        ),
                    ));
                    __flight_record.push((
                        "x".to_owned(),
                        crate::FlightValue::Number(
                            *(&((&((&__flight_portable_source).rect)).x)) as f64,
                        ),
                    ));
                    __flight_record.push((
                        "y".to_owned(),
                        crate::FlightValue::Number(
                            *(&((&((&__flight_portable_source).rect)).y)) as f64,
                        ),
                    ));
                    __flight_record
                }),
            ));
            __flight_record.push((
                "contours".to_owned(),
                match (&((&__flight_portable_source).contours)).as_ref() {
                    Some(value) => crate::FlightValue::Array(
                        (value)
                            .iter()
                            .map(|value| {
                                crate::FlightValue::Array(
                                    (value)
                                        .iter()
                                        .map(|value| crate::FlightValue::Number(*(value) as f64))
                                        .collect(),
                                )
                            })
                            .collect(),
                    ),
                    None => crate::FlightValue::Null,
                },
            ));
            __flight_record.push((
                "winding".to_owned(),
                crate::FlightValue::String((&((&__flight_portable_source).winding)).clone()),
            ));
            __flight_record.push((
                "version".to_owned(),
                crate::FlightValue::Number(*(&((&__flight_portable_source).version)) as f64),
            ));
            __flight_record
        })
    }) {
        return;
    }
    copy_rectangle(&mut out.rect, &{
        let __flight_source = &(source.rect);
        RectangleLike {
            __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
            __flight_entity_runtime: std::sync::Arc::clone(
                &__flight_source.__flight_entity_runtime,
            ),
            __flight_entity_snapshot: __flight_source.__flight_entity_snapshot.clone(),
            height: __flight_source.height,
            width: __flight_source.width,
            x: __flight_source.x,
            y: __flight_source.y,
        }
    });
    out.contours = if ((source.contours).clone()).is_none() {
        None
    } else {
        Some(
            (source.contours.as_ref().unwrap())
                .iter()
                .cloned()
                .map(|c: Vec<f64>| -> Vec<f64> { (c).clone() })
                .collect::<Vec<_>>(),
        )
    };
    out.winding = (source.winding).clone();
    out.version =
        (__flight_js_to_u32((out.version + 1.0_f64)) >> (__flight_js_to_u32(0.0_f64) & 31)) as f64;
}

// Source: upstream/packages/clip/src/clipRegion.ts:120 (sha256:64e74b5706330628f76ed802a209caab6bee71a91ca765f58220e71b166da818)
pub fn create_clip_region_from_circle(
    x: f64,
    y: f64,
    radius: f64,
    tolerance: Option<f64>,
) -> ClipRegion {
    let tolerance = tolerance.unwrap_or(0.25_f64);
    let mut path = create_path(Some(("nonZero".to_owned()).clone()));
    append_circle_to_path(&mut path, x, y, radius);
    return create_clip_region_from_path(&path, Some(tolerance));
}

// Source: upstream/packages/clip/src/clipRegion.ts:126 (sha256:83b212068d72af5e21cd25ad258e41b7b65071796034ad5753f228b2323ca2a5)
pub fn create_clip_region_from_contours(
    contours: &Vec<Vec<f64>>,
    winding: PathWinding,
) -> ClipRegion {
    let mut out = allocate_entity();
    initialize_clip_region_from_contours((out).clone(), contours, (winding).clone());
    return finish_entity((out).clone());
}

// Source: upstream/packages/clip/src/clipRegion.ts:137 (sha256:9f646d72c2ea5a6d574b1ce5b3f6f6354a89f2e823a4a6c2596099a2f0dcb6ac)
pub fn create_clip_region_from_ellipse(
    rectangle: &RectangleLike,
    tolerance: Option<f64>,
) -> ClipRegion {
    let tolerance = tolerance.unwrap_or(0.25_f64);
    let mut path = create_path(Some(("nonZero".to_owned()).clone()));
    append_ellipse_to_path(
        &mut path,
        rectangle.x,
        rectangle.y,
        rectangle.width,
        rectangle.height,
    );
    return create_clip_region_from_path(&path, Some(tolerance));
}

// Source: upstream/packages/clip/src/clipRegion.ts:143 (sha256:2229eba694e0ce7bd0ad0450e7164b2d8650626dbcff20d27111cde4b572d2c9)
pub fn create_clip_region_from_path(path: &Path, tolerance: Option<f64>) -> ClipRegion {
    let tolerance = tolerance.unwrap_or(0.25_f64);
    let mut out = allocate_entity();
    initialize_clip_region_from_path((out).clone(), path, Some(tolerance));
    return finish_entity((out).clone());
}

// Source: upstream/packages/clip/src/clipRegion.ts:149 (sha256:e4030f38e44e6dec1b713168a36cd8fe0a306aa5e8ce2c38dc93f590abc2d632)
pub fn create_clip_region_from_rectangle(rectangle: &RectangleLike) -> ClipRegion {
    let mut out = allocate_entity();
    initialize_clip_region_from_rectangle((out).clone(), rectangle);
    return finish_entity((out).clone());
}

// Source: upstream/packages/clip/src/clipRegion.ts:157 (sha256:a799cca9ae977991e2400ad11363335fa6809b104bde6bb25a741cd35c51a16e)
pub fn create_clip_region_from_rounded_rectangle(
    rectangle: &RectangleLike,
    radius: f64,
    tolerance: Option<f64>,
) -> ClipRegion {
    let tolerance = tolerance.unwrap_or(0.25_f64);
    if (radius <= 0.0_f64) {
        return create_clip_region_from_rectangle(rectangle);
    }
    let mut path = create_path(Some(("nonZero".to_owned()).clone()));
    append_rounded_rect_to_path(
        &mut path,
        rectangle.x,
        rectangle.y,
        rectangle.width,
        rectangle.height,
        radius,
    );
    return create_clip_region_from_path(&path, Some(tolerance));
}

// Source: upstream/packages/clip/src/clipRegion.ts:170 (sha256:6b123bb5406ebe5047fe15ce6a63efb994930ae46bb8f93479bc5204e0f8eeaa)
pub fn equals_clip_region(a: &ClipRegion, b: &ClipRegion) -> bool {
    guard_clip_region_use(a);
    guard_clip_region_use(b);
    if (a == b) {
        return true;
    }
    if ((a.winding).clone() != (b.winding).clone()) {
        return false;
    }
    if (((a.rect.x != b.rect.x) || (a.rect.y != b.rect.y)) || (a.rect.width != b.rect.width))
        || (a.rect.height != b.rect.height)
    {
        return false;
    }
    if (((a.contours).clone()).is_none()) && (((b.contours).clone()).is_none()) {
        return true;
    }
    if (((a.contours).clone()).is_none()) || (((b.contours).clone()).is_none()) {
        return false;
    }
    let ac = (a.contours).clone();
    let bc = (b.contours).clone();
    if ((ac.as_ref().unwrap().len() as f64) != (bc.as_ref().unwrap().len() as f64)) {
        return false;
    }
    {
        let mut i = 0.0_f64;
        while (i < (ac.as_ref().unwrap().len() as f64)) {
            let ai = ac.as_ref().unwrap()[i as usize].clone();
            let bi = bc.as_ref().unwrap()[i as usize].clone();
            if ((ai.len() as f64) != (bi.len() as f64)) {
                return false;
            }
            {
                let mut j = 0.0_f64;
                while (j < (ai.len() as f64)) {
                    if (ai[j as usize].clone() != bi[j as usize].clone()) {
                        return false;
                    }
                    {
                        j += 1.0;
                        j
                    };
                }
            }
            {
                i += 1.0;
                i
            };
        }
    }
    return true;
}

// Source: upstream/packages/clip/src/clipRegion.ts:196 (sha256:89736209686aaed225c5766c3f16833b340c68f5fe8c71c45dd3b6e03ec75f28)
pub fn explain_clip_region(clip: &ClipRegion) -> ClipRegionExplanation {
    return ClipRegionExplanation {
        __flight_identity: std::sync::Arc::new(()),
        conservative: ((clip.contours).clone()).is_some(),
        status: if {
            let __flight_value = (*clip).clone();
            (CLIP_REGION_POOL.lock().unwrap())
                .iter()
                .any(|item| item == &__flight_value)
        } {
            "released".to_owned()
        } else {
            "active".to_owned()
        },
    };
}

// Source: upstream/packages/clip/src/clipRegion.ts:205 (sha256:7c387937e8a95632d7061dbeabd83116f4996294ec9318c104dc39f71115f277)
pub fn explain_clip_region_contours(
    contours: &Vec<Vec<f64>>,
) -> Option<ClipRegionContoursExplanation> {
    {
        let mut i = 0.0_f64;
        while (i < (contours.len() as f64)) {
            let coordinate_count = (contours[i as usize].len() as f64);
            if ((__flight_js_to_i32(coordinate_count) & __flight_js_to_i32(1.0_f64)) as f64
                != 0.0_f64)
            {
                return Some(ClipRegionContoursExplanation {
                    __flight_identity: std::sync::Arc::new(()),
                    contour_index: i,
                    coordinate_count: coordinate_count,
                    reason: "odd-coordinate-count".to_owned(),
                });
            }
            if (coordinate_count < 6.0_f64) {
                return Some(ClipRegionContoursExplanation {
                    __flight_identity: std::sync::Arc::new(()),
                    contour_index: i,
                    coordinate_count: coordinate_count,
                    reason: "too-few-points".to_owned(),
                });
            }
            {
                i += 1.0;
                i
            };
        }
    }
    return None;
}

// Source: upstream/packages/clip/src/clipRegion.ts:217 (sha256:21dd36b3f3946a77ab26a1d82e022c3b3f5999aaa9bc90634cea0818e45d3713)
pub fn get_clip_region_bounds(out: &mut RectangleLike, clip: &ClipRegion) -> () {
    guard_clip_region_use(clip);
    out.x = clip.rect.x;
    out.y = clip.rect.y;
    out.width = clip.rect.width;
    out.height = clip.rect.height;
}

// Source: upstream/packages/clip/src/clipRegion.ts:230 (sha256:82e2a2ccb29610df3a9472d2293d0a236d084d5f76bd5364524462bc1215246c)
pub fn initialize_clip_region_from_contours(
    out: EntityConstruction<ClipRegion>,
    contours: &Vec<Vec<f64>>,
    winding: PathWinding,
) -> () {
    let explanation = explain_clip_region_contours(contours);
    if ((explanation).is_some()) && (((*_CONTOURS_GUARD.lock().unwrap()).clone()).is_some()) {
        {
            let __flight_callback = ((*_CONTOURS_GUARD.lock().unwrap()).as_ref().unwrap()).clone();
            __flight_callback.lock().unwrap()(
                (explanation.as_ref().unwrap()).clone(),
                (*contours).clone(),
            )
        };
    }
    let mut rect = create_rectangle(None, None, None, None);
    (|| -> () {
        let mut min_x = f64::INFINITY;
        let mut min_y = f64::INFINITY;
        let mut max_x = (-f64::INFINITY);
        let mut max_y = (-f64::INFINITY);
        {
            let mut c = 0.0_f64;
            while (c < (contours.len() as f64)) {
                let contour = contours[c as usize].clone();
                if ((contour.len() as f64) < 6.0_f64)
                    || ((__flight_js_to_i32((contour.len() as f64)) & __flight_js_to_i32(1.0_f64))
                        as f64
                        != 0.0_f64)
                {
                    {
                        c += 1.0;
                        c
                    };
                    continue;
                }
                {
                    let mut i = 0.0_f64;
                    while (i < (contour.len() as f64)) {
                        let x = contour[i as usize].clone();
                        let y = contour[(i + 1.0_f64) as usize].clone();
                        if (x < min_x) {
                            min_x = x;
                        }
                        if (x > max_x) {
                            max_x = x;
                        }
                        if (y < min_y) {
                            min_y = y;
                        }
                        if (y > max_y) {
                            max_y = y;
                        }
                        {
                            i += 2.0_f64;
                            i.clone()
                        };
                    }
                }
                {
                    c += 1.0;
                    c
                };
            }
        }
        if (min_x > max_x) {
            rect.x = 0.0_f64;
            rect.y = 0.0_f64;
            rect.width = 0.0_f64;
            rect.height = 0.0_f64;
            return;
        }
        rect.x = min_x;
        rect.y = min_y;
        rect.width = (max_x - min_x);
        rect.height = (max_y - min_y);
    })();
    let owned = (contours)
        .iter()
        .cloned()
        .map(|c: Vec<f64>| -> Vec<f64> { (c).clone() })
        .collect::<Vec<_>>();
    crate::host_set("host.contours", owned);
    crate::host_set("host.rect", rect);
    crate::host_set("host.version", 0.0_f64);
    crate::host_set("host.winding", winding);
}

// Source: upstream/packages/clip/src/clipRegion.ts:250 (sha256:43e8e6313194095e2d0168ee522b12d6b2e57f16399c880836115a00a71f9de7)
pub fn initialize_clip_region_from_path(
    out: EntityConstruction<ClipRegion>,
    path: &Path,
    tolerance: Option<f64>,
) -> () {
    let tolerance = tolerance.unwrap_or(0.25_f64);
    let contours = flatten_path(path, Some(tolerance));
    let mut rect = create_rectangle(None, None, None, None);
    (|| -> () {
        let mut min_x = f64::INFINITY;
        let mut min_y = f64::INFINITY;
        let mut max_x = (-f64::INFINITY);
        let mut max_y = (-f64::INFINITY);
        {
            let mut c = 0.0_f64;
            while (c < (contours.len() as f64)) {
                let contour = contours[c as usize].clone();
                if ((contour.len() as f64) < 6.0_f64)
                    || ((__flight_js_to_i32((contour.len() as f64)) & __flight_js_to_i32(1.0_f64))
                        as f64
                        != 0.0_f64)
                {
                    {
                        c += 1.0;
                        c
                    };
                    continue;
                }
                {
                    let mut i = 0.0_f64;
                    while (i < (contour.len() as f64)) {
                        let x = contour[i as usize].clone();
                        let y = contour[(i + 1.0_f64) as usize].clone();
                        if (x < min_x) {
                            min_x = x;
                        }
                        if (x > max_x) {
                            max_x = x;
                        }
                        if (y < min_y) {
                            min_y = y;
                        }
                        if (y > max_y) {
                            max_y = y;
                        }
                        {
                            i += 2.0_f64;
                            i.clone()
                        };
                    }
                }
                {
                    c += 1.0;
                    c
                };
            }
        }
        if (min_x > max_x) {
            rect.x = 0.0_f64;
            rect.y = 0.0_f64;
            rect.width = 0.0_f64;
            rect.height = 0.0_f64;
            return;
        }
        rect.x = min_x;
        rect.y = min_y;
        rect.width = (max_x - min_x);
        rect.height = (max_y - min_y);
    })();
    crate::host_set("host.contours", contours);
    crate::host_set("host.rect", rect);
    crate::host_set("host.version", 0.0_f64);
    crate::host_set("host.winding", (path.winding).clone());
}

// Source: upstream/packages/clip/src/clipRegion.ts:266 (sha256:cb6793e4c3e30bab00dbb3e0341acd2f3a02a78a317bd7a8575514783c22dabf)
pub fn initialize_clip_region_from_rectangle(
    out: EntityConstruction<ClipRegion>,
    rectangle: &RectangleLike,
) -> () {
    crate::host_set("host.contours", None);
    crate::host_set("host.rect", clone_rectangle(rectangle));
    crate::host_set("host.version", 0.0_f64);
    crate::host_set("host.winding", "nonZero");
}

// Source: upstream/packages/clip/src/clipRegion.ts:280 (sha256:7796e9c3640e627b1bfa5e57400f147817a821c22fb3bb6137c9eab8e886c9f3)
pub fn intersect_clip_regions(out: &mut ClipRegion, a: &ClipRegion, b: &ClipRegion) -> () {
    guard_clip_region_use(out);
    guard_clip_region_use(a);
    guard_clip_region_use(b);
    let ax = a.rect.x;
    let ay = a.rect.y;
    let aw = a.rect.width;
    let ah = a.rect.height;
    let bx = b.rect.x;
    let by = b.rect.y;
    let bw = b.rect.width;
    let bh = b.rect.height;
    let a_contours = (a.contours).clone();
    let b_contours = (b.contours).clone();
    let a_winding = (a.winding).clone();
    let b_winding = (b.winding).clone();
    let x0 = (ax).max(bx);
    let y0 = (ay).max(by);
    let x1 = (ax + aw).min((bx + bw));
    let y1 = (ay + ah).min((by + bh));
    if (x1 <= x0) || (y1 <= y0) {
        out.rect.x = 0.0_f64;
        out.rect.y = 0.0_f64;
        out.rect.width = 0.0_f64;
        out.rect.height = 0.0_f64;
        out.contours = None;
        out.winding = "nonZero".to_owned();
        out.version = (__flight_js_to_u32((out.version + 1.0_f64))
            >> (__flight_js_to_u32(0.0_f64) & 31)) as f64;
        return;
    }
    out.rect.x = x0;
    out.rect.y = y0;
    out.rect.width = (x1 - x0);
    out.rect.height = (y1 - y0);
    if ((a_contours).is_none()) && ((b_contours).is_none()) {
        out.contours = None;
        out.winding = "nonZero".to_owned();
    } else {
        if ((a_contours).is_some()) && ((b_contours).is_none()) {
            out.contours = Some(
                (a_contours.as_ref().unwrap())
                    .iter()
                    .cloned()
                    .map(|c: Vec<f64>| -> Vec<f64> { (c).clone() })
                    .collect::<Vec<_>>(),
            );
            out.winding = (a_winding).clone();
        } else {
            if ((a_contours).is_none()) && ((b_contours).is_some()) {
                out.contours = Some(
                    (b_contours.as_ref().unwrap())
                        .iter()
                        .cloned()
                        .map(|c: Vec<f64>| -> Vec<f64> { (c).clone() })
                        .collect::<Vec<_>>(),
                );
                out.winding = (b_winding).clone();
            } else {
                let keep_a = ((a_contours.as_ref().unwrap().len() as f64)
                    >= (b_contours.as_ref().unwrap().len() as f64));
                out.contours = Some(
                    (if keep_a {
                        (a_contours).clone()
                    } else {
                        (b_contours).clone()
                    }
                    .as_ref()
                    .unwrap())
                    .iter()
                    .cloned()
                    .map(|c: Vec<f64>| -> Vec<f64> { (c).clone() })
                    .collect::<Vec<_>>(),
                );
                out.winding = if keep_a {
                    (a_winding).clone()
                } else {
                    (b_winding).clone()
                };
            }
        }
    }
    out.version =
        (__flight_js_to_u32((out.version + 1.0_f64)) >> (__flight_js_to_u32(0.0_f64) & 31)) as f64;
}

// Source: upstream/packages/clip/src/clipRegion.ts:346 (sha256:20caecd991fe757f55299732e1c719f396a19a87891cf4b51a097d07305ebc6c)
pub fn invalidate_clip_region(clip: &mut ClipRegion) -> () {
    guard_clip_region_use(clip);
    clip.version =
        (__flight_js_to_u32((clip.version + 1.0_f64)) >> (__flight_js_to_u32(0.0_f64) & 31)) as f64;
}

// Source: upstream/packages/clip/src/clipRegion.ts:353 (sha256:70f286ac3ffddbe7a7f446ed9fea972ba08584b5e643032e39d635754c0c9972)
pub fn is_clip_region_empty(clip: &ClipRegion) -> bool {
    guard_clip_region_use(clip);
    if is_empty_rectangle(&{
        let __flight_source = &(clip.rect);
        RectangleLike {
            __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
            __flight_entity_runtime: std::sync::Arc::clone(
                &__flight_source.__flight_entity_runtime,
            ),
            __flight_entity_snapshot: __flight_source.__flight_entity_snapshot.clone(),
            height: __flight_source.height,
            width: __flight_source.width,
            x: __flight_source.x,
            y: __flight_source.y,
        }
    }) {
        return true;
    }
    if (((clip.contours).clone()).is_some())
        && ((clip.contours.as_ref().unwrap().len() as f64) == 0.0_f64)
    {
        return true;
    }
    return false;
}

// Source: upstream/packages/clip/src/clipRegion.ts:361 (sha256:8b2be51ba7b9e7779afb34fbf5d0dba07df3427df43dd3f1fa1e56a2b7a13092)
pub fn is_clip_region_rectangular(clip: &ClipRegion) -> bool {
    guard_clip_region_use(clip);
    return ((clip.contours).clone()).is_none();
}

// Source: upstream/packages/clip/src/clipRegion.ts:373 (sha256:68ae779088bfcae7e6f33afe4defc77d899e9319be34be6aa84198aa5d70f17e)
pub fn normalize_clip_region(out: &mut ClipRegion, clip: &ClipRegion) -> () {
    guard_clip_region_use(out);
    guard_clip_region_use(clip);
    let in_contours = (clip.contours).clone();
    let in_winding = (clip.winding).clone();
    if (in_contours).is_none() {
        copy_rectangle(&mut out.rect, &{
            let __flight_source = &(clip.rect);
            RectangleLike {
                __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
                __flight_entity_runtime: std::sync::Arc::clone(
                    &__flight_source.__flight_entity_runtime,
                ),
                __flight_entity_snapshot: __flight_source.__flight_entity_snapshot.clone(),
                height: __flight_source.height,
                width: __flight_source.width,
                x: __flight_source.x,
                y: __flight_source.y,
            }
        });
        out.contours = None;
        out.winding = (in_winding).clone();
        out.version = (__flight_js_to_u32((out.version + 1.0_f64))
            >> (__flight_js_to_u32(0.0_f64) & 31)) as f64;
        return;
    }
    if ((in_contours.as_ref().unwrap().len() as f64) == 1.0_f64)
        && ((in_contours.as_ref().unwrap()[0.0_f64 as usize].len() as f64) == 8.0_f64)
    {
        let c = in_contours.as_ref().unwrap()[0.0_f64 as usize].clone();
        let e = NORMALIZE_EPSILON;
        let mut min_x = f64::INFINITY;
        let mut min_y = f64::INFINITY;
        let mut max_x = (-f64::INFINITY);
        let mut max_y = (-f64::INFINITY);
        {
            let mut i = 0.0_f64;
            while (i < 8.0_f64) {
                let cx = c[i as usize].clone();
                let cy = c[(i + 1.0_f64) as usize].clone();
                if (cx < min_x) {
                    min_x = cx;
                }
                if (cx > max_x) {
                    max_x = cx;
                }
                if (cy < min_y) {
                    min_y = cy;
                }
                if (cy > max_y) {
                    max_y = cy;
                }
                {
                    i += 2.0_f64;
                    i.clone()
                };
            }
        }
        let mut is_axis_aligned = true;
        {
            let mut i = 0.0_f64;
            while (i < 8.0_f64) {
                let cx = c[i as usize].clone();
                let cy = c[(i + 1.0_f64) as usize].clone();
                if (!((cx - min_x).abs() <= e) || ((cx - max_x).abs() <= e)) {
                    is_axis_aligned = false;
                    break;
                }
                if (!((cy - min_y).abs() <= e) || ((cy - max_y).abs() <= e)) {
                    is_axis_aligned = false;
                    break;
                }
                {
                    i += 2.0_f64;
                    i.clone()
                };
            }
        }
        if is_axis_aligned {
            out.rect.x = min_x;
            out.rect.y = min_y;
            out.rect.width = (max_x - min_x);
            out.rect.height = (max_y - min_y);
            out.contours = None;
            out.winding = "nonZero".to_owned();
            out.version = (__flight_js_to_u32((out.version + 1.0_f64))
                >> (__flight_js_to_u32(0.0_f64) & 31)) as f64;
            return;
        }
    }
    copy_rectangle(&mut out.rect, &{
        let __flight_source = &(clip.rect);
        RectangleLike {
            __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
            __flight_entity_runtime: std::sync::Arc::clone(
                &__flight_source.__flight_entity_runtime,
            ),
            __flight_entity_snapshot: __flight_source.__flight_entity_snapshot.clone(),
            height: __flight_source.height,
            width: __flight_source.width,
            x: __flight_source.x,
            y: __flight_source.y,
        }
    });
    out.contours = Some(
        (in_contours.as_ref().unwrap())
            .iter()
            .cloned()
            .map(|c: Vec<f64>| -> Vec<f64> { (c).clone() })
            .collect::<Vec<_>>(),
    );
    out.winding = (in_winding).clone();
    out.version =
        (__flight_js_to_u32((out.version + 1.0_f64)) >> (__flight_js_to_u32(0.0_f64) & 31)) as f64;
}

// Source: upstream/packages/clip/src/clipRegion.ts:447 (sha256:b60719b33f8ca88b3d8b414f1d26a9a8fca380cae5e2f39973a8ae110e31793a)
pub fn release_clip_region(clip: &ClipRegion) -> () {
    if (((*_RELEASE_GUARD.lock().unwrap()).clone()).is_some())
        && ({
            let __flight_value = (*clip).clone();
            (CLIP_REGION_POOL.lock().unwrap())
                .iter()
                .any(|item| item == &__flight_value)
        })
    {
        {
            let __flight_callback = ((*_RELEASE_GUARD.lock().unwrap()).as_ref().unwrap()).clone();
            __flight_callback.lock().unwrap()((*clip).clone())
        };
    }
    CLIP_REGION_POOL
        .lock()
        .unwrap()
        .push(((*clip).clone()).clone());
}

// Source: upstream/packages/clip/src/clipRegion.ts:452 (sha256:82daa1cf04938b927137228ba82e87c6af3f27f6a6466f0f1796494a76012ad2)
pub fn set_clip_region_contours_guard(guard: &Option<ClipRegionContoursGuard>) -> () {
    (*_CONTOURS_GUARD.lock().unwrap()) = (*guard).clone();
}

// Source: upstream/packages/clip/src/clipRegion.ts:458 (sha256:ddcc96d5cedaa98bc154b844117c856eecc7e217412bf41bae5ade375531b657)
pub fn set_clip_region_release_guard(guard: &Option<ClipRegionReleaseGuard>) -> () {
    (*_RELEASE_GUARD.lock().unwrap()) = (*guard).clone();
}

// Source: upstream/packages/clip/src/clipRegion.ts:464 (sha256:d27a3fe3fd769f540c26dc53e8f862429c4e6c83899553f6e681a91eb49b906b)
pub fn set_clip_region_to_contours(
    out: &mut ClipRegion,
    contours: &Vec<Vec<f64>>,
    winding: PathWinding,
) -> () {
    guard_clip_region_use(out);
    let explanation = explain_clip_region_contours(contours);
    if ((explanation).is_some()) && (((*_CONTOURS_GUARD.lock().unwrap()).clone()).is_some()) {
        {
            let __flight_callback = ((*_CONTOURS_GUARD.lock().unwrap()).as_ref().unwrap()).clone();
            __flight_callback.lock().unwrap()(
                (explanation.as_ref().unwrap()).clone(),
                (*contours).clone(),
            )
        };
    }
    (|| -> () {
        let mut min_x = f64::INFINITY;
        let mut min_y = f64::INFINITY;
        let mut max_x = (-f64::INFINITY);
        let mut max_y = (-f64::INFINITY);
        {
            let mut c = 0.0_f64;
            while (c < (contours.len() as f64)) {
                let contour = contours[c as usize].clone();
                if ((contour.len() as f64) < 6.0_f64)
                    || ((__flight_js_to_i32((contour.len() as f64)) & __flight_js_to_i32(1.0_f64))
                        as f64
                        != 0.0_f64)
                {
                    {
                        c += 1.0;
                        c
                    };
                    continue;
                }
                {
                    let mut i = 0.0_f64;
                    while (i < (contour.len() as f64)) {
                        let x = contour[i as usize].clone();
                        let y = contour[(i + 1.0_f64) as usize].clone();
                        if (x < min_x) {
                            min_x = x;
                        }
                        if (x > max_x) {
                            max_x = x;
                        }
                        if (y < min_y) {
                            min_y = y;
                        }
                        if (y > max_y) {
                            max_y = y;
                        }
                        {
                            i += 2.0_f64;
                            i.clone()
                        };
                    }
                }
                {
                    c += 1.0;
                    c
                };
            }
        }
        if (min_x > max_x) {
            out.rect.x = 0.0_f64;
            out.rect.y = 0.0_f64;
            out.rect.width = 0.0_f64;
            out.rect.height = 0.0_f64;
            return;
        }
        out.rect.x = min_x;
        out.rect.y = min_y;
        out.rect.width = (max_x - min_x);
        out.rect.height = (max_y - min_y);
    })();
    out.contours = Some(
        (contours)
            .iter()
            .cloned()
            .map(|contour: Vec<f64>| -> Vec<f64> { (contour).clone() })
            .collect::<Vec<_>>(),
    );
    out.winding = (winding).clone();
    out.version =
        (__flight_js_to_u32((out.version + 1.0_f64)) >> (__flight_js_to_u32(0.0_f64) & 31)) as f64;
}

// Source: upstream/packages/clip/src/clipRegion.ts:480 (sha256:b7cc30d3af53b689bb52496bb6bbe74ea96f241d5b7955f22d131218e6579928)
pub fn set_clip_region_to_rectangle(out: &mut ClipRegion, rectangle: &RectangleLike) -> () {
    guard_clip_region_use(out);
    copy_rectangle(&mut out.rect, rectangle);
    out.contours = None;
    out.winding = "nonZero".to_owned();
    out.version =
        (__flight_js_to_u32((out.version + 1.0_f64)) >> (__flight_js_to_u32(0.0_f64) & 31)) as f64;
}

// Source: upstream/packages/clip/src/clipRegion.ts:488 (sha256:db84f7325c0f27fb05793268bc4dc48db7e6f7f9c714bc2b8c3a597e1a9b69eb)
pub fn set_clip_region_use_guard(guard: &Option<ClipRegionUseGuard>) -> () {
    (*_USE_GUARD.lock().unwrap()) = (*guard).clone();
}

// Source: upstream/packages/clip/src/clipRegion.ts:497 (sha256:aacc773b2eebadd1e36bf94b7547594c1201854c31d5edd7e0f16a5d847b108e)
pub fn transform_clip_region(out: &mut ClipRegion, clip: &ClipRegion, matrix: &MatrixLike) -> () {
    guard_clip_region_use(out);
    guard_clip_region_use(clip);
    let ma = matrix.a;
    let mb = matrix.b;
    let mc = matrix.c;
    let md = matrix.d;
    let mtx = matrix.tx;
    let mty = matrix.ty;
    let in_contours = (clip.contours).clone();
    let in_winding = (clip.winding).clone();
    if (in_contours).is_none() {
        let axis_aligned = (mb == 0.0_f64) && (mc == 0.0_f64);
        if axis_aligned {
            matrix_transform_rectangle(&mut out.rect, matrix, &{
                let __flight_source = &(clip.rect);
                RectangleLike {
                    __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
                    __flight_entity_runtime: std::sync::Arc::clone(
                        &__flight_source.__flight_entity_runtime,
                    ),
                    __flight_entity_snapshot: __flight_source.__flight_entity_snapshot.clone(),
                    height: __flight_source.height,
                    width: __flight_source.width,
                    x: __flight_source.x,
                    y: __flight_source.y,
                }
            });
            out.contours = None;
            out.winding = "nonZero".to_owned();
        } else {
            let rx = clip.rect.x;
            let ry = clip.rect.y;
            let rw = clip.rect.width;
            let rh = clip.rect.height;
            let tl_x = (((ma * rx) + (mc * ry)) + mtx);
            let tl_y = (((mb * rx) + (md * ry)) + mty);
            let tr_x = (((ma * (rx + rw)) + (mc * ry)) + mtx);
            let tr_y = (((mb * (rx + rw)) + (md * ry)) + mty);
            let br_x = (((ma * (rx + rw)) + (mc * (ry + rh))) + mtx);
            let br_y = (((mb * (rx + rw)) + (md * (ry + rh))) + mty);
            let bl_x = (((ma * rx) + (mc * (ry + rh))) + mtx);
            let bl_y = (((mb * rx) + (md * (ry + rh))) + mty);
            let quad = vec![tl_x, tl_y, tr_x, tr_y, br_x, br_y, bl_x, bl_y];
            out.contours = Some(vec![(quad).clone()]);
            out.winding = "nonZero".to_owned();
            (|| -> () {
                let mut min_x = f64::INFINITY;
                let mut min_y = f64::INFINITY;
                let mut max_x = (-f64::INFINITY);
                let mut max_y = (-f64::INFINITY);
                {
                    let mut c = 0.0_f64;
                    while (c < (vec![(quad).clone()].len() as f64)) {
                        let contour = vec![(quad).clone()][c as usize].clone();
                        if ((contour.len() as f64) < 6.0_f64)
                            || ((__flight_js_to_i32((contour.len() as f64))
                                & __flight_js_to_i32(1.0_f64))
                                as f64
                                != 0.0_f64)
                        {
                            {
                                c += 1.0;
                                c
                            };
                            continue;
                        }
                        {
                            let mut i = 0.0_f64;
                            while (i < (contour.len() as f64)) {
                                let x = contour[i as usize].clone();
                                let y = contour[(i + 1.0_f64) as usize].clone();
                                if (x < min_x) {
                                    min_x = x;
                                }
                                if (x > max_x) {
                                    max_x = x;
                                }
                                if (y < min_y) {
                                    min_y = y;
                                }
                                if (y > max_y) {
                                    max_y = y;
                                }
                                {
                                    i += 2.0_f64;
                                    i.clone()
                                };
                            }
                        }
                        {
                            c += 1.0;
                            c
                        };
                    }
                }
                if (min_x > max_x) {
                    out.rect.x = 0.0_f64;
                    out.rect.y = 0.0_f64;
                    out.rect.width = 0.0_f64;
                    out.rect.height = 0.0_f64;
                    return;
                }
                out.rect.x = min_x;
                out.rect.y = min_y;
                out.rect.width = (max_x - min_x);
                out.rect.height = (max_y - min_y);
            })();
        }
    } else {
        let mut new_contours: Vec<Vec<f64>> =
            vec![Default::default(); (in_contours.as_ref().unwrap().len() as f64) as usize];
        {
            let mut c = 0.0_f64;
            while (c < (in_contours.as_ref().unwrap().len() as f64)) {
                let src = in_contours.as_ref().unwrap()[c as usize].clone();
                let mut dst: Vec<f64> = vec![Default::default(); (src.len() as f64) as usize];
                {
                    let mut i = 0.0_f64;
                    while (i < (src.len() as f64)) {
                        let ox = src[i as usize].clone();
                        let oy = src[(i + 1.0_f64) as usize].clone();
                        {
                            let __flight_index = (i) as usize;
                            let __flight_value = (((ma * ox) + (mc * oy)) + mtx);
                            if __flight_index == dst.len() {
                                dst.push(__flight_value);
                            } else {
                                dst[__flight_index] = __flight_value;
                            }
                        };
                        {
                            let __flight_index = (i + 1.0_f64) as usize;
                            let __flight_value = (((mb * ox) + (md * oy)) + mty);
                            if __flight_index == dst.len() {
                                dst.push(__flight_value);
                            } else {
                                dst[__flight_index] = __flight_value;
                            }
                        };
                        {
                            i += 2.0_f64;
                            i.clone()
                        };
                    }
                }
                {
                    let __flight_index = (c) as usize;
                    let __flight_value = (dst).clone();
                    if __flight_index == new_contours.len() {
                        new_contours.push(__flight_value);
                    } else {
                        new_contours[__flight_index] = __flight_value;
                    }
                };
                {
                    c += 1.0;
                    c
                };
            }
        }
        out.contours = Some((new_contours).clone());
        out.winding = (in_winding).clone();
        (|| -> () {
            let mut min_x = f64::INFINITY;
            let mut min_y = f64::INFINITY;
            let mut max_x = (-f64::INFINITY);
            let mut max_y = (-f64::INFINITY);
            {
                let mut c = 0.0_f64;
                while (c < (new_contours.len() as f64)) {
                    let contour = new_contours[c as usize].clone();
                    if ((contour.len() as f64) < 6.0_f64)
                        || ((__flight_js_to_i32((contour.len() as f64))
                            & __flight_js_to_i32(1.0_f64)) as f64
                            != 0.0_f64)
                    {
                        {
                            c += 1.0;
                            c
                        };
                        continue;
                    }
                    {
                        let mut i = 0.0_f64;
                        while (i < (contour.len() as f64)) {
                            let x = contour[i as usize].clone();
                            let y = contour[(i + 1.0_f64) as usize].clone();
                            if (x < min_x) {
                                min_x = x;
                            }
                            if (x > max_x) {
                                max_x = x;
                            }
                            if (y < min_y) {
                                min_y = y;
                            }
                            if (y > max_y) {
                                max_y = y;
                            }
                            {
                                i += 2.0_f64;
                                i.clone()
                            };
                        }
                    }
                    {
                        c += 1.0;
                        c
                    };
                }
            }
            if (min_x > max_x) {
                out.rect.x = 0.0_f64;
                out.rect.y = 0.0_f64;
                out.rect.width = 0.0_f64;
                out.rect.height = 0.0_f64;
                return;
            }
            out.rect.x = min_x;
            out.rect.y = min_y;
            out.rect.width = (max_x - min_x);
            out.rect.height = (max_y - min_y);
        })();
    }
    out.version =
        (__flight_js_to_u32((out.version + 1.0_f64)) >> (__flight_js_to_u32(0.0_f64) & 31)) as f64;
}

// Source: upstream/packages/clip/src/clipRegion.ts:563 (sha256:719264e7695d5e09d75632a0261dc478f85cdbc55b308dc744d1cfc17b0b0bab)
pub fn union_clip_regions(out: &mut ClipRegion, a: &ClipRegion, b: &ClipRegion) -> () {
    guard_clip_region_use(out);
    guard_clip_region_use(a);
    guard_clip_region_use(b);
    let a_contours = (a.contours).clone();
    let b_contours = (b.contours).clone();
    let a_winding = (a.winding).clone();
    let b_winding = (b.winding).clone();
    merge_rectangle(
        &mut out.rect,
        &{
            let __flight_source = &(a.rect);
            RectangleLike {
                __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
                __flight_entity_runtime: std::sync::Arc::clone(
                    &__flight_source.__flight_entity_runtime,
                ),
                __flight_entity_snapshot: __flight_source.__flight_entity_snapshot.clone(),
                height: __flight_source.height,
                width: __flight_source.width,
                x: __flight_source.x,
                y: __flight_source.y,
            }
        },
        &{
            let __flight_source = &(b.rect);
            RectangleLike {
                __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
                __flight_entity_runtime: std::sync::Arc::clone(
                    &__flight_source.__flight_entity_runtime,
                ),
                __flight_entity_snapshot: __flight_source.__flight_entity_snapshot.clone(),
                height: __flight_source.height,
                width: __flight_source.width,
                x: __flight_source.x,
                y: __flight_source.y,
            }
        },
    );
    if ((a_contours).is_none()) && ((b_contours).is_none()) {
        out.contours = None;
        out.winding = "nonZero".to_owned();
    } else {
        if ((a_contours).is_some()) && ((b_contours).is_none()) {
            out.contours = Some(
                (a_contours.as_ref().unwrap())
                    .iter()
                    .cloned()
                    .map(|c: Vec<f64>| -> Vec<f64> { (c).clone() })
                    .collect::<Vec<_>>(),
            );
            out.winding = (a_winding).clone();
        } else {
            if ((a_contours).is_none()) && ((b_contours).is_some()) {
                out.contours = Some(
                    (b_contours.as_ref().unwrap())
                        .iter()
                        .cloned()
                        .map(|c: Vec<f64>| -> Vec<f64> { (c).clone() })
                        .collect::<Vec<_>>(),
                );
                out.winding = (b_winding).clone();
            } else {
                let keep_a = ((a_contours.as_ref().unwrap().len() as f64)
                    >= (b_contours.as_ref().unwrap().len() as f64));
                out.contours = Some(
                    (if keep_a {
                        (a_contours).clone()
                    } else {
                        (b_contours).clone()
                    }
                    .as_ref()
                    .unwrap())
                    .iter()
                    .cloned()
                    .map(|c: Vec<f64>| -> Vec<f64> { (c).clone() })
                    .collect::<Vec<_>>(),
                );
                out.winding = if keep_a {
                    (a_winding).clone()
                } else {
                    (b_winding).clone()
                };
            }
        }
    }
    out.version =
        (__flight_js_to_u32((out.version + 1.0_f64)) >> (__flight_js_to_u32(0.0_f64) & 31)) as f64;
}

// Source: upstream/packages/clip/src/clipRegion.ts:598 (sha256:6b1600a4654adbc8826ce84b24365a41d8698cca8957d72488f536c23316cf84)
const NORMALIZE_EPSILON: f64 = 0.000001_f64;

// Source: upstream/packages/clip/src/clipRegion.ts:602 (sha256:953b91ed5c1058a15614034ecd10d4be21df3e76ef003ee1519ee2a9f2c06c51)
static CLIP_REGION_POOL: std::sync::LazyLock<std::sync::Mutex<Vec<ClipRegion>>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(vec![]));

// Source: upstream/packages/clip/src/clipRegion.ts:604 (sha256:39fd740820b20b6fe4dfd72a3d944127590af9ba92a35adb7aef30f20bb64f5d)
static _CONTOURS_GUARD: std::sync::LazyLock<std::sync::Mutex<Option<ClipRegionContoursGuard>>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(None));

// Source: upstream/packages/clip/src/clipRegion.ts:605 (sha256:e1830f0353f013df27b3fb93d99fa0d01a9a4d5a672f7f68bf59d360066fe60f)
static _RELEASE_GUARD: std::sync::LazyLock<std::sync::Mutex<Option<ClipRegionReleaseGuard>>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(None));

// Source: upstream/packages/clip/src/clipRegion.ts:606 (sha256:7b706a5dc6cebb41e9b8560aaec8183bc6c8e6013c79d4cce2498ff6897652db)
static _USE_GUARD: std::sync::LazyLock<std::sync::Mutex<Option<ClipRegionUseGuard>>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(None));

// Source: upstream/packages/clip/src/clipRegion.ts:608 (sha256:4a0d5b16702eed7af84e960081d953a3b76dae8cc664dd517a51bc7475e4b0e5)
fn make_empty_clip_region() -> ClipRegion {
    let mut out = allocate_entity();
    crate::host_set("host.contours", None);
    crate::host_set("host.rect", create_rectangle(None, None, None, None));
    crate::host_set("host.version", 0.0_f64);
    crate::host_set("host.winding", "nonZero");
    return finish_entity((out).clone());
}

// Source: upstream/packages/clip/src/clipRegion.ts:617 (sha256:7ccbc62301090826d8149af320170a729a3effcd5f6ab1cc92e3fa528ed4f334)
fn guard_clip_region_use(clip: &ClipRegion) -> () {
    if (((*_USE_GUARD.lock().unwrap()).clone()).is_some())
        && ({
            let __flight_value = (*clip).clone();
            (CLIP_REGION_POOL.lock().unwrap())
                .iter()
                .any(|item| item == &__flight_value)
        })
    {
        {
            let __flight_callback = ((*_USE_GUARD.lock().unwrap()).as_ref().unwrap()).clone();
            __flight_callback.lock().unwrap()((*clip).clone())
        };
    }
}

// Source: upstream/packages/clip/src/clipRegion.ts:623 (sha256:142523eaff6a5e7d402a9815a3f91cccf5c22467e0346f866cc512bf6e4989d2)
fn point_in_contours(contours: &Vec<Vec<f64>>, winding: PathWinding, px: f64, py: f64) -> bool {
    let mut winding_number = 0.0_f64;
    {
        let mut c = 0.0_f64;
        while (c < (contours.len() as f64)) {
            let contour = contours[c as usize].clone();
            let n = (contour.len() as f64);
            if (n < 6.0_f64)
                || ((__flight_js_to_i32(n) & __flight_js_to_i32(1.0_f64)) as f64 != 0.0_f64)
            {
                {
                    c += 1.0;
                    c
                };
                continue;
            }
            {
                let mut i = 0.0_f64;
                while (i < n) {
                    let x0 = contour[i as usize].clone();
                    let y0 = contour[(i + 1.0_f64) as usize].clone();
                    let x1 = contour[((i + 2.0_f64) % n) as usize].clone();
                    let y1 = contour[((i + 3.0_f64) % n) as usize].clone();
                    if (y0 <= py) {
                        if (y1 > py) {
                            if ((((x1 - x0) * (py - y0)) - ((px - x0) * (y1 - y0))) > 0.0_f64) {
                                {
                                    winding_number += 1.0;
                                    winding_number
                                };
                            }
                        }
                    } else {
                        if (y1 <= py) {
                            if ((((x1 - x0) * (py - y0)) - ((px - x0) * (y1 - y0))) < 0.0_f64) {
                                {
                                    winding_number -= 1.0;
                                    winding_number
                                };
                            }
                        }
                    }
                    {
                        i += 2.0_f64;
                        i.clone()
                    };
                }
            }
            {
                c += 1.0;
                c
            };
        }
    }
    if (winding == "evenOdd") {
        return ((__flight_js_to_i32(winding_number) & __flight_js_to_i32(1.0_f64)) as f64
            != 0.0_f64);
    }
    return (winding_number != 0.0_f64);
}

// Source: upstream/packages/clip/src/clipRegion.ts:662 (sha256:32c4e283d2a7dc643965dbed5ae7a215575d6869984023a0258d2a55b3d8a519)
fn append_circle_to_path(path: &mut Path, cx: f64, cy: f64, r: f64) -> () {
    let k = (r * circle_kappa_constant);
    append_path_move_to(path, cx, (cy - r));
    append_path_cubic_curve_to(path, (cx + k), (cy - r), (cx + r), (cy - k), (cx + r), cy);
    append_path_cubic_curve_to(path, (cx + r), (cy + k), (cx + k), (cy + r), cx, (cy + r));
    append_path_cubic_curve_to(path, (cx - k), (cy + r), (cx - r), (cy + k), (cx - r), cy);
    append_path_cubic_curve_to(path, (cx - r), (cy - k), (cx - k), (cy - r), cx, (cy - r));
}

// Source: upstream/packages/clip/src/clipRegion.ts:671 (sha256:ad3e6aeddeb6eaa106fbe41572dadd98945e8d80e359bd76a9d876eb3ecb80be)
fn append_ellipse_to_path(path: &mut Path, x: f64, y: f64, w: f64, h: f64) -> () {
    let cx = (x + (w / 2.0_f64));
    let cy = (y + (h / 2.0_f64));
    let rx = (w / 2.0_f64);
    let ry = (h / 2.0_f64);
    let kx = (rx * circle_kappa_constant);
    let ky = (ry * circle_kappa_constant);
    append_path_move_to(path, cx, (cy - ry));
    append_path_cubic_curve_to(
        path,
        (cx + kx),
        (cy - ry),
        (cx + rx),
        (cy - ky),
        (cx + rx),
        cy,
    );
    append_path_cubic_curve_to(
        path,
        (cx + rx),
        (cy + ky),
        (cx + kx),
        (cy + ry),
        cx,
        (cy + ry),
    );
    append_path_cubic_curve_to(
        path,
        (cx - kx),
        (cy + ry),
        (cx - rx),
        (cy + ky),
        (cx - rx),
        cy,
    );
    append_path_cubic_curve_to(
        path,
        (cx - rx),
        (cy - ky),
        (cx - kx),
        (cy - ry),
        cx,
        (cy - ry),
    );
}

// Source: upstream/packages/clip/src/clipRegion.ts:685 (sha256:4295c327f27979bfbb44d23d505d2ce612a6b49a036697c177bfdaf832db0dab)
fn append_rounded_rect_to_path(path: &mut Path, x: f64, y: f64, w: f64, h: f64, r: f64) -> () {
    let max_r = ((w).min(h) / 2.0_f64);
    let cr = (r).min(max_r);
    let k = (cr * circle_kappa_constant);
    let x1 = (x + cr);
    let x2 = ((x + w) - cr);
    let y1 = (y + cr);
    let y2 = ((y + h) - cr);
    append_path_move_to(path, x1, y);
    append_path_line_to(path, x2, y);
    append_path_cubic_curve_to(path, (x2 + k), y, (x + w), (y1 - k), (x + w), y1);
    append_path_line_to(path, (x + w), y2);
    append_path_cubic_curve_to(path, (x + w), (y2 + k), (x2 + k), (y + h), x2, (y + h));
    append_path_line_to(path, x1, (y + h));
    append_path_cubic_curve_to(path, (x1 - k), (y + h), x, (y2 + k), x, y2);
    append_path_line_to(path, x, y1);
    append_path_cubic_curve_to(path, x, (y1 - k), (x1 - k), y, x1, y);
}

// Source: upstream/packages/clip/src/clipRegion.ts:704 (sha256:835b82a98898f01e6714573b8486d1e8c5e2ea4a38b1495283a91423dd3b7013)
fn set_rectangle_to_contours_bounds(out: &mut RectangleLike, contours: &Vec<Vec<f64>>) -> () {
    let mut min_x = f64::INFINITY;
    let mut min_y = f64::INFINITY;
    let mut max_x = (-f64::INFINITY);
    let mut max_y = (-f64::INFINITY);
    {
        let mut c = 0.0_f64;
        while (c < (contours.len() as f64)) {
            let contour = contours[c as usize].clone();
            if ((contour.len() as f64) < 6.0_f64)
                || ((__flight_js_to_i32((contour.len() as f64)) & __flight_js_to_i32(1.0_f64))
                    as f64
                    != 0.0_f64)
            {
                {
                    c += 1.0;
                    c
                };
                continue;
            }
            {
                let mut i = 0.0_f64;
                while (i < (contour.len() as f64)) {
                    let x = contour[i as usize].clone();
                    let y = contour[(i + 1.0_f64) as usize].clone();
                    if (x < min_x) {
                        min_x = x;
                    }
                    if (x > max_x) {
                        max_x = x;
                    }
                    if (y < min_y) {
                        min_y = y;
                    }
                    if (y > max_y) {
                        max_y = y;
                    }
                    {
                        i += 2.0_f64;
                        i.clone()
                    };
                }
            }
            {
                c += 1.0;
                c
            };
        }
    }
    if (min_x > max_x) {
        out.x = 0.0_f64;
        out.y = 0.0_f64;
        out.width = 0.0_f64;
        out.height = 0.0_f64;
        return;
    }
    out.x = min_x;
    out.y = min_y;
    out.width = (max_x - min_x);
    out.height = (max_y - min_y);
}
