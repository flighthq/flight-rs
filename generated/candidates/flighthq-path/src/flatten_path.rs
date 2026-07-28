// @generated from upstream/packages/path/src/flattenPath.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::{Path, PathCommand};

// Source: upstream/packages/path/src/flattenPath.ts:10 (sha256:e63fdc34a9624685caa5ae7f18ba69c504e432c041d979bd8a03291b17474e13)
pub fn flatten_path(path: &Path, tolerance: Option<f64>) -> Vec<Vec<f64>> {
    let tolerance = tolerance.unwrap_or(0.25_f64);
    let tolerance_sq = (tolerance * tolerance);
    let mut contours: Vec<Vec<f64>> = vec![];
    let mut contour: Option<Vec<f64>> = None;
    let mut x = 0.0_f64;
    let mut y = 0.0_f64;
    let mut contour_start_x = 0.0_f64;
    let mut contour_start_y = 0.0_f64;
    let mut di = 0.0_f64;
    {
        let mut ci = 0.0_f64;
        while (ci < (path.commands.len() as f64)) {
            let command = path.commands[ci as usize].clone();
            if (command == PathCommand::MOVE_TO) {
                x = path.data[di as usize].clone();
                y = path.data[(di + 1.0_f64) as usize].clone();
                di += 2.0_f64;
                contour_start_x = x;
                contour_start_y = y;
                contour = Some(vec![x, y]);
                contours.push(((contour).clone().unwrap()).clone());
            } else {
                if (command == PathCommand::WIDE_MOVE_TO) {
                    x = path.data[(di + 2.0_f64) as usize].clone();
                    y = path.data[(di + 3.0_f64) as usize].clone();
                    di += 4.0_f64;
                    contour_start_x = x;
                    contour_start_y = y;
                    contour = Some(vec![x, y]);
                    contours.push(((contour).clone().unwrap()).clone());
                } else {
                    if (command == PathCommand::LINE_TO) {
                        contour = Some(ensure_contour(&mut contours, ((contour).clone()).clone()));
                        x = path.data[di as usize].clone();
                        y = path.data[(di + 1.0_f64) as usize].clone();
                        di += 2.0_f64;
                        contour.as_mut().unwrap().extend(vec![x, y]);
                    } else {
                        if (command == PathCommand::WIDE_LINE_TO) {
                            contour =
                                Some(ensure_contour(&mut contours, ((contour).clone()).clone()));
                            x = path.data[(di + 2.0_f64) as usize].clone();
                            y = path.data[(di + 3.0_f64) as usize].clone();
                            di += 4.0_f64;
                            contour.as_mut().unwrap().extend(vec![x, y]);
                        } else {
                            if (command == PathCommand::CURVE_TO) {
                                contour = Some(ensure_contour(
                                    &mut contours,
                                    ((contour).clone()).clone(),
                                ));
                                flatten_quadratic(
                                    contour.as_mut().unwrap(),
                                    x,
                                    y,
                                    path.data[di as usize].clone(),
                                    path.data[(di + 1.0_f64) as usize].clone(),
                                    path.data[(di + 2.0_f64) as usize].clone(),
                                    path.data[(di + 3.0_f64) as usize].clone(),
                                    tolerance_sq,
                                    0.0_f64,
                                );
                                x = path.data[(di + 2.0_f64) as usize].clone();
                                y = path.data[(di + 3.0_f64) as usize].clone();
                                di += 4.0_f64;
                            } else {
                                if (command == PathCommand::CUBIC_CURVE_TO) {
                                    contour = Some(ensure_contour(
                                        &mut contours,
                                        ((contour).clone()).clone(),
                                    ));
                                    flatten_cubic(
                                        contour.as_mut().unwrap(),
                                        x,
                                        y,
                                        path.data[di as usize].clone(),
                                        path.data[(di + 1.0_f64) as usize].clone(),
                                        path.data[(di + 2.0_f64) as usize].clone(),
                                        path.data[(di + 3.0_f64) as usize].clone(),
                                        path.data[(di + 4.0_f64) as usize].clone(),
                                        path.data[(di + 5.0_f64) as usize].clone(),
                                        tolerance_sq,
                                        0.0_f64,
                                    );
                                    x = path.data[(di + 4.0_f64) as usize].clone();
                                    y = path.data[(di + 5.0_f64) as usize].clone();
                                    di += 6.0_f64;
                                } else {
                                    if (command == PathCommand::CLOSE) {
                                        if ((contour).is_some())
                                            && ((x != contour_start_x) || (y != contour_start_y))
                                        {
                                            contour
                                                .as_mut()
                                                .unwrap()
                                                .extend(vec![contour_start_x, contour_start_y]);
                                        }
                                        x = contour_start_x;
                                        y = contour_start_y;
                                        contour = None;
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
    return contours;
}

// Source: upstream/packages/path/src/flattenPath.ts:93 (sha256:c149cd18371e61d38f1ab74d74b8844e0e996af976d1183480658ba491b014ef)
const MAX_SUBDIVISION_DEPTH: f64 = 16.0_f64;

// Source: upstream/packages/path/src/flattenPath.ts:97 (sha256:a483ae92fe8e2d3b6fa3a032c474e38acf538d41bdb550a9b323d4eaad406445)
fn ensure_contour(contours: &mut Vec<Vec<f64>>, contour: Option<Vec<f64>>) -> Vec<f64> {
    if (contour).is_some() {
        return ((contour.as_ref().unwrap()).clone()).clone();
    }
    let started = vec![0.0_f64, 0.0_f64];
    contours.push(((started).clone()).clone());
    return started;
}

// Source: upstream/packages/path/src/flattenPath.ts:106 (sha256:8ed98f73237bb53812125f0becf17548a48b721622141ae7b21ece6ae93a2790)
fn distance_to_chord_sq(px: f64, py: f64, x0: f64, y0: f64, x1: f64, y1: f64) -> f64 {
    let dx = (x1 - x0);
    let dy = (y1 - y0);
    let length_sq = ((dx * dx) + (dy * dy));
    if (length_sq == 0.0_f64) {
        let ax = (px - x0);
        let ay = (py - y0);
        return ((ax * ax) + (ay * ay));
    }
    let cross = ((dx * (y0 - py)) - (dy * (x0 - px)));
    return ((cross * cross) / length_sq);
}

// Source: upstream/packages/path/src/flattenPath.ts:121 (sha256:fafd98f3a9b8580bf67974d2860224102b271785a4c72c66c3a5e386759c4664)
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
    let d1 = distance_to_chord_sq(c1x, c1y, x0, y0, x1, y1);
    let d2 = distance_to_chord_sq(c2x, c2y, x0, y0, x1, y1);
    if (depth >= MAX_SUBDIVISION_DEPTH) || ((d1 <= tolerance_sq) && (d2 <= tolerance_sq)) {
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

// Source: upstream/packages/path/src/flattenPath.ts:157 (sha256:0fe38f09e3d35edb50586025a6ebf1fc10d5b371a2cc8b778b804d8d4f20b4b1)
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
    if (depth >= MAX_SUBDIVISION_DEPTH)
        || (distance_to_chord_sq(cx, cy, x0, y0, x1, y1) <= tolerance_sq)
    {
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
