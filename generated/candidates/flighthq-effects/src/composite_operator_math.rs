// @generated from upstream/packages/effects/src/compositeOperatorMath.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::COMPOSITE_OPERATOR as composite_operator_constant;

// Source: upstream/packages/effects/src/compositeOperatorMath.ts:11 (sha256:02faf86ac1121be852b58eacb36418d41ca918c3a2a3839e057679deaa518c12)
pub fn get_composite_operator_factors(
    operator: CompositeOperator,
    source_alpha: f64,
    backdrop_alpha: f64,
    out: &mut crate::FlightUnion2<Vec<f64>, Vec<f32>>,
) -> () {
    let as_ = source_alpha;
    let ab = backdrop_alpha;
    let mut fa: f64;
    let mut fb: f64;
    {
        let __switch_value = operator;
        let __flight_case = if __switch_value == composite_operator_constant.clear {
            0_usize
        } else if __switch_value == composite_operator_constant.copy {
            1_usize
        } else if __switch_value == composite_operator_constant.destination_atop {
            2_usize
        } else if __switch_value == composite_operator_constant.destination_in {
            3_usize
        } else if __switch_value == composite_operator_constant.destination_out {
            4_usize
        } else if __switch_value == composite_operator_constant.destination_over {
            5_usize
        } else if __switch_value == composite_operator_constant.source_atop {
            6_usize
        } else if __switch_value == composite_operator_constant.source_in {
            7_usize
        } else if __switch_value == composite_operator_constant.source_out {
            8_usize
        } else if __switch_value == composite_operator_constant.xor {
            9_usize
        } else {
            10_usize
        };
        '__flight_switch: {
            if __flight_case <= 0_usize {
                fa = 0.0_f64;
                fb = 0.0_f64;
                break '__flight_switch;
            }
            if __flight_case <= 1_usize {
                fa = 1.0_f64;
                fb = 0.0_f64;
                break '__flight_switch;
            }
            if __flight_case <= 2_usize {
                fa = (1.0_f64 - ab);
                fb = as_;
                break '__flight_switch;
            }
            if __flight_case <= 3_usize {
                fa = 0.0_f64;
                fb = as_;
                break '__flight_switch;
            }
            if __flight_case <= 4_usize {
                fa = 0.0_f64;
                fb = (1.0_f64 - as_);
                break '__flight_switch;
            }
            if __flight_case <= 5_usize {
                fa = (1.0_f64 - ab);
                fb = 1.0_f64;
                break '__flight_switch;
            }
            if __flight_case <= 6_usize {
                fa = ab;
                fb = (1.0_f64 - as_);
                break '__flight_switch;
            }
            if __flight_case <= 7_usize {
                fa = ab;
                fb = 0.0_f64;
                break '__flight_switch;
            }
            if __flight_case <= 8_usize {
                fa = (1.0_f64 - ab);
                fb = 0.0_f64;
                break '__flight_switch;
            }
            if __flight_case <= 9_usize {
                fa = (1.0_f64 - ab);
                fb = (1.0_f64 - as_);
                break '__flight_switch;
            }
            if __flight_case <= 10_usize {
                fa = 1.0_f64;
                fb = (1.0_f64 - as_);
                break '__flight_switch;
            }
            unreachable!("exhaustive TypeScript switch completed without exiting");
        }
    }
    {
        let __flight_index = (0.0_f64) as usize;
        let __flight_value = fa;
        match out {
            crate::FlightUnion2::A(values) => {
                values[__flight_index] = __flight_value;
            }
            crate::FlightUnion2::B(values) => {
                values[__flight_index] = (__flight_value) as f32;
            }
        };
    };
    {
        let __flight_index = (1.0_f64) as usize;
        let __flight_value = fb;
        match out {
            crate::FlightUnion2::A(values) => {
                values[__flight_index] = __flight_value;
            }
            crate::FlightUnion2::B(values) => {
                values[__flight_index] = (__flight_value) as f32;
            }
        };
    };
}
