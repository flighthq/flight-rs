// @generated from upstream/packages/path/src/pathMorphGeometry.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::for_each_path_segment;
use flighthq_types::{Path, PathCommand, PathMorph, PathSegment};

// Source: upstream/packages/path/src/pathMorphGeometry.ts:6 (sha256:d4e58b8909ce3e0147800275d9160ab08d4c83a9d363722c7f78189051bbe1d9)
pub const PATH_MORPH_ISSUE_NONE: f64 = 0.0_f64;

// Source: upstream/packages/path/src/pathMorphGeometry.ts:7 (sha256:00ed7aeadc294809e7849bdb9a7acb1d192a09246932ba091f3e12962edaabf8)
pub const PATH_MORPH_ISSUE_WINDING_MISMATCH: f64 = 1.0_f64;

// Source: upstream/packages/path/src/pathMorphGeometry.ts:8 (sha256:37e02692fe374dc13b3a0fcc22dc51e89cad8bcdd3d62824573c0051c785230b)
pub const PATH_MORPH_ISSUE_CONTOUR_COUNT_MISMATCH: f64 = 2.0_f64;

// Source: upstream/packages/path/src/pathMorphGeometry.ts:9 (sha256:ab3db316e022353f90dda27fa2e829014ab2f877989f17531532b8bc8b11a8db)
pub const PATH_MORPH_ISSUE_CONTOUR_CLOSEDNESS_MISMATCH: f64 = 3.0_f64;

// Source: upstream/packages/path/src/pathMorphGeometry.ts:10 (sha256:cc6879ec38ba6ed2fabbed910fc06e0282a7fbc52ef1a1d253110835331963f3)
pub const PATH_MORPH_ISSUE_CONTOUR_ORIENTATION_MISMATCH: f64 = 4.0_f64;

// Source: upstream/packages/path/src/pathMorphGeometry.ts:12 (sha256:b4cc1b651e98b866ade30255491cfab8e68a4dc432bec53e0ccb36f4177baad2)
#[derive(Clone, Default)]
pub(crate) struct PathMorphBuildResult {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub contour: Option<f64>,
    pub issue: f64,
    pub morph: Option<PathMorph>,
}
impl PartialEq for PathMorphBuildResult {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/path/src/pathMorphGeometry.ts:22 (sha256:2fe4210904798753455dc31e99dcb9ffb9a131b7ed11093dca22311cee9c40b5)
pub fn build_path_morph(start: &Path, end: &Path) -> PathMorphBuildResult {
    if ((start.winding).clone() != (end.winding).clone()) {
        return PathMorphBuildResult {
            __flight_identity: std::sync::Arc::new(()),
            contour: None,
            issue: PATH_MORPH_ISSUE_WINDING_MISMATCH,
            morph: None,
        };
    }
    let mut start_contours = decode_cubic_contours(start);
    let mut end_contours = decode_cubic_contours(end);
    if ((start_contours.len() as f64) != (end_contours.len() as f64)) {
        return PathMorphBuildResult {
            __flight_identity: std::sync::Arc::new(()),
            contour: None,
            issue: PATH_MORPH_ISSUE_CONTOUR_COUNT_MISMATCH,
            morph: None,
        };
    }
    {
        let mut i = 0.0_f64;
        while (i < (start_contours.len() as f64)) {
            if (start_contours[i as usize].closed != end_contours[i as usize].closed) {
                return PathMorphBuildResult {
                    __flight_identity: std::sync::Arc::new(()),
                    contour: Some(i),
                    issue: PATH_MORPH_ISSUE_CONTOUR_CLOSEDNESS_MISMATCH,
                    morph: None,
                };
            }
            {
                i += 1.0;
                i
            };
        }
    }
    let orientation_mismatch = normalize_cubic_contour_orientations(
        &start_contours,
        &mut end_contours,
        (start.winding).clone(),
    );
    if (orientation_mismatch).is_some() {
        return PathMorphBuildResult {
            __flight_identity: std::sync::Arc::new(()),
            contour: Some(*(orientation_mismatch.as_ref().unwrap())),
            issue: PATH_MORPH_ISSUE_CONTOUR_ORIENTATION_MISMATCH,
            morph: None,
        };
    }
    {
        let mut i = 0.0_f64;
        while (i < (start_contours.len() as f64)) {
            let mut start_contour = start_contours[i as usize].clone();
            let mut end_contour = end_contours[i as usize].clone();
            equalize_cubic_contour_segments(&mut start_contour, &mut end_contour);
            align_closed_cubic_contour(&start_contour, &mut end_contour);
            {
                i += 1.0;
                i
            };
        }
    }
    let mut commands: Vec<f64> = vec![];
    let mut start_data: Vec<f64> = vec![];
    let mut end_data: Vec<f64> = vec![];
    {
        let mut i = 0.0_f64;
        while (i < (start_contours.len() as f64)) {
            append_cubic_contour_pair(
                &mut commands,
                &mut start_data,
                &mut end_data,
                &start_contours[i as usize],
                &end_contours[i as usize],
            );
            {
                i += 1.0;
                i
            };
        }
    }
    return PathMorphBuildResult {
        __flight_identity: std::sync::Arc::new(()),
        contour: None,
        issue: PATH_MORPH_ISSUE_NONE,
        morph: Some(PathMorph {
            __flight_identity: std::sync::Arc::new(()),
            commands: (commands).clone(),
            end_data: (end_data).clone(),
            start_data: (start_data).clone(),
            winding: (start.winding).clone(),
        }),
    };
}

// Source: upstream/packages/path/src/pathMorphGeometry.ts:69 (sha256:03e90eca31d908acacce12ab132a1423827e5be76ee3c9d19412eaf6b5440a52)
fn normalize_cubic_contour_orientations(
    start: &Vec<CubicContour>,
    end: &mut Vec<CubicContour>,
    winding: String,
) -> Option<f64> {
    if (winding == "evenOdd") {
        {
            let mut i = 0.0_f64;
            while (i < (start.len() as f64)) {
                let start_orientation = get_cubic_contour_orientation(&start[i as usize]);
                let end_orientation = get_cubic_contour_orientation(&end[i as usize]);
                if ((start_orientation != 0.0_f64) && (end_orientation != 0.0_f64))
                    && (start_orientation != end_orientation)
                {
                    reverse_closed_cubic_contour(&mut end[i as usize]);
                }
                {
                    i += 1.0;
                    i
                };
            }
        }
        return None;
    }
    let mut reverse_end: Option<bool> = None;
    {
        let mut i = 0.0_f64;
        while (i < (start.len() as f64)) {
            let start_orientation = get_cubic_contour_orientation(&start[i as usize]);
            let end_orientation = get_cubic_contour_orientation(&end[i as usize]);
            if (start_orientation == 0.0_f64) || (end_orientation == 0.0_f64) {
                {
                    i += 1.0;
                    i
                };
                continue;
            }
            let reversed = (start_orientation != end_orientation);
            if (reverse_end).is_none() {
                reverse_end = Some(reversed);
            } else {
                if (*(reverse_end.as_mut().unwrap()) != reversed) {
                    return Some(i);
                }
            }
            {
                i += 1.0;
                i
            };
        }
    }
    if (reverse_end) == Some(true) {
        {
            let mut i = 0.0_f64;
            while (i < (end.len() as f64)) {
                if (get_cubic_contour_orientation(&end[i as usize]) != 0.0_f64) {
                    reverse_closed_cubic_contour(&mut end[i as usize]);
                }
                {
                    i += 1.0;
                    i
                };
            }
        }
    }
    return None;
}

// Source: upstream/packages/path/src/pathMorphGeometry.ts:102 (sha256:6d45eab2f7270163c3f99f5fde0c25d0afbc1de166ca104f838bd11c09232023)
fn get_cubic_contour_orientation(contour: &CubicContour) -> f64 {
    if (!contour.closed) {
        return 0.0_f64;
    }
    let area = get_cubic_contour_signed_area(contour);
    return if (area < 0.0_f64) {
        (-1.0_f64)
    } else {
        if (area > 0.0_f64) { 1.0_f64 } else { 0.0_f64 }
    };
}

// Source: upstream/packages/path/src/pathMorphGeometry.ts:108 (sha256:2db04adfa086117973ed34f3a923651140291e71dec2c807c9e536dcfe0e1833)
fn reverse_closed_cubic_contour(contour: &mut CubicContour) -> () {
    let mut reversed: Vec<CubicSegment> = vec![];
    {
        let mut i = ((contour.segments.len() as f64) - 1.0_f64);
        while (i >= 0.0_f64) {
            let segment = contour.segments[i as usize].clone();
            reversed.push(CubicSegment {
                __flight_identity: std::sync::Arc::new(()),
                control1_x: segment.control2_x,
                control1_y: segment.control2_y,
                control2_x: segment.control1_x,
                control2_y: segment.control1_y,
                x0: segment.x1,
                x1: segment.x0,
                y0: segment.y1,
                y1: segment.y0,
            });
            {
                i -= 1.0;
                i
            };
        }
    }
    contour.segments = (reversed).clone();
    if ((reversed.len() as f64) > 0.0_f64) {
        contour.x = reversed[0.0_f64 as usize].x0;
        contour.y = reversed[0.0_f64 as usize].y0;
    }
    contour.current_x = contour.x;
    contour.current_y = contour.y;
}

// Source: upstream/packages/path/src/pathMorphGeometry.ts:135 (sha256:0df6f342d9ae6e3ceb1f99e4f83a2ded4b5d7d7d95ca3bcdd8c61a31581738f6)
fn get_cubic_contour_signed_area(contour: &CubicContour) -> f64 {
    let mut twice_area = 0.0_f64;
    let mut extent = 0.0_f64;
    {
        let mut i = 0.0_f64;
        while (i < (contour.segments.len() as f64)) {
            let segment = contour.segments[i as usize].clone();
            let x0 = (segment.x0 - contour.x);
            let control1_x = (segment.control1_x - contour.x);
            let control2_x = (segment.control2_x - contour.x);
            let x1 = (segment.x1 - contour.x);
            let y0 = (segment.y0 - contour.y);
            let control1_y = (segment.control1_y - contour.y);
            let control2_y = (segment.control2_y - contour.y);
            let y1 = (segment.y1 - contour.y);
            extent = ((((((((extent).max((x0).abs())).max((control1_x).abs()))
                .max((control2_x).abs()))
            .max((x1).abs()))
            .max((y0).abs()))
            .max((control1_y).abs()))
            .max((control2_y).abs()))
            .max((y1).abs());
            let x = get_cubic_power_coefficients(x0, control1_x, control2_x, x1);
            let y = get_cubic_power_coefficients(y0, control1_y, control2_y, y1);
            {
                let mut xi = 0.0_f64;
                while (xi < 4.0_f64) {
                    {
                        let mut yi = 1.0_f64;
                        while (yi < 4.0_f64) {
                            twice_area += ((((x[xi as usize].clone() * yi)
                                * y[yi as usize].clone())
                                - ((y[xi as usize].clone() * yi) * x[yi as usize].clone()))
                                / (xi + yi));
                            {
                                yi += 1.0;
                                yi
                            };
                        }
                    }
                    {
                        xi += 1.0;
                        xi
                    };
                }
            }
            {
                i += 1.0;
                i
            };
        }
    }
    let area = (twice_area / 2.0_f64);
    let area_epsilon = (((1.0_f64).max((extent * extent)) * f64::EPSILON) * 64.0_f64);
    return if ((area).abs() <= area_epsilon) {
        0.0_f64
    } else {
        area
    };
}

// Source: upstream/packages/path/src/pathMorphGeometry.ts:172 (sha256:1c4611f1b8c1af1d5ab605958d7ae92ac0ca4fc8b69d9fadc16a79ea7925a1f6)
fn get_cubic_power_coefficients(p0: f64, p1: f64, p2: f64, p3: f64) -> Vec<f64> {
    return vec![
        p0,
        (3.0_f64 * (p1 - p0)),
        (3.0_f64 * ((p0 - (2.0_f64 * p1)) + p2)),
        ((((-p0) + (3.0_f64 * p1)) - (3.0_f64 * p2)) + p3),
    ];
}

// Source: upstream/packages/path/src/pathMorphGeometry.ts:176 (sha256:bae7083eb2fc356ff0581fac3b4f4fb5820b7fe4150c63589c6c5081bdede083)
fn align_closed_cubic_contour(start: &CubicContour, end: &mut CubicContour) -> () {
    let count = (start.segments.len() as f64);
    if (!start.closed) || (count < 2.0_f64) {
        return;
    }
    let mut best_offset = 0.0_f64;
    let mut best_distance = f64::INFINITY;
    {
        let mut offset = 0.0_f64;
        while (offset < count) {
            let mut distance = 0.0_f64;
            {
                let mut i = 0.0_f64;
                while (i < count) {
                    let a = start.segments[i as usize].clone();
                    let b = end.segments[((i + offset) % count) as usize].clone();
                    let dx = (a.x0 - b.x0);
                    let dy = (a.y0 - b.y0);
                    distance += ((dx * dx) + (dy * dy));
                    {
                        i += 1.0;
                        i
                    };
                }
            }
            if (distance < best_distance) {
                best_distance = distance;
                best_offset = offset;
            }
            {
                offset += 1.0;
                offset
            };
        }
    }
    if (best_offset == 0.0_f64) {
        return;
    }
    end.segments = {
        let mut __flight_values = ((end.segments).clone())
            [(best_offset) as usize..(((end.segments).clone()).len() as f64) as usize]
            .to_vec();
        let __flight_concat_0 =
            ((end.segments).clone())[(0.0_f64) as usize..(best_offset) as usize].to_vec();
        __flight_values.extend(__flight_concat_0.iter().cloned());
        __flight_values
    };
    end.x = end.segments[0.0_f64 as usize].x0;
    end.y = end.segments[0.0_f64 as usize].y0;
}

// Source: upstream/packages/path/src/pathMorphGeometry.ts:201 (sha256:2b8f6524a77e1b1024d2454da29baee6859a3175ece8ddf8d7f1a6f7c4b533d8)
fn append_cubic_contour_pair(
    commands: &mut Vec<f64>,
    start_data: &mut Vec<f64>,
    end_data: &mut Vec<f64>,
    start: &CubicContour,
    end: &CubicContour,
) -> () {
    commands.push(PathCommand::MOVE_TO);
    start_data.extend(vec![start.x, start.y]);
    end_data.extend(vec![end.x, end.y]);
    {
        let mut i = 0.0_f64;
        while (i < (start.segments.len() as f64)) {
            let a = start.segments[i as usize].clone();
            let b = end.segments[i as usize].clone();
            commands.push(PathCommand::CUBIC_CURVE_TO);
            start_data.extend(vec![
                a.control1_x,
                a.control1_y,
                a.control2_x,
                a.control2_y,
                a.x1,
                a.y1,
            ]);
            end_data.extend(vec![
                b.control1_x,
                b.control1_y,
                b.control2_x,
                b.control2_y,
                b.x1,
                b.y1,
            ]);
            {
                i += 1.0;
                i
            };
        }
    }
    if start.closed {
        commands.push(PathCommand::CLOSE);
    }
}

// Source: upstream/packages/path/src/pathMorphGeometry.ts:221 (sha256:86cf2c3ed1a2e26412ccb93b71d4111a34702c3e46bbc007afa5370a8dbe874b)
fn append_cubic_segment(
    contour: &mut CubicContour,
    control1_x: f64,
    control1_y: f64,
    control2_x: f64,
    control2_y: f64,
    x: f64,
    y: f64,
) -> () {
    contour.segments.push(CubicSegment {
        __flight_identity: std::sync::Arc::new(()),
        control1_x: control1_x,
        control1_y: control1_y,
        control2_x: control2_x,
        control2_y: control2_y,
        x0: contour.current_x,
        x1: x,
        y0: contour.current_y,
        y1: y,
    });
    contour.current_x = x;
    contour.current_y = y;
}

// Source: upstream/packages/path/src/pathMorphGeometry.ts:244 (sha256:aaaee865f9aa03b101914fa098b27d8096b1e654fbb37bbf38134722210f000a)
fn append_line_as_cubic(contour: &mut CubicContour, x: f64, y: f64) -> () {
    let x0 = contour.current_x;
    let y0 = contour.current_y;
    append_cubic_segment(
        contour,
        (x0 + ((x - x0) / 3.0_f64)),
        (y0 + ((y - y0) / 3.0_f64)),
        (x0 + (((x - x0) * 2.0_f64) / 3.0_f64)),
        (y0 + (((y - y0) * 2.0_f64) / 3.0_f64)),
        x,
        y,
    );
}

// Source: upstream/packages/path/src/pathMorphGeometry.ts:258 (sha256:c42f7e3cca579e43540790a74c38e5fb1da43474a86caff708858594b79fc292)
fn append_quadratic_as_cubic(
    contour: &mut CubicContour,
    control_x: f64,
    control_y: f64,
    x: f64,
    y: f64,
) -> () {
    let x0 = contour.current_x;
    let y0 = contour.current_y;
    append_cubic_segment(
        contour,
        (x0 + (((control_x - x0) * 2.0_f64) / 3.0_f64)),
        (y0 + (((control_y - y0) * 2.0_f64) / 3.0_f64)),
        (x + (((control_x - x) * 2.0_f64) / 3.0_f64)),
        (y + (((control_y - y) * 2.0_f64) / 3.0_f64)),
        x,
        y,
    );
}

// Source: upstream/packages/path/src/pathMorphGeometry.ts:272 (sha256:2f8039b66cd578a3da4d8d4babe18fbee4ed50b18869e5bc73bd5786d1fa8d17)
fn close_cubic_contour(contour: &mut CubicContour) -> () {
    if (contour.current_x != contour.x) || (contour.current_y != contour.y) {
        {
            let __flight_argument_1 = contour.x;
            let __flight_argument_2 = contour.y;
            let __flight_result =
                append_line_as_cubic(contour, __flight_argument_1, __flight_argument_2);
            __flight_result
        };
    }
    contour.closed = true;
}

// Source: upstream/packages/path/src/pathMorphGeometry.ts:279 (sha256:257d248d23c3b0d94210a90bccc93f36b74f509212f38ec8209d11412c4b88b2)
fn create_cubic_contour(x: f64, y: f64) -> CubicContour {
    return CubicContour {
        __flight_identity: std::sync::Arc::new(()),
        closed: false,
        current_x: x,
        current_y: y,
        segments: vec![],
        x: x,
        y: y,
    };
}

// Source: upstream/packages/path/src/pathMorphGeometry.ts:283 (sha256:446f2786768c552694221f1799ac0db00f2714800bf2f63272888516500b6d79)
fn cubic_control_polygon_length(segment: &CubicSegment) -> f64 {
    return ((point_distance(
        segment.x0,
        segment.y0,
        segment.control1_x,
        segment.control1_y,
    ) + point_distance(
        segment.control1_x,
        segment.control1_y,
        segment.control2_x,
        segment.control2_y,
    )) + point_distance(
        segment.control2_x,
        segment.control2_y,
        segment.x1,
        segment.y1,
    ));
}

// Source: upstream/packages/path/src/pathMorphGeometry.ts:291 (sha256:89a17dd318457c2b2c9ef2702055f7fc7b5f9db265c54a13cdc16c1501f6edc3)
fn decode_cubic_contours(path: &Path) -> Vec<CubicContour> {
    let contours: std::sync::Arc<std::sync::Mutex<Vec<CubicContour>>> =
        std::sync::Arc::new(std::sync::Mutex::new(vec![]));
    let contour: std::sync::Arc<std::sync::Mutex<Option<CubicContour>>> =
        std::sync::Arc::new(std::sync::Mutex::new(None));
    let ensure_contour: std::sync::Arc<
        std::sync::Mutex<
            std::sync::Arc<std::sync::Mutex<Box<dyn FnMut() -> CubicContour + Send + 'static>>>,
        >,
    > = std::sync::Arc::new(std::sync::Mutex::new(std::sync::Arc::new(
        std::sync::Mutex::new(Box::new({
            let mut contour = contour.clone();
            let mut contours = contours.clone();
            move || -> CubicContour {
                if ((*contour.lock().unwrap()).clone()).is_some() {
                    return (((*contour.lock().unwrap()).clone()).clone().unwrap()).clone();
                }
                (*contour.lock().unwrap()) = Some(create_cubic_contour(0.0_f64, 0.0_f64));
                (*contours.lock().unwrap())
                    .push((((*contour.lock().unwrap()).clone()).clone().unwrap()).clone());
                return (((*contour.lock().unwrap()).clone()).clone().unwrap()).clone();
            }
        })
            as Box<dyn FnMut() -> CubicContour + Send + 'static>),
    )));
    for_each_path_segment(path, &mut |segment: PathSegment| -> () {
        if matches!(&(segment), flighthq_types::PathSegment::A(_)) {
            (*contour.lock().unwrap()) = Some(create_cubic_contour(
                (match (segment).clone() {
                    flighthq_types::PathSegment::A(value) => value,
                    flighthq_types::PathSegment::B(_) => {
                        panic!("TypeScript union narrowing failed")
                    }
                })
                .x,
                (match (segment).clone() {
                    flighthq_types::PathSegment::A(value) => value,
                    flighthq_types::PathSegment::B(_) => {
                        panic!("TypeScript union narrowing failed")
                    }
                })
                .y,
            ));
            (*contours.lock().unwrap())
                .push((((*contour.lock().unwrap()).clone()).clone().unwrap()).clone());
        } else {
            if matches!(
                &(segment),
                crate::FlightUnion2::B(crate::FlightUnion2::A(_))
            ) {
                append_line_as_cubic(
                    &mut {
                        let __flight_callback = (*ensure_contour.lock().unwrap()).clone();
                        let __flight_result = __flight_callback.lock().unwrap()();
                        __flight_result
                    },
                    (match (segment).clone() {
                        flighthq_types::PathSegment::A(_) => {
                            panic!("TypeScript union narrowing failed")
                        }
                        flighthq_types::PathSegment::B(value) => match value {
                            crate::FlightUnion2::A(value) => value,
                            crate::FlightUnion2::B(_) => {
                                panic!("TypeScript union narrowing failed")
                            }
                        },
                    })
                    .x,
                    (match (segment).clone() {
                        flighthq_types::PathSegment::A(_) => {
                            panic!("TypeScript union narrowing failed")
                        }
                        flighthq_types::PathSegment::B(value) => match value {
                            crate::FlightUnion2::A(value) => value,
                            crate::FlightUnion2::B(_) => {
                                panic!("TypeScript union narrowing failed")
                            }
                        },
                    })
                    .y,
                );
            } else {
                if matches!(
                    &(segment),
                    crate::FlightUnion2::B(crate::FlightUnion2::B(crate::FlightUnion2::A(_)))
                ) {
                    append_quadratic_as_cubic(
                        &mut {
                            let __flight_callback = (*ensure_contour.lock().unwrap()).clone();
                            let __flight_result = __flight_callback.lock().unwrap()();
                            __flight_result
                        },
                        (match (segment).clone() {
                            flighthq_types::PathSegment::A(_) => {
                                panic!("TypeScript union narrowing failed")
                            }
                            flighthq_types::PathSegment::B(value) => match value {
                                crate::FlightUnion2::A(_) => {
                                    panic!("TypeScript union narrowing failed")
                                }
                                crate::FlightUnion2::B(value) => match value {
                                    crate::FlightUnion2::A(value) => value,
                                    crate::FlightUnion2::B(_) => {
                                        panic!("TypeScript union narrowing failed")
                                    }
                                },
                            },
                        })
                        .control_x,
                        (match (segment).clone() {
                            flighthq_types::PathSegment::A(_) => {
                                panic!("TypeScript union narrowing failed")
                            }
                            flighthq_types::PathSegment::B(value) => match value {
                                crate::FlightUnion2::A(_) => {
                                    panic!("TypeScript union narrowing failed")
                                }
                                crate::FlightUnion2::B(value) => match value {
                                    crate::FlightUnion2::A(value) => value,
                                    crate::FlightUnion2::B(_) => {
                                        panic!("TypeScript union narrowing failed")
                                    }
                                },
                            },
                        })
                        .control_y,
                        (match (segment).clone() {
                            flighthq_types::PathSegment::A(_) => {
                                panic!("TypeScript union narrowing failed")
                            }
                            flighthq_types::PathSegment::B(value) => match value {
                                crate::FlightUnion2::A(_) => {
                                    panic!("TypeScript union narrowing failed")
                                }
                                crate::FlightUnion2::B(value) => match value {
                                    crate::FlightUnion2::A(value) => value,
                                    crate::FlightUnion2::B(_) => {
                                        panic!("TypeScript union narrowing failed")
                                    }
                                },
                            },
                        })
                        .x,
                        (match (segment).clone() {
                            flighthq_types::PathSegment::A(_) => {
                                panic!("TypeScript union narrowing failed")
                            }
                            flighthq_types::PathSegment::B(value) => match value {
                                crate::FlightUnion2::A(_) => {
                                    panic!("TypeScript union narrowing failed")
                                }
                                crate::FlightUnion2::B(value) => match value {
                                    crate::FlightUnion2::A(value) => value,
                                    crate::FlightUnion2::B(_) => {
                                        panic!("TypeScript union narrowing failed")
                                    }
                                },
                            },
                        })
                        .y,
                    );
                } else {
                    if matches!(
                        &(segment),
                        crate::FlightUnion2::B(crate::FlightUnion2::B(crate::FlightUnion2::B(
                            crate::FlightUnion2::A(_)
                        )))
                    ) {
                        append_cubic_segment(
                            &mut {
                                let __flight_callback = (*ensure_contour.lock().unwrap()).clone();
                                let __flight_result = __flight_callback.lock().unwrap()();
                                __flight_result
                            },
                            (match (segment).clone() {
                                flighthq_types::PathSegment::A(_) => {
                                    panic!("TypeScript union narrowing failed")
                                }
                                flighthq_types::PathSegment::B(value) => match value {
                                    crate::FlightUnion2::A(_) => {
                                        panic!("TypeScript union narrowing failed")
                                    }
                                    crate::FlightUnion2::B(value) => match value {
                                        crate::FlightUnion2::A(_) => {
                                            panic!("TypeScript union narrowing failed")
                                        }
                                        crate::FlightUnion2::B(value) => match value {
                                            crate::FlightUnion2::A(value) => value,
                                            crate::FlightUnion2::B(_) => {
                                                panic!("TypeScript union narrowing failed")
                                            }
                                        },
                                    },
                                },
                            })
                            .control1_x,
                            (match (segment).clone() {
                                flighthq_types::PathSegment::A(_) => {
                                    panic!("TypeScript union narrowing failed")
                                }
                                flighthq_types::PathSegment::B(value) => match value {
                                    crate::FlightUnion2::A(_) => {
                                        panic!("TypeScript union narrowing failed")
                                    }
                                    crate::FlightUnion2::B(value) => match value {
                                        crate::FlightUnion2::A(_) => {
                                            panic!("TypeScript union narrowing failed")
                                        }
                                        crate::FlightUnion2::B(value) => match value {
                                            crate::FlightUnion2::A(value) => value,
                                            crate::FlightUnion2::B(_) => {
                                                panic!("TypeScript union narrowing failed")
                                            }
                                        },
                                    },
                                },
                            })
                            .control1_y,
                            (match (segment).clone() {
                                flighthq_types::PathSegment::A(_) => {
                                    panic!("TypeScript union narrowing failed")
                                }
                                flighthq_types::PathSegment::B(value) => match value {
                                    crate::FlightUnion2::A(_) => {
                                        panic!("TypeScript union narrowing failed")
                                    }
                                    crate::FlightUnion2::B(value) => match value {
                                        crate::FlightUnion2::A(_) => {
                                            panic!("TypeScript union narrowing failed")
                                        }
                                        crate::FlightUnion2::B(value) => match value {
                                            crate::FlightUnion2::A(value) => value,
                                            crate::FlightUnion2::B(_) => {
                                                panic!("TypeScript union narrowing failed")
                                            }
                                        },
                                    },
                                },
                            })
                            .control2_x,
                            (match (segment).clone() {
                                flighthq_types::PathSegment::A(_) => {
                                    panic!("TypeScript union narrowing failed")
                                }
                                flighthq_types::PathSegment::B(value) => match value {
                                    crate::FlightUnion2::A(_) => {
                                        panic!("TypeScript union narrowing failed")
                                    }
                                    crate::FlightUnion2::B(value) => match value {
                                        crate::FlightUnion2::A(_) => {
                                            panic!("TypeScript union narrowing failed")
                                        }
                                        crate::FlightUnion2::B(value) => match value {
                                            crate::FlightUnion2::A(value) => value,
                                            crate::FlightUnion2::B(_) => {
                                                panic!("TypeScript union narrowing failed")
                                            }
                                        },
                                    },
                                },
                            })
                            .control2_y,
                            (match (segment).clone() {
                                flighthq_types::PathSegment::A(_) => {
                                    panic!("TypeScript union narrowing failed")
                                }
                                flighthq_types::PathSegment::B(value) => match value {
                                    crate::FlightUnion2::A(_) => {
                                        panic!("TypeScript union narrowing failed")
                                    }
                                    crate::FlightUnion2::B(value) => match value {
                                        crate::FlightUnion2::A(_) => {
                                            panic!("TypeScript union narrowing failed")
                                        }
                                        crate::FlightUnion2::B(value) => match value {
                                            crate::FlightUnion2::A(value) => value,
                                            crate::FlightUnion2::B(_) => {
                                                panic!("TypeScript union narrowing failed")
                                            }
                                        },
                                    },
                                },
                            })
                            .x,
                            (match (segment).clone() {
                                flighthq_types::PathSegment::A(_) => {
                                    panic!("TypeScript union narrowing failed")
                                }
                                flighthq_types::PathSegment::B(value) => match value {
                                    crate::FlightUnion2::A(_) => {
                                        panic!("TypeScript union narrowing failed")
                                    }
                                    crate::FlightUnion2::B(value) => match value {
                                        crate::FlightUnion2::A(_) => {
                                            panic!("TypeScript union narrowing failed")
                                        }
                                        crate::FlightUnion2::B(value) => match value {
                                            crate::FlightUnion2::A(value) => value,
                                            crate::FlightUnion2::B(_) => {
                                                panic!("TypeScript union narrowing failed")
                                            }
                                        },
                                    },
                                },
                            })
                            .y,
                        );
                    } else {
                        if (((match (segment).clone() {
                            flighthq_types::PathSegment::A(_) => {
                                panic!("TypeScript union narrowing failed")
                            }
                            flighthq_types::PathSegment::B(value) => match value {
                                crate::FlightUnion2::A(_) => {
                                    panic!("TypeScript union narrowing failed")
                                }
                                crate::FlightUnion2::B(value) => match value {
                                    crate::FlightUnion2::A(_) => {
                                        panic!("TypeScript union narrowing failed")
                                    }
                                    crate::FlightUnion2::B(value) => match value {
                                        crate::FlightUnion2::A(_) => {
                                            panic!("TypeScript union narrowing failed")
                                        }
                                        crate::FlightUnion2::B(value) => value,
                                    },
                                },
                            },
                        })
                        .kind)
                            .clone()
                            == "close")
                            && (((*contour.lock().unwrap()).clone()).is_some())
                        {
                            close_cubic_contour((*contour.lock().unwrap()).as_mut().unwrap());
                            (*contour.lock().unwrap()) = None;
                        }
                    }
                }
            }
        }
    });
    return (*contours.lock().unwrap()).clone();
}

// Source: upstream/packages/path/src/pathMorphGeometry.ts:326 (sha256:30bc6c3b06b528f05063f4b03bef78713dcfacdf30a4dbf9f261a2e6c40e6ec9)
fn equalize_cubic_contour_segments(start: &mut CubicContour, end: &mut CubicContour) -> () {
    let target_count = (start.segments.len() as f64).max((end.segments.len() as f64));
    if (target_count == 0.0_f64) {
        return;
    }
    start.segments = {
        let __flight_argument_2 = start.x;
        let __flight_argument_3 = start.y;
        let __flight_result = subdivide_cubic_segments(
            &mut start.segments,
            target_count,
            __flight_argument_2,
            __flight_argument_3,
        );
        __flight_result
    };
    end.segments = {
        let __flight_argument_2 = end.x;
        let __flight_argument_3 = end.y;
        let __flight_result = subdivide_cubic_segments(
            &mut end.segments,
            target_count,
            __flight_argument_2,
            __flight_argument_3,
        );
        __flight_result
    };
}

// Source: upstream/packages/path/src/pathMorphGeometry.ts:333 (sha256:e1ff1cf37098a6f8e235b9a86a8c03ed0150cf1913a31ec28be7553eb8cb1795)
fn point_distance(x0: f64, y0: f64, x1: f64, y1: f64) -> f64 {
    let dx = (x1 - x0);
    let dy = (y1 - y0);
    return ((dx * dx) + (dy * dy)).sqrt();
}

// Source: upstream/packages/path/src/pathMorphGeometry.ts:339 (sha256:9cd7b65230285c333a21f3807e9dfaaf05d255c3dbe762574ebbc7191f480e13)
fn split_cubic_segment(segment: &CubicSegment, t: f64) -> Vec<CubicSegment> {
    let x01 = (segment.x0 + ((segment.control1_x - segment.x0) * t));
    let y01 = (segment.y0 + ((segment.control1_y - segment.y0) * t));
    let x12 = (segment.control1_x + ((segment.control2_x - segment.control1_x) * t));
    let y12 = (segment.control1_y + ((segment.control2_y - segment.control1_y) * t));
    let x23 = (segment.control2_x + ((segment.x1 - segment.control2_x) * t));
    let y23 = (segment.control2_y + ((segment.y1 - segment.control2_y) * t));
    let x012 = (x01 + ((x12 - x01) * t));
    let y012 = (y01 + ((y12 - y01) * t));
    let x123 = (x12 + ((x23 - x12) * t));
    let y123 = (y12 + ((y23 - y12) * t));
    let x = (x012 + ((x123 - x012) * t));
    let y = (y012 + ((y123 - y012) * t));
    return vec![
        CubicSegment {
            __flight_identity: std::sync::Arc::new(()),
            control1_x: x01,
            control1_y: y01,
            control2_x: x012,
            control2_y: y012,
            x0: segment.x0,
            x1: x,
            y0: segment.y0,
            y1: y,
        },
        CubicSegment {
            __flight_identity: std::sync::Arc::new(()),
            control1_x: x123,
            control1_y: y123,
            control2_x: x23,
            control2_y: y23,
            x0: x,
            x1: segment.x1,
            y0: y,
            y1: segment.y1,
        },
    ];
}

// Source: upstream/packages/path/src/pathMorphGeometry.ts:376 (sha256:37911281eec93ed2fd2d0bad022a7787a821f66a983abe3ef48c1ec2072568a3)
fn subdivide_cubic_segments(
    source: &mut Vec<CubicSegment>,
    target_count: f64,
    point_x: f64,
    point_y: f64,
) -> Vec<CubicSegment> {
    if ((source.len() as f64) == target_count) {
        return (source).clone();
    }
    if ((source.len() as f64) == 0.0_f64) {
        let mut segments: Vec<CubicSegment> = vec![];
        {
            let mut i = 0.0_f64;
            while (i < target_count) {
                segments.push(CubicSegment {
                    __flight_identity: std::sync::Arc::new(()),
                    control1_x: point_x,
                    control1_y: point_y,
                    control2_x: point_x,
                    control2_y: point_y,
                    x0: point_x,
                    x1: point_x,
                    y0: point_y,
                    y1: point_y,
                });
                {
                    i += 1.0;
                    i
                };
            }
        }
        return segments;
    }
    let mut part_counts = vec![1.0_f64; (source.len() as f64) as usize];
    let lengths = (source)
        .iter()
        .cloned()
        .map(|__flight_item| cubic_control_polygon_length(&__flight_item))
        .collect::<Vec<_>>();
    {
        let mut total = (source.len() as f64);
        while (total < target_count) {
            let mut best = 0.0_f64;
            let mut best_length = (-1.0_f64);
            {
                let mut i = 0.0_f64;
                while (i < (source.len() as f64)) {
                    let part_length =
                        (lengths[i as usize].clone() / part_counts[i as usize].clone());
                    if (part_length > best_length) {
                        best = i;
                        best_length = part_length;
                    }
                    {
                        i += 1.0;
                        i
                    };
                }
            }
            {
                part_counts[best as usize] += 1.0;
                part_counts[best as usize]
            };
            {
                total += 1.0;
                total
            };
        }
    }
    let mut segments: Vec<CubicSegment> = vec![];
    {
        let mut i = 0.0_f64;
        while (i < (source.len() as f64)) {
            let mut remainder = source[i as usize].clone();
            {
                let mut parts = part_counts[i as usize].clone();
                while (parts > 1.0_f64) {
                    let split = split_cubic_segment(&remainder, (1.0_f64 / parts));
                    segments.push(split[0.0_f64 as usize].clone());
                    remainder = split[1.0_f64 as usize].clone();
                    {
                        parts -= 1.0;
                        parts
                    };
                }
            }
            segments.push(((remainder).clone()).clone());
            {
                i += 1.0;
                i
            };
        }
    }
    return segments;
}

// Source: upstream/packages/path/src/pathMorphGeometry.ts:428 (sha256:ac3205ff708c859dca3e9fd9e31b9c1162a007014370ad04231a50ba3df79cf2)
#[derive(Clone, Default)]
pub(crate) struct CubicContour {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub closed: bool,
    pub current_x: f64,
    pub current_y: f64,
    pub segments: Vec<CubicSegment>,
    pub x: f64,
    pub y: f64,
}
impl PartialEq for CubicContour {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/path/src/pathMorphGeometry.ts:437 (sha256:d08b075f16e874536d80123a5877b2c65e77fdb0ca88c1db689a62d702233704)
#[derive(Clone, Default)]
pub(crate) struct CubicSegment {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub control1_x: f64,
    pub control1_y: f64,
    pub control2_x: f64,
    pub control2_y: f64,
    pub x0: f64,
    pub x1: f64,
    pub y0: f64,
    pub y1: f64,
}
impl PartialEq for CubicSegment {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}
