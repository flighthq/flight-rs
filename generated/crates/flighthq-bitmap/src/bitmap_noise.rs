// @generated from upstream/packages/bitmap/src/bitmapNoise.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::invalidate_bitmap;
use flighthq_types::BitmapRegion;

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

// Source: upstream/packages/bitmap/src/bitmapNoise.ts:14 (sha256:c83a554dcf7dd5908d01723200a82f7d3b1992fc201d023c780e93988f20817f)
pub fn fill_bitmap_noise(
    dest: &mut BitmapRegion,
    seed: f64,
    low: Option<f64>,
    high: Option<f64>,
    gray_scale: Option<bool>,
) -> () {
    let low = low.unwrap_or(0.0_f64);
    let high = high.unwrap_or(255.0_f64);
    let gray_scale = gray_scale.unwrap_or(false);
    let mut state =
        if ((__flight_js_to_u32((__flight_js_to_i32(seed) | __flight_js_to_i32(0.0_f64)) as f64)
            >> (__flight_js_to_u32(0.0_f64) & 31)) as f64)
            != 0.0_f64
        {
            (__flight_js_to_u32((__flight_js_to_i32(seed) | __flight_js_to_i32(0.0_f64)) as f64)
                >> (__flight_js_to_u32(0.0_f64) & 31)) as f64
        } else {
            1.0_f64
        };
    let lo = (0.0_f64).max((255.0_f64).min(low));
    let span = ((0.0_f64).max((255.0_f64).min(high)) - lo);
    let bitmap_width = dest.bitmap.width;
    {
        let mut py = 0.0_f64;
        while (py < dest.height) {
            let y = (dest.y + py);
            {
                let mut px = 0.0_f64;
                while (px < dest.width) {
                    state = next_random_state(state);
                    let r = (lo + ((state / 4294967296.0_f64) * span));
                    let mut g = r;
                    let mut b = r;
                    if (!gray_scale) {
                        state = next_random_state(state);
                        g = (lo + ((state / 4294967296.0_f64) * span));
                        state = next_random_state(state);
                        b = (lo + ((state / 4294967296.0_f64) * span));
                    }
                    let x = (dest.x + px);
                    if (((y < 0.0_f64) || (y >= dest.bitmap.height)) || (x < 0.0_f64))
                        || (x >= bitmap_width)
                    {
                        {
                            px += 1.0;
                            px
                        };
                        continue;
                    }
                    let i = (((y * bitmap_width) + x) * 4.0_f64);
                    dest.bitmap.data[i as usize] = ((r).round()) as u8;
                    dest.bitmap.data[(i + 1.0_f64) as usize] = ((g).round()) as u8;
                    dest.bitmap.data[(i + 2.0_f64) as usize] = ((b).round()) as u8;
                    dest.bitmap.data[(i + 3.0_f64) as usize] = (255.0_f64) as u8;
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
    invalidate_bitmap(&mut dest.bitmap);
}

// Source: upstream/packages/bitmap/src/bitmapNoise.ts:74 (sha256:1b5687d4dac3cfaeb7a528dd64071b113db7e172818c0830a4e56a3d4d392c2b)
pub fn fill_bitmap_perlin_noise(
    dest: &mut BitmapRegion,
    base_x: f64,
    base_y: f64,
    octaves: f64,
    seed: f64,
    gray_scale: Option<bool>,
    stitch: Option<bool>,
    channel_options: Option<f64>,
) -> () {
    let gray_scale = gray_scale.unwrap_or(false);
    let stitch = stitch.unwrap_or(false);
    let channel_options = channel_options.unwrap_or(7.0_f64);
    let fx0 = if (base_x > 0.0_f64) {
        (1.0_f64 / base_x)
    } else {
        0.0_f64
    };
    let fy0 = if (base_y > 0.0_f64) {
        (1.0_f64 / base_y)
    } else {
        0.0_f64
    };
    let passes = (1.0_f64).max((octaves).round());
    let bitmap_width = dest.bitmap.width;
    let w = dest.width;
    let h = dest.height;
    {
        let mut py = 0.0_f64;
        while (py < dest.height) {
            let y = (dest.y + py);
            if (y < 0.0_f64) || (y >= dest.bitmap.height) {
                {
                    py += 1.0;
                    py
                };
                continue;
            }
            {
                let mut px = 0.0_f64;
                while (px < dest.width) {
                    let x = (dest.x + px);
                    if (x < 0.0_f64) || (x >= bitmap_width) {
                        {
                            px += 1.0;
                            px
                        };
                        continue;
                    }
                    let di = (((y * bitmap_width) + x) * 4.0_f64);
                    let nx = if stitch {
                        stitched_coord((px * fx0), (w * fx0))
                    } else {
                        (px * fx0)
                    };
                    let ny = if stitch {
                        stitched_coord((py * fy0), (h * fy0))
                    } else {
                        (py * fy0)
                    };
                    if gray_scale {
                        let value = fractal_value_noise(
                            nx,
                            ny,
                            passes,
                            (__flight_js_to_i32(seed) | __flight_js_to_i32(0.0_f64)) as f64,
                        );
                        let byte = (value * 255.0_f64).round();
                        if ((__flight_js_to_i32(channel_options)
                            & __flight_js_to_i32(BITMAP_NOISE_CHANNEL_R))
                            as f64)
                            != 0.0_f64
                        {
                            dest.bitmap.data[di as usize] = (byte) as u8;
                        }
                        if ((__flight_js_to_i32(channel_options)
                            & __flight_js_to_i32(BITMAP_NOISE_CHANNEL_G))
                            as f64)
                            != 0.0_f64
                        {
                            dest.bitmap.data[(di + 1.0_f64) as usize] = (byte) as u8;
                        }
                        if ((__flight_js_to_i32(channel_options)
                            & __flight_js_to_i32(BITMAP_NOISE_CHANNEL_B))
                            as f64)
                            != 0.0_f64
                        {
                            dest.bitmap.data[(di + 2.0_f64) as usize] = (byte) as u8;
                        }
                    } else {
                        if ((__flight_js_to_i32(channel_options)
                            & __flight_js_to_i32(BITMAP_NOISE_CHANNEL_R))
                            as f64)
                            != 0.0_f64
                        {
                            dest.bitmap.data[di as usize] = ((fractal_value_noise(
                                nx,
                                ny,
                                passes,
                                (__flight_js_to_i32(seed) | __flight_js_to_i32(0.0_f64)) as f64,
                            ) * 255.0_f64)
                                .round())
                                as u8;
                        }
                        if ((__flight_js_to_i32(channel_options)
                            & __flight_js_to_i32(BITMAP_NOISE_CHANNEL_G))
                            as f64)
                            != 0.0_f64
                        {
                            dest.bitmap.data[(di + 1.0_f64) as usize] = ((fractal_value_noise(
                                nx,
                                ny,
                                passes,
                                ((__flight_js_to_i32(seed) | __flight_js_to_i32(0.0_f64)) as f64
                                    + 2654435761.0_f64),
                            ) * 255.0_f64)
                                .round())
                                as u8;
                        }
                        if ((__flight_js_to_i32(channel_options)
                            & __flight_js_to_i32(BITMAP_NOISE_CHANNEL_B))
                            as f64)
                            != 0.0_f64
                        {
                            dest.bitmap.data[(di + 2.0_f64) as usize] = ((fractal_value_noise(
                                nx,
                                ny,
                                passes,
                                ((__flight_js_to_i32(seed) | __flight_js_to_i32(0.0_f64)) as f64
                                    + 2654435762.0_f64),
                            ) * 255.0_f64)
                                .round())
                                as u8;
                        }
                    }
                    if ((__flight_js_to_i32(channel_options)
                        & __flight_js_to_i32(BITMAP_NOISE_CHANNEL_A))
                        as f64)
                        != 0.0_f64
                    {
                        dest.bitmap.data[(di + 3.0_f64) as usize] = ((fractal_value_noise(
                            nx,
                            ny,
                            passes,
                            ((__flight_js_to_i32(seed) | __flight_js_to_i32(0.0_f64)) as f64
                                + 2654435763.0_f64),
                        ) * 255.0_f64)
                            .round())
                            as u8;
                    } else {
                        dest.bitmap.data[(di + 3.0_f64) as usize] = (255.0_f64) as u8;
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
    invalidate_bitmap(&mut dest.bitmap);
}

// Source: upstream/packages/bitmap/src/bitmapNoise.ts:138 (sha256:d0928320221dbb25695fe3fe792b0cfb602c840e228a03d8fb23e21746a6611b)
pub fn fill_bitmap_turbulence(
    dest: &mut BitmapRegion,
    base_x: f64,
    base_y: f64,
    octaves: f64,
    seed: f64,
    gray_scale: Option<bool>,
    stitch: Option<bool>,
    channel_options: Option<f64>,
) -> () {
    let gray_scale = gray_scale.unwrap_or(false);
    let stitch = stitch.unwrap_or(false);
    let channel_options = channel_options.unwrap_or(7.0_f64);
    let fx0 = if (base_x > 0.0_f64) {
        (1.0_f64 / base_x)
    } else {
        0.0_f64
    };
    let fy0 = if (base_y > 0.0_f64) {
        (1.0_f64 / base_y)
    } else {
        0.0_f64
    };
    let passes = (1.0_f64).max((octaves).round());
    let bitmap_width = dest.bitmap.width;
    let w = dest.width;
    let h = dest.height;
    {
        let mut py = 0.0_f64;
        while (py < dest.height) {
            let y = (dest.y + py);
            if (y < 0.0_f64) || (y >= dest.bitmap.height) {
                {
                    py += 1.0;
                    py
                };
                continue;
            }
            {
                let mut px = 0.0_f64;
                while (px < dest.width) {
                    let x = (dest.x + px);
                    if (x < 0.0_f64) || (x >= bitmap_width) {
                        {
                            px += 1.0;
                            px
                        };
                        continue;
                    }
                    let di = (((y * bitmap_width) + x) * 4.0_f64);
                    let nx = if stitch {
                        stitched_coord((px * fx0), (w * fx0))
                    } else {
                        (px * fx0)
                    };
                    let ny = if stitch {
                        stitched_coord((py * fy0), (h * fy0))
                    } else {
                        (py * fy0)
                    };
                    if gray_scale {
                        let value = turbulence_noise(
                            nx,
                            ny,
                            passes,
                            (__flight_js_to_i32(seed) | __flight_js_to_i32(0.0_f64)) as f64,
                        );
                        let byte = (value * 255.0_f64).round();
                        if ((__flight_js_to_i32(channel_options)
                            & __flight_js_to_i32(BITMAP_NOISE_CHANNEL_R))
                            as f64)
                            != 0.0_f64
                        {
                            dest.bitmap.data[di as usize] = (byte) as u8;
                        }
                        if ((__flight_js_to_i32(channel_options)
                            & __flight_js_to_i32(BITMAP_NOISE_CHANNEL_G))
                            as f64)
                            != 0.0_f64
                        {
                            dest.bitmap.data[(di + 1.0_f64) as usize] = (byte) as u8;
                        }
                        if ((__flight_js_to_i32(channel_options)
                            & __flight_js_to_i32(BITMAP_NOISE_CHANNEL_B))
                            as f64)
                            != 0.0_f64
                        {
                            dest.bitmap.data[(di + 2.0_f64) as usize] = (byte) as u8;
                        }
                    } else {
                        if ((__flight_js_to_i32(channel_options)
                            & __flight_js_to_i32(BITMAP_NOISE_CHANNEL_R))
                            as f64)
                            != 0.0_f64
                        {
                            dest.bitmap.data[di as usize] = ((turbulence_noise(
                                nx,
                                ny,
                                passes,
                                (__flight_js_to_i32(seed) | __flight_js_to_i32(0.0_f64)) as f64,
                            ) * 255.0_f64)
                                .round())
                                as u8;
                        }
                        if ((__flight_js_to_i32(channel_options)
                            & __flight_js_to_i32(BITMAP_NOISE_CHANNEL_G))
                            as f64)
                            != 0.0_f64
                        {
                            dest.bitmap.data[(di + 1.0_f64) as usize] = ((turbulence_noise(
                                nx,
                                ny,
                                passes,
                                ((__flight_js_to_i32(seed) | __flight_js_to_i32(0.0_f64)) as f64
                                    + 2654435761.0_f64),
                            ) * 255.0_f64)
                                .round())
                                as u8;
                        }
                        if ((__flight_js_to_i32(channel_options)
                            & __flight_js_to_i32(BITMAP_NOISE_CHANNEL_B))
                            as f64)
                            != 0.0_f64
                        {
                            dest.bitmap.data[(di + 2.0_f64) as usize] = ((turbulence_noise(
                                nx,
                                ny,
                                passes,
                                ((__flight_js_to_i32(seed) | __flight_js_to_i32(0.0_f64)) as f64
                                    + 2654435762.0_f64),
                            ) * 255.0_f64)
                                .round())
                                as u8;
                        }
                    }
                    if ((__flight_js_to_i32(channel_options)
                        & __flight_js_to_i32(BITMAP_NOISE_CHANNEL_A))
                        as f64)
                        != 0.0_f64
                    {
                        dest.bitmap.data[(di + 3.0_f64) as usize] = ((turbulence_noise(
                            nx,
                            ny,
                            passes,
                            ((__flight_js_to_i32(seed) | __flight_js_to_i32(0.0_f64)) as f64
                                + 2654435763.0_f64),
                        ) * 255.0_f64)
                            .round())
                            as u8;
                    } else {
                        dest.bitmap.data[(di + 3.0_f64) as usize] = (255.0_f64) as u8;
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
    invalidate_bitmap(&mut dest.bitmap);
}

// Source: upstream/packages/bitmap/src/bitmapNoise.ts:194 (sha256:acd1108e901f39582b5ef02961873aa4c558060898402ed181aa23fb30c25ee9)
pub const BITMAP_NOISE_CHANNEL_A: f64 = 8.0_f64;

// Source: upstream/packages/bitmap/src/bitmapNoise.ts:195 (sha256:aa3261d010508451059dd6219e2745759834bc16a91e1977ee3434ad7e0252bb)
pub const BITMAP_NOISE_CHANNEL_B: f64 = 4.0_f64;

// Source: upstream/packages/bitmap/src/bitmapNoise.ts:196 (sha256:68f3f6f23c3a5443020831203203a8708630da7d97f5bbb1730b993b71b73eea)
pub const BITMAP_NOISE_CHANNEL_G: f64 = 2.0_f64;

// Source: upstream/packages/bitmap/src/bitmapNoise.ts:197 (sha256:1ce736ff24d27a4da36e2be789d666f3e868d488cb781ccc396fee9caf2f6a03)
pub const BITMAP_NOISE_CHANNEL_R: f64 = 1.0_f64;

// Source: upstream/packages/bitmap/src/bitmapNoise.ts:201 (sha256:fadd978e095745c51e88e69ab8d8c48f7168b7e0059b349a89ca6dcb8b68581a)
fn fractal_value_noise(x: f64, y: f64, octaves: f64, seed: f64) -> f64 {
    let mut sum = 0.0_f64;
    let mut amplitude = 1.0_f64;
    let mut amplitude_sum = 0.0_f64;
    let mut frequency = 1.0_f64;
    {
        let mut o = 0.0_f64;
        while (o < octaves) {
            sum += (value_noise(
                (x * frequency),
                (y * frequency),
                (seed + (o * 2246822507.0_f64)),
            ) * amplitude);
            amplitude_sum += amplitude;
            amplitude *= 0.5_f64;
            frequency *= 2.0_f64;
            {
                o += 1.0;
                o
            };
        }
    }
    return if (amplitude_sum > 0.0_f64) {
        (sum / amplitude_sum)
    } else {
        0.0_f64
    };
}

// Source: upstream/packages/bitmap/src/bitmapNoise.ts:216 (sha256:51819a6b76e2a15769b17bfe04a84bffbd3604a53eee7fe6f5734675db87d130)
fn hash_lattice(ix: f64, iy: f64, seed: f64) -> f64 {
    let mut h = (__flight_js_to_i32(
        ((__flight_js_to_i32(ix).wrapping_mul(__flight_js_to_i32(374761393.0_f64)) as f64
            + __flight_js_to_i32(iy).wrapping_mul(__flight_js_to_i32(668265263.0_f64)) as f64)
            + __flight_js_to_i32(seed).wrapping_mul(__flight_js_to_i32(2654435761.0_f64)) as f64),
    ) | __flight_js_to_i32(0.0_f64)) as f64;
    h = __flight_js_to_i32(
        (__flight_js_to_i32(h)
            ^ __flight_js_to_i32(
                (__flight_js_to_u32(h) >> (__flight_js_to_u32(13.0_f64) & 31)) as f64,
            )) as f64,
    )
    .wrapping_mul(__flight_js_to_i32(1274126177.0_f64)) as f64;
    return ((__flight_js_to_u32(
        (__flight_js_to_i32(h)
            ^ __flight_js_to_i32(
                (__flight_js_to_u32(h) >> (__flight_js_to_u32(16.0_f64) & 31)) as f64,
            )) as f64,
    ) >> (__flight_js_to_u32(0.0_f64) & 31)) as f64
        / 4294967296.0_f64);
}

// Source: upstream/packages/bitmap/src/bitmapNoise.ts:223 (sha256:3543676fb6d37ecefc880aa3529ef4db73c3e86bc9d7c881849b552cb010c70f)
fn next_random_state(state: f64) -> f64 {
    let mut t =
        (__flight_js_to_i32((state + 1831565813.0_f64)) | __flight_js_to_i32(0.0_f64)) as f64;
    t = __flight_js_to_i32(
        (__flight_js_to_i32(t)
            ^ __flight_js_to_i32(
                (__flight_js_to_u32(t) >> (__flight_js_to_u32(15.0_f64) & 31)) as f64,
            )) as f64,
    )
    .wrapping_mul(__flight_js_to_i32(
        (__flight_js_to_i32(t) | __flight_js_to_i32(1.0_f64)) as f64,
    )) as f64;
    t = (__flight_js_to_i32(t)
        ^ __flight_js_to_i32(
            (t + __flight_js_to_i32(
                (__flight_js_to_i32(t)
                    ^ __flight_js_to_i32(
                        (__flight_js_to_u32(t) >> (__flight_js_to_u32(7.0_f64) & 31)) as f64,
                    )) as f64,
            )
            .wrapping_mul(__flight_js_to_i32(
                (__flight_js_to_i32(t) | __flight_js_to_i32(61.0_f64)) as f64,
            )) as f64),
        )) as f64;
    return if ((__flight_js_to_u32(
        (__flight_js_to_i32(t)
            ^ __flight_js_to_i32(
                (__flight_js_to_u32(t) >> (__flight_js_to_u32(14.0_f64) & 31)) as f64,
            )) as f64,
    ) >> (__flight_js_to_u32(0.0_f64) & 31)) as f64)
        != 0.0_f64
    {
        (__flight_js_to_u32(
            (__flight_js_to_i32(t)
                ^ __flight_js_to_i32(
                    (__flight_js_to_u32(t) >> (__flight_js_to_u32(14.0_f64) & 31)) as f64,
                )) as f64,
        ) >> (__flight_js_to_u32(0.0_f64) & 31)) as f64
    } else {
        1.0_f64
    };
}

// Source: upstream/packages/bitmap/src/bitmapNoise.ts:231 (sha256:c6635f75c7724dfe4af6179e02cb7ef28777eeee92225c81e6f1a233b3982fe2)
fn smooth_step(t: f64) -> f64 {
    return ((t * t) * (3.0_f64 - (2.0_f64 * t)));
}

// Source: upstream/packages/bitmap/src/bitmapNoise.ts:236 (sha256:54db4c8cd629eb735a401cbc347ba396b68ffaaaaf2a1de024b29b0b65972ea4)
fn stitched_coord(t: f64, period: f64) -> f64 {
    if (period <= 0.0_f64) {
        return t;
    }
    return (((t % period) + period) % period);
}

// Source: upstream/packages/bitmap/src/bitmapNoise.ts:243 (sha256:7d34e667c6717b60ee2f811f9e93fecd3563c2c6b263ce94d4d6c5b523cfc6cc)
fn turbulence_noise(x: f64, y: f64, octaves: f64, seed: f64) -> f64 {
    let mut sum = 0.0_f64;
    let mut amplitude = 1.0_f64;
    let mut amplitude_sum = 0.0_f64;
    let mut frequency = 1.0_f64;
    {
        let mut o = 0.0_f64;
        while (o < octaves) {
            sum += (((value_noise(
                (x * frequency),
                (y * frequency),
                (seed + (o * 2246822507.0_f64)),
            ) * 2.0_f64)
                - 1.0_f64)
                .abs()
                * amplitude);
            amplitude_sum += amplitude;
            amplitude *= 0.5_f64;
            frequency *= 2.0_f64;
            {
                o += 1.0;
                o
            };
        }
    }
    return if (amplitude_sum > 0.0_f64) {
        (sum / amplitude_sum)
    } else {
        0.0_f64
    };
}

// Source: upstream/packages/bitmap/src/bitmapNoise.ts:258 (sha256:6083df565a235ee60d40b5d7e5c6771a44af5a1392eb652ee870a7526f125051)
fn value_noise(x: f64, y: f64, seed: f64) -> f64 {
    let ix = (x).floor();
    let iy = (y).floor();
    let fx = smooth_step((x - ix));
    let fy = smooth_step((y - iy));
    let v00 = hash_lattice(ix, iy, seed);
    let v10 = hash_lattice((ix + 1.0_f64), iy, seed);
    let v01 = hash_lattice(ix, (iy + 1.0_f64), seed);
    let v11 = hash_lattice((ix + 1.0_f64), (iy + 1.0_f64), seed);
    let top = (v00 + ((v10 - v00) * fx));
    let bottom = (v01 + ((v11 - v01) * fx));
    return (top + ((bottom - top) * fy));
}
