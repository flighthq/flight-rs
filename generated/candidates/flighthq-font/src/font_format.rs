// @generated from upstream/packages/font/src/fontFormat.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/font/src/fontFormat.ts:1 (sha256:2605a29785898e7bd98897129c2d215bb3508b22c94923b6bac00202f961d6da)
pub fn detect_font_format(bytes: &crate::FlightUnion2<Vec<u8>, Vec<u8>>) -> Option<String> {
    let b = if false {
        (*bytes).clone()
    } else {
        crate::FlightUnion2::<Vec<u8>, Vec<u8>>::B(vec![0_u8; (bytes) as usize])
    };
    if (b.byte_length < 4.0_f64) {
        return None;
    }
    if (((b[0.0_f64 as usize].clone() == 0.0_f64) && (b[1.0_f64 as usize].clone() == 1.0_f64))
        && (b[2.0_f64 as usize].clone() == 0.0_f64))
        && (b[3.0_f64 as usize].clone() == 0.0_f64)
    {
        return Some("truetype".to_owned());
    }
    if (((b[0.0_f64 as usize].clone() == 79.0_f64) && (b[1.0_f64 as usize].clone() == 84.0_f64))
        && (b[2.0_f64 as usize].clone() == 84.0_f64))
        && (b[3.0_f64 as usize].clone() == 79.0_f64)
    {
        return Some("opentype".to_owned());
    }
    if (((b[0.0_f64 as usize].clone() == 119.0_f64) && (b[1.0_f64 as usize].clone() == 79.0_f64))
        && (b[2.0_f64 as usize].clone() == 70.0_f64))
        && (b[3.0_f64 as usize].clone() == 70.0_f64)
    {
        return Some("woff".to_owned());
    }
    if (((b[0.0_f64 as usize].clone() == 119.0_f64) && (b[1.0_f64 as usize].clone() == 79.0_f64))
        && (b[2.0_f64 as usize].clone() == 70.0_f64))
        && (b[3.0_f64 as usize].clone() == 50.0_f64)
    {
        return Some("woff2".to_owned());
    }
    if (((b[0.0_f64 as usize].clone() == 116.0_f64) && (b[1.0_f64 as usize].clone() == 116.0_f64))
        && (b[2.0_f64 as usize].clone() == 99.0_f64))
        && (b[3.0_f64 as usize].clone() == 102.0_f64)
    {
        return Some("collection".to_owned());
    }
    if (((b[0.0_f64 as usize].clone() == 116.0_f64) && (b[1.0_f64 as usize].clone() == 114.0_f64))
        && (b[2.0_f64 as usize].clone() == 117.0_f64))
        && (b[3.0_f64 as usize].clone() == 101.0_f64)
    {
        return Some("truetype".to_owned());
    }
    return None;
}

// Source: upstream/packages/font/src/fontFormat.ts:26 (sha256:dfa637db218446142a62747161f64174423cae276382e132c6a4d0b5e45b6ae9)
pub fn infer_font_format_from_url(url: String) -> Option<String> {
    let ext = (((url)
        .split("?".to_owned().as_str())
        .map(|part| part.to_owned())
        .collect::<Vec<_>>()[0.0_f64 as usize]
        .clone())
    .split(".".to_owned().as_str())
    .map(|part| part.to_owned())
    .collect::<Vec<_>>()
    .pop()
    .expect("TypeScript Array.pop returned undefined"))
    .to_lowercase();
    {
        let __switch_value = ext;
        let __flight_case = if __switch_value == "woff" {
            0_usize
        } else if __switch_value == "woff2" {
            1_usize
        } else if __switch_value == "ttf" {
            2_usize
        } else if __switch_value == "otf" {
            3_usize
        } else if __switch_value == "eot" {
            4_usize
        } else if __switch_value == "svg" {
            5_usize
        } else {
            6_usize
        };
        '__flight_switch: {
            if __flight_case <= 0_usize {
                return Some("woff".to_owned());
            }
            if __flight_case <= 1_usize {
                return Some("woff2".to_owned());
            }
            if __flight_case <= 2_usize {
                return Some("truetype".to_owned());
            }
            if __flight_case <= 3_usize {
                return Some("opentype".to_owned());
            }
            if __flight_case <= 4_usize {
                return Some("embedded-opentype".to_owned());
            }
            if __flight_case <= 5_usize {
                return Some("svg".to_owned());
            }
            if __flight_case <= 6_usize {
                return None;
            }
            unreachable!("exhaustive TypeScript switch completed without returning");
        }
    }
}
