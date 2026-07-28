// @generated from upstream/packages/audio/src/audioFormat.ts; do not edit.
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

// Source: upstream/packages/audio/src/audioFormat.ts:3 (sha256:a0c07fd3dbdcedc831a2ce6a313463481a4969d3090abfd3ead56cb8d6a4c44d)
pub fn can_play_audio_type(mime_type: String) -> bool {
    if (mime_type == "") {
        return false;
    }
    return (crate::host_value::<()>("host.canPlayType") != "");
}

// Source: upstream/packages/audio/src/audioFormat.ts:11 (sha256:2f54d54dbf9fd89bfc8dcf91baf8e9e3709aff21c425eef3c26a10a53e338add)
pub fn detect_audio_mime_type(data: &crate::FlightUnion2<Vec<u8>, Vec<u8>>) -> Option<String> {
    let b = if false {
        (*data).clone()
    } else {
        crate::FlightUnion2::<Vec<u8>, Vec<u8>>::B(vec![0_u8; (data) as usize])
    };
    if (b.byte_length < 4.0_f64) {
        return None;
    }
    if ((((((((b.byte_length >= 12.0_f64) && (b[0.0_f64 as usize].clone() == 82.0_f64))
        && (b[1.0_f64 as usize].clone() == 73.0_f64))
        && (b[2.0_f64 as usize].clone() == 70.0_f64))
        && (b[3.0_f64 as usize].clone() == 70.0_f64))
        && (b[8.0_f64 as usize].clone() == 87.0_f64))
        && (b[9.0_f64 as usize].clone() == 65.0_f64))
        && (b[10.0_f64 as usize].clone() == 86.0_f64))
        && (b[11.0_f64 as usize].clone() == 69.0_f64)
    {
        return Some("audio/wav".to_owned());
    }
    if (((b[0.0_f64 as usize].clone() == 102.0_f64) && (b[1.0_f64 as usize].clone() == 76.0_f64))
        && (b[2.0_f64 as usize].clone() == 97.0_f64))
        && (b[3.0_f64 as usize].clone() == 67.0_f64)
    {
        return Some("audio/flac".to_owned());
    }
    if (((b[0.0_f64 as usize].clone() == 79.0_f64) && (b[1.0_f64 as usize].clone() == 103.0_f64))
        && (b[2.0_f64 as usize].clone() == 103.0_f64))
        && (b[3.0_f64 as usize].clone() == 83.0_f64)
    {
        return Some("audio/ogg".to_owned());
    }
    if ((b[0.0_f64 as usize].clone() == 73.0_f64) && (b[1.0_f64 as usize].clone() == 68.0_f64))
        && (b[2.0_f64 as usize].clone() == 51.0_f64)
    {
        return Some("audio/mpeg".to_owned());
    }
    if (b[0.0_f64 as usize].clone() == 255.0_f64)
        && ((__flight_js_to_i32(b[1.0_f64 as usize].clone()) & __flight_js_to_i32(224.0_f64))
            as f64
            == 224.0_f64)
    {
        return Some("audio/mpeg".to_owned());
    }
    if ((((b.byte_length >= 8.0_f64) && (b[4.0_f64 as usize].clone() == 102.0_f64))
        && (b[5.0_f64 as usize].clone() == 116.0_f64))
        && (b[6.0_f64 as usize].clone() == 121.0_f64))
        && (b[7.0_f64 as usize].clone() == 112.0_f64)
    {
        return Some("audio/mp4".to_owned());
    }
    if (((b[0.0_f64 as usize].clone() == 26.0_f64) && (b[1.0_f64 as usize].clone() == 69.0_f64))
        && (b[2.0_f64 as usize].clone() == 223.0_f64))
        && (b[3.0_f64 as usize].clone() == 163.0_f64)
    {
        return Some("audio/webm".to_owned());
    }
    return None;
}

// Source: upstream/packages/audio/src/audioFormat.ts:50 (sha256:a9e392b3e7e3409dac620f8f4300c1266c19ab0282bf3c83abd71b794fc0fa32)
pub fn infer_audio_mime_type(url: String) -> Option<String> {
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
        let __flight_case = if __switch_value == "mp3" {
            0_usize
        } else if __switch_value == "ogg" {
            1_usize
        } else if __switch_value == "wav" {
            2_usize
        } else if __switch_value == "aac" {
            3_usize
        } else if __switch_value == "flac" {
            4_usize
        } else if __switch_value == "webm" {
            5_usize
        } else if __switch_value == "m4a" {
            6_usize
        } else {
            7_usize
        };
        '__flight_switch: {
            if __flight_case <= 0_usize {
                return Some("audio/mpeg".to_owned());
            }
            if __flight_case <= 1_usize {
                return Some("audio/ogg".to_owned());
            }
            if __flight_case <= 2_usize {
                return Some("audio/wav".to_owned());
            }
            if __flight_case <= 3_usize {
                return Some("audio/aac".to_owned());
            }
            if __flight_case <= 4_usize {
                return Some("audio/flac".to_owned());
            }
            if __flight_case <= 5_usize {
                return Some("audio/webm".to_owned());
            }
            if __flight_case <= 6_usize {
                return Some("audio/mp4".to_owned());
            }
            if __flight_case <= 7_usize {
                return None;
            }
            unreachable!("exhaustive TypeScript switch completed without returning");
        }
    }
}
