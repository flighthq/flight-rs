// @generated from upstream/packages/math/src/statistics.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/math/src/statistics.ts:5 (sha256:6c98ef0e57a21415d7b3c4b3c7b12fd471facc6ee6c9bea4606fb8def3d38eb3)
pub fn mean(values: &Vec<f64>) -> f64 {
    if ((values.len() as f64) == 0.0_f64) {
        return f64::NAN;
    }
    let scale = finite_absolute_scale(values);
    if (!(scale).is_finite()) {
        return unscaled_mean(values);
    }
    if (scale == 0.0_f64) {
        return 0.0_f64;
    }
    return ((scaled_sum(values, scale) / (values.len() as f64)) * scale);
}

// Source: upstream/packages/math/src/statistics.ts:18 (sha256:6f4d32e2c4e79b255ba7637fb908e27a36c4698d62aa9f530b0e03fc6553bb6d)
pub fn median(values: &mut Vec<f64>) -> f64 {
    if ((values.len() as f64) == 0.0_f64) {
        return f64::NAN;
    }
    let sorted = {
        let mut __flight_values = (values).clone();
        __flight_values.sort_by(|left, right| {
            let __flight_order = (|a: f64, b: f64| -> f64 { (a - b) })(left.clone(), right.clone());
            __flight_order
                .partial_cmp(&0.0_f64)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        __flight_values
    };
    let mid = ((sorted.len() as f64) / 2.0_f64).floor();
    return if (((sorted.len() as f64) % 2.0_f64) != 0.0_f64) {
        sorted[mid as usize].clone()
    } else {
        midpoint(
            sorted[(mid - 1.0_f64) as usize].clone(),
            sorted[mid as usize].clone(),
        )
    };
}

// Source: upstream/packages/math/src/statistics.ts:29 (sha256:94721ce2187a5a7391220a75fa8514c9d7f066fdb6c41a72b0c8de6416a50c5a)
pub fn standard_deviation(values: &Vec<f64>) -> f64 {
    if ((values.len() as f64) == 0.0_f64) {
        return f64::NAN;
    }
    let scale = finite_absolute_scale(values);
    if (!(scale).is_finite()) {
        return (unscaled_variance(values)).sqrt();
    }
    if (scale == 0.0_f64) {
        return 0.0_f64;
    }
    return ((scaled_variance(values, scale)).sqrt() * scale);
}

// Source: upstream/packages/math/src/statistics.ts:41 (sha256:3e008b550e8c0a943add3c27d6462af7c96afafcecfec2521c2f99eb775dbf77)
pub fn variance(values: &Vec<f64>) -> f64 {
    if ((values.len() as f64) == 0.0_f64) {
        return f64::NAN;
    }
    let scale = finite_absolute_scale(values);
    if (!(scale).is_finite()) {
        return unscaled_variance(values);
    }
    if (scale == 0.0_f64) {
        return 0.0_f64;
    }
    let normalized = scaled_variance(values, scale);
    if (normalized == 0.0_f64) {
        return 0.0_f64;
    }
    return ((normalized * scale) * scale);
}

// Source: upstream/packages/math/src/statistics.ts:57 (sha256:cb5110ae9598b89e72991bd76ca824d8240e8a9da4996246bccccf48a671ea1d)
pub fn weighted_average(values: &Vec<f64>, weights: &Vec<f64>) -> f64 {
    if ((values.len() as f64) != (weights.len() as f64)) {
        panic!("{}", "generated Flight function threw");
    }
    if ((values.len() as f64) == 0.0_f64) {
        return f64::NAN;
    }
    let value_scale = finite_absolute_scale(values);
    let weight_scale = finite_absolute_scale(weights);
    if (!(value_scale).is_finite()) || (!(weight_scale).is_finite()) {
        return unscaled_weighted_average(values, weights);
    }
    if (weight_scale == 0.0_f64) {
        return f64::NAN;
    }
    let sum_weights = scaled_sum(weights, weight_scale);
    if (sum_weights == 0.0_f64) {
        return f64::NAN;
    }
    if (value_scale == 0.0_f64) {
        return 0.0_f64;
    }
    let mut sum_product = 0.0_f64;
    let mut correction = 0.0_f64;
    {
        let mut i = 0.0_f64;
        while (i < (values.len() as f64)) {
            let term = (((values[i as usize].clone() / value_scale)
                * (weights[i as usize].clone() / weight_scale))
                - correction);
            let next = (sum_product + term);
            correction = ((next - sum_product) - term);
            sum_product = next;
            {
                i += 1.0;
                i
            };
        }
    }
    return ((sum_product / sum_weights) * value_scale);
}

// Source: upstream/packages/math/src/statistics.ts:83 (sha256:266b4d2fc7b95f63a7684719f9719fc2bf506776df371bc9d577f24f697a300a)
fn finite_absolute_scale(values: &Vec<f64>) -> f64 {
    let mut scale = 0.0_f64;
    {
        let mut i = 0.0_f64;
        while (i < (values.len() as f64)) {
            scale = (scale).max((values[i as usize].clone()).abs());
            {
                i += 1.0;
                i
            };
        }
    }
    return scale;
}

// Source: upstream/packages/math/src/statistics.ts:89 (sha256:fefa13e74ac957c3cdbaaa7b56cfa1906d63eca69ec2e95823ef9f542350b3f2)
fn midpoint(a: f64, b: f64) -> f64 {
    if (!(a).is_finite()) || (!(b).is_finite()) {
        return ((a + b) / 2.0_f64);
    }
    if {
        let __flight_left = a;
        let __flight_right = b;
        __flight_left.to_bits() == __flight_right.to_bits()
            || (__flight_left.is_nan() && __flight_right.is_nan())
    } {
        return a;
    }
    if ((a >= 0.0_f64) && (b >= 0.0_f64)) || ((a <= 0.0_f64) && (b <= 0.0_f64)) {
        return (a + ((b - a) / 2.0_f64));
    }
    return ((a / 2.0_f64) + (b / 2.0_f64));
}

// Source: upstream/packages/math/src/statistics.ts:96 (sha256:0d7acace7dd0473546addeb9fd0859fba37ef8c9422c8857cbc3ea04f0a3bd69)
fn scaled_sum(values: &Vec<f64>, scale: f64) -> f64 {
    let mut sum = 0.0_f64;
    let mut correction = 0.0_f64;
    {
        let mut i = 0.0_f64;
        while (i < (values.len() as f64)) {
            let term = ((values[i as usize].clone() / scale) - correction);
            let next = (sum + term);
            correction = ((next - sum) - term);
            sum = next;
            {
                i += 1.0;
                i
            };
        }
    }
    return sum;
}

// Source: upstream/packages/math/src/statistics.ts:108 (sha256:211dac5708f1ec28efe6d113a4d99bbe6a21a5ed2cec4147d5ac8728b23997ee)
fn scaled_variance(values: &Vec<f64>, scale: f64) -> f64 {
    let normalized_mean = (scaled_sum(values, scale) / (values.len() as f64));
    let mut sum = 0.0_f64;
    let mut correction = 0.0_f64;
    {
        let mut i = 0.0_f64;
        while (i < (values.len() as f64)) {
            let difference = ((values[i as usize].clone() / scale) - normalized_mean);
            let term = ((difference * difference) - correction);
            let next = (sum + term);
            correction = ((next - sum) - term);
            sum = next;
            {
                i += 1.0;
                i
            };
        }
    }
    return (0.0_f64).max((sum / (values.len() as f64)));
}

// Source: upstream/packages/math/src/statistics.ts:122 (sha256:097cba4210d8f995ceeb5793257370b8ed45c7cb0cfb120138df1ba7dd7d42d8)
fn unscaled_mean(values: &Vec<f64>) -> f64 {
    let mut sum = 0.0_f64;
    {
        let mut i = 0.0_f64;
        while (i < (values.len() as f64)) {
            sum += values[i as usize].clone();
            {
                i += 1.0;
                i
            };
        }
    }
    return (sum / (values.len() as f64));
}

// Source: upstream/packages/math/src/statistics.ts:128 (sha256:b2561cb860476cd22a1a78009d1bf5969e8e198b9b75d63a6c5c32676772a056)
fn unscaled_variance(values: &Vec<f64>) -> f64 {
    let average = unscaled_mean(values);
    let mut sum = 0.0_f64;
    {
        let mut i = 0.0_f64;
        while (i < (values.len() as f64)) {
            let difference = (values[i as usize].clone() - average);
            sum += (difference * difference);
            {
                i += 1.0;
                i
            };
        }
    }
    return (sum / (values.len() as f64));
}

// Source: upstream/packages/math/src/statistics.ts:138 (sha256:8c0c2f86fb2cc2a6ad30608f20d3f9035f3d05400fcc098f34905dce65c2cb8f)
fn unscaled_weighted_average(values: &Vec<f64>, weights: &Vec<f64>) -> f64 {
    let mut sum_weights = 0.0_f64;
    let mut sum_product = 0.0_f64;
    {
        let mut i = 0.0_f64;
        while (i < (values.len() as f64)) {
            sum_weights += weights[i as usize].clone();
            sum_product += (values[i as usize].clone() * weights[i as usize].clone());
            {
                i += 1.0;
                i
            };
        }
    }
    return if (sum_weights == 0.0_f64) {
        f64::NAN
    } else {
        (sum_product / sum_weights)
    };
}
