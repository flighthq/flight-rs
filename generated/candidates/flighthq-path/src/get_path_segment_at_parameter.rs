// @generated from upstream/packages/path/src/getPathSegmentAtParameter.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::{Path, PathCommand, Vector2Like};

// Source: upstream/packages/path/src/getPathSegmentAtParameter.ts:6 (sha256:0f0ef405560990bfcef1d1e91f8781bdc97e0c55b8695e4ccf3444c0f5fcf90a)
pub fn get_cubic_bezier_point(
    x0: f64,
    y0: f64,
    c1x: f64,
    c1y: f64,
    c2x: f64,
    c2y: f64,
    x1: f64,
    y1: f64,
    t: f64,
    out: &mut Vector2Like,
) -> Vector2Like {
    let u = (1.0_f64 - t);
    let u2 = (u * u);
    let u3 = (u2 * u);
    let t2 = (t * t);
    let t3 = (t2 * t);
    out.x =
        ((((u3 * x0) + (((3.0_f64 * u2) * t) * c1x)) + (((3.0_f64 * u) * t2) * c2x)) + (t3 * x1));
    out.y =
        ((((u3 * y0) + (((3.0_f64 * u2) * t) * c1y)) + (((3.0_f64 * u) * t2) * c2y)) + (t3 * y1));
    return out.clone();
}

// Source: upstream/packages/path/src/getPathSegmentAtParameter.ts:31 (sha256:e6e61c7041f92f22295f470c0571f3cffeef811f2b1519e87a27bddd3f706142)
pub fn get_cubic_bezier_tangent(
    x0: f64,
    y0: f64,
    c1x: f64,
    c1y: f64,
    c2x: f64,
    c2y: f64,
    x1: f64,
    y1: f64,
    t: f64,
    out: &mut Vector2Like,
) -> Vector2Like {
    let u = (1.0_f64 - t);
    let u2 = (u * u);
    let t2 = (t * t);
    out.x =
        (3.0_f64 * (((u2 * (c1x - x0)) + (((2.0_f64 * u) * t) * (c2x - c1x))) + (t2 * (x1 - c2x))));
    out.y =
        (3.0_f64 * (((u2 * (c1y - y0)) + (((2.0_f64 * u) * t) * (c2y - c1y))) + (t2 * (y1 - c2y))));
    return out.clone();
}

// Source: upstream/packages/path/src/getPathSegmentAtParameter.ts:58 (sha256:0774b1d540e108867e9836acd1e7eff82fc4e1955147e9e8153292693beec139)
pub fn get_path_segment_point_at_parameter(
    path: &Path,
    segment_index: f64,
    t: f64,
    out: &mut Vector2Like,
) -> bool {
    return walk_path_segment(path, segment_index, t, out, false);
}

// Source: upstream/packages/path/src/getPathSegmentAtParameter.ts:70 (sha256:d22dc63d1627a3b3001262e96a209a570826a7f7e41ec9e028e3ae93601c49fa)
pub fn get_path_segment_tangent_at_parameter(
    path: &Path,
    segment_index: f64,
    t: f64,
    out: &mut Vector2Like,
) -> bool {
    return walk_path_segment(path, segment_index, t, out, true);
}

// Source: upstream/packages/path/src/getPathSegmentAtParameter.ts:81 (sha256:f9a97f1a1b1a0956e19f5600f000c51c05dd0bb5b91b06b97a3b45d3446a1198)
pub fn get_quadratic_bezier_point(
    x0: f64,
    y0: f64,
    cx: f64,
    cy: f64,
    x1: f64,
    y1: f64,
    t: f64,
    out: &mut Vector2Like,
) -> Vector2Like {
    let u = (1.0_f64 - t);
    out.x = ((((u * u) * x0) + (((2.0_f64 * u) * t) * cx)) + ((t * t) * x1));
    out.y = ((((u * u) * y0) + (((2.0_f64 * u) * t) * cy)) + ((t * t) * y1));
    return out.clone();
}

// Source: upstream/packages/path/src/getPathSegmentAtParameter.ts:100 (sha256:86bf80e6594969a25e4b5c2012d55444a4639368f9f48db8a72464ed533b660f)
pub fn get_quadratic_bezier_tangent(
    x0: f64,
    y0: f64,
    cx: f64,
    cy: f64,
    x1: f64,
    y1: f64,
    t: f64,
    out: &mut Vector2Like,
) -> Vector2Like {
    let u = (1.0_f64 - t);
    out.x = (2.0_f64 * ((u * (cx - x0)) + (t * (x1 - cx))));
    out.y = (2.0_f64 * ((u * (cy - y0)) + (t * (y1 - cy))));
    return out.clone();
}

// Source: upstream/packages/path/src/getPathSegmentAtParameter.ts:118 (sha256:7e098420dac4eb9e13e074167577566e91159fcb251b1ad5e7f866f88b012928)
fn walk_path_segment(
    path: &Path,
    segment_index: f64,
    t: f64,
    out: &mut Vector2Like,
    want_tangent: bool,
) -> bool {
    let mut current_segment = 0.0_f64;
    let mut x = 0.0_f64;
    let mut y = 0.0_f64;
    let mut di = 0.0_f64;
    {
        let mut ci = 0.0_f64;
        while (ci < (path.commands.len() as f64)) {
            let command = path.commands[ci as usize].clone();
            if (command == PathCommand::MOVE_TO) {
                x = path.data[di as usize].clone();
                y = path.data[(di + 1.0_f64) as usize].clone();
                di += 2.0_f64;
            } else {
                if (command == PathCommand::WIDE_MOVE_TO) {
                    x = path.data[(di + 2.0_f64) as usize].clone();
                    y = path.data[(di + 3.0_f64) as usize].clone();
                    di += 4.0_f64;
                } else {
                    if (command == PathCommand::LINE_TO) {
                        let x1 = path.data[di as usize].clone();
                        let y1 = path.data[(di + 1.0_f64) as usize].clone();
                        di += 2.0_f64;
                        if (current_segment == segment_index) {
                            if want_tangent {
                                out.x = (x1 - x);
                                out.y = (y1 - y);
                            } else {
                                out.x = (x + (t * (x1 - x)));
                                out.y = (y + (t * (y1 - y)));
                            }
                            return true;
                        }
                        x = x1;
                        y = y1;
                        {
                            current_segment += 1.0;
                            current_segment
                        };
                    } else {
                        if (command == PathCommand::WIDE_LINE_TO) {
                            let x1 = path.data[(di + 2.0_f64) as usize].clone();
                            let y1 = path.data[(di + 3.0_f64) as usize].clone();
                            di += 4.0_f64;
                            if (current_segment == segment_index) {
                                if want_tangent {
                                    out.x = (x1 - x);
                                    out.y = (y1 - y);
                                } else {
                                    out.x = (x + (t * (x1 - x)));
                                    out.y = (y + (t * (y1 - y)));
                                }
                                return true;
                            }
                            x = x1;
                            y = y1;
                            {
                                current_segment += 1.0;
                                current_segment
                            };
                        } else {
                            if (command == PathCommand::CURVE_TO) {
                                let cx = path.data[di as usize].clone();
                                let cy = path.data[(di + 1.0_f64) as usize].clone();
                                let x1 = path.data[(di + 2.0_f64) as usize].clone();
                                let y1 = path.data[(di + 3.0_f64) as usize].clone();
                                di += 4.0_f64;
                                if (current_segment == segment_index) {
                                    if want_tangent {
                                        get_quadratic_bezier_tangent(x, y, cx, cy, x1, y1, t, out);
                                    } else {
                                        get_quadratic_bezier_point(x, y, cx, cy, x1, y1, t, out);
                                    }
                                    return true;
                                }
                                x = x1;
                                y = y1;
                                {
                                    current_segment += 1.0;
                                    current_segment
                                };
                            } else {
                                if (command == PathCommand::CUBIC_CURVE_TO) {
                                    let c1x = path.data[di as usize].clone();
                                    let c1y = path.data[(di + 1.0_f64) as usize].clone();
                                    let c2x = path.data[(di + 2.0_f64) as usize].clone();
                                    let c2y = path.data[(di + 3.0_f64) as usize].clone();
                                    let x1 = path.data[(di + 4.0_f64) as usize].clone();
                                    let y1 = path.data[(di + 5.0_f64) as usize].clone();
                                    di += 6.0_f64;
                                    if (current_segment == segment_index) {
                                        if want_tangent {
                                            get_cubic_bezier_tangent(
                                                x, y, c1x, c1y, c2x, c2y, x1, y1, t, out,
                                            );
                                        } else {
                                            get_cubic_bezier_point(
                                                x, y, c1x, c1y, c2x, c2y, x1, y1, t, out,
                                            );
                                        }
                                        return true;
                                    }
                                    x = x1;
                                    y = y1;
                                    {
                                        current_segment += 1.0;
                                        current_segment
                                    };
                                } else {
                                    if (command == PathCommand::CLOSE) {}
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
    return false;
}
