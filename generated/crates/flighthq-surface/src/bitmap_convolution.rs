// @generated from upstream/packages/bitmap/src/bitmapConvolution.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::{BitmapConvolutionOptions, BitmapRegion};

// Source: upstream/packages/bitmap/src/bitmapConvolution.ts:17 (sha256:09d9cdc744a58451068c3449c621e0a503c7b4932e7cba2cda3c7ff1de0ab160)
pub fn convolve_bitmap(
    out: &mut Vec<u8>,
    source: &BitmapRegion,
    options: &BitmapConvolutionOptions,
) -> () {
    let matrix_x = options.matrix_x;
    let matrix_y = options.matrix_y;
    if (matrix_x <= 0.0_f64) || (matrix_y <= 0.0_f64) {
        panic!(
            "{}",
            "Convolution filter matrix dimensions must be positive"
        );
    }
    if ((options.matrix.len() as f64) < (matrix_x * matrix_y)) {
        panic!(
            "{}",
            "Convolution filter matrix does not match its dimensions"
        );
    }
    let raw_divisor = (options.divisor).unwrap_or(get_convolution_divisor(
        &options.matrix,
        (matrix_x * matrix_y),
    ));
    let divisor = if (raw_divisor == 0.0_f64) {
        1.0_f64
    } else {
        raw_divisor
    };
    let bias = (options.bias).unwrap_or(0.0_f64);
    let edge = ((options.edge).clone()).unwrap_or("clamp".to_owned());
    let preserve_alpha = (options.preserve_alpha).unwrap_or(true);
    let offset_x = (matrix_x / 2.0_f64).floor();
    let offset_y = (matrix_y / 2.0_f64).floor();
    let bitmap_width = source.bitmap.width;
    let bitmap_height = source.bitmap.height;
    {
        let mut py = 0.0_f64;
        while (py < source.height) {
            {
                let mut px = 0.0_f64;
                while (px < source.width) {
                    let mut r = 0.0_f64;
                    let mut g = 0.0_f64;
                    let mut b = 0.0_f64;
                    let mut a = 0.0_f64;
                    {
                        let mut ky = 0.0_f64;
                        while (ky < matrix_y) {
                            let raw_sample_y = (((source.y + py) + ky) - offset_y);
                            let weight_row_start = (ky * matrix_x);
                            {
                                let mut kx = 0.0_f64;
                                while (kx < matrix_x) {
                                    let raw_sample_x = (((source.x + px) + kx) - offset_x);
                                    let weight =
                                        options.matrix[(weight_row_start + kx) as usize].clone();
                                    let mut sample_x: f64;
                                    let mut sample_y: f64;
                                    if (((raw_sample_y < 0.0_f64)
                                        || (raw_sample_y >= bitmap_height))
                                        || (raw_sample_x < 0.0_f64))
                                        || (raw_sample_x >= bitmap_width)
                                    {
                                        if (edge == "transparent") {
                                            {
                                                kx += 1.0;
                                                kx
                                            };
                                            continue;
                                        } else {
                                            if (edge == "wrap") {
                                                sample_x = (((raw_sample_x % bitmap_width)
                                                    + bitmap_width)
                                                    % bitmap_width);
                                                sample_y = (((raw_sample_y % bitmap_height)
                                                    + bitmap_height)
                                                    % bitmap_height);
                                            } else {
                                                if (edge == "mirror") {
                                                    sample_x = resolve_convolution_mirror(
                                                        raw_sample_x,
                                                        bitmap_width,
                                                    );
                                                    sample_y = resolve_convolution_mirror(
                                                        raw_sample_y,
                                                        bitmap_height,
                                                    );
                                                } else {
                                                    sample_x = if (raw_sample_x < 0.0_f64) {
                                                        0.0_f64
                                                    } else {
                                                        if (raw_sample_x >= bitmap_width) {
                                                            (bitmap_width - 1.0_f64)
                                                        } else {
                                                            raw_sample_x
                                                        }
                                                    };
                                                    sample_y = if (raw_sample_y < 0.0_f64) {
                                                        0.0_f64
                                                    } else {
                                                        if (raw_sample_y >= bitmap_height) {
                                                            (bitmap_height - 1.0_f64)
                                                        } else {
                                                            raw_sample_y
                                                        }
                                                    };
                                                }
                                            }
                                        }
                                    } else {
                                        sample_x = raw_sample_x;
                                        sample_y = raw_sample_y;
                                    }
                                    let i = (((sample_y * bitmap_width) + sample_x) * 4.0_f64);
                                    r += ((source.bitmap.data[i as usize] as f64) * weight);
                                    g += ((source.bitmap.data[(i + 1.0_f64) as usize] as f64)
                                        * weight);
                                    b += ((source.bitmap.data[(i + 2.0_f64) as usize] as f64)
                                        * weight);
                                    a += ((source.bitmap.data[(i + 3.0_f64) as usize] as f64)
                                        * weight);
                                    {
                                        kx += 1.0;
                                        kx
                                    };
                                }
                            }
                            {
                                ky += 1.0;
                                ky
                            };
                        }
                    }
                    let di = (((py * source.width) + px) * 4.0_f64);
                    out[di as usize] = (clamp_byte(((r / divisor) + bias))) as u8;
                    out[(di + 1.0_f64) as usize] = (clamp_byte(((g / divisor) + bias))) as u8;
                    out[(di + 2.0_f64) as usize] = (clamp_byte(((b / divisor) + bias))) as u8;
                    if preserve_alpha {
                        let cy = (0.0_f64).max((bitmap_height - 1.0_f64).min((source.y + py)));
                        let cx = (0.0_f64).max((bitmap_width - 1.0_f64).min((source.x + px)));
                        out[(di + 3.0_f64) as usize] = (source.bitmap.data
                            [((((cy * bitmap_width) + cx) * 4.0_f64) + 3.0_f64) as usize]
                            as f64) as u8;
                    } else {
                        out[(di + 3.0_f64) as usize] = (clamp_byte(((a / divisor) + bias))) as u8;
                    }
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

// Source: upstream/packages/bitmap/src/bitmapConvolution.ts:92 (sha256:c06b5172b1cceb605b49e48481c95614cef14ecc1c40b33115d5ac45594da56a)
fn clamp_byte(value: f64) -> f64 {
    return (0.0_f64).max((255.0_f64).min((value).round()));
}

// Source: upstream/packages/bitmap/src/bitmapConvolution.ts:96 (sha256:5145087e39afcc5b9d5160ec07ef68d0324c0580fbe2b6ff2e1fc068bb760b5c)
fn get_convolution_divisor(matrix: &Vec<f64>, length: f64) -> f64 {
    let mut sum = 0.0_f64;
    {
        let mut i = 0.0_f64;
        while (i < length) {
            sum += matrix[i as usize].clone();
            {
                i += 1.0;
                i
            };
        }
    }
    return if (sum == 0.0_f64) { 1.0_f64 } else { sum };
}

// Source: upstream/packages/bitmap/src/bitmapConvolution.ts:104 (sha256:eef5932d4599caea4bcb79904e41f31e01bdbb55e210c85844231f29ac2ad62a)
fn resolve_convolution_mirror(v: f64, size: f64) -> f64 {
    let period = (2.0_f64 * size);
    let wrapped = (((v % period) + period) % period);
    return if (wrapped < size) {
        wrapped
    } else {
        ((period - 1.0_f64) - wrapped)
    };
}
