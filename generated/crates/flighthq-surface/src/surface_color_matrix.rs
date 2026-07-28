// @generated from upstream/packages/surface/src/surfaceColorMatrix.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::SurfaceRegion;

// Source: upstream/packages/surface/src/surfaceColorMatrix.ts:10 (sha256:5cd5ab68fd998a7fadaf0c99395211fcaf93385345bf0b865a38443a4091fcf0)
pub fn build_surface_brightness_color_matrix(out: &mut Vec<f64>, amount: f64) -> () {
    set_color_matrix(
        out,
        vec![
            amount, 0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64, amount, 0.0_f64, 0.0_f64, 0.0_f64,
            0.0_f64, 0.0_f64, amount, 0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64, 1.0_f64,
            0.0_f64,
        ],
    );
}

// Source: upstream/packages/surface/src/surfaceColorMatrix.ts:19 (sha256:ae8d48ea77b1c05120d454308160348dbe176b6945f40b3d8af6c80ba29a5a48)
pub fn build_surface_contrast_color_matrix(out: &mut Vec<f64>, amount: f64) -> () {
    let t = (127.5_f64 * (1.0_f64 - amount));
    set_color_matrix(
        out,
        vec![
            amount, 0.0_f64, 0.0_f64, 0.0_f64, t, 0.0_f64, amount, 0.0_f64, 0.0_f64, t, 0.0_f64,
            0.0_f64, amount, 0.0_f64, t, 0.0_f64, 0.0_f64, 0.0_f64, 1.0_f64, 0.0_f64,
        ],
    );
}

// Source: upstream/packages/surface/src/surfaceColorMatrix.ts:28 (sha256:069a10741276a956d5d3e287e4620bdec0e7a289446029be067f7bf7a098b8f7)
pub fn build_surface_grayscale_color_matrix(out: &mut Vec<f64>) -> () {
    build_surface_saturation_color_matrix(out, 0.0_f64);
}

// Source: upstream/packages/surface/src/surfaceColorMatrix.ts:37 (sha256:8962777e0533ee8070e2655ba9b5ec771a8ed269a8b60667a18be202b9c311c8)
pub fn build_surface_hue_rotation_color_matrix(out: &mut Vec<f64>, degrees: f64) -> () {
    let radians = ((degrees * std::f64::consts::PI) / 180.0_f64);
    let c = (radians).cos();
    let s = (radians).sin();
    set_color_matrix(
        out,
        vec![
            ((0.213_f64 + (c * 0.787_f64)) - (s * 0.213_f64)),
            ((0.715_f64 - (c * 0.715_f64)) - (s * 0.715_f64)),
            ((0.072_f64 - (c * 0.072_f64)) + (s * 0.928_f64)),
            0.0_f64,
            0.0_f64,
            ((0.213_f64 - (c * 0.213_f64)) + (s * 0.143_f64)),
            ((0.715_f64 + (c * 0.285_f64)) + (s * 0.14_f64)),
            ((0.072_f64 - (c * 0.072_f64)) - (s * 0.283_f64)),
            0.0_f64,
            0.0_f64,
            ((0.213_f64 - (c * 0.213_f64)) - (s * 0.787_f64)),
            ((0.715_f64 - (c * 0.715_f64)) + (s * 0.715_f64)),
            ((0.072_f64 + (c * 0.928_f64)) + (s * 0.072_f64)),
            0.0_f64,
            0.0_f64,
            0.0_f64,
            0.0_f64,
            0.0_f64,
            1.0_f64,
            0.0_f64,
        ],
    );
}

// Source: upstream/packages/surface/src/surfaceColorMatrix.ts:70 (sha256:683cfc77d56cd39bc18835f88d36095b457b3be23639a017f3f1833ecedc5314)
pub fn build_surface_invert_color_matrix(out: &mut Vec<f64>) -> () {
    set_color_matrix(
        out,
        vec![
            (-1.0_f64),
            0.0_f64,
            0.0_f64,
            0.0_f64,
            255.0_f64,
            0.0_f64,
            (-1.0_f64),
            0.0_f64,
            0.0_f64,
            255.0_f64,
            0.0_f64,
            0.0_f64,
            (-1.0_f64),
            0.0_f64,
            255.0_f64,
            0.0_f64,
            0.0_f64,
            0.0_f64,
            1.0_f64,
            0.0_f64,
        ],
    );
}

// Source: upstream/packages/surface/src/surfaceColorMatrix.ts:79 (sha256:52fc5efce2fb0a7d31fe63094f27cb85bfce6b0a485ee3a245c847213963e934)
pub fn build_surface_saturation_color_matrix(out: &mut Vec<f64>, amount: f64) -> () {
    let inv = (1.0_f64 - amount);
    let r = (LUMA_R * inv);
    let g = (LUMA_G * inv);
    let b = (LUMA_B * inv);
    set_color_matrix(
        out,
        vec![
            (r + amount),
            g,
            b,
            0.0_f64,
            0.0_f64,
            r,
            (g + amount),
            b,
            0.0_f64,
            0.0_f64,
            r,
            g,
            (b + amount),
            0.0_f64,
            0.0_f64,
            0.0_f64,
            0.0_f64,
            0.0_f64,
            1.0_f64,
            0.0_f64,
        ],
    );
}

// Source: upstream/packages/surface/src/surfaceColorMatrix.ts:90 (sha256:99fa07d105b7a270a8f9c524b14d4a82059ec64bb403795b091d7f9cf35dbd21)
pub fn build_surface_sepia_color_matrix(out: &mut Vec<f64>) -> () {
    set_color_matrix(
        out,
        vec![
            0.393_f64, 0.769_f64, 0.189_f64, 0.0_f64, 0.0_f64, 0.349_f64, 0.686_f64, 0.168_f64,
            0.0_f64, 0.0_f64, 0.272_f64, 0.534_f64, 0.131_f64, 0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64,
            0.0_f64, 1.0_f64, 0.0_f64,
        ],
    );
}

// Source: upstream/packages/surface/src/surfaceColorMatrix.ts:102 (sha256:2783d62795f430ec2c47013d9ad1b1fdece7f9104f275d9bb4c97a9a7fe1d357)
pub fn color_matrix_surface(out: &mut Vec<u8>, source: &SurfaceRegion, matrix: &Vec<f64>) -> () {
    if ((matrix.len() as f64) < 20.0_f64) {
        panic!("Color matrix filter requires 20 values");
    }
    {
        let mut py = 0.0_f64;
        while (py < source.height) {
            let source_y = (source.y + py);
            if ((source_y < 0.0_f64) || (source_y >= source.surface.height)) {
                {
                    py += 1.0;
                    py
                };
                continue;
            }
            {
                let mut px = 0.0_f64;
                while (px < source.width) {
                    let source_x = (source.x + px);
                    if ((source_x < 0.0_f64) || (source_x >= source.surface.width)) {
                        {
                            px += 1.0;
                            px
                        };
                        continue;
                    }
                    let si = (((source_y * source.surface.width) + source_x) * 4.0_f64);
                    let di = (((py * source.width) + px) * 4.0_f64);
                    let r = (source.surface.data[si as usize] as f64);
                    let g = (source.surface.data[(si + 1.0_f64) as usize] as f64);
                    let b = (source.surface.data[(si + 2.0_f64) as usize] as f64);
                    let a = (source.surface.data[(si + 3.0_f64) as usize] as f64);
                    out[di as usize] = (clamp_byte(
                        (((((r * matrix[0.0_f64 as usize].clone())
                            + (g * matrix[1.0_f64 as usize].clone()))
                            + (b * matrix[2.0_f64 as usize].clone()))
                            + (a * matrix[3.0_f64 as usize].clone()))
                            + matrix[4.0_f64 as usize].clone()),
                    )) as u8;
                    out[(di + 1.0_f64) as usize] = (clamp_byte(
                        (((((r * matrix[5.0_f64 as usize].clone())
                            + (g * matrix[6.0_f64 as usize].clone()))
                            + (b * matrix[7.0_f64 as usize].clone()))
                            + (a * matrix[8.0_f64 as usize].clone()))
                            + matrix[9.0_f64 as usize].clone()),
                    )) as u8;
                    out[(di + 2.0_f64) as usize] = (clamp_byte(
                        (((((r * matrix[10.0_f64 as usize].clone())
                            + (g * matrix[11.0_f64 as usize].clone()))
                            + (b * matrix[12.0_f64 as usize].clone()))
                            + (a * matrix[13.0_f64 as usize].clone()))
                            + matrix[14.0_f64 as usize].clone()),
                    )) as u8;
                    out[(di + 3.0_f64) as usize] = (clamp_byte(
                        (((((r * matrix[15.0_f64 as usize].clone())
                            + (g * matrix[16.0_f64 as usize].clone()))
                            + (b * matrix[17.0_f64 as usize].clone()))
                            + (a * matrix[18.0_f64 as usize].clone()))
                            + matrix[19.0_f64 as usize].clone()),
                    )) as u8;
                    {
                        px += 1.0;
                        px
                    };
                }
            }
            {
                py += 1.0;
                py
            };
        }
    }
}

// Source: upstream/packages/surface/src/surfaceColorMatrix.ts:133 (sha256:f73077f90c2950a1d6d0b25d94b7049b207a87f825ed425b4482cf950df49fb2)
pub fn concat_surface_color_matrix(out: &mut Vec<f64>, first: &Vec<f64>, second: &Vec<f64>) -> () {
    {
        let mut row = 0.0_f64;
        while (row < 4.0_f64) {
            {
                let mut col = 0.0_f64;
                while (col < 5.0_f64) {
                    let mut sum = if (col == 4.0_f64) {
                        second[((row * 5.0_f64) + 4.0_f64) as usize].clone()
                    } else {
                        0.0_f64
                    };
                    {
                        let mut k = 0.0_f64;
                        while (k < 4.0_f64) {
                            sum += (second[((row * 5.0_f64) + k) as usize].clone()
                                * if (col == 4.0_f64) {
                                    first[((k * 5.0_f64) + 4.0_f64) as usize].clone()
                                } else {
                                    first[((k * 5.0_f64) + col) as usize].clone()
                                });
                            {
                                k += 1.0;
                                k
                            };
                        }
                    }
                    out[((row * 5.0_f64) + col) as usize] = sum;
                    {
                        col += 1.0;
                        col
                    };
                }
            }
            {
                row += 1.0;
                row
            };
        }
    }
}

// Source: upstream/packages/surface/src/surfaceColorMatrix.ts:152 (sha256:b3adff938cb05477d8a11e11cb93411f32c7d78297b0d7305a7fcc3c5cbe7325)
pub fn set_surface_color_matrix_identity(out: &mut Vec<f64>) -> () {
    set_color_matrix(
        out,
        vec![
            1.0_f64, 0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64, 1.0_f64, 0.0_f64, 0.0_f64,
            0.0_f64, 0.0_f64, 0.0_f64, 1.0_f64, 0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64,
            1.0_f64, 0.0_f64,
        ],
    );
}

// Source: upstream/packages/surface/src/surfaceColorMatrix.ts:158 (sha256:2a302ed45599ea75aeebeba19271e15169e82d310567d6d3c2d9c562eb21cb9e)
const LUMA_R: f64 = 0.213_f64;

// Source: upstream/packages/surface/src/surfaceColorMatrix.ts:159 (sha256:b26588b559b83e271bb1a34292f68ad5e15b8ca8bad00b727b919952f9f4f916)
const LUMA_G: f64 = 0.715_f64;

// Source: upstream/packages/surface/src/surfaceColorMatrix.ts:160 (sha256:84d3c94e8e08bc4d90b457cfc9db5493eb93ce704c18c73ca51762ce663c3cc6)
const LUMA_B: f64 = 0.072_f64;

// Source: upstream/packages/surface/src/surfaceColorMatrix.ts:162 (sha256:c06b5172b1cceb605b49e48481c95614cef14ecc1c40b33115d5ac45594da56a)
fn clamp_byte(value: f64) -> f64 {
    return (0.0_f64).max((255.0_f64).min((value).round()));
}

// Source: upstream/packages/surface/src/surfaceColorMatrix.ts:166 (sha256:6af38fd4148f273001d1e45c8a0c367998c72e12cd3435288ecacafcbe5e7bf4)
fn set_color_matrix(out: &mut Vec<f64>, values: Vec<f64>) -> () {
    {
        let mut i = 0.0_f64;
        while (i < 20.0_f64) {
            out[i as usize] = values[i as usize].clone();
            {
                i += 1.0;
                i
            };
        }
    }
}
