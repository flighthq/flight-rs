// @generated from upstream/packages/bitmap/src/bitmapColorMatrix.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::BitmapRegion;

// Source: upstream/packages/bitmap/src/bitmapColorMatrix.ts:10 (sha256:d8044ee9bfbf76aa9e4bee8a1fa5905c9f7be1159398460bb8a91c9cf2d844b6)
pub fn build_bitmap_brightness_color_matrix(out: &mut Vec<f64>, amount: f64) -> () {
    set_color_matrix(
        out,
        vec![
            amount, 0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64, amount, 0.0_f64, 0.0_f64, 0.0_f64,
            0.0_f64, 0.0_f64, amount, 0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64, 1.0_f64,
            0.0_f64,
        ],
    );
}

// Source: upstream/packages/bitmap/src/bitmapColorMatrix.ts:19 (sha256:5f0792f10aa13e3276b33312ed0eb8dbe7576e6cc3f6d07f005ad8a1182799fd)
pub fn build_bitmap_contrast_color_matrix(out: &mut Vec<f64>, amount: f64) -> () {
    let t = (127.5_f64 * (1.0_f64 - amount));
    set_color_matrix(
        out,
        vec![
            amount, 0.0_f64, 0.0_f64, 0.0_f64, t, 0.0_f64, amount, 0.0_f64, 0.0_f64, t, 0.0_f64,
            0.0_f64, amount, 0.0_f64, t, 0.0_f64, 0.0_f64, 0.0_f64, 1.0_f64, 0.0_f64,
        ],
    );
}

// Source: upstream/packages/bitmap/src/bitmapColorMatrix.ts:28 (sha256:ff1e27425c4db95628c11f6c2e7e184fbb99ac443ab90c1feaf5d91c32eca3a9)
pub fn build_bitmap_grayscale_color_matrix(out: &mut Vec<f64>) -> () {
    build_bitmap_saturation_color_matrix(out, 0.0_f64);
}

// Source: upstream/packages/bitmap/src/bitmapColorMatrix.ts:37 (sha256:f0b43d88ba4ca61fae12f4bf987b72726536bd0dec94b9014f6cfd9b63fa540d)
pub fn build_bitmap_hue_rotation_color_matrix(out: &mut Vec<f64>, degrees: f64) -> () {
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

// Source: upstream/packages/bitmap/src/bitmapColorMatrix.ts:70 (sha256:1277694d5bed23ceddbb16c808c5500c749e02f5cbf44cfe9a788288e5662472)
pub fn build_bitmap_invert_color_matrix(out: &mut Vec<f64>) -> () {
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

// Source: upstream/packages/bitmap/src/bitmapColorMatrix.ts:79 (sha256:ccc868f570d54a09a31cb13d795ce7f80d41474a85d19450b963ffaa96dae19e)
pub fn build_bitmap_saturation_color_matrix(out: &mut Vec<f64>, amount: f64) -> () {
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

// Source: upstream/packages/bitmap/src/bitmapColorMatrix.ts:90 (sha256:daf6021c3f536909e25d2a2b08c622e85dd49778200fa256418a751b6ff79289)
pub fn build_bitmap_sepia_color_matrix(out: &mut Vec<f64>) -> () {
    set_color_matrix(
        out,
        vec![
            0.393_f64, 0.769_f64, 0.189_f64, 0.0_f64, 0.0_f64, 0.349_f64, 0.686_f64, 0.168_f64,
            0.0_f64, 0.0_f64, 0.272_f64, 0.534_f64, 0.131_f64, 0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64,
            0.0_f64, 1.0_f64, 0.0_f64,
        ],
    );
}

// Source: upstream/packages/bitmap/src/bitmapColorMatrix.ts:102 (sha256:ac763a6a19acdc4e50146235ccfcc389cf222af67d060d846abfee8858f2c0b0)
pub fn color_matrix_bitmap(out: &mut Vec<u8>, source: &BitmapRegion, matrix: &Vec<f64>) -> () {
    if ((matrix.len() as f64) < 20.0_f64) {
        panic!("{}", "Color matrix filter requires 20 values");
    }
    {
        let mut py = 0.0_f64;
        while (py < source.height) {
            let source_y = (source.y + py);
            if (source_y < 0.0_f64) || (source_y >= source.bitmap.height) {
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
                    if (source_x < 0.0_f64) || (source_x >= source.bitmap.width) {
                        {
                            px += 1.0;
                            px
                        };
                        continue;
                    }
                    let si = (((source_y * source.bitmap.width) + source_x) * 4.0_f64);
                    let di = (((py * source.width) + px) * 4.0_f64);
                    let r = (source.bitmap.data[si as usize] as f64);
                    let g = (source.bitmap.data[(si + 1.0_f64) as usize] as f64);
                    let b = (source.bitmap.data[(si + 2.0_f64) as usize] as f64);
                    let a = (source.bitmap.data[(si + 3.0_f64) as usize] as f64);
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

// Source: upstream/packages/bitmap/src/bitmapColorMatrix.ts:133 (sha256:fbc8c3a5b07cbb24eca770b30551afa714110450e6935587551b65bee855010c)
pub fn concat_bitmap_color_matrix(out: &mut Vec<f64>, first: &Vec<f64>, second: &Vec<f64>) -> () {
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
                    {
                        let __flight_index = ((row * 5.0_f64) + col) as usize;
                        let __flight_value = sum;
                        if __flight_index == out.len() {
                            out.push(__flight_value);
                        } else {
                            out[__flight_index] = __flight_value;
                        }
                    };
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

// Source: upstream/packages/bitmap/src/bitmapColorMatrix.ts:152 (sha256:d73280c6e6af05cdcb257f4df5c8aac7375c46b66a5b8f7aa50e96a182a5af2a)
pub fn set_bitmap_color_matrix_identity(out: &mut Vec<f64>) -> () {
    set_color_matrix(
        out,
        vec![
            1.0_f64, 0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64, 1.0_f64, 0.0_f64, 0.0_f64,
            0.0_f64, 0.0_f64, 0.0_f64, 1.0_f64, 0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64,
            1.0_f64, 0.0_f64,
        ],
    );
}

// Source: upstream/packages/bitmap/src/bitmapColorMatrix.ts:158 (sha256:2a302ed45599ea75aeebeba19271e15169e82d310567d6d3c2d9c562eb21cb9e)
const LUMA_R: f64 = 0.213_f64;

// Source: upstream/packages/bitmap/src/bitmapColorMatrix.ts:159 (sha256:b26588b559b83e271bb1a34292f68ad5e15b8ca8bad00b727b919952f9f4f916)
const LUMA_G: f64 = 0.715_f64;

// Source: upstream/packages/bitmap/src/bitmapColorMatrix.ts:160 (sha256:84d3c94e8e08bc4d90b457cfc9db5493eb93ce704c18c73ca51762ce663c3cc6)
const LUMA_B: f64 = 0.072_f64;

// Source: upstream/packages/bitmap/src/bitmapColorMatrix.ts:162 (sha256:c06b5172b1cceb605b49e48481c95614cef14ecc1c40b33115d5ac45594da56a)
fn clamp_byte(value: f64) -> f64 {
    return (0.0_f64).max((255.0_f64).min((value).round()));
}

// Source: upstream/packages/bitmap/src/bitmapColorMatrix.ts:166 (sha256:6af38fd4148f273001d1e45c8a0c367998c72e12cd3435288ecacafcbe5e7bf4)
fn set_color_matrix(out: &mut Vec<f64>, values: Vec<f64>) -> () {
    {
        let mut i = 0.0_f64;
        while (i < 20.0_f64) {
            {
                let __flight_index = (i) as usize;
                let __flight_value = values[i as usize].clone();
                if __flight_index == out.len() {
                    out.push(__flight_value);
                } else {
                    out[__flight_index] = __flight_value;
                }
            };
            {
                i += 1.0;
                i
            };
        }
    }
}
