// @generated from upstream/packages/adjustments/src/colorMatrixMath.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

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

// Source: upstream/packages/adjustments/src/colorMatrixMath.ts:20 (sha256:ba3b8edd31726e45d54f425825ee34acb048ab60f3f8c8c4aa312325a2d04df4)
pub const COLOR_MATRIX_LENGTH: f64 = 20.0_f64;

// Source: upstream/packages/adjustments/src/colorMatrixMath.ts:27 (sha256:18af6c4d4b98f1173d98bf7e73723b368e2a04de4556c7795b4845c74d823646)
pub fn apply_color_matrix_to_color(matrix: &Vec<f64>, packed_rgba: f64) -> f64 {
    let r = (__flight_js_to_i32(
        (__flight_js_to_u32(packed_rgba) >> (__flight_js_to_u32(24.0_f64) & 31)) as f64,
    ) & __flight_js_to_i32(255.0_f64)) as f64;
    let g = (__flight_js_to_i32(
        (__flight_js_to_u32(packed_rgba) >> (__flight_js_to_u32(16.0_f64) & 31)) as f64,
    ) & __flight_js_to_i32(255.0_f64)) as f64;
    let b = (__flight_js_to_i32(
        (__flight_js_to_u32(packed_rgba) >> (__flight_js_to_u32(8.0_f64) & 31)) as f64,
    ) & __flight_js_to_i32(255.0_f64)) as f64;
    let a = (__flight_js_to_i32(packed_rgba) & __flight_js_to_i32(255.0_f64)) as f64;
    let r_out = clamp_byte(
        (((((matrix[0.0_f64 as usize].clone() * r) + (matrix[1.0_f64 as usize].clone() * g))
            + (matrix[2.0_f64 as usize].clone() * b))
            + (matrix[3.0_f64 as usize].clone() * a))
            + matrix[4.0_f64 as usize].clone()),
    );
    let g_out = clamp_byte(
        (((((matrix[5.0_f64 as usize].clone() * r) + (matrix[6.0_f64 as usize].clone() * g))
            + (matrix[7.0_f64 as usize].clone() * b))
            + (matrix[8.0_f64 as usize].clone() * a))
            + matrix[9.0_f64 as usize].clone()),
    );
    let b_out = clamp_byte(
        (((((matrix[10.0_f64 as usize].clone() * r) + (matrix[11.0_f64 as usize].clone() * g))
            + (matrix[12.0_f64 as usize].clone() * b))
            + (matrix[13.0_f64 as usize].clone() * a))
            + matrix[14.0_f64 as usize].clone()),
    );
    let a_out = clamp_byte(
        (((((matrix[15.0_f64 as usize].clone() * r) + (matrix[16.0_f64 as usize].clone() * g))
            + (matrix[17.0_f64 as usize].clone() * b))
            + (matrix[18.0_f64 as usize].clone() * a))
            + matrix[19.0_f64 as usize].clone()),
    );
    return (__flight_js_to_u32(
        (__flight_js_to_i32(
            (__flight_js_to_i32(
                (__flight_js_to_i32(
                    __flight_js_to_i32(r_out).wrapping_shl((__flight_js_to_u32(24.0_f64) & 31))
                        as f64,
                ) | __flight_js_to_i32(
                    __flight_js_to_i32(g_out).wrapping_shl((__flight_js_to_u32(16.0_f64) & 31))
                        as f64,
                )) as f64,
            ) | __flight_js_to_i32(
                __flight_js_to_i32(b_out).wrapping_shl((__flight_js_to_u32(8.0_f64) & 31)) as f64,
            )) as f64,
        ) | __flight_js_to_i32(a_out)) as f64,
    ) >> (__flight_js_to_u32(0.0_f64) & 31)) as f64;
}

// Source: upstream/packages/adjustments/src/colorMatrixMath.ts:44 (sha256:61b3e4e454e16e54d41846f5805429208a76fdc2c0303e0299cff0077ebefb34)
pub fn concat_color_matrix(target: &Vec<f64>, source: &Vec<f64>) -> () {
    multiply_color_matrix(target, source, Some(((*target).clone()).clone()));
}

// Source: upstream/packages/adjustments/src/colorMatrixMath.ts:53 (sha256:7db2ab588e394f0d596229b0cd87ed27c4e07c9e6dad93d808d1f9b00fcad8ff)
pub fn create_brightness_color_matrix(amount: f64) -> Vec<f64> {
    return vec![
        1.0_f64, 0.0_f64, 0.0_f64, 0.0_f64, amount, 0.0_f64, 1.0_f64, 0.0_f64, 0.0_f64, amount,
        0.0_f64, 0.0_f64, 1.0_f64, 0.0_f64, amount, 0.0_f64, 0.0_f64, 0.0_f64, 1.0_f64, 0.0_f64,
    ];
}

// Source: upstream/packages/adjustments/src/colorMatrixMath.ts:71 (sha256:29f50d554a2148ae661ff1c1faaf4660c4699d2bc1d9425b50cc77361fb991f8)
pub fn create_channel_mixer_color_matrix(
    red_out: &Vec<f64>,
    green_out: &Vec<f64>,
    blue_out: &Vec<f64>,
) -> Vec<f64> {
    return vec![
        red_out[0.0_f64 as usize].clone(),
        red_out[1.0_f64 as usize].clone(),
        red_out[2.0_f64 as usize].clone(),
        0.0_f64,
        0.0_f64,
        green_out[0.0_f64 as usize].clone(),
        green_out[1.0_f64 as usize].clone(),
        green_out[2.0_f64 as usize].clone(),
        0.0_f64,
        0.0_f64,
        blue_out[0.0_f64 as usize].clone(),
        blue_out[1.0_f64 as usize].clone(),
        blue_out[2.0_f64 as usize].clone(),
        0.0_f64,
        0.0_f64,
        0.0_f64,
        0.0_f64,
        0.0_f64,
        1.0_f64,
        0.0_f64,
    ];
}

// Source: upstream/packages/adjustments/src/colorMatrixMath.ts:96 (sha256:289f24615aeb450a9ed5c1416474e730a3e32050d8ebc072d88b861c9519b5d6)
pub fn create_color_balance_color_matrix(
    shadows: &Vec<f64>,
    midtones: &Vec<f64>,
    highlights: &Vec<f64>,
) -> Vec<f64> {
    let scale = (255.0_f64 / 100.0_f64);
    let r_off = ((((shadows[0.0_f64 as usize].clone() * 0.25_f64)
        + (midtones[0.0_f64 as usize].clone() * 0.5_f64))
        + (highlights[0.0_f64 as usize].clone() * 0.25_f64))
        * scale);
    let g_off = ((((shadows[1.0_f64 as usize].clone() * 0.25_f64)
        + (midtones[1.0_f64 as usize].clone() * 0.5_f64))
        + (highlights[1.0_f64 as usize].clone() * 0.25_f64))
        * scale);
    let b_off = ((((shadows[2.0_f64 as usize].clone() * 0.25_f64)
        + (midtones[2.0_f64 as usize].clone() * 0.5_f64))
        + (highlights[2.0_f64 as usize].clone() * 0.25_f64))
        * scale);
    return vec![
        1.0_f64, 0.0_f64, 0.0_f64, 0.0_f64, r_off, 0.0_f64, 1.0_f64, 0.0_f64, 0.0_f64, g_off,
        0.0_f64, 0.0_f64, 1.0_f64, 0.0_f64, b_off, 0.0_f64, 0.0_f64, 0.0_f64, 1.0_f64, 0.0_f64,
    ];
}

// Source: upstream/packages/adjustments/src/colorMatrixMath.ts:121 (sha256:eb6a30a2c71b8aa5b6a2804dedc00654b9e56223bcb8ab21945d6fdd93b77424)
pub fn create_color_matrix_from_tint(packed_rgba: f64, amount: f64) -> Vec<f64> {
    let tr = ((__flight_js_to_i32(
        (__flight_js_to_u32(packed_rgba) >> (__flight_js_to_u32(24.0_f64) & 31)) as f64,
    ) & __flight_js_to_i32(255.0_f64)) as f64
        * amount);
    let tg = ((__flight_js_to_i32(
        (__flight_js_to_u32(packed_rgba) >> (__flight_js_to_u32(16.0_f64) & 31)) as f64,
    ) & __flight_js_to_i32(255.0_f64)) as f64
        * amount);
    let tb = ((__flight_js_to_i32(
        (__flight_js_to_u32(packed_rgba) >> (__flight_js_to_u32(8.0_f64) & 31)) as f64,
    ) & __flight_js_to_i32(255.0_f64)) as f64
        * amount);
    let keep = (1.0_f64 - amount);
    return vec![
        keep, 0.0_f64, 0.0_f64, 0.0_f64, tr, 0.0_f64, keep, 0.0_f64, 0.0_f64, tg, 0.0_f64, 0.0_f64,
        keep, 0.0_f64, tb, 0.0_f64, 0.0_f64, 0.0_f64, 1.0_f64, 0.0_f64,
    ];
}

// Source: upstream/packages/adjustments/src/colorMatrixMath.ts:139 (sha256:726e0686898ad8aff31063cd7404548c50674af6d6d7b01f314757567bfb0926)
pub fn create_contrast_color_matrix(amount: f64) -> Vec<f64> {
    let offset = (128.0_f64 * (1.0_f64 - amount));
    return vec![
        amount, 0.0_f64, 0.0_f64, 0.0_f64, offset, 0.0_f64, amount, 0.0_f64, 0.0_f64, offset,
        0.0_f64, 0.0_f64, amount, 0.0_f64, offset, 0.0_f64, 0.0_f64, 0.0_f64, 1.0_f64, 0.0_f64,
    ];
}

// Source: upstream/packages/adjustments/src/colorMatrixMath.ts:155 (sha256:c5c343dbd1b2e6e1062c853b5abc22286c23783c12599943cb885dc00afec8b1)
pub fn create_desaturate_color_matrix(amount: f64) -> Vec<f64> {
    return create_saturation_color_matrix((1.0_f64 - amount));
}

// Source: upstream/packages/adjustments/src/colorMatrixMath.ts:163 (sha256:836d9ceb761e896ee4ce639422d1596abfa15df76ba193c22263de11c94f0677)
pub fn create_grayscale_color_matrix() -> Vec<f64> {
    let r = 0.299_f64;
    let g = 0.587_f64;
    let b = 0.114_f64;
    return vec![
        r, g, b, 0.0_f64, 0.0_f64, r, g, b, 0.0_f64, 0.0_f64, r, g, b, 0.0_f64, 0.0_f64, 0.0_f64,
        0.0_f64, 0.0_f64, 1.0_f64, 0.0_f64,
    ];
}

// Source: upstream/packages/adjustments/src/colorMatrixMath.ts:180 (sha256:d056bd688b2779b70022e7e32897d868449ce34428de60929415fdf58521e44e)
pub fn create_hue_rotate_color_matrix(degrees: f64) -> Vec<f64> {
    let rad = ((degrees * std::f64::consts::PI) / 180.0_f64);
    let cos = (rad).cos();
    let sin = (rad).sin();
    let lum_r = 0.213_f64;
    let lum_g = 0.715_f64;
    let lum_b = 0.072_f64;
    return vec![
        ((lum_r + (cos * (1.0_f64 - lum_r))) + (sin * (-lum_r))),
        ((lum_g + (cos * (-lum_g))) + (sin * (-lum_g))),
        ((lum_b + (cos * (-lum_b))) + (sin * (1.0_f64 - lum_b))),
        0.0_f64,
        0.0_f64,
        ((lum_r + (cos * (-lum_r))) + (sin * 0.143_f64)),
        ((lum_g + (cos * (1.0_f64 - lum_g))) + (sin * 0.14_f64)),
        ((lum_b + (cos * (-lum_b))) + (sin * (-0.283_f64))),
        0.0_f64,
        0.0_f64,
        ((lum_r + (cos * (-lum_r))) + (sin * (-(1.0_f64 - lum_r)))),
        ((lum_g + (cos * (-lum_g))) + (sin * lum_g)),
        ((lum_b + (cos * (1.0_f64 - lum_b))) + (sin * lum_b)),
        0.0_f64,
        0.0_f64,
        0.0_f64,
        0.0_f64,
        0.0_f64,
        1.0_f64,
        0.0_f64,
    ];
}

// Source: upstream/packages/adjustments/src/colorMatrixMath.ts:201 (sha256:a960da5918de3f281a916deebf24bf376e2471ead946284c63f58e7dd2860766)
pub fn create_identity_color_matrix() -> Vec<f64> {
    return vec![
        1.0_f64, 0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64, 1.0_f64, 0.0_f64, 0.0_f64, 0.0_f64,
        0.0_f64, 0.0_f64, 1.0_f64, 0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64, 1.0_f64, 0.0_f64,
    ];
}

// Source: upstream/packages/adjustments/src/colorMatrixMath.ts:215 (sha256:af59b45668e62a301a0c3f58d851bce8ebc6146d5c6c4fac853937cc9441e056)
pub fn create_invert_color_matrix() -> Vec<f64> {
    return vec![
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
    ];
}

// Source: upstream/packages/adjustments/src/colorMatrixMath.ts:236 (sha256:1d4a754ba8dea8799fd793a71a5e955df180cb8ab57cbdd794cbc4fa78c6ac6d)
pub fn create_levels_color_matrix(
    in_black: f64,
    in_white: f64,
    out_black: f64,
    out_white: f64,
    gamma: Option<f64>,
) -> Vec<f64> {
    let gamma = gamma.unwrap_or(1.0_f64);
    let in_range = (in_white - in_black);
    let scale = if (in_range == 0.0_f64) {
        1.0_f64
    } else {
        ((out_white - out_black) / in_range)
    };
    let gamma_corrected_scale = (scale
        * if (gamma == 1.0_f64) {
            1.0_f64
        } else {
            (0.5_f64).powf(((1.0_f64 / gamma) - 1.0_f64))
        });
    let offset = (out_black - (in_black * gamma_corrected_scale));
    return vec![
        gamma_corrected_scale,
        0.0_f64,
        0.0_f64,
        0.0_f64,
        offset,
        0.0_f64,
        gamma_corrected_scale,
        0.0_f64,
        0.0_f64,
        offset,
        0.0_f64,
        0.0_f64,
        gamma_corrected_scale,
        0.0_f64,
        offset,
        0.0_f64,
        0.0_f64,
        0.0_f64,
        1.0_f64,
        0.0_f64,
    ];
}

// Source: upstream/packages/adjustments/src/colorMatrixMath.ts:262 (sha256:da03ad0a9038e18cf4b387c5c3bd9522814759456c51918a3f06528974b259a5)
pub fn create_opacity_color_matrix(alpha: f64) -> Vec<f64> {
    return vec![
        1.0_f64, 0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64, 1.0_f64, 0.0_f64, 0.0_f64, 0.0_f64,
        0.0_f64, 0.0_f64, 1.0_f64, 0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64, alpha, 0.0_f64,
    ];
}

// Source: upstream/packages/adjustments/src/colorMatrixMath.ts:277 (sha256:0bd944e51e9a46fcb7238204e0759fe5ff713d2d3e2e5e1edd0f3952aed5aa1e)
pub fn create_polaroid_color_matrix() -> Vec<f64> {
    return vec![
        1.438_f64,
        (-0.062_f64),
        (-0.062_f64),
        0.0_f64,
        (-31.8_f64),
        (-0.122_f64),
        1.378_f64,
        (-0.122_f64),
        0.0_f64,
        16.2_f64,
        (-0.016_f64),
        (-0.016_f64),
        1.484_f64,
        0.0_f64,
        (-47.6_f64),
        0.0_f64,
        0.0_f64,
        0.0_f64,
        1.0_f64,
        0.0_f64,
    ];
}

// Source: upstream/packages/adjustments/src/colorMatrixMath.ts:292 (sha256:6196e1dd49e6f66ab59265b74e433927c913c77f9ca2e1bece2734291265b05c)
pub fn create_saturation_color_matrix(amount: f64) -> Vec<f64> {
    let r = (0.299_f64 * (1.0_f64 - amount));
    let g = (0.587_f64 * (1.0_f64 - amount));
    let b = (0.114_f64 * (1.0_f64 - amount));
    return vec![
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
    ];
}

// Source: upstream/packages/adjustments/src/colorMatrixMath.ts:308 (sha256:932a1d57bee0664ef2f25067d4f878d9ca97f0019945cbc277e3d893db5d5c44)
pub fn create_sepia_color_matrix() -> Vec<f64> {
    return vec![
        0.393_f64, 0.769_f64, 0.189_f64, 0.0_f64, 0.0_f64, 0.349_f64, 0.686_f64, 0.168_f64,
        0.0_f64, 0.0_f64, 0.272_f64, 0.534_f64, 0.131_f64, 0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64,
        0.0_f64, 1.0_f64, 0.0_f64,
    ];
}

// Source: upstream/packages/adjustments/src/colorMatrixMath.ts:323 (sha256:04e99b43c3216638f23499de3e1efbffd8fee7be83db1131ee62eb49d34ec1a9)
pub fn create_technicolor_color_matrix() -> Vec<f64> {
    return vec![
        1.9126_f64,
        (-0.8_f64),
        (-0.09_f64),
        0.0_f64,
        11.79_f64,
        (-0.2_f64),
        1.7_f64,
        (-0.27_f64),
        0.0_f64,
        (-14.69_f64),
        (-0.14_f64),
        (-0.21_f64),
        1.62_f64,
        0.0_f64,
        (-3.38_f64),
        0.0_f64,
        0.0_f64,
        0.0_f64,
        1.0_f64,
        0.0_f64,
    ];
}

// Source: upstream/packages/adjustments/src/colorMatrixMath.ts:338 (sha256:0fc204c518478c2a460e958a2c1b5757110052aadcf0d05f73da8c4d433b1016)
pub fn create_vintage_color_matrix() -> Vec<f64> {
    return vec![
        0.9_f64, 0.05_f64, 0.05_f64, 0.0_f64, 10.0_f64, 0.0_f64, 0.85_f64, 0.0_f64, 0.0_f64,
        5.0_f64, 0.0_f64, 0.0_f64, 0.75_f64, 0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64, 1.0_f64,
        0.0_f64,
    ];
}

// Source: upstream/packages/adjustments/src/colorMatrixMath.ts:357 (sha256:b93baef9ef6aa956f1a22971df3019a13744f7e9bbe3165eb34ba1745d603fb5)
pub fn create_white_balance_color_matrix(temperature: f64, tint: f64) -> Vec<f64> {
    let temp_scale = (temperature / 100.0_f64);
    let tint_scale = (tint / 100.0_f64);
    let r_gain = (1.0_f64 + (temp_scale * 0.3_f64));
    let g_gain = (1.0_f64 + (tint_scale * 0.15_f64));
    let b_gain = (1.0_f64 - (temp_scale * 0.3_f64));
    return vec![
        r_gain, 0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64, g_gain, 0.0_f64, 0.0_f64, 0.0_f64,
        0.0_f64, 0.0_f64, b_gain, 0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64, 1.0_f64, 0.0_f64,
    ];
}

// Source: upstream/packages/adjustments/src/colorMatrixMath.ts:381 (sha256:b341621d7bd834eaa545334aeb5c7f3bfb4a150a96409ce54ab4aa65b1f7cca9)
pub fn fuse_color_matrices(matrices: &Vec<Vec<f64>>) -> Vec<f64> {
    if ((matrices.len() as f64) == 0.0_f64) {
        return create_identity_color_matrix();
    }
    let out = (matrices[0.0_f64 as usize].clone()).clone();
    {
        let mut i = 1.0_f64;
        while (i < (matrices.len() as f64)) {
            multiply_color_matrix(&matrices[i as usize], &out, Some(((out).clone()).clone()));
            {
                i += 1.0;
                i
            };
        }
    }
    return out;
}

// Source: upstream/packages/adjustments/src/colorMatrixMath.ts:393 (sha256:477a25b07bbc715a2201905557dad93c44e863152b5294bff2476e8681d66e7c)
pub fn multiply_color_matrix(a: &Vec<f64>, b: &Vec<f64>, out: Option<Vec<f64>>) -> Vec<f64> {
    let a0 = a[0.0_f64 as usize].clone();
    let a1 = a[1.0_f64 as usize].clone();
    let a2 = a[2.0_f64 as usize].clone();
    let a3 = a[3.0_f64 as usize].clone();
    let a4 = a[4.0_f64 as usize].clone();
    let a5 = a[5.0_f64 as usize].clone();
    let a6 = a[6.0_f64 as usize].clone();
    let a7 = a[7.0_f64 as usize].clone();
    let a8 = a[8.0_f64 as usize].clone();
    let a9 = a[9.0_f64 as usize].clone();
    let a10 = a[10.0_f64 as usize].clone();
    let a11 = a[11.0_f64 as usize].clone();
    let a12 = a[12.0_f64 as usize].clone();
    let a13 = a[13.0_f64 as usize].clone();
    let a14 = a[14.0_f64 as usize].clone();
    let a15 = a[15.0_f64 as usize].clone();
    let a16 = a[16.0_f64 as usize].clone();
    let a17 = a[17.0_f64 as usize].clone();
    let a18 = a[18.0_f64 as usize].clone();
    let a19 = a[19.0_f64 as usize].clone();
    let b0 = b[0.0_f64 as usize].clone();
    let b1 = b[1.0_f64 as usize].clone();
    let b2 = b[2.0_f64 as usize].clone();
    let b3 = b[3.0_f64 as usize].clone();
    let b4 = b[4.0_f64 as usize].clone();
    let b5 = b[5.0_f64 as usize].clone();
    let b6 = b[6.0_f64 as usize].clone();
    let b7 = b[7.0_f64 as usize].clone();
    let b8 = b[8.0_f64 as usize].clone();
    let b9 = b[9.0_f64 as usize].clone();
    let b10 = b[10.0_f64 as usize].clone();
    let b11 = b[11.0_f64 as usize].clone();
    let b12 = b[12.0_f64 as usize].clone();
    let b13 = b[13.0_f64 as usize].clone();
    let b14 = b[14.0_f64 as usize].clone();
    let b15 = b[15.0_f64 as usize].clone();
    let b16 = b[16.0_f64 as usize].clone();
    let b17 = b[17.0_f64 as usize].clone();
    let b18 = b[18.0_f64 as usize].clone();
    let b19 = b[19.0_f64 as usize].clone();
    let mut result = (out).unwrap_or(vec![Default::default(); (20.0_f64) as usize]);
    {
        let __flight_index = (0.0_f64) as usize;
        let __flight_value = ((((a0 * b0) + (a1 * b5)) + (a2 * b10)) + (a3 * b15));
        if __flight_index == result.len() {
            result.push(__flight_value);
        } else {
            result[__flight_index] = __flight_value;
        }
    };
    {
        let __flight_index = (1.0_f64) as usize;
        let __flight_value = ((((a0 * b1) + (a1 * b6)) + (a2 * b11)) + (a3 * b16));
        if __flight_index == result.len() {
            result.push(__flight_value);
        } else {
            result[__flight_index] = __flight_value;
        }
    };
    {
        let __flight_index = (2.0_f64) as usize;
        let __flight_value = ((((a0 * b2) + (a1 * b7)) + (a2 * b12)) + (a3 * b17));
        if __flight_index == result.len() {
            result.push(__flight_value);
        } else {
            result[__flight_index] = __flight_value;
        }
    };
    {
        let __flight_index = (3.0_f64) as usize;
        let __flight_value = ((((a0 * b3) + (a1 * b8)) + (a2 * b13)) + (a3 * b18));
        if __flight_index == result.len() {
            result.push(__flight_value);
        } else {
            result[__flight_index] = __flight_value;
        }
    };
    {
        let __flight_index = (4.0_f64) as usize;
        let __flight_value = (((((a0 * b4) + (a1 * b9)) + (a2 * b14)) + (a3 * b19)) + a4);
        if __flight_index == result.len() {
            result.push(__flight_value);
        } else {
            result[__flight_index] = __flight_value;
        }
    };
    {
        let __flight_index = (5.0_f64) as usize;
        let __flight_value = ((((a5 * b0) + (a6 * b5)) + (a7 * b10)) + (a8 * b15));
        if __flight_index == result.len() {
            result.push(__flight_value);
        } else {
            result[__flight_index] = __flight_value;
        }
    };
    {
        let __flight_index = (6.0_f64) as usize;
        let __flight_value = ((((a5 * b1) + (a6 * b6)) + (a7 * b11)) + (a8 * b16));
        if __flight_index == result.len() {
            result.push(__flight_value);
        } else {
            result[__flight_index] = __flight_value;
        }
    };
    {
        let __flight_index = (7.0_f64) as usize;
        let __flight_value = ((((a5 * b2) + (a6 * b7)) + (a7 * b12)) + (a8 * b17));
        if __flight_index == result.len() {
            result.push(__flight_value);
        } else {
            result[__flight_index] = __flight_value;
        }
    };
    {
        let __flight_index = (8.0_f64) as usize;
        let __flight_value = ((((a5 * b3) + (a6 * b8)) + (a7 * b13)) + (a8 * b18));
        if __flight_index == result.len() {
            result.push(__flight_value);
        } else {
            result[__flight_index] = __flight_value;
        }
    };
    {
        let __flight_index = (9.0_f64) as usize;
        let __flight_value = (((((a5 * b4) + (a6 * b9)) + (a7 * b14)) + (a8 * b19)) + a9);
        if __flight_index == result.len() {
            result.push(__flight_value);
        } else {
            result[__flight_index] = __flight_value;
        }
    };
    {
        let __flight_index = (10.0_f64) as usize;
        let __flight_value = ((((a10 * b0) + (a11 * b5)) + (a12 * b10)) + (a13 * b15));
        if __flight_index == result.len() {
            result.push(__flight_value);
        } else {
            result[__flight_index] = __flight_value;
        }
    };
    {
        let __flight_index = (11.0_f64) as usize;
        let __flight_value = ((((a10 * b1) + (a11 * b6)) + (a12 * b11)) + (a13 * b16));
        if __flight_index == result.len() {
            result.push(__flight_value);
        } else {
            result[__flight_index] = __flight_value;
        }
    };
    {
        let __flight_index = (12.0_f64) as usize;
        let __flight_value = ((((a10 * b2) + (a11 * b7)) + (a12 * b12)) + (a13 * b17));
        if __flight_index == result.len() {
            result.push(__flight_value);
        } else {
            result[__flight_index] = __flight_value;
        }
    };
    {
        let __flight_index = (13.0_f64) as usize;
        let __flight_value = ((((a10 * b3) + (a11 * b8)) + (a12 * b13)) + (a13 * b18));
        if __flight_index == result.len() {
            result.push(__flight_value);
        } else {
            result[__flight_index] = __flight_value;
        }
    };
    {
        let __flight_index = (14.0_f64) as usize;
        let __flight_value = (((((a10 * b4) + (a11 * b9)) + (a12 * b14)) + (a13 * b19)) + a14);
        if __flight_index == result.len() {
            result.push(__flight_value);
        } else {
            result[__flight_index] = __flight_value;
        }
    };
    {
        let __flight_index = (15.0_f64) as usize;
        let __flight_value = ((((a15 * b0) + (a16 * b5)) + (a17 * b10)) + (a18 * b15));
        if __flight_index == result.len() {
            result.push(__flight_value);
        } else {
            result[__flight_index] = __flight_value;
        }
    };
    {
        let __flight_index = (16.0_f64) as usize;
        let __flight_value = ((((a15 * b1) + (a16 * b6)) + (a17 * b11)) + (a18 * b16));
        if __flight_index == result.len() {
            result.push(__flight_value);
        } else {
            result[__flight_index] = __flight_value;
        }
    };
    {
        let __flight_index = (17.0_f64) as usize;
        let __flight_value = ((((a15 * b2) + (a16 * b7)) + (a17 * b12)) + (a18 * b17));
        if __flight_index == result.len() {
            result.push(__flight_value);
        } else {
            result[__flight_index] = __flight_value;
        }
    };
    {
        let __flight_index = (18.0_f64) as usize;
        let __flight_value = ((((a15 * b3) + (a16 * b8)) + (a17 * b13)) + (a18 * b18));
        if __flight_index == result.len() {
            result.push(__flight_value);
        } else {
            result[__flight_index] = __flight_value;
        }
    };
    {
        let __flight_index = (19.0_f64) as usize;
        let __flight_value = (((((a15 * b4) + (a16 * b9)) + (a17 * b14)) + (a18 * b19)) + a19);
        if __flight_index == result.len() {
            result.push(__flight_value);
        } else {
            result[__flight_index] = __flight_value;
        }
    };
    return result;
}

// Source: upstream/packages/adjustments/src/colorMatrixMath.ts:463 (sha256:54307b8c58c723ac0cc3c83e20c8fc21ec4c6dc30fa4eb85d07c76320fbb77f9)
fn clamp_byte(v: f64) -> f64 {
    return (0.0_f64).max((255.0_f64).min((v).round()));
}
