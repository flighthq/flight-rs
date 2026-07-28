// @generated from upstream/packages/path/src/containsPathPoint.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

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

// Source: upstream/packages/path/src/containsPathPoint.ts:14 (sha256:d0d655a0aa053a569856a85db398197d41de42d92d6001cc684b5c5eb68858c4)
pub fn contains_path_point(path: &Path, px: f64, py: f64, tolerance: Option<f64>) -> bool {
    let tolerance = tolerance.unwrap_or(0.25_f64);
    let winding = compute_path_winding_number(path, px, py, tolerance);
    if ((path.winding).clone() == "evenOdd") {
        return ((__flight_js_to_i32(winding) & __flight_js_to_i32(1.0_f64)) as f64 != 0.0_f64);
    }
    return (winding != 0.0_f64);
}

// Source: upstream/packages/path/src/containsPathPoint.ts:21 (sha256:924bcc41beff433211af011182edc9f6064938b7c056fe129221586043625ec6)
fn chord_dist_sq(px: f64, py: f64, x0: f64, y0: f64, x1: f64, y1: f64) -> f64 {
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

// Source: upstream/packages/path/src/containsPathPoint.ts:36 (sha256:7ba67379dc09d9f60eb6460f6d637614c668602eb56e7e0f8d6e56d078b9f654)
fn compute_path_winding_number(path: &Path, px: f64, py: f64, tolerance: f64) -> f64 {
    let tolerance_sq = (tolerance * tolerance);
    let winding_number: std::sync::Arc<std::sync::Mutex<f64>> =
        std::sync::Arc::new(std::sync::Mutex::new(0.0_f64));
    let mut x = 0.0_f64;
    let mut y = 0.0_f64;
    let contour_start_x: std::sync::Arc<std::sync::Mutex<f64>> =
        std::sync::Arc::new(std::sync::Mutex::new(0.0_f64));
    let contour_start_y: std::sync::Arc<std::sync::Mutex<f64>> =
        std::sync::Arc::new(std::sync::Mutex::new(0.0_f64));
    let has_contour: std::sync::Arc<std::sync::Mutex<bool>> =
        std::sync::Arc::new(std::sync::Mutex::new(false));
    let last_x: std::sync::Arc<std::sync::Mutex<f64>> =
        std::sync::Arc::new(std::sync::Mutex::new(0.0_f64));
    let last_y: std::sync::Arc<std::sync::Mutex<f64>> =
        std::sync::Arc::new(std::sync::Mutex::new(0.0_f64));
    let mut di = 0.0_f64;
    let mut flush_contour: std::sync::Arc<
        std::sync::Mutex<Box<dyn FnMut() -> () + Send + 'static>>,
    > = std::sync::Arc::new(std::sync::Mutex::new(Box::new({
        let mut contour_start_x = contour_start_x.clone();
        let mut contour_start_y = contour_start_y.clone();
        let mut has_contour = has_contour.clone();
        let mut last_x = last_x.clone();
        let mut last_y = last_y.clone();
        let mut winding_number = winding_number.clone();
        move || -> () {
            if (*has_contour.lock().unwrap()).clone() {
                (*winding_number.lock().unwrap()) += count_segment_crossings(
                    px,
                    py,
                    (*last_x.lock().unwrap()).clone(),
                    (*last_y.lock().unwrap()).clone(),
                    (*contour_start_x.lock().unwrap()).clone(),
                    (*contour_start_y.lock().unwrap()).clone(),
                );
            }
        }
    })
        as Box<dyn FnMut() -> () + Send + 'static>));
    {
        let mut ci = 0.0_f64;
        while (ci < (path.commands.len() as f64)) {
            let command = path.commands[ci as usize].clone();
            if (command == PathCommand::MOVE_TO) {
                ((flush_contour).clone()).lock().unwrap()();
                x = path.data[di as usize].clone();
                y = path.data[(di + 1.0_f64) as usize].clone();
                di += 2.0_f64;
                (*contour_start_x.lock().unwrap()) = x;
                (*contour_start_y.lock().unwrap()) = y;
                (*last_x.lock().unwrap()) = x;
                (*last_y.lock().unwrap()) = y;
                (*has_contour.lock().unwrap()) = true;
            } else {
                if (command == PathCommand::WIDE_MOVE_TO) {
                    ((flush_contour).clone()).lock().unwrap()();
                    x = path.data[(di + 2.0_f64) as usize].clone();
                    y = path.data[(di + 3.0_f64) as usize].clone();
                    di += 4.0_f64;
                    (*contour_start_x.lock().unwrap()) = x;
                    (*contour_start_y.lock().unwrap()) = y;
                    (*last_x.lock().unwrap()) = x;
                    (*last_y.lock().unwrap()) = y;
                    (*has_contour.lock().unwrap()) = true;
                } else {
                    if (command == PathCommand::LINE_TO) {
                        let nx = path.data[di as usize].clone();
                        let ny = path.data[(di + 1.0_f64) as usize].clone();
                        di += 2.0_f64;
                        if (*has_contour.lock().unwrap()).clone() {
                            (*winding_number.lock().unwrap()) += count_segment_crossings(
                                px,
                                py,
                                (*last_x.lock().unwrap()).clone(),
                                (*last_y.lock().unwrap()).clone(),
                                nx,
                                ny,
                            );
                        }
                        (*last_x.lock().unwrap()) = nx;
                        (*last_y.lock().unwrap()) = ny;
                        x = nx;
                        y = ny;
                    } else {
                        if (command == PathCommand::WIDE_LINE_TO) {
                            let nx = path.data[(di + 2.0_f64) as usize].clone();
                            let ny = path.data[(di + 3.0_f64) as usize].clone();
                            di += 4.0_f64;
                            if (*has_contour.lock().unwrap()).clone() {
                                (*winding_number.lock().unwrap()) += count_segment_crossings(
                                    px,
                                    py,
                                    (*last_x.lock().unwrap()).clone(),
                                    (*last_y.lock().unwrap()).clone(),
                                    nx,
                                    ny,
                                );
                            }
                            (*last_x.lock().unwrap()) = nx;
                            (*last_y.lock().unwrap()) = ny;
                            x = nx;
                            y = ny;
                        } else {
                            if (command == PathCommand::CURVE_TO) {
                                let cx = path.data[di as usize].clone();
                                let cy = path.data[(di + 1.0_f64) as usize].clone();
                                let ax = path.data[(di + 2.0_f64) as usize].clone();
                                let ay = path.data[(di + 3.0_f64) as usize].clone();
                                di += 4.0_f64;
                                if (*has_contour.lock().unwrap()).clone() {
                                    (*winding_number.lock().unwrap()) +=
                                        flatten_quadratic_winding_number(
                                            px,
                                            py,
                                            (*last_x.lock().unwrap()).clone(),
                                            (*last_y.lock().unwrap()).clone(),
                                            cx,
                                            cy,
                                            ax,
                                            ay,
                                            tolerance_sq,
                                            0.0_f64,
                                        );
                                }
                                (*last_x.lock().unwrap()) = ax;
                                (*last_y.lock().unwrap()) = ay;
                                x = ax;
                                y = ay;
                            } else {
                                if (command == PathCommand::CUBIC_CURVE_TO) {
                                    let c1x = path.data[di as usize].clone();
                                    let c1y = path.data[(di + 1.0_f64) as usize].clone();
                                    let c2x = path.data[(di + 2.0_f64) as usize].clone();
                                    let c2y = path.data[(di + 3.0_f64) as usize].clone();
                                    let ax = path.data[(di + 4.0_f64) as usize].clone();
                                    let ay = path.data[(di + 5.0_f64) as usize].clone();
                                    di += 6.0_f64;
                                    if (*has_contour.lock().unwrap()).clone() {
                                        (*winding_number.lock().unwrap()) +=
                                            flatten_cubic_winding_number(
                                                px,
                                                py,
                                                (*last_x.lock().unwrap()).clone(),
                                                (*last_y.lock().unwrap()).clone(),
                                                c1x,
                                                c1y,
                                                c2x,
                                                c2y,
                                                ax,
                                                ay,
                                                tolerance_sq,
                                                0.0_f64,
                                            );
                                    }
                                    (*last_x.lock().unwrap()) = ax;
                                    (*last_y.lock().unwrap()) = ay;
                                    x = ax;
                                    y = ay;
                                } else {
                                    if (command == PathCommand::CLOSE) {
                                        if (*has_contour.lock().unwrap()).clone() {
                                            (*winding_number.lock().unwrap()) +=
                                                count_segment_crossings(
                                                    px,
                                                    py,
                                                    (*last_x.lock().unwrap()).clone(),
                                                    (*last_y.lock().unwrap()).clone(),
                                                    (*contour_start_x.lock().unwrap()).clone(),
                                                    (*contour_start_y.lock().unwrap()).clone(),
                                                );
                                            (*last_x.lock().unwrap()) =
                                                (*contour_start_x.lock().unwrap()).clone();
                                            (*last_y.lock().unwrap()) =
                                                (*contour_start_y.lock().unwrap()).clone();
                                            x = (*contour_start_x.lock().unwrap()).clone();
                                            y = (*contour_start_y.lock().unwrap()).clone();
                                            (*has_contour.lock().unwrap()) = false;
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
    ((flush_contour).clone()).lock().unwrap()();
    return ((*winding_number.lock().unwrap()).clone()).abs();
}

// Source: upstream/packages/path/src/containsPathPoint.ts:146 (sha256:2d8460c6d1d7eae4b9cdaf0cf6789700dbae74522972cd0358fefa500510b87e)
fn count_segment_crossings(px: f64, py: f64, x0: f64, y0: f64, x1: f64, y1: f64) -> f64 {
    if (((y0 <= py) && (y1 > py)) || ((y1 <= py) && (y0 > py))) {
        let cross_x = (x0 + (((py - y0) * (x1 - x0)) / (y1 - y0)));
        if (px < cross_x) {
            return if (y1 > y0) { 1.0_f64 } else { (-1.0_f64) };
        }
    }
    return 0.0_f64;
}

// Source: upstream/packages/path/src/containsPathPoint.ts:158 (sha256:0e76693aaf9738b899934f5347cba56c478534d043a8d837d508e7cf9828621b)
fn flatten_cubic_winding_number(
    px: f64,
    py: f64,
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
) -> f64 {
    let d1 = chord_dist_sq(c1x, c1y, x0, y0, x1, y1);
    let d2 = chord_dist_sq(c2x, c2y, x0, y0, x1, y1);
    if ((depth >= MAX_SUBDIVISION_DEPTH) || ((d1 <= tolerance_sq) && (d2 <= tolerance_sq))) {
        return count_segment_crossings(px, py, x0, y0, x1, y1);
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
    return (flatten_cubic_winding_number(
        px,
        py,
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
    ) + flatten_cubic_winding_number(
        px,
        py,
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
    ));
}

// Source: upstream/packages/path/src/containsPathPoint.ts:195 (sha256:0f3ff38729bb97f6bab01726d200103615f2e0a8bff7ace307a74a2525d19ac2)
fn flatten_quadratic_winding_number(
    px: f64,
    py: f64,
    x0: f64,
    y0: f64,
    cx: f64,
    cy: f64,
    x1: f64,
    y1: f64,
    tolerance_sq: f64,
    depth: f64,
) -> f64 {
    if ((depth >= MAX_SUBDIVISION_DEPTH) || (chord_dist_sq(cx, cy, x0, y0, x1, y1) <= tolerance_sq))
    {
        return count_segment_crossings(px, py, x0, y0, x1, y1);
    }
    let mx01 = ((x0 + cx) / 2.0_f64);
    let my01 = ((y0 + cy) / 2.0_f64);
    let mx12 = ((cx + x1) / 2.0_f64);
    let my12 = ((cy + y1) / 2.0_f64);
    let mx = ((mx01 + mx12) / 2.0_f64);
    let my = ((my01 + my12) / 2.0_f64);
    return (flatten_quadratic_winding_number(
        px,
        py,
        x0,
        y0,
        mx01,
        my01,
        mx,
        my,
        tolerance_sq,
        (depth + 1.0_f64),
    ) + flatten_quadratic_winding_number(
        px,
        py,
        mx,
        my,
        mx12,
        my12,
        x1,
        y1,
        tolerance_sq,
        (depth + 1.0_f64),
    ));
}

// Source: upstream/packages/path/src/containsPathPoint.ts:222 (sha256:c149cd18371e61d38f1ab74d74b8844e0e996af976d1183480658ba491b014ef)
const MAX_SUBDIVISION_DEPTH: f64 = 16.0_f64;
