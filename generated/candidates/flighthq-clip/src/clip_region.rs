// @generated from upstream/packages/clip/src/clipRegion.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

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
    ClipRegion, ClipRegionReleaseGuard, MatrixLike, Path, PathWinding, RectangleLike,
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

// Source: upstream/packages/clip/src/clipRegion.ts:32 (sha256:9bd8a98dcb93e1f3dafd093011d6275b352395814339f972aff4b903dff5fe15)
pub fn acquire_clip_region() -> ClipRegion {
    let mut region = CLIP_REGION_POOL
        .lock()
        .unwrap()
        .pop()
        .expect("TypeScript Array.pop returned undefined");
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

// Source: upstream/packages/clip/src/clipRegion.ts:50 (sha256:7a59178716662e1a52c5113a1b6c898fa61193e9a0b969c40b3435c1f46024d8)
pub fn clip_region_contains_point(clip: &ClipRegion, x: f64, y: f64) -> bool {
    if (!contains_rectangle_point_xy(
        &{
            let __flight_source = &(clip.rect);
            RectangleLike {
                __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
                __flight_entity_runtime: std::sync::Arc::clone(
                    &__flight_source.__flight_entity_runtime,
                ),
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

// Source: upstream/packages/clip/src/clipRegion.ts:58 (sha256:bc05f50cfaa45bc8f6657c3c73ed5b0edbec840f524721dc704249f233b929e9)
pub fn clip_region_contains_rectangle(clip: &ClipRegion, rectangle: &RectangleLike) -> bool {
    return encloses_rectangle(
        &{
            let __flight_source = &(clip.rect);
            RectangleLike {
                __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
                __flight_entity_runtime: std::sync::Arc::clone(
                    &__flight_source.__flight_entity_runtime,
                ),
                height: __flight_source.height,
                width: __flight_source.width,
                x: __flight_source.x,
                y: __flight_source.y,
            }
        },
        rectangle,
    );
}

// Source: upstream/packages/clip/src/clipRegion.ts:64 (sha256:8c3b33804cc8b658c651dd137e601da6ea8f8039849f22478253edccc50c7e70)
pub fn clip_region_intersects_rectangle(clip: &ClipRegion, rectangle: &RectangleLike) -> bool {
    return intersects_rectangle(
        &{
            let __flight_source = &(clip.rect);
            RectangleLike {
                __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
                __flight_entity_runtime: std::sync::Arc::clone(
                    &__flight_source.__flight_entity_runtime,
                ),
                height: __flight_source.height,
                width: __flight_source.width,
                x: __flight_source.x,
                y: __flight_source.y,
            }
        },
        rectangle,
    );
}

// Source: upstream/packages/clip/src/clipRegion.ts:70 (sha256:ab7a1eeb04d49249d5e43a72e3c7a1223ef820a46c339e72919d69223b3e2fa2)
pub fn clone_clip_region(clip: &ClipRegion) -> ClipRegion {
    let rect = clone_rectangle(&{
        let __flight_source = &(clip.rect);
        RectangleLike {
            __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
            __flight_entity_runtime: std::sync::Arc::clone(
                &__flight_source.__flight_entity_runtime,
            ),
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
                .map(|c: Vec<f64>| -> crate::OpaqueHostValue {
                    {
                        let __flight_portable_source = (c).clone();
                        crate::FlightValue::Array(
                            (&__flight_portable_source)
                                .iter()
                                .map(|value| crate::FlightValue::Number(*(value) as f64))
                                .collect(),
                        )
                    }
                })
                .collect::<Vec<_>>(),
        )
    };
    return ClipRegion {
        __flight_identity: std::sync::Arc::new(()),
        contours: contours,
        rect: (rect).clone(),
        version: clip.version,
        winding: (clip.winding).clone(),
    };
}

// Source: upstream/packages/clip/src/clipRegion.ts:78 (sha256:07f4b3d6d9b05a2187cea6ce26b2d21531d312237d680078415f155fa47ab403)
pub fn copy_clip_region(out: &mut ClipRegion, source: &ClipRegion) -> () {
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
                .map(|c: Vec<f64>| -> crate::OpaqueHostValue {
                    {
                        let __flight_portable_source = (c).clone();
                        crate::FlightValue::Array(
                            (&__flight_portable_source)
                                .iter()
                                .map(|value| crate::FlightValue::Number(*(value) as f64))
                                .collect(),
                        )
                    }
                })
                .collect::<Vec<_>>(),
        )
    };
    out.winding = (source.winding).clone();
    out.version =
        (__flight_js_to_u32((out.version + 1.0_f64)) >> (__flight_js_to_u32(0.0_f64) & 31)) as f64;
}

// Source: upstream/packages/clip/src/clipRegion.ts:87 (sha256:64e74b5706330628f76ed802a209caab6bee71a91ca765f58220e71b166da818)
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

// Source: upstream/packages/clip/src/clipRegion.ts:97 (sha256:9cd5e83fd878ff6afa5aca751a01d04215ef2f0dff9c546744d5665b2327ad3e)
pub fn create_clip_region_from_contours(
    contours: &Vec<Vec<f64>>,
    winding: PathWinding,
) -> ClipRegion {
    let mut rect = create_rectangle(None, None, None, None);
    set_rectangle_to_contours_bounds(&mut rect, contours);
    let owned = (contours)
        .iter()
        .cloned()
        .map(|c: Vec<f64>| -> crate::OpaqueHostValue {
            {
                let __flight_portable_source = (c).clone();
                crate::FlightValue::Array(
                    (&__flight_portable_source)
                        .iter()
                        .map(|value| crate::FlightValue::Number(*(value) as f64))
                        .collect(),
                )
            }
        })
        .collect::<Vec<_>>();
    return ClipRegion {
        __flight_identity: std::sync::Arc::new(()),
        contours: Some(owned),
        rect: (rect).clone(),
        version: 0.0_f64,
        winding: (winding).clone(),
    };
}

// Source: upstream/packages/clip/src/clipRegion.ts:109 (sha256:9f646d72c2ea5a6d574b1ce5b3f6f6354a89f2e823a4a6c2596099a2f0dcb6ac)
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

// Source: upstream/packages/clip/src/clipRegion.ts:119 (sha256:071c762e6a69df2d5bd9a6da45d4c3c608092e0fae4c952991ec2955fccdba2f)
pub fn create_clip_region_from_path(path: &Path, tolerance: Option<f64>) -> ClipRegion {
    let tolerance = tolerance.unwrap_or(0.25_f64);
    let contours = flatten_path(path, Some(tolerance));
    let mut rect = create_rectangle(None, None, None, None);
    set_rectangle_to_contours_bounds(&mut rect, &contours);
    return ClipRegion {
        __flight_identity: std::sync::Arc::new(()),
        contours: Some((contours).clone()),
        rect: (rect).clone(),
        version: 0.0_f64,
        winding: (path.winding).clone(),
    };
}

// Source: upstream/packages/clip/src/clipRegion.ts:128 (sha256:4cf76946e9d73b2fdbd9d0356a973733dfd549da004338df0b635d1419824e75)
pub fn create_clip_region_from_rectangle(rectangle: &RectangleLike) -> ClipRegion {
    return ClipRegion {
        __flight_identity: std::sync::Arc::new(()),
        contours: None,
        rect: clone_rectangle(rectangle),
        version: 0.0_f64,
        winding: "nonZero".to_owned(),
    };
}

// Source: upstream/packages/clip/src/clipRegion.ts:134 (sha256:a799cca9ae977991e2400ad11363335fa6809b104bde6bb25a741cd35c51a16e)
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

// Source: upstream/packages/clip/src/clipRegion.ts:147 (sha256:076bb4c5a0c8afc34f3c1ddf09f9a10b8152c929b2ca8d2eebc61048a03837a7)
pub fn equals_clip_region(a: &ClipRegion, b: &ClipRegion) -> bool {
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

// Source: upstream/packages/clip/src/clipRegion.ts:170 (sha256:e3a11d0a7fa5ad9ddefdcb92b9e38b673c4710b87ea50fabc0dfd8a2d69e105a)
pub fn get_clip_region_bounds(out: &mut RectangleLike, clip: &ClipRegion) -> () {
    out.x = clip.rect.x;
    out.y = clip.rect.y;
    out.width = clip.rect.width;
    out.height = clip.rect.height;
}

// Source: upstream/packages/clip/src/clipRegion.ts:182 (sha256:c28abc3e7b4a98760fdf0bff5eb77881218959eed7c3f1f94bbb83e07f5406b4)
pub fn intersect_clip_regions(out: &mut ClipRegion, a: &ClipRegion, b: &ClipRegion) -> () {
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
                    .map(|c: Vec<f64>| -> crate::OpaqueHostValue {
                        {
                            let __flight_portable_source = (c).clone();
                            crate::FlightValue::Array(
                                (&__flight_portable_source)
                                    .iter()
                                    .map(|value| crate::FlightValue::Number(*(value) as f64))
                                    .collect(),
                            )
                        }
                    })
                    .collect::<Vec<_>>(),
            );
            out.winding = (a_winding).clone();
        } else {
            if ((a_contours).is_none()) && ((b_contours).is_some()) {
                out.contours = Some(
                    (b_contours.as_ref().unwrap())
                        .iter()
                        .cloned()
                        .map(|c: Vec<f64>| -> crate::OpaqueHostValue {
                            {
                                let __flight_portable_source = (c).clone();
                                crate::FlightValue::Array(
                                    (&__flight_portable_source)
                                        .iter()
                                        .map(|value| crate::FlightValue::Number(*(value) as f64))
                                        .collect(),
                                )
                            }
                        })
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
                    .map(|c: Vec<f64>| -> crate::OpaqueHostValue {
                        {
                            let __flight_portable_source = (c).clone();
                            crate::FlightValue::Array(
                                (&__flight_portable_source)
                                    .iter()
                                    .map(|value| crate::FlightValue::Number(*(value) as f64))
                                    .collect(),
                            )
                        }
                    })
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

// Source: upstream/packages/clip/src/clipRegion.ts:245 (sha256:cf1212730548f45c6b8f6c73e8ca1d70261f2299393b7951a2f7849d76dc1e60)
pub fn invalidate_clip_region(clip: &mut ClipRegion) -> () {
    clip.version =
        (__flight_js_to_u32((clip.version + 1.0_f64)) >> (__flight_js_to_u32(0.0_f64) & 31)) as f64;
}

// Source: upstream/packages/clip/src/clipRegion.ts:251 (sha256:7a43038b37ceffae64da4f681a143bdeaa2c7545e4af66bb6c413e8fe9bc7e96)
pub fn is_clip_region_empty(clip: &ClipRegion) -> bool {
    if is_empty_rectangle(&{
        let __flight_source = &(clip.rect);
        RectangleLike {
            __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
            __flight_entity_runtime: std::sync::Arc::clone(
                &__flight_source.__flight_entity_runtime,
            ),
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

// Source: upstream/packages/clip/src/clipRegion.ts:258 (sha256:8df1bcc26a92928ad8fa3b6673f7daadfba5d6bc8bce20c19f9ca002374ba2e6)
pub fn is_clip_region_rectangular(clip: &ClipRegion) -> bool {
    return ((clip.contours).clone()).is_none();
}

// Source: upstream/packages/clip/src/clipRegion.ts:269 (sha256:f81b44d88a8b1851186a39f15f257c75cec24c77e2f4132c19bd1caea3cfe55a)
pub fn normalize_clip_region(out: &mut ClipRegion, clip: &ClipRegion) -> () {
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
            .map(|c: Vec<f64>| -> crate::OpaqueHostValue {
                {
                    let __flight_portable_source = (c).clone();
                    crate::FlightValue::Array(
                        (&__flight_portable_source)
                            .iter()
                            .map(|value| crate::FlightValue::Number(*(value) as f64))
                            .collect(),
                    )
                }
            })
            .collect::<Vec<_>>(),
    );
    out.winding = (in_winding).clone();
    out.version =
        (__flight_js_to_u32((out.version + 1.0_f64)) >> (__flight_js_to_u32(0.0_f64) & 31)) as f64;
}

// Source: upstream/packages/clip/src/clipRegion.ts:341 (sha256:b60719b33f8ca88b3d8b414f1d26a9a8fca380cae5e2f39973a8ae110e31793a)
pub fn release_clip_region(clip: &ClipRegion) -> () {
    if (((*_RELEASE_GUARD.lock().unwrap()).clone()).is_some())
        && ({
            let __flight_value = (*clip).clone();
            (CLIP_REGION_POOL)
                .iter()
                .any(|item| item == &__flight_value)
        })
    {
        {
            let __flight_callback = ((*_RELEASE_GUARD.lock().unwrap()).as_ref().unwrap()).clone();
            let __flight_result = __flight_callback.lock().unwrap()((*clip).clone());
            __flight_result
        };
    }
    CLIP_REGION_POOL
        .lock()
        .unwrap()
        .push(((*clip).clone()).clone());
}

// Source: upstream/packages/clip/src/clipRegion.ts:348 (sha256:ddcc96d5cedaa98bc154b844117c856eecc7e217412bf41bae5ade375531b657)
pub fn set_clip_region_release_guard(guard: &Option<ClipRegionReleaseGuard>) -> () {
    (*_RELEASE_GUARD.lock().unwrap()) = (*guard).clone();
}

// Source: upstream/packages/clip/src/clipRegion.ts:352 (sha256:e1830f0353f013df27b3fb93d99fa0d01a9a4d5a672f7f68bf59d360066fe60f)
static _RELEASE_GUARD: std::sync::LazyLock<std::sync::Mutex<Option<ClipRegionReleaseGuard>>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(None));

// Source: upstream/packages/clip/src/clipRegion.ts:356 (sha256:735a4aa9dc416720c9953159b297b52d877041cf192e1307755da225ad6b21de)
pub fn set_clip_region_to_rectangle(out: &mut ClipRegion, rectangle: &RectangleLike) -> () {
    copy_rectangle(&mut out.rect, rectangle);
    out.contours = None;
    out.winding = "nonZero".to_owned();
    out.version =
        (__flight_js_to_u32((out.version + 1.0_f64)) >> (__flight_js_to_u32(0.0_f64) & 31)) as f64;
}

// Source: upstream/packages/clip/src/clipRegion.ts:368 (sha256:11d785f5afbd65075fabaeb2a1d69714e7dc784117829fc08c67470c6334f051)
pub fn transform_clip_region(out: &mut ClipRegion, clip: &ClipRegion, matrix: &MatrixLike) -> () {
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
            set_rectangle_to_contours_bounds(&mut out.rect, &vec![quad]);
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
        set_rectangle_to_contours_bounds(&mut out.rect, &new_contours);
    }
    out.version =
        (__flight_js_to_u32((out.version + 1.0_f64)) >> (__flight_js_to_u32(0.0_f64) & 31)) as f64;
}

// Source: upstream/packages/clip/src/clipRegion.ts:432 (sha256:83a3038200932c2362e84e56b6888892402a2802f921480ac6804bc316930a41)
pub fn union_clip_regions(out: &mut ClipRegion, a: &ClipRegion, b: &ClipRegion) -> () {
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
                    .map(|c: Vec<f64>| -> crate::OpaqueHostValue {
                        {
                            let __flight_portable_source = (c).clone();
                            crate::FlightValue::Array(
                                (&__flight_portable_source)
                                    .iter()
                                    .map(|value| crate::FlightValue::Number(*(value) as f64))
                                    .collect(),
                            )
                        }
                    })
                    .collect::<Vec<_>>(),
            );
            out.winding = (a_winding).clone();
        } else {
            if ((a_contours).is_none()) && ((b_contours).is_some()) {
                out.contours = Some(
                    (b_contours.as_ref().unwrap())
                        .iter()
                        .cloned()
                        .map(|c: Vec<f64>| -> crate::OpaqueHostValue {
                            {
                                let __flight_portable_source = (c).clone();
                                crate::FlightValue::Array(
                                    (&__flight_portable_source)
                                        .iter()
                                        .map(|value| crate::FlightValue::Number(*(value) as f64))
                                        .collect(),
                                )
                            }
                        })
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
                    .map(|c: Vec<f64>| -> crate::OpaqueHostValue {
                        {
                            let __flight_portable_source = (c).clone();
                            crate::FlightValue::Array(
                                (&__flight_portable_source)
                                    .iter()
                                    .map(|value| crate::FlightValue::Number(*(value) as f64))
                                    .collect(),
                            )
                        }
                    })
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

// Source: upstream/packages/clip/src/clipRegion.ts:464 (sha256:6b1600a4654adbc8826ce84b24365a41d8698cca8957d72488f536c23316cf84)
const NORMALIZE_EPSILON: f64 = 0.000001_f64;

// Source: upstream/packages/clip/src/clipRegion.ts:468 (sha256:953b91ed5c1058a15614034ecd10d4be21df3e76ef003ee1519ee2a9f2c06c51)
static CLIP_REGION_POOL: std::sync::LazyLock<std::sync::Mutex<Vec<ClipRegion>>> =
    std::sync::LazyLock::new(|| std::sync::Mutex::new(vec![]));

// Source: upstream/packages/clip/src/clipRegion.ts:470 (sha256:307a5fe83db98ca824a571274be87d91132f11be585bdb1b3487eb5e8bd5b3b0)
fn make_empty_clip_region() -> ClipRegion {
    return ClipRegion {
        __flight_identity: std::sync::Arc::new(()),
        contours: None,
        rect: create_rectangle(None, None, None, None),
        version: 0.0_f64,
        winding: "nonZero".to_owned(),
    };
}

// Source: upstream/packages/clip/src/clipRegion.ts:476 (sha256:7118165e911c7cb6dd6474d722a3838ba5c2e66fb0c87c4d87184298183d9d28)
fn point_in_contours(contours: &Vec<Vec<f64>>, winding: PathWinding, px: f64, py: f64) -> bool {
    let mut winding_number = 0.0_f64;
    {
        let mut c = 0.0_f64;
        while (c < (contours.len() as f64)) {
            let contour = contours[c as usize].clone();
            let n = (contour.len() as f64);
            if (n < 4.0_f64) {
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

// Source: upstream/packages/clip/src/clipRegion.ts:515 (sha256:32c4e283d2a7dc643965dbed5ae7a215575d6869984023a0258d2a55b3d8a519)
fn append_circle_to_path(path: &mut Path, cx: f64, cy: f64, r: f64) -> () {
    let k = (r * circle_kappa_constant);
    append_path_move_to(path, cx, (cy - r));
    append_path_cubic_curve_to(path, (cx + k), (cy - r), (cx + r), (cy - k), (cx + r), cy);
    append_path_cubic_curve_to(path, (cx + r), (cy + k), (cx + k), (cy + r), cx, (cy + r));
    append_path_cubic_curve_to(path, (cx - k), (cy + r), (cx - r), (cy + k), (cx - r), cy);
    append_path_cubic_curve_to(path, (cx - r), (cy - k), (cx - k), (cy - r), cx, (cy - r));
}

// Source: upstream/packages/clip/src/clipRegion.ts:524 (sha256:ad3e6aeddeb6eaa106fbe41572dadd98945e8d80e359bd76a9d876eb3ecb80be)
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

// Source: upstream/packages/clip/src/clipRegion.ts:538 (sha256:4295c327f27979bfbb44d23d505d2ce612a6b49a036697c177bfdaf832db0dab)
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

// Source: upstream/packages/clip/src/clipRegion.ts:557 (sha256:7f7a3cb695e49db6dc5a6d56dca443b141f97aca34142fd65dfd14485843d2c1)
fn set_rectangle_to_contours_bounds(out: &mut RectangleLike, contours: &Vec<Vec<f64>>) -> () {
    let mut min_x = f64::INFINITY;
    let mut min_y = f64::INFINITY;
    let mut max_x = (-f64::INFINITY);
    let mut max_y = (-f64::INFINITY);
    {
        let mut c = 0.0_f64;
        while (c < (contours.len() as f64)) {
            let contour = contours[c as usize].clone();
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
