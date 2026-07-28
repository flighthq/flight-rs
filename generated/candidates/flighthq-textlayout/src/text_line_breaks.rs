// @generated from upstream/packages/textlayout/src/textLineBreaks.ts; do not edit.
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

// Source: upstream/packages/textlayout/src/textLineBreaks.ts:6 (sha256:0fe495bbae03040e742b4355a27d163d7f8efe45c45fa8a09404a030160011de)
pub fn get_text_line_break_index(line_breaks: &Vec<f64>, start_index: Option<f64>) -> f64 {
    let start_index = start_index.unwrap_or(0.0_f64);
    if ((line_breaks.len() as f64) == 0.0_f64) {
        return (-1.0_f64);
    }
    let mut lo = 0.0_f64;
    let mut hi = ((line_breaks.len() as f64) - 1.0_f64);
    let mut result = (-1.0_f64);
    while (lo <= hi) {
        let mid = (__flight_js_to_u32((lo + hi)) >> (__flight_js_to_u32(1.0_f64) & 31)) as f64;
        if (line_breaks[mid as usize].clone() >= start_index) {
            result = line_breaks[mid as usize].clone();
            hi = (mid - 1.0_f64);
        } else {
            lo = (mid + 1.0_f64);
        }
    }
    return result;
}

// Source: upstream/packages/textlayout/src/textLineBreaks.ts:23 (sha256:b6b1ce39a3c10d6c4a788e4cbc5baf4a3adf959363adea1434916632c865eef0)
pub fn get_text_line_breaks(out: &mut Vec<f64>, text: String) -> () {
    out.clear();
    let mut index = (-1.0_f64);
    while (index < text.length) {
        let lf = (text.index_of)("\n", (index + 1.0_f64));
        let cr = (text.index_of)("\r", (index + 1.0_f64));
        if ((lf == (-1.0_f64)) && (cr == (-1.0_f64))) {
            break;
        }
        index = if (cr == (-1.0_f64)) {
            lf
        } else {
            if (lf == (-1.0_f64)) { cr } else { (cr).min(lf) }
        };
        out.push(index);
    }
}
