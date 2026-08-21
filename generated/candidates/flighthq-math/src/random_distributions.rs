// @generated from upstream/packages/math/src/randomDistributions.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::{RandomSource, Vector2Like, Vector3Like};

// Source: upstream/packages/math/src/randomDistributions.ts:7 (sha256:e89d1aa8eca58889f13ba413275fd5fee1a826902346ee4dd1cf0f2242725a4b)
pub fn pick<T: Clone>(random: RandomSource, items: &Vec<T>) -> Option<T> {
    if ((items.len() as f64) == 0.0_f64) {
        return None;
    }
    return Some(
        items[({
            let __flight_callback = (random).clone();
            let __flight_result = __flight_callback.lock().unwrap()();
            __flight_result
        } * (items.len() as f64))
            .floor() as usize]
            .clone(),
    );
}

// Source: upstream/packages/math/src/randomDistributions.ts:23 (sha256:33ce5762679247a26efbb000ea4d671ed90c3981dfbfc77e6afce9dec2df33b9)
pub fn random_exponential(random: RandomSource, rate: Option<f64>) -> f64 {
    let rate = rate.unwrap_or(1.0_f64);
    if (!(rate).is_finite()) || (rate <= 0.0_f64) {
        panic!("{}", "generated Flight function threw");
    }
    let u = {
        let __flight_callback = (random).clone();
        let __flight_result = __flight_callback.lock().unwrap()();
        __flight_result
    };
    return ((-(if (u == 0.0_f64) { f64::EPSILON } else { u }).ln()) / rate);
}

// Source: upstream/packages/math/src/randomDistributions.ts:41 (sha256:eb8827c1debc9da1307b7c1b52f0015810f4e9e68d3b82f83bda4082a979e7ba)
pub fn random_gaussian(
    random: RandomSource,
    mean: Option<f64>,
    standard_deviation: Option<f64>,
) -> f64 {
    let mean = mean.unwrap_or(0.0_f64);
    let standard_deviation = standard_deviation.unwrap_or(1.0_f64);
    let u1 = {
        let __flight_callback = (random).clone();
        let __flight_result = __flight_callback.lock().unwrap()();
        __flight_result
    };
    let u2 = {
        let __flight_callback = (random).clone();
        let __flight_result = __flight_callback.lock().unwrap()();
        __flight_result
    };
    let z = (((-2.0_f64) * (if (u1 == 0.0_f64) { f64::EPSILON } else { u1 }).ln()).sqrt()
        * ((std::f64::consts::PI * 2.0_f64) * u2).cos());
    return (mean + (z * standard_deviation));
}

// Source: upstream/packages/math/src/randomDistributions.ts:56 (sha256:c29855702000ecfde3bb078fb4643dbc8af90105273495d51007deee32647f0f)
pub fn random_gaussian_pair(
    random: RandomSource,
    mean: Option<f64>,
    standard_deviation: Option<f64>,
) -> Vec<f64> {
    let mean = mean.unwrap_or(0.0_f64);
    let standard_deviation = standard_deviation.unwrap_or(1.0_f64);
    let u1 = {
        let __flight_callback = (random).clone();
        let __flight_result = __flight_callback.lock().unwrap()();
        __flight_result
    };
    let u2 = {
        let __flight_callback = (random).clone();
        let __flight_result = __flight_callback.lock().unwrap()();
        __flight_result
    };
    let mag = ((-2.0_f64) * (if (u1 == 0.0_f64) { f64::EPSILON } else { u1 }).ln()).sqrt();
    let angle = ((std::f64::consts::PI * 2.0_f64) * u2);
    let z0 = (mean + ((mag * (angle).cos()) * standard_deviation));
    let z1 = (mean + ((mag * (angle).sin()) * standard_deviation));
    return vec![z0, z1];
}

// Source: upstream/packages/math/src/randomDistributions.ts:79 (sha256:e3c23ab0b5264c65f38afaf48d51cb42ab4c471b98304927a548dd53c38ced66)
pub fn random_inside_unit_disc(random: RandomSource, out: &mut Vector2Like) -> () {
    let mut x: f64;
    let mut y: f64;
    loop {
        {
            x = (({
                let __flight_callback = (random).clone();
                let __flight_result = __flight_callback.lock().unwrap()();
                __flight_result
            } * 2.0_f64)
                - 1.0_f64);
            y = (({
                let __flight_callback = (random).clone();
                let __flight_result = __flight_callback.lock().unwrap()();
                __flight_result
            } * 2.0_f64)
                - 1.0_f64);
        }
        if !(((x * x) + (y * y)) > 1.0_f64) {
            break;
        }
    }
    out.x = x;
    out.y = y;
}

// Source: upstream/packages/math/src/randomDistributions.ts:101 (sha256:d0a2e19be4d7f5cba4a1b64eb7249442f867e3db4e387b70a58b20945e37f7f2)
pub fn random_inside_unit_sphere(random: RandomSource, out: &mut Vector3Like) -> () {
    let mut x: f64;
    let mut y: f64;
    let mut z: f64;
    loop {
        {
            x = (({
                let __flight_callback = (random).clone();
                let __flight_result = __flight_callback.lock().unwrap()();
                __flight_result
            } * 2.0_f64)
                - 1.0_f64);
            y = (({
                let __flight_callback = (random).clone();
                let __flight_result = __flight_callback.lock().unwrap()();
                __flight_result
            } * 2.0_f64)
                - 1.0_f64);
            z = (({
                let __flight_callback = (random).clone();
                let __flight_result = __flight_callback.lock().unwrap()();
                __flight_result
            } * 2.0_f64)
                - 1.0_f64);
        }
        if !((((x * x) + (y * y)) + (z * z)) > 1.0_f64) {
            break;
        }
    }
    out.x = x;
    out.y = y;
    out.z = z;
}

// Source: upstream/packages/math/src/randomDistributions.ts:121 (sha256:0c741e456083f0969f2611dc205f231a1f080c78eb089b3a54aca8bd4615c97c)
pub fn random_on_unit_circle(random: RandomSource, out: &mut Vector2Like) -> () {
    let angle = (({
        let __flight_callback = (random).clone();
        let __flight_result = __flight_callback.lock().unwrap()();
        __flight_result
    } * std::f64::consts::PI)
        * 2.0_f64);
    let x = (angle).cos();
    let y = (angle).sin();
    out.x = x;
    out.y = y;
}

// Source: upstream/packages/math/src/randomDistributions.ts:138 (sha256:e8f7890f86f1ea324e7fe7046041b3ef30c030baaadd9c436fc847a4b7a76931)
pub fn random_on_unit_sphere(random: RandomSource, out: &mut Vector3Like) -> () {
    let mut x: f64;
    let mut y: f64;
    let mut s: f64;
    loop {
        {
            x = (({
                let __flight_callback = (random).clone();
                let __flight_result = __flight_callback.lock().unwrap()();
                __flight_result
            } * 2.0_f64)
                - 1.0_f64);
            y = (({
                let __flight_callback = (random).clone();
                let __flight_result = __flight_callback.lock().unwrap()();
                __flight_result
            } * 2.0_f64)
                - 1.0_f64);
            s = ((x * x) + (y * y));
        }
        if !(s >= 1.0_f64) {
            break;
        }
    }
    let f = (2.0_f64 * (1.0_f64 - s).sqrt());
    let rx = (x * f);
    let ry = (y * f);
    let rz = (1.0_f64 - (2.0_f64 * s));
    out.x = rx;
    out.y = ry;
    out.z = rz;
}

// Source: upstream/packages/math/src/randomDistributions.ts:168 (sha256:e938800cbec8d2cb66bc7b771f4a1d4288f86d7810c354ee6a56edb290f4d46a)
pub fn random_poisson(mut random: RandomSource, lambda: Option<f64>) -> f64 {
    let lambda = lambda.unwrap_or(1.0_f64);
    if (!(lambda).is_finite()) || (lambda <= 0.0_f64) {
        panic!("{}", "generated Flight function threw");
    }
    let limit = (-lambda).exp();
    let mut k = 0.0_f64;
    let mut product = {
        let __flight_callback = (random).clone();
        let __flight_result = __flight_callback.lock().unwrap()();
        __flight_result
    };
    while (product > limit) {
        {
            k += 1.0;
            k
        };
        product *= {
            let __flight_callback = (random).clone();
            let __flight_result = __flight_callback.lock().unwrap()();
            __flight_result
        };
    }
    return k;
}

// Source: upstream/packages/math/src/randomDistributions.ts:189 (sha256:dd0bc04ff69841bef002d97d7b2daaac9b8ff423276a9c33d75bc89423078d09)
pub fn random_weighted(random: RandomSource, weights: &Vec<f64>) -> f64 {
    let mut total = 0.0_f64;
    {
        let mut i = 0.0_f64;
        while (i < (weights.len() as f64)) {
            total += weights[i as usize].clone();
            {
                i += 1.0;
                i
            };
        }
    }
    if (total <= 0.0_f64) {
        return (-1.0_f64);
    }
    let mut r = ({
        let __flight_callback = (random).clone();
        let __flight_result = __flight_callback.lock().unwrap()();
        __flight_result
    } * total);
    {
        let mut i = 0.0_f64;
        while (i < (weights.len() as f64)) {
            r -= weights[i as usize].clone();
            if (r <= 0.0_f64) {
                return i;
            }
            {
                i += 1.0;
                i
            };
        }
    }
    return ((weights.len() as f64) - 1.0_f64);
}

// Source: upstream/packages/math/src/randomDistributions.ts:206 (sha256:a1bf49861762efc4b535fda196a5a72eba76c4ff1202f37daf82690daaa03da6)
pub fn shuffle<T: Clone>(random: RandomSource, items: &mut Vec<T>) -> Vec<T> {
    let mut copy = (items).clone();
    shuffle_in_place((random).clone(), &mut copy);
    return copy;
}

// Source: upstream/packages/math/src/randomDistributions.ts:216 (sha256:fbf2fde3966c0c44d95ccd4a2218b04511672fdfc56e9c38a92b9c36e7f0fc13)
pub fn shuffle_in_place<T: Clone>(random: RandomSource, items: &mut Vec<T>) -> () {
    {
        let mut i = ((items.len() as f64) - 1.0_f64);
        while (i > 0.0_f64) {
            let j = ({
                let __flight_callback = (random).clone();
                let __flight_result = __flight_callback.lock().unwrap()();
                __flight_result
            } * (i + 1.0_f64))
                .floor();
            let tmp = items[i as usize].clone();
            {
                let __flight_index = (i) as usize;
                let __flight_value = items[j as usize].clone();
                if __flight_index == items.len() {
                    items.push(__flight_value);
                } else {
                    items[__flight_index] = __flight_value;
                }
            };
            {
                let __flight_index = (j) as usize;
                let __flight_value = (tmp).clone();
                if __flight_index == items.len() {
                    items.push(__flight_value);
                } else {
                    items[__flight_index] = __flight_value;
                }
            };
            {
                i -= 1.0;
                i
            };
        }
    }
}
