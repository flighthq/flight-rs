// @generated from upstream/packages/textbidi/src/reorderBidiLine.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/textbidi/src/reorderBidiLine.ts:10 (sha256:af63210025a0cabbc402ef179b760d9c1b16951675cc66283eaaf8e96dd0ff3c)
pub fn reorder_bidi_line(levels: &Vec<u8>, start: f64, end: f64, out: &mut Vec<f64>) -> () {
    let count = (end - start);
    out.truncate((count) as usize);
    if (count <= 0.0_f64) {
        return;
    }
    let mut highest = 0.0_f64;
    let mut lowest_odd = 255.0_f64;
    {
        let mut i = start;
        while (i < end) {
            let mut level = (levels[i as usize] as f64);
            {
                let __flight_index = (i - start) as usize;
                let __flight_value = i;
                if __flight_index == out.len() {
                    out.push(__flight_value);
                } else {
                    out[__flight_index] = __flight_value;
                }
            };
            if ((level).clone() > highest) {
                highest = (level).clone();
            }
            if (((level).clone() % 2.0_f64) == 1.0_f64) && ((level).clone() < lowest_odd) {
                lowest_odd = (level).clone();
            }
            {
                i += 1.0;
                i
            };
        }
    }
    {
        let mut level = highest;
        while (level >= lowest_odd) {
            {
                let mut k = 0.0_f64;
                while (k < count) {
                    if ((levels[out[k as usize].clone() as usize] as f64) >= level) {
                        let mut j = k;
                        while (j < count)
                            && ((levels[out[j as usize].clone() as usize] as f64) >= level)
                        {
                            {
                                j += 1.0;
                                j
                            };
                        }
                        {
                            let mut lo = k;
                            let mut hi = (j - 1.0_f64);
                            while (lo < hi) {
                                let tmp = out[lo as usize].clone();
                                {
                                    let __flight_index = (lo) as usize;
                                    let __flight_value = out[hi as usize].clone();
                                    if __flight_index == out.len() {
                                        out.push(__flight_value);
                                    } else {
                                        out[__flight_index] = __flight_value;
                                    }
                                };
                                {
                                    let __flight_index = (hi) as usize;
                                    let __flight_value = tmp;
                                    if __flight_index == out.len() {
                                        out.push(__flight_value);
                                    } else {
                                        out[__flight_index] = __flight_value;
                                    }
                                };
                                (
                                    {
                                        lo += 1.0;
                                        lo
                                    },
                                    {
                                        hi -= 1.0;
                                        hi
                                    },
                                );
                            }
                        }
                        k = j;
                    } else {
                        {
                            k += 1.0;
                            k
                        };
                    }
                }
            }
            {
                level -= 1.0;
                level
            };
        }
    }
}
