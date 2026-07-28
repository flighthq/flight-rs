// @generated from upstream/packages/math/src/statistics.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/math/src/statistics.ts:5 (sha256:c4b648516f5eea00b981f1e376a9ca02c992bcea9c2b8dfa60c3886524d53d4a)
pub fn mean(values: &Vec<f64>) -> f64 {
    if ((values.len() as f64) == 0.0_f64) {
        return f64::NAN;
    }
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

// Source: upstream/packages/math/src/statistics.ts:17 (sha256:6e4879aebb64bea327e99527553da6c9af8cc93e5e02da75b9ea1c97b789d2c4)
pub fn median(values: &Vec<f64>) -> f64 {
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
        ((sorted[(mid - 1.0_f64) as usize].clone() + sorted[mid as usize].clone()) / 2.0_f64)
    };
}

// Source: upstream/packages/math/src/statistics.ts:28 (sha256:56353557dcda7e0bfa547f431a35a6d705a4912d8d74cb428a8b6f749119665b)
pub fn standard_deviation(values: &Vec<f64>) -> f64 {
    return (variance(values)).sqrt();
}

// Source: upstream/packages/math/src/statistics.ts:36 (sha256:b463b85697c0a746fab895a44c2cd8543ebbfec49f3e40ce73611eb4cf93ba23)
pub fn variance(values: &Vec<f64>) -> f64 {
    if ((values.len() as f64) == 0.0_f64) {
        return f64::NAN;
    }
    let m = mean(values);
    let mut sum = 0.0_f64;
    {
        let mut i = 0.0_f64;
        while (i < (values.len() as f64)) {
            let d = (values[i as usize].clone() - m);
            sum += (d * d);
            {
                i += 1.0;
                i
            };
        }
    }
    return (sum / (values.len() as f64));
}

// Source: upstream/packages/math/src/statistics.ts:53 (sha256:219cfaea8a14ff17e0f613195e468507087a3eec84713e8d90f80b4fea53c4be)
pub fn weighted_average(values: &Vec<f64>, weights: &Vec<f64>) -> f64 {
    if ((values.len() as f64) != (weights.len() as f64)) {
        panic!("{}", "generated Flight function threw");
    }
    if ((values.len() as f64) == 0.0_f64) {
        return f64::NAN;
    }
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
