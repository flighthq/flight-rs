// @generated from upstream/packages/adjustments/src/colorLut.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::{ColorLut, ColorTransformFunction};

// Source: upstream/packages/adjustments/src/colorLut.ts:16 (sha256:c13a9de328f2a2fa17ff7a48a32452dcfa6b4c1d6e9e3fb4a2634d76bdd56a16)
pub const COLOR_LUT_DEFAULT_SIZE: f64 = 32.0_f64;

// Source: upstream/packages/adjustments/src/colorLut.ts:25 (sha256:851d3851d5d0f304f733959b7e351dc5ce7a3364f5a8208fd970c5578defb119)
pub fn bake_color_lut(transforms: &Vec<ColorTransformFunction>, size: Option<f64>) -> ColorLut {
    let size = size.unwrap_or(COLOR_LUT_DEFAULT_SIZE);
    let n = (2.0_f64).max((size).floor());
    let mut samples = vec![Default::default(); (((n * n) * n) * 3.0_f64) as usize];
    let denom = (n - 1.0_f64);
    let mut cell: Vec<f64> = vec![0.0_f64, 0.0_f64, 0.0_f64];
    let mut i = 0.0_f64;
    {
        let mut bi = 0.0_f64;
        while (bi < n) {
            let b = (bi / denom);
            {
                let mut gi = 0.0_f64;
                while (gi < n) {
                    let g = (gi / denom);
                    {
                        let mut ri = 0.0_f64;
                        while (ri < n) {
                            {
                                let __flight_index = (0.0_f64) as usize;
                                let __flight_value = (ri / denom);
                                if __flight_index == cell.len() {
                                    cell.push(__flight_value);
                                } else {
                                    cell[__flight_index] = __flight_value;
                                }
                            };
                            {
                                let __flight_index = (1.0_f64) as usize;
                                let __flight_value = g;
                                if __flight_index == cell.len() {
                                    cell.push(__flight_value);
                                } else {
                                    cell[__flight_index] = __flight_value;
                                }
                            };
                            {
                                let __flight_index = (2.0_f64) as usize;
                                let __flight_value = b;
                                if __flight_index == cell.len() {
                                    cell.push(__flight_value);
                                } else {
                                    cell[__flight_index] = __flight_value;
                                }
                            };
                            {
                                let mut k = 0.0_f64;
                                while (k < (transforms.len() as f64)) {
                                    {
                                        let __flight_callback = transforms[k as usize].clone();
                                        let __flight_result = __flight_callback.lock().unwrap()(
                                            (cell).clone(),
                                            cell[0.0_f64 as usize].clone(),
                                            cell[1.0_f64 as usize].clone(),
                                            cell[2.0_f64 as usize].clone(),
                                        );
                                        __flight_result
                                    };
                                    {
                                        k += 1.0;
                                        k
                                    };
                                }
                            }
                            {
                                let __flight_index = ({
                                    i += 1.0;
                                    i
                                }) as usize;
                                let __flight_value = clamp01(cell[0.0_f64 as usize].clone());
                                if __flight_index == samples.len() {
                                    samples.push(__flight_value);
                                } else {
                                    samples[__flight_index] = __flight_value;
                                }
                            };
                            {
                                let __flight_index = ({
                                    i += 1.0;
                                    i
                                }) as usize;
                                let __flight_value = clamp01(cell[1.0_f64 as usize].clone());
                                if __flight_index == samples.len() {
                                    samples.push(__flight_value);
                                } else {
                                    samples[__flight_index] = __flight_value;
                                }
                            };
                            {
                                let __flight_index = ({
                                    i += 1.0;
                                    i
                                }) as usize;
                                let __flight_value = clamp01(cell[2.0_f64 as usize].clone());
                                if __flight_index == samples.len() {
                                    samples.push(__flight_value);
                                } else {
                                    samples[__flight_index] = __flight_value;
                                }
                            };
                            {
                                ri += 1.0;
                                ri
                            };
                        }
                    }
                    {
                        gi += 1.0;
                        gi
                    };
                }
            }
            {
                bi += 1.0;
                bi
            };
        }
    }
    return ColorLut {
        __flight_identity: std::sync::Arc::new(()),
        size: n,
        samples: (samples).clone(),
    };
}

// Source: upstream/packages/adjustments/src/colorLut.ts:57 (sha256:4eb53f0361f1d16af24b45ed8c9a65c0f2095379a237a09183617c2e4ccdee4e)
pub fn sample_color_lut(lut: &ColorLut, out: &mut Vec<f64>, r: f64, g: f64, b: f64) -> () {
    let n = lut.size;
    let max = (n - 1.0_f64);
    let fr = (clamp01(r) * max);
    let fg = (clamp01(g) * max);
    let fb = (clamp01(b) * max);
    let r0 = (fr).floor();
    let g0 = (fg).floor();
    let b0 = (fb).floor();
    let r1 = (r0 + 1.0_f64).min(max);
    let g1 = (g0 + 1.0_f64).min(max);
    let b1 = (b0 + 1.0_f64).min(max);
    let dr = (fr - r0);
    let dg = (fg - g0);
    let db = (fb - b0);
    {
        let mut c = 0.0_f64;
        while (c < 3.0_f64) {
            let c000 = lut.samples[((((((b0 * n) + g0) * n) + r0) * 3.0_f64) + c) as usize].clone();
            let c100 = lut.samples[((((((b0 * n) + g0) * n) + r1) * 3.0_f64) + c) as usize].clone();
            let c010 = lut.samples[((((((b0 * n) + g1) * n) + r0) * 3.0_f64) + c) as usize].clone();
            let c110 = lut.samples[((((((b0 * n) + g1) * n) + r1) * 3.0_f64) + c) as usize].clone();
            let c001 = lut.samples[((((((b1 * n) + g0) * n) + r0) * 3.0_f64) + c) as usize].clone();
            let c101 = lut.samples[((((((b1 * n) + g0) * n) + r1) * 3.0_f64) + c) as usize].clone();
            let c011 = lut.samples[((((((b1 * n) + g1) * n) + r0) * 3.0_f64) + c) as usize].clone();
            let c111 = lut.samples[((((((b1 * n) + g1) * n) + r1) * 3.0_f64) + c) as usize].clone();
            let c00 = (c000 + ((c100 - c000) * dr));
            let c10 = (c010 + ((c110 - c010) * dr));
            let c01 = (c001 + ((c101 - c001) * dr));
            let c11 = (c011 + ((c111 - c011) * dr));
            let c0 = (c00 + ((c10 - c00) * dg));
            let c1 = (c01 + ((c11 - c01) * dg));
            {
                let __flight_index = (c) as usize;
                let __flight_value = (c0 + ((c1 - c0) * db));
                if __flight_index == out.len() {
                    out.push(__flight_value);
                } else {
                    out[__flight_index] = __flight_value;
                }
            };
            {
                c += 1.0;
                c
            };
        }
    }
}

// Source: upstream/packages/adjustments/src/colorLut.ts:98 (sha256:92c4452839ded0362c28adef5c15154deeaad9b404aff5129f0596af7fea21ad)
fn clamp01(v: f64) -> f64 {
    return if (v < 0.0_f64) {
        0.0_f64
    } else {
        if (v > 1.0_f64) { 1.0_f64 } else { v }
    };
}
