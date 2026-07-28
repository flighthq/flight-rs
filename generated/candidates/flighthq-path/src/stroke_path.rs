// @generated from upstream/packages/path/src/strokePath.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::append_path_close;
pub use flighthq_types::StrokeStyle;
use flighthq_types::{Path, PathCommand};

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

// Source: upstream/packages/path/src/strokePath.ts:18 (sha256:ff09778339802ad40f5eb6181a9146b87ef39a3da709c527b12088488334a2f5)
pub fn stroke_path(path: &Path, style: &StrokeStyle, tolerance: Option<f64>) -> Path {
    let tolerance = tolerance.unwrap_or(0.25_f64);
    let width = (style.width).unwrap_or(1.0_f64);
    let join = ((style.join).clone()).unwrap_or("miter".to_owned());
    let cap = ((style.cap).clone()).unwrap_or("butt".to_owned());
    let miter_limit = (style.miter_limit).unwrap_or(4.0_f64);
    let half_width = (width / 2.0_f64);
    let mut result: Path = Path {
        __flight_identity: std::sync::Arc::new(()),
        commands: vec![],
        data: vec![],
        winding: "nonZero".to_owned(),
    };
    let subpaths = decode_subpaths(path, tolerance);
    let dash = if ((style.dash).clone()).is_some()
        && ((style.dash.as_ref().unwrap().len() as f64) > 0.0_f64)
    {
        (style.dash).clone()
    } else {
        None
    };
    let dash_offset = (style.dash_offset).unwrap_or(0.0_f64);
    for subpath in (subpaths).iter().cloned() {
        if ((subpath.points.len() as f64) < 2.0_f64) {
            continue;
        }
        let segments = if (dash).is_some() {
            apply_dash(
                &subpath.points,
                subpath.closed,
                &dash.as_ref().unwrap(),
                dash_offset,
            )
        } else {
            vec![DashSegment {
                __flight_identity: std::sync::Arc::new(()),
                points: (subpath.points).clone(),
                closed: subpath.closed,
            }]
        };
        for seg in (segments).iter().cloned() {
            if ((seg.points.len() as f64) < 2.0_f64) {
                continue;
            }
            stroke_subpath(
                &seg.points,
                seg.closed,
                half_width,
                (join).clone(),
                (cap).clone(),
                miter_limit,
                &mut result,
                tolerance,
            );
        }
    }
    return result;
}

// Source: upstream/packages/path/src/strokePath.ts:44 (sha256:e6e1586e17306f1abe35eca6d375805bb2afdb4f6855e5393a2acb1ea0d61251)
fn add_arc_points(
    cx: f64,
    cy: f64,
    r: f64,
    start_angle: f64,
    end_angle: f64,
    ccw: bool,
    tolerance: f64,
    out: &mut Vec<f64>,
) -> () {
    let ratio = (0.0_f64).max((1.0_f64).min((tolerance / r)));
    let n = (4.0_f64).max((std::f64::consts::PI / (1.0_f64 - ratio).acos()).ceil());
    let mut delta = (end_angle - start_angle);
    if ccw {
        if (delta > 0.0_f64) {
            delta -= (std::f64::consts::PI * 2.0_f64);
        }
    } else {
        if (delta < 0.0_f64) {
            delta += (std::f64::consts::PI * 2.0_f64);
        }
    }
    let steps = ((delta).abs() / ((std::f64::consts::PI * 2.0_f64) / n)).ceil();
    if (steps <= 1.0_f64) {
        return;
    }
    let step_angle = (delta / steps);
    {
        let mut i = 1.0_f64;
        while (i < steps) {
            let angle = (start_angle + (i * step_angle));
            out.extend(vec![(cx + ((angle).cos() * r)), (cy + ((angle).sin() * r))]);
            {
                i += 1.0;
                i
            };
        }
    }
}

// Source: upstream/packages/path/src/strokePath.ts:78 (sha256:f866e0e0c75ac7eb4d6bb44f203831972454eae641f324d79eba28542180897d)
fn add_cap(
    px: f64,
    py: f64,
    nx: f64,
    ny: f64,
    edx: f64,
    edy: f64,
    half_width: f64,
    cap: String,
    left: &mut Vec<f64>,
    right: &mut Vec<f64>,
    tolerance: f64,
    is_start: bool,
) -> () {
    let lx = (px + (nx * half_width));
    let ly = (py + (ny * half_width));
    let rx = (px - (nx * half_width));
    let ry = (py - (ny * half_width));
    if (cap == "butt") {
        left.extend(vec![lx, ly]);
        right.extend(vec![rx, ry]);
    } else {
        if (cap == "square") {
            left.extend(vec![(lx + (edx * half_width)), (ly + (edy * half_width))]);
            right.extend(vec![(rx + (edx * half_width)), (ry + (edy * half_width))]);
        } else {
            if is_start {
                left.extend(vec![lx, ly]);
                right.extend(vec![rx, ry]);
                let start_angle = (-ny).atan2((-nx));
                let end_angle = (ny).atan2(nx);
                add_arc_points(
                    px,
                    py,
                    half_width,
                    start_angle,
                    end_angle,
                    false,
                    tolerance,
                    right,
                );
            } else {
                left.extend(vec![lx, ly]);
                add_arc_points(
                    px,
                    py,
                    half_width,
                    (ny).atan2(nx),
                    (-ny).atan2((-nx)),
                    true,
                    tolerance,
                    left,
                );
                right.extend(vec![rx, ry]);
            }
        }
    }
}

// Source: upstream/packages/path/src/strokePath.ts:122 (sha256:ec00084b2b9e3cac8e718e203f635dee7a46134eab6ba97ad30f987aa234681b)
fn add_join(
    px: f64,
    py: f64,
    nx0: f64,
    ny0: f64,
    nx1: f64,
    ny1: f64,
    half_width: f64,
    join: String,
    miter_limit: f64,
    left: &mut Vec<f64>,
    right: &mut Vec<f64>,
    tolerance: f64,
) -> () {
    let lx0 = (px + (nx0 * half_width));
    let ly0 = (py + (ny0 * half_width));
    let rx0 = (px - (nx0 * half_width));
    let ry0 = (py - (ny0 * half_width));
    let lx1 = (px + (nx1 * half_width));
    let ly1 = (py + (ny1 * half_width));
    let rx1 = (px - (nx1 * half_width));
    let ry1 = (py - (ny1 * half_width));
    if (join == "miter") {
        let cross = ((nx0 * ny1) - (ny0 * nx1));
        if ((cross).abs() < 1e-8_f64) {
            left.extend(vec![lx0, ly0]);
            right.extend(vec![rx0, ry0]);
        } else {
            let dx = (lx1 - lx0);
            let dy = (ly1 - ly0);
            let t = (((dx * ny1) - (dy * nx1)) / cross);
            let mx = (lx0 + (t * nx0));
            let my = (ly0 + (t * ny0));
            let miter_len = (((mx - px) * (mx - px)) + ((my - py) * (my - py))).sqrt();
            if (miter_len <= (half_width * miter_limit)) {
                left.extend(vec![mx, my]);
                let rmx = ((px * 2.0_f64) - mx);
                let rmy = ((py * 2.0_f64) - my);
                right.extend(vec![rmx, rmy]);
            } else {
                left.extend(vec![lx0, ly0, lx1, ly1]);
                right.extend(vec![rx0, ry0, rx1, ry1]);
            }
        }
    } else {
        if (join == "round") {
            left.extend(vec![lx0, ly0]);
            add_arc_points(
                px,
                py,
                half_width,
                (ny0).atan2(nx0),
                (ny1).atan2(nx1),
                true,
                tolerance,
                left,
            );
            left.extend(vec![lx1, ly1]);
            right.extend(vec![rx0, ry0]);
            add_arc_points(
                px,
                py,
                half_width,
                (-ny0).atan2((-nx0)),
                (-ny1).atan2((-nx1)),
                false,
                tolerance,
                right,
            );
            right.extend(vec![rx1, ry1]);
        } else {
            left.extend(vec![lx0, ly0, lx1, ly1]);
            right.extend(vec![rx0, ry0, rx1, ry1]);
        }
    }
}

// Source: upstream/packages/path/src/strokePath.ts:188 (sha256:c3585a9a1917a40a888aac999c112cd63cf284bef6c3039c0f68739b68f25ea5)
fn apply_dash(pts: &Vec<f64>, closed: bool, dash: &Vec<f64>, dash_offset: f64) -> Vec<DashSegment> {
    let mut result: Vec<DashSegment> = vec![];
    if ((dash.len() as f64) == 0.0_f64) {
        result.push(DashSegment {
            __flight_identity: std::sync::Arc::new(()),
            points: (*pts).clone(),
            closed: closed,
        });
        return result;
    }
    let total_dash_length = (dash)
        .iter()
        .cloned()
        .fold(0.0_f64, |s: f64, d: f64| -> f64 { (s + d) });
    if (total_dash_length <= 0.0_f64) {
        result.push(DashSegment {
            __flight_identity: std::sync::Arc::new(()),
            points: (*pts).clone(),
            closed: closed,
        });
        return result;
    }
    let offset = (((dash_offset % total_dash_length) + total_dash_length) % total_dash_length);
    let mut dash_index = 0.0_f64;
    let mut remaining = 0.0_f64;
    let mut is_on = true;
    {
        let mut acc = 0.0_f64;
        {
            let mut i = 0.0_f64;
            while (i < (dash.len() as f64)) {
                if ((acc + dash[i as usize].clone()) > offset) {
                    dash_index = i;
                    remaining = (dash[i as usize].clone() - (offset - acc));
                    is_on = ((i % 2.0_f64) == 0.0_f64);
                    break;
                }
                acc += dash[i as usize].clone();
                {
                    i += 1.0;
                    i
                };
            }
        }
    }
    let mut current: Option<Vec<f64>> = None;
    let n = (__flight_js_to_i32((pts.len() as f64)) >> (__flight_js_to_u32(1.0_f64) & 31)) as f64;
    {
        let mut i = 0.0_f64;
        while (i < (n - 1.0_f64)) {
            let x0 = pts[(i * 2.0_f64) as usize].clone();
            let y0 = pts[((i * 2.0_f64) + 1.0_f64) as usize].clone();
            let x1 = pts[((i + 1.0_f64) * 2.0_f64) as usize].clone();
            let y1 = pts[(((i + 1.0_f64) * 2.0_f64) + 1.0_f64) as usize].clone();
            let dx = (x1 - x0);
            let dy = (y1 - y0);
            let seg_len = ((dx * dx) + (dy * dy)).sqrt();
            if (is_on) && ((current).is_none()) {
                current = Some(vec![x0, y0]);
            }
            let mut consumed = 0.0_f64;
            while (consumed < seg_len) {
                let step = (remaining).min((seg_len - consumed));
                let t = ((consumed + step) / seg_len);
                let ix = (x0 + (t * dx));
                let iy = (y0 + (t * dy));
                if is_on {
                    if (current).is_none() {
                        current = Some(vec![
                            (x0 + ((consumed / seg_len) * dx)),
                            (y0 + ((consumed / seg_len) * dy)),
                        ]);
                    }
                    current.as_mut().unwrap().extend(vec![ix, iy]);
                } else {
                    if (current).is_some() {
                        if ((current.as_mut().unwrap().len() as f64) >= 4.0_f64) {
                            result.push(DashSegment {
                                __flight_identity: std::sync::Arc::new(()),
                                points: (current.as_mut().unwrap()).clone(),
                                closed: false,
                            });
                        }
                        current = None;
                    }
                    if (step >= remaining) {
                        current = Some(vec![ix, iy]);
                    }
                }
                consumed += step;
                remaining -= step;
                if (remaining <= 1e-10_f64) {
                    dash_index = ((dash_index + 1.0_f64) % (dash.len() as f64));
                    remaining = dash[dash_index as usize].clone();
                    is_on = ((dash_index % 2.0_f64) == 0.0_f64);
                    if (is_on) && ((current).is_none()) {
                        current = Some(vec![ix, iy]);
                    } else {
                        if (!is_on) && ((current).is_some()) {
                            if ((current.as_ref().unwrap().len() as f64) >= 4.0_f64) {
                                result.push(DashSegment {
                                    __flight_identity: std::sync::Arc::new(()),
                                    points: (current).clone().unwrap(),
                                    closed: false,
                                });
                            }
                            current = None;
                        }
                    }
                }
            }
            {
                i += 1.0;
                i
            };
        }
    }
    if ((current).is_some()) && ((current.as_ref().unwrap().len() as f64) >= 4.0_f64) {
        result.push(DashSegment {
            __flight_identity: std::sync::Arc::new(()),
            points: (current).clone().unwrap(),
            closed: false,
        });
    }
    return result;
}

// Source: upstream/packages/path/src/strokePath.ts:274 (sha256:24d231d16c218a934d68588524c7ccd6681ba4d45f152a41cb192346ba7811bd)
#[derive(Clone)]
struct DecodeSubpathsRecord1 {
    __flight_identity: std::sync::Arc<()>,
    points: Vec<f64>,
    closed: bool,
}
impl PartialEq for DecodeSubpathsRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

fn decode_subpaths(path: &Path, tolerance: f64) -> Vec<StrokeSubpath> {
    let tolerance_sq = (tolerance * tolerance);
    let subpaths: std::sync::Arc<std::sync::Mutex<Vec<StrokeSubpath>>> =
        std::sync::Arc::new(std::sync::Mutex::new(vec![]));
    let current: std::sync::Arc<std::sync::Mutex<Option<StrokeSubpath>>> =
        std::sync::Arc::new(std::sync::Mutex::new(None));
    let x: std::sync::Arc<std::sync::Mutex<f64>> =
        std::sync::Arc::new(std::sync::Mutex::new(0.0_f64));
    let y: std::sync::Arc<std::sync::Mutex<f64>> =
        std::sync::Arc::new(std::sync::Mutex::new(0.0_f64));
    let mut contour_start_x = 0.0_f64;
    let mut contour_start_y = 0.0_f64;
    let mut di = 0.0_f64;
    let mut ensure_current: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut() -> StrokeSubpath + Send + 'static>>,
    > = std::sync::Arc::new(std::sync::Mutex::new(Box::new({
        let mut current = current.clone();
        let mut subpaths = subpaths.clone();
        let mut x = x.clone();
        let mut y = y.clone();
        move || -> StrokeSubpath {
            if ((*current.lock().unwrap()).clone()).is_none() {
                (*current.lock().unwrap()) = Some(StrokeSubpath {
                    __flight_identity: std::sync::Arc::new(()),
                    points: vec![0.0_f64, 0.0_f64],
                    closed: false,
                });
                (*subpaths.lock().unwrap())
                    .push((((*current.lock().unwrap()).clone()).clone().unwrap()).clone());
                (*x.lock().unwrap()) = 0.0_f64;
                (*y.lock().unwrap()) = 0.0_f64;
            }
            return (((*current.lock().unwrap()).clone()).clone().unwrap()).clone();
        }
    })
        as Box<dyn FnMut() -> StrokeSubpath + Send + 'static>));
    {
        let mut ci = 0.0_f64;
        while (ci < (path.commands.len() as f64)) {
            let command = path.commands[ci as usize].clone();
            if (command == PathCommand::MOVE_TO) {
                (*x.lock().unwrap()) = path.data[di as usize].clone();
                (*y.lock().unwrap()) = path.data[(di + 1.0_f64) as usize].clone();
                di += 2.0_f64;
                contour_start_x = (*x.lock().unwrap()).clone();
                contour_start_y = (*y.lock().unwrap()).clone();
                (*current.lock().unwrap()) = Some(StrokeSubpath {
                    __flight_identity: std::sync::Arc::new(()),
                    points: vec![(*x.lock().unwrap()).clone(), (*y.lock().unwrap()).clone()],
                    closed: false,
                });
                (*subpaths.lock().unwrap())
                    .push((((*current.lock().unwrap()).clone()).clone().unwrap()).clone());
            } else {
                if (command == PathCommand::WIDE_MOVE_TO) {
                    (*x.lock().unwrap()) = path.data[(di + 2.0_f64) as usize].clone();
                    (*y.lock().unwrap()) = path.data[(di + 3.0_f64) as usize].clone();
                    di += 4.0_f64;
                    contour_start_x = (*x.lock().unwrap()).clone();
                    contour_start_y = (*y.lock().unwrap()).clone();
                    (*current.lock().unwrap()) = Some(StrokeSubpath {
                        __flight_identity: std::sync::Arc::new(()),
                        points: vec![(*x.lock().unwrap()).clone(), (*y.lock().unwrap()).clone()],
                        closed: false,
                    });
                    (*subpaths.lock().unwrap())
                        .push((((*current.lock().unwrap()).clone()).clone().unwrap()).clone());
                } else {
                    if (command == PathCommand::LINE_TO) {
                        let mut sp = {
                            let __flight_callback = (ensure_current).clone();
                            let __flight_result = __flight_callback.lock().unwrap()();
                            __flight_result
                        };
                        (*x.lock().unwrap()) = path.data[di as usize].clone();
                        (*y.lock().unwrap()) = path.data[(di + 1.0_f64) as usize].clone();
                        di += 2.0_f64;
                        sp.points.extend(vec![
                            (*x.lock().unwrap()).clone(),
                            (*y.lock().unwrap()).clone(),
                        ]);
                    } else {
                        if (command == PathCommand::WIDE_LINE_TO) {
                            let mut sp = {
                                let __flight_callback = (ensure_current).clone();
                                let __flight_result = __flight_callback.lock().unwrap()();
                                __flight_result
                            };
                            (*x.lock().unwrap()) = path.data[(di + 2.0_f64) as usize].clone();
                            (*y.lock().unwrap()) = path.data[(di + 3.0_f64) as usize].clone();
                            di += 4.0_f64;
                            sp.points.extend(vec![
                                (*x.lock().unwrap()).clone(),
                                (*y.lock().unwrap()).clone(),
                            ]);
                        } else {
                            if (command == PathCommand::CURVE_TO) {
                                let mut sp = {
                                    let __flight_callback = (ensure_current).clone();
                                    let __flight_result = __flight_callback.lock().unwrap()();
                                    __flight_result
                                };
                                flatten_quadratic(
                                    &mut sp.points,
                                    (*x.lock().unwrap()).clone(),
                                    (*y.lock().unwrap()).clone(),
                                    path.data[di as usize].clone(),
                                    path.data[(di + 1.0_f64) as usize].clone(),
                                    path.data[(di + 2.0_f64) as usize].clone(),
                                    path.data[(di + 3.0_f64) as usize].clone(),
                                    tolerance_sq,
                                    0.0_f64,
                                );
                                (*x.lock().unwrap()) = path.data[(di + 2.0_f64) as usize].clone();
                                (*y.lock().unwrap()) = path.data[(di + 3.0_f64) as usize].clone();
                                di += 4.0_f64;
                            } else {
                                if (command == PathCommand::CUBIC_CURVE_TO) {
                                    let mut sp = {
                                        let __flight_callback = (ensure_current).clone();
                                        let __flight_result = __flight_callback.lock().unwrap()();
                                        __flight_result
                                    };
                                    flatten_cubic(
                                        &mut sp.points,
                                        (*x.lock().unwrap()).clone(),
                                        (*y.lock().unwrap()).clone(),
                                        path.data[di as usize].clone(),
                                        path.data[(di + 1.0_f64) as usize].clone(),
                                        path.data[(di + 2.0_f64) as usize].clone(),
                                        path.data[(di + 3.0_f64) as usize].clone(),
                                        path.data[(di + 4.0_f64) as usize].clone(),
                                        path.data[(di + 5.0_f64) as usize].clone(),
                                        tolerance_sq,
                                        0.0_f64,
                                    );
                                    (*x.lock().unwrap()) =
                                        path.data[(di + 4.0_f64) as usize].clone();
                                    (*y.lock().unwrap()) =
                                        path.data[(di + 5.0_f64) as usize].clone();
                                    di += 6.0_f64;
                                } else {
                                    if (command == PathCommand::CLOSE) {
                                        if ((*current.lock().unwrap()).clone()).is_some() {
                                            (*current.lock().unwrap()).as_mut().unwrap().closed =
                                                true;
                                            (*x.lock().unwrap()) = contour_start_x;
                                            (*y.lock().unwrap()) = contour_start_y;
                                            (*current.lock().unwrap()) = None;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            {
                ci += 1.0;
                ci
            };
        }
    }
    return (*subpaths.lock().unwrap()).clone();
}

// Source: upstream/packages/path/src/strokePath.ts:362 (sha256:27dd871c823c8c25d702aecc092aaacc540a5741ba7e6a8d336ce401d9fea5e3)
fn dist_chord_sq(px: f64, py: f64, x0: f64, y0: f64, x1: f64, y1: f64) -> f64 {
    let dx = (x1 - x0);
    let dy = (y1 - y0);
    let len_sq = ((dx * dx) + (dy * dy));
    if (len_sq == 0.0_f64) {
        let ax = (px - x0);
        let ay = (py - y0);
        return ((ax * ax) + (ay * ay));
    }
    let cross = ((dx * (y0 - py)) - (dy * (x0 - px)));
    return ((cross * cross) / len_sq);
}

// Source: upstream/packages/path/src/strokePath.ts:375 (sha256:7f139590643bbf02d73b7955a227937c77a6605a59c51105700c281ec3987732)
fn flatten_cubic(
    out: &mut Vec<f64>,
    x0: f64,
    y0: f64,
    c1x: f64,
    c1y: f64,
    c2x: f64,
    c2y: f64,
    x1: f64,
    y1: f64,
    tolerance_sq: f64,
    depth: f64,
) -> () {
    let dxc1 = dist_chord_sq(c1x, c1y, x0, y0, x1, y1);
    let dxc2 = dist_chord_sq(c2x, c2y, x0, y0, x1, y1);
    if (depth >= MAX_SUBDIVISION_DEPTH) || ((dxc1 <= tolerance_sq) && (dxc2 <= tolerance_sq)) {
        out.extend(vec![x1, y1]);
        return;
    }
    let x01 = ((x0 + c1x) / 2.0_f64);
    let y01 = ((y0 + c1y) / 2.0_f64);
    let x12 = ((c1x + c2x) / 2.0_f64);
    let y12 = ((c1y + c2y) / 2.0_f64);
    let x23 = ((c2x + x1) / 2.0_f64);
    let y23 = ((c2y + y1) / 2.0_f64);
    let x012 = ((x01 + x12) / 2.0_f64);
    let y012 = ((y01 + y12) / 2.0_f64);
    let x123 = ((x12 + x23) / 2.0_f64);
    let y123 = ((y12 + y23) / 2.0_f64);
    let xm = ((x012 + x123) / 2.0_f64);
    let ym = ((y012 + y123) / 2.0_f64);
    flatten_cubic(
        out,
        x0,
        y0,
        x01,
        y01,
        x012,
        y012,
        xm,
        ym,
        tolerance_sq,
        (depth + 1.0_f64),
    );
    flatten_cubic(
        out,
        xm,
        ym,
        x123,
        y123,
        x23,
        y23,
        x1,
        y1,
        tolerance_sq,
        (depth + 1.0_f64),
    );
}

// Source: upstream/packages/path/src/strokePath.ts:410 (sha256:19ec44a36ffcd73871f1e3e4e7f2f27e090739f9b28c363cad576dbccde05f6e)
fn flatten_quadratic(
    out: &mut Vec<f64>,
    x0: f64,
    y0: f64,
    cx: f64,
    cy: f64,
    x1: f64,
    y1: f64,
    tolerance_sq: f64,
    depth: f64,
) -> () {
    let dx = (x1 - x0);
    let dy = (y1 - y0);
    let length_sq = ((dx * dx) + (dy * dy));
    let mut dist_sq: f64;
    if (length_sq == 0.0_f64) {
        let ax = (cx - x0);
        let ay = (cy - y0);
        dist_sq = ((ax * ax) + (ay * ay));
    } else {
        let cross = ((dx * (y0 - cy)) - (dy * (x0 - cx)));
        dist_sq = ((cross * cross) / length_sq);
    }
    if (depth >= MAX_SUBDIVISION_DEPTH) || (dist_sq <= tolerance_sq) {
        out.extend(vec![x1, y1]);
        return;
    }
    let x01 = ((x0 + cx) / 2.0_f64);
    let y01 = ((y0 + cy) / 2.0_f64);
    let x12 = ((cx + x1) / 2.0_f64);
    let y12 = ((cy + y1) / 2.0_f64);
    let xm = ((x01 + x12) / 2.0_f64);
    let ym = ((y01 + y12) / 2.0_f64);
    flatten_quadratic(
        out,
        x0,
        y0,
        x01,
        y01,
        xm,
        ym,
        tolerance_sq,
        (depth + 1.0_f64),
    );
    flatten_quadratic(
        out,
        xm,
        ym,
        x12,
        y12,
        x1,
        y1,
        tolerance_sq,
        (depth + 1.0_f64),
    );
}

// Source: upstream/packages/path/src/strokePath.ts:448 (sha256:452b65ea3f1a4b7c2d8555293acc1d9c05524784fc5e27d3d3586369aee1564b)
fn stroke_subpath(
    pts: &Vec<f64>,
    closed: bool,
    half_width: f64,
    join: String,
    cap: String,
    miter_limit: f64,
    out: &mut Path,
    tolerance: f64,
) -> () {
    let n = (__flight_js_to_i32((pts.len() as f64)) >> (__flight_js_to_u32(1.0_f64) & 31)) as f64;
    if (n < 2.0_f64) {
        return;
    }
    let mut left: Vec<f64> = vec![];
    let mut right: Vec<f64> = vec![];
    let mut normals: Vec<f64> = vec![Default::default(); ((n - 1.0_f64) * 2.0_f64) as usize];
    {
        let mut i = 0.0_f64;
        while (i < (n - 1.0_f64)) {
            let dx = (pts[((i + 1.0_f64) * 2.0_f64) as usize].clone()
                - pts[(i * 2.0_f64) as usize].clone());
            let dy = (pts[(((i + 1.0_f64) * 2.0_f64) + 1.0_f64) as usize].clone()
                - pts[((i * 2.0_f64) + 1.0_f64) as usize].clone());
            let len = ((dx * dx) + (dy * dy)).sqrt();
            if (len > 0.0_f64) {
                {
                    let __flight_index = (i * 2.0_f64) as usize;
                    let __flight_value = ((-dy) / len);
                    if __flight_index == normals.len() {
                        normals.push(__flight_value);
                    } else {
                        normals[__flight_index] = __flight_value;
                    }
                };
                {
                    let __flight_index = ((i * 2.0_f64) + 1.0_f64) as usize;
                    let __flight_value = (dx / len);
                    if __flight_index == normals.len() {
                        normals.push(__flight_value);
                    } else {
                        normals[__flight_index] = __flight_value;
                    }
                };
            } else {
                {
                    let __flight_index = (i * 2.0_f64) as usize;
                    let __flight_value = if (i > 0.0_f64) {
                        normals[((i - 1.0_f64) * 2.0_f64) as usize].clone()
                    } else {
                        0.0_f64
                    };
                    if __flight_index == normals.len() {
                        normals.push(__flight_value);
                    } else {
                        normals[__flight_index] = __flight_value;
                    }
                };
                {
                    let __flight_index = ((i * 2.0_f64) + 1.0_f64) as usize;
                    let __flight_value = if (i > 0.0_f64) {
                        normals[(((i - 1.0_f64) * 2.0_f64) + 1.0_f64) as usize].clone()
                    } else {
                        1.0_f64
                    };
                    if __flight_index == normals.len() {
                        normals.push(__flight_value);
                    } else {
                        normals[__flight_index] = __flight_value;
                    }
                };
            }
            {
                i += 1.0;
                i
            };
        }
    }
    if closed {
        {
            let mut i = 0.0_f64;
            while (i < (n - 1.0_f64)) {
                let prev = (((i + n) - 2.0_f64) % (n - 1.0_f64));
                let curr = i;
                let nx0 = normals[(prev * 2.0_f64) as usize].clone();
                let ny0 = normals[((prev * 2.0_f64) + 1.0_f64) as usize].clone();
                let nx1 = normals[(curr * 2.0_f64) as usize].clone();
                let ny1 = normals[((curr * 2.0_f64) + 1.0_f64) as usize].clone();
                add_join(
                    pts[(i * 2.0_f64) as usize].clone(),
                    pts[((i * 2.0_f64) + 1.0_f64) as usize].clone(),
                    nx0,
                    ny0,
                    nx1,
                    ny1,
                    half_width,
                    (join).clone(),
                    miter_limit,
                    &mut left,
                    &mut right,
                    tolerance,
                );
                {
                    i += 1.0;
                    i
                };
            }
        }
    } else {
        let sn0x = normals[0.0_f64 as usize].clone();
        let sn0y = normals[1.0_f64 as usize].clone();
        add_cap(
            pts[0.0_f64 as usize].clone(),
            pts[1.0_f64 as usize].clone(),
            sn0x,
            sn0y,
            (-sn0y),
            sn0x,
            half_width,
            (cap).clone(),
            &mut left,
            &mut right,
            tolerance,
            true,
        );
        {
            let mut i = 1.0_f64;
            while (i < (n - 1.0_f64)) {
                let nx0 = normals[((i - 1.0_f64) * 2.0_f64) as usize].clone();
                let ny0 = normals[(((i - 1.0_f64) * 2.0_f64) + 1.0_f64) as usize].clone();
                let nx1 = normals[(i * 2.0_f64) as usize].clone();
                let ny1 = normals[((i * 2.0_f64) + 1.0_f64) as usize].clone();
                add_join(
                    pts[(i * 2.0_f64) as usize].clone(),
                    pts[((i * 2.0_f64) + 1.0_f64) as usize].clone(),
                    nx0,
                    ny0,
                    nx1,
                    ny1,
                    half_width,
                    (join).clone(),
                    miter_limit,
                    &mut left,
                    &mut right,
                    tolerance,
                );
                {
                    i += 1.0;
                    i
                };
            }
        }
        let sn_lx = normals[((n - 2.0_f64) * 2.0_f64) as usize].clone();
        let sn_ly = normals[(((n - 2.0_f64) * 2.0_f64) + 1.0_f64) as usize].clone();
        add_cap(
            pts[((n - 1.0_f64) * 2.0_f64) as usize].clone(),
            pts[(((n - 1.0_f64) * 2.0_f64) + 1.0_f64) as usize].clone(),
            sn_lx,
            sn_ly,
            sn_ly,
            (-sn_lx),
            half_width,
            (cap).clone(),
            &mut left,
            &mut right,
            tolerance,
            false,
        );
    }
    if ((left.len() as f64) < 4.0_f64) {
        return;
    }
    out.commands.push(PathCommand::MOVE_TO);
    out.data.extend(vec![
        left[0.0_f64 as usize].clone(),
        left[1.0_f64 as usize].clone(),
    ]);
    {
        let mut i = 2.0_f64;
        while (i < (left.len() as f64)) {
            out.commands.push(PathCommand::LINE_TO);
            out.data.extend(vec![
                left[i as usize].clone(),
                left[(i + 1.0_f64) as usize].clone(),
            ]);
            {
                i += 2.0_f64;
                i
            };
        }
    }
    {
        let mut i = ((right.len() as f64) - 2.0_f64);
        while (i >= 0.0_f64) {
            out.commands.push(PathCommand::LINE_TO);
            out.data.extend(vec![
                right[i as usize].clone(),
                right[(i + 1.0_f64) as usize].clone(),
            ]);
            {
                i -= 2.0_f64;
                i
            };
        }
    }
    append_path_close(out);
}

// Source: upstream/packages/path/src/strokePath.ts:539 (sha256:b575a87005f395f30ed6c04a51e8b7bdc6a918aec1b8569ac6540de3723a24a0)
#[derive(Clone)]
struct DashSegment {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub closed: bool,
    pub points: Vec<f64>,
}
impl PartialEq for DashSegment {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/path/src/strokePath.ts:544 (sha256:f821d1e4b6316a1842811cb358ebce36ac387ed35affddd0adcdcb253ed2b79b)
#[derive(Clone)]
struct StrokeSubpath {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub closed: bool,
    pub points: Vec<f64>,
}
impl PartialEq for StrokeSubpath {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/path/src/strokePath.ts:549 (sha256:c149cd18371e61d38f1ab74d74b8844e0e996af976d1183480658ba491b014ef)
const MAX_SUBDIVISION_DEPTH: f64 = 16.0_f64;
