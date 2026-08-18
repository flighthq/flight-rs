// @generated from upstream/packages/color/src/oklab.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/color/src/oklab.ts:3 (sha256:4a9a7dfe968b5def4e9243b8df109fdee80ea94b61c6175befd517b6802d0175)
pub fn clamp_linear_rgb(out: &mut Vec<f64>, r: f64, g: f64, b: f64) -> () {
    {
        let __flight_index = (0.0_f64) as usize;
        let __flight_value = (0.0_f64).max((1.0_f64).min(r));
        if __flight_index == out.len() {
            out.push(__flight_value);
        } else {
            out[__flight_index] = __flight_value;
        }
    };
    {
        let __flight_index = (1.0_f64) as usize;
        let __flight_value = (0.0_f64).max((1.0_f64).min(g));
        if __flight_index == out.len() {
            out.push(__flight_value);
        } else {
            out[__flight_index] = __flight_value;
        }
    };
    {
        let __flight_index = (2.0_f64) as usize;
        let __flight_value = (0.0_f64).max((1.0_f64).min(b));
        if __flight_index == out.len() {
            out.push(__flight_value);
        } else {
            out[__flight_index] = __flight_value;
        }
    };
}

// Source: upstream/packages/color/src/oklab.ts:12 (sha256:a4247846be226fc75de52571e6b46c6a508a034131cc0c185db09e77487cec35)
pub fn linear_rgb_to_oklab(out: &mut Vec<f64>, r: f64, g: f64, b: f64) -> () {
    let l = (((0.4122214708_f64 * r) + (0.5363325363_f64 * g)) + (0.0514459929_f64 * b));
    let m = (((0.2119034982_f64 * r) + (0.6806995451_f64 * g)) + (0.1073969566_f64 * b));
    let s = (((0.0883024619_f64 * r) + (0.2817188376_f64 * g)) + (0.6299787005_f64 * b));
    let lc = ((0.0_f64).max(l)).cbrt();
    let mc = ((0.0_f64).max(m)).cbrt();
    let sc = ((0.0_f64).max(s)).cbrt();
    {
        let __flight_index = (0.0_f64) as usize;
        let __flight_value =
            (((0.2104542553_f64 * lc) + (0.793617785_f64 * mc)) - (0.0040720468_f64 * sc));
        if __flight_index == out.len() {
            out.push(__flight_value);
        } else {
            out[__flight_index] = __flight_value;
        }
    };
    {
        let __flight_index = (1.0_f64) as usize;
        let __flight_value =
            (((1.9779984951_f64 * lc) - (2.428592205_f64 * mc)) + (0.4505937099_f64 * sc));
        if __flight_index == out.len() {
            out.push(__flight_value);
        } else {
            out[__flight_index] = __flight_value;
        }
    };
    {
        let __flight_index = (2.0_f64) as usize;
        let __flight_value =
            (((0.0259040371_f64 * lc) + (0.7827717662_f64 * mc)) - (0.808675766_f64 * sc));
        if __flight_index == out.len() {
            out.push(__flight_value);
        } else {
            out[__flight_index] = __flight_value;
        }
    };
}

// Source: upstream/packages/color/src/oklab.ts:30 (sha256:c8ce1612676875c4b7c09081bed77fcb23f337a0a008f332a927e5cc938489f9)
pub fn oklab_to_linear_rgb(out: &mut Vec<f64>, l: f64, a: f64, b: f64) -> () {
    let lc = ((l + (0.3963377774_f64 * a)) + (0.2158037573_f64 * b));
    let mc = ((l - (0.1055613458_f64 * a)) - (0.0638541728_f64 * b));
    let sc = ((l - (0.0894841775_f64 * a)) - (1.291485548_f64 * b));
    let l = ((lc * lc) * lc);
    let m = ((mc * mc) * mc);
    let s = ((sc * sc) * sc);
    {
        let __flight_index = (0.0_f64) as usize;
        let __flight_value =
            (((4.0767416621_f64 * l) - (3.3077115913_f64 * m)) + (0.2309699292_f64 * s));
        if __flight_index == out.len() {
            out.push(__flight_value);
        } else {
            out[__flight_index] = __flight_value;
        }
    };
    {
        let __flight_index = (1.0_f64) as usize;
        let __flight_value =
            ((((-1.2684380046_f64) * l) + (2.6097574011_f64 * m)) - (0.3413193965_f64 * s));
        if __flight_index == out.len() {
            out.push(__flight_value);
        } else {
            out[__flight_index] = __flight_value;
        }
    };
    {
        let __flight_index = (2.0_f64) as usize;
        let __flight_value =
            ((((-0.0041960863_f64) * l) - (0.7034186147_f64 * m)) + (1.707614701_f64 * s));
        if __flight_index == out.len() {
            out.push(__flight_value);
        } else {
            out[__flight_index] = __flight_value;
        }
    };
}
