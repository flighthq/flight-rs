// @generated from upstream/packages/surface/src/surfaceNoise.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::SurfaceRegion;

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

// Source: upstream/packages/surface/src/surfaceNoise.ts:13 (sha256:0d6d2dcb872f143f40a8cb5d0d1e68bd354ac5153115d28e8021c08cfcf9137e)
pub fn fill_surface_noise(
    dest: &mut SurfaceRegion,
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
    let surface_width = dest.surface.width;
    {
        let mut py = 0.0_f64;
        while (py < dest.height) {
            let y = (dest.y + py);
            {
                let mut px = 0.0_f64;
                while (px < dest.width) {
                    state = next_random_state(state);
                    let mut r = (lo + ((state / 4294967296.0_f64) * span));
                    let mut g = r;
                    let mut b = r;
                    if (!gray_scale) {
                        state = next_random_state(state);
                        g = (lo + ((state / 4294967296.0_f64) * span));
                        state = next_random_state(state);
                        b = (lo + ((state / 4294967296.0_f64) * span));
                    }
                    let x = (dest.x + px);
                    if ((((y < 0.0_f64) || (y >= dest.surface.height)) || (x < 0.0_f64))
                        || (x >= surface_width))
                    {
                        {
                            px += 1.0;
                            px
                        };
                        continue;
                    }
                    let i = (((y * surface_width) + x) * 4.0_f64);
                    dest.surface.data[i as usize] = ((r).round()) as u8;
                    dest.surface.data[(i + 1.0_f64) as usize] = ((g).round()) as u8;
                    dest.surface.data[(i + 2.0_f64) as usize] = ((b).round()) as u8;
                    dest.surface.data[(i + 3.0_f64) as usize] = (255.0_f64) as u8;
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
    {
        dest.surface.version = (__flight_js_to_u32((dest.surface.version + 1.0_f64))
            >> (__flight_js_to_u32(0.0_f64) & 31)) as f64;
    };
}

// Source: upstream/packages/surface/src/surfaceNoise.ts:73 (sha256:7e06ca0a0528ba82803404e49d7aaee54a6efa6bc70b05da425424da672b6cdb)
pub fn fill_surface_perlin_noise(
    dest: &mut SurfaceRegion,
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
    let surface_width = dest.surface.width;
    let w = dest.width;
    let h = dest.height;
    {
        let mut py = 0.0_f64;
        while (py < dest.height) {
            let y = (dest.y + py);
            if ((y < 0.0_f64) || (y >= dest.surface.height)) {
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
                    if ((x < 0.0_f64) || (x >= surface_width)) {
                        {
                            px += 1.0;
                            px
                        };
                        continue;
                    }
                    let di = (((y * surface_width) + x) * 4.0_f64);
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
                            & __flight_js_to_i32(SURFACE_NOISE_CHANNEL_R))
                            as f64)
                            != 0.0_f64
                        {
                            dest.surface.data[di as usize] = (byte) as u8;
                        }
                        if ((__flight_js_to_i32(channel_options)
                            & __flight_js_to_i32(SURFACE_NOISE_CHANNEL_G))
                            as f64)
                            != 0.0_f64
                        {
                            dest.surface.data[(di + 1.0_f64) as usize] = (byte) as u8;
                        }
                        if ((__flight_js_to_i32(channel_options)
                            & __flight_js_to_i32(SURFACE_NOISE_CHANNEL_B))
                            as f64)
                            != 0.0_f64
                        {
                            dest.surface.data[(di + 2.0_f64) as usize] = (byte) as u8;
                        }
                    } else {
                        if ((__flight_js_to_i32(channel_options)
                            & __flight_js_to_i32(SURFACE_NOISE_CHANNEL_R))
                            as f64)
                            != 0.0_f64
                        {
                            dest.surface.data[di as usize] = ((fractal_value_noise(
                                nx,
                                ny,
                                passes,
                                (__flight_js_to_i32(seed) | __flight_js_to_i32(0.0_f64)) as f64,
                            ) * 255.0_f64)
                                .round())
                                as u8;
                        }
                        if ((__flight_js_to_i32(channel_options)
                            & __flight_js_to_i32(SURFACE_NOISE_CHANNEL_G))
                            as f64)
                            != 0.0_f64
                        {
                            dest.surface.data[(di + 1.0_f64) as usize] = ((fractal_value_noise(
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
                            & __flight_js_to_i32(SURFACE_NOISE_CHANNEL_B))
                            as f64)
                            != 0.0_f64
                        {
                            dest.surface.data[(di + 2.0_f64) as usize] = ((fractal_value_noise(
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
                        & __flight_js_to_i32(SURFACE_NOISE_CHANNEL_A))
                        as f64)
                        != 0.0_f64
                    {
                        dest.surface.data[(di + 3.0_f64) as usize] = ((fractal_value_noise(
                            nx,
                            ny,
                            passes,
                            ((__flight_js_to_i32(seed) | __flight_js_to_i32(0.0_f64)) as f64
                                + 2654435763.0_f64),
                        ) * 255.0_f64)
                            .round())
                            as u8;
                    } else {
                        dest.surface.data[(di + 3.0_f64) as usize] = (255.0_f64) as u8;
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
    {
        dest.surface.version = (__flight_js_to_u32((dest.surface.version + 1.0_f64))
            >> (__flight_js_to_u32(0.0_f64) & 31)) as f64;
    };
}

// Source: upstream/packages/surface/src/surfaceNoise.ts:137 (sha256:ddf0c6aa5d7709cd14df990146884683a20755ea1c57d5580f452a484620b1f9)
pub fn fill_surface_turbulence(
    dest: &mut SurfaceRegion,
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
    let surface_width = dest.surface.width;
    let w = dest.width;
    let h = dest.height;
    {
        let mut py = 0.0_f64;
        while (py < dest.height) {
            let y = (dest.y + py);
            if ((y < 0.0_f64) || (y >= dest.surface.height)) {
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
                    if ((x < 0.0_f64) || (x >= surface_width)) {
                        {
                            px += 1.0;
                            px
                        };
                        continue;
                    }
                    let di = (((y * surface_width) + x) * 4.0_f64);
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
                            & __flight_js_to_i32(SURFACE_NOISE_CHANNEL_R))
                            as f64)
                            != 0.0_f64
                        {
                            dest.surface.data[di as usize] = (byte) as u8;
                        }
                        if ((__flight_js_to_i32(channel_options)
                            & __flight_js_to_i32(SURFACE_NOISE_CHANNEL_G))
                            as f64)
                            != 0.0_f64
                        {
                            dest.surface.data[(di + 1.0_f64) as usize] = (byte) as u8;
                        }
                        if ((__flight_js_to_i32(channel_options)
                            & __flight_js_to_i32(SURFACE_NOISE_CHANNEL_B))
                            as f64)
                            != 0.0_f64
                        {
                            dest.surface.data[(di + 2.0_f64) as usize] = (byte) as u8;
                        }
                    } else {
                        if ((__flight_js_to_i32(channel_options)
                            & __flight_js_to_i32(SURFACE_NOISE_CHANNEL_R))
                            as f64)
                            != 0.0_f64
                        {
                            dest.surface.data[di as usize] = ((turbulence_noise(
                                nx,
                                ny,
                                passes,
                                (__flight_js_to_i32(seed) | __flight_js_to_i32(0.0_f64)) as f64,
                            ) * 255.0_f64)
                                .round())
                                as u8;
                        }
                        if ((__flight_js_to_i32(channel_options)
                            & __flight_js_to_i32(SURFACE_NOISE_CHANNEL_G))
                            as f64)
                            != 0.0_f64
                        {
                            dest.surface.data[(di + 1.0_f64) as usize] = ((turbulence_noise(
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
                            & __flight_js_to_i32(SURFACE_NOISE_CHANNEL_B))
                            as f64)
                            != 0.0_f64
                        {
                            dest.surface.data[(di + 2.0_f64) as usize] = ((turbulence_noise(
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
                        & __flight_js_to_i32(SURFACE_NOISE_CHANNEL_A))
                        as f64)
                        != 0.0_f64
                    {
                        dest.surface.data[(di + 3.0_f64) as usize] = ((turbulence_noise(
                            nx,
                            ny,
                            passes,
                            ((__flight_js_to_i32(seed) | __flight_js_to_i32(0.0_f64)) as f64
                                + 2654435763.0_f64),
                        ) * 255.0_f64)
                            .round())
                            as u8;
                    } else {
                        dest.surface.data[(di + 3.0_f64) as usize] = (255.0_f64) as u8;
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
    {
        dest.surface.version = (__flight_js_to_u32((dest.surface.version + 1.0_f64))
            >> (__flight_js_to_u32(0.0_f64) & 31)) as f64;
    };
}

// Source: upstream/packages/surface/src/surfaceNoise.ts:193 (sha256:6434ec960d00e9606717024712dd3c8853d4b497ec5bb6e01a8970f2ccb65e93)
pub const SURFACE_NOISE_CHANNEL_A: f64 = 8.0_f64;

// Source: upstream/packages/surface/src/surfaceNoise.ts:194 (sha256:c83a43ff0a7338ef9d1f7b5097af848e5cd7e7e9ccdb076015c036b98165daff)
pub const SURFACE_NOISE_CHANNEL_B: f64 = 4.0_f64;

// Source: upstream/packages/surface/src/surfaceNoise.ts:195 (sha256:bfe6ad7d3dd4962042f5b3b6304fb74583e2b8ebedd6bfdc8b40d4fc1fded054)
pub const SURFACE_NOISE_CHANNEL_G: f64 = 2.0_f64;

// Source: upstream/packages/surface/src/surfaceNoise.ts:196 (sha256:2acb69a81ee94a2345bca32887df690d58dd44e5cb8a18812e62e7aef4f0e2cb)
pub const SURFACE_NOISE_CHANNEL_R: f64 = 1.0_f64;

// Source: upstream/packages/surface/src/surfaceNoise.ts:200 (sha256:fadd978e095745c51e88e69ab8d8c48f7168b7e0059b349a89ca6dcb8b68581a)
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

// Source: upstream/packages/surface/src/surfaceNoise.ts:215 (sha256:51819a6b76e2a15769b17bfe04a84bffbd3604a53eee7fe6f5734675db87d130)
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

// Source: upstream/packages/surface/src/surfaceNoise.ts:222 (sha256:3543676fb6d37ecefc880aa3529ef4db73c3e86bc9d7c881849b552cb010c70f)
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

// Source: upstream/packages/surface/src/surfaceNoise.ts:230 (sha256:c6635f75c7724dfe4af6179e02cb7ef28777eeee92225c81e6f1a233b3982fe2)
fn smooth_step(t: f64) -> f64 {
    return ((t * t) * (3.0_f64 - (2.0_f64 * t)));
}

// Source: upstream/packages/surface/src/surfaceNoise.ts:235 (sha256:54db4c8cd629eb735a401cbc347ba396b68ffaaaaf2a1de024b29b0b65972ea4)
fn stitched_coord(t: f64, period: f64) -> f64 {
    if (period <= 0.0_f64) {
        return t;
    }
    return (((t % period) + period) % period);
}

// Source: upstream/packages/surface/src/surfaceNoise.ts:242 (sha256:7d34e667c6717b60ee2f811f9e93fecd3563c2c6b263ce94d4d6c5b523cfc6cc)
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

// Source: upstream/packages/surface/src/surfaceNoise.ts:257 (sha256:6083df565a235ee60d40b5d7e5c6771a44af5a1392eb652ee870a7526f125051)
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
