// @generated from upstream/packages/path/src/transformPath.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::copy_path;
use flighthq_types::{Matrix, MatrixLike, Path, PathCommand};

// Source: upstream/packages/path/src/transformPath.ts:12 (sha256:196c70663e7fa56160a411be71604f40cf01472133e577447f96205043a975e4)
pub fn transform_path(source: &Path, matrix: &MatrixLike, out: &mut Path) -> () {
    let a = matrix.a;
    let b = matrix.b;
    let c = matrix.c;
    let d = matrix.d;
    let tx = matrix.tx;
    let ty = matrix.ty;
    copy_path(source, Some(((*out).clone()).clone()));
    let mut di = 0.0_f64;
    {
        let mut ci = 0.0_f64;
        while (ci < (source.commands.len() as f64)) {
            let command = source.commands[ci as usize].clone();
            if (command == PathCommand::MOVE_TO) || (command == PathCommand::LINE_TO) {
                let x = out.data[di as usize].clone();
                let y = out.data[(di + 1.0_f64) as usize].clone();
                {
                    let __flight_index = (di) as usize;
                    let __flight_value = (((a * x) + (c * y)) + tx);
                    if __flight_index == out.data.len() {
                        out.data.push(__flight_value);
                    } else {
                        out.data[__flight_index] = __flight_value;
                    }
                };
                {
                    let __flight_index = (di + 1.0_f64) as usize;
                    let __flight_value = (((b * x) + (d * y)) + ty);
                    if __flight_index == out.data.len() {
                        out.data.push(__flight_value);
                    } else {
                        out.data[__flight_index] = __flight_value;
                    }
                };
                di += 2.0_f64;
            } else {
                if (command == PathCommand::CURVE_TO) {
                    {
                        let mut k = 0.0_f64;
                        while (k < 4.0_f64) {
                            let x = out.data[(di + k) as usize].clone();
                            let y = out.data[((di + k) + 1.0_f64) as usize].clone();
                            {
                                let __flight_index = (di + k) as usize;
                                let __flight_value = (((a * x) + (c * y)) + tx);
                                if __flight_index == out.data.len() {
                                    out.data.push(__flight_value);
                                } else {
                                    out.data[__flight_index] = __flight_value;
                                }
                            };
                            {
                                let __flight_index = ((di + k) + 1.0_f64) as usize;
                                let __flight_value = (((b * x) + (d * y)) + ty);
                                if __flight_index == out.data.len() {
                                    out.data.push(__flight_value);
                                } else {
                                    out.data[__flight_index] = __flight_value;
                                }
                            };
                            {
                                k += 2.0_f64;
                                k
                            };
                        }
                    }
                    di += 4.0_f64;
                } else {
                    if (command == PathCommand::CUBIC_CURVE_TO) {
                        {
                            let mut k = 0.0_f64;
                            while (k < 6.0_f64) {
                                let x = out.data[(di + k) as usize].clone();
                                let y = out.data[((di + k) + 1.0_f64) as usize].clone();
                                {
                                    let __flight_index = (di + k) as usize;
                                    let __flight_value = (((a * x) + (c * y)) + tx);
                                    if __flight_index == out.data.len() {
                                        out.data.push(__flight_value);
                                    } else {
                                        out.data[__flight_index] = __flight_value;
                                    }
                                };
                                {
                                    let __flight_index = ((di + k) + 1.0_f64) as usize;
                                    let __flight_value = (((b * x) + (d * y)) + ty);
                                    if __flight_index == out.data.len() {
                                        out.data.push(__flight_value);
                                    } else {
                                        out.data[__flight_index] = __flight_value;
                                    }
                                };
                                {
                                    k += 2.0_f64;
                                    k
                                };
                            }
                        }
                        di += 6.0_f64;
                    } else {
                        if (command == PathCommand::WIDE_MOVE_TO)
                            || (command == PathCommand::WIDE_LINE_TO)
                        {
                            let x = out.data[(di + 2.0_f64) as usize].clone();
                            let y = out.data[(di + 3.0_f64) as usize].clone();
                            {
                                let __flight_index = (di + 2.0_f64) as usize;
                                let __flight_value = (((a * x) + (c * y)) + tx);
                                if __flight_index == out.data.len() {
                                    out.data.push(__flight_value);
                                } else {
                                    out.data[__flight_index] = __flight_value;
                                }
                            };
                            {
                                let __flight_index = (di + 3.0_f64) as usize;
                                let __flight_value = (((b * x) + (d * y)) + ty);
                                if __flight_index == out.data.len() {
                                    out.data.push(__flight_value);
                                } else {
                                    out.data[__flight_index] = __flight_value;
                                }
                            };
                            di += 4.0_f64;
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
}

// Source: upstream/packages/path/src/transformPath.ts:60 (sha256:f763f9b442a73afa9bace92d5e948cde272b6e2a9dca9d8dace3bbbd4696049f)
pub fn translate_path(source: &Path, dx: f64, dy: f64, out: &mut Path) -> () {
    transform_path(
        source,
        &Matrix {
            __flight_identity: std::sync::Arc::new(()),
            __flight_entity_runtime: Default::default(),
            a: 1.0_f64,
            b: 0.0_f64,
            c: 0.0_f64,
            d: 1.0_f64,
            tx: dx,
            ty: dy,
        },
        out,
    );
}
