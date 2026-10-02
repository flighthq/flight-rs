// @generated from upstream/packages/encoding/src/utf8.ts; do not edit.
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

#[inline]

fn __flight_string_from_char_code(units: &[f64]) -> String {
    let mut encoded = Vec::with_capacity(units.len());
    for unit in units {
        assert!(
            unit.is_finite(),
            "String.fromCharCode received a non-finite code unit"
        );
        encoded.push(((unit.trunc() as i64) & 0xFFFF) as u16);
    }
    String::from_utf16(&encoded).expect("Rust strings cannot represent unpaired surrogates")
}

// Source: upstream/packages/encoding/src/utf8.ts:1 (sha256:79d7c05499c4783ea2d37165feacf5e4e66fa42d57c9d7a521f53395b4028330)
pub fn decode_utf8(bytes: &Vec<u8>, offset: Option<f64>, length: Option<f64>) -> String {
    let offset = offset.unwrap_or(0.0_f64);
    let length = length.unwrap_or(((bytes.len() as f64) - offset));
    assert_utf8_window((bytes.len() as f64), offset, length);
    let end = (offset + length);
    let mut result = "".to_owned();
    let mut index = offset;
    while (index < end) {
        let first = (bytes[{
            index += 1.0;
            index
        } as usize] as f64);
        if ((first).clone() <= 127.0_f64) {
            result.push_str(&(__flight_string_from_char_code(&[(first).clone()])));
            continue;
        }
        if ((first).clone() >= 194.0_f64) && ((first).clone() <= 223.0_f64) {
            if (index < end) {
                let second = (bytes[index as usize] as f64);
                if is_utf8_continuation((second).clone()) {
                    {
                        index += 1.0;
                        index
                    };
                    result.push_str(
                        &(__flight_string_from_char_code(&[(__flight_js_to_i32(
                            __flight_js_to_i32(
                                (__flight_js_to_i32((first).clone()) & __flight_js_to_i32(31.0_f64))
                                    as f64,
                            )
                            .wrapping_shl((__flight_js_to_u32(6.0_f64) & 31))
                                as f64,
                        ) | __flight_js_to_i32(
                            (__flight_js_to_i32((second).clone()) & __flight_js_to_i32(63.0_f64))
                                as f64,
                        )) as f64])),
                    );
                    continue;
                }
            }
            result.push_str(&(((UTF8_REPLACEMENT_CHARACTER).clone()).to_owned()));
            continue;
        }
        if ((first).clone() >= 224.0_f64) && ((first).clone() <= 239.0_f64) {
            let second_minimum = if ((first).clone() == 224.0_f64) {
                160.0_f64
            } else {
                128.0_f64
            };
            let second_maximum = if ((first).clone() == 237.0_f64) {
                159.0_f64
            } else {
                191.0_f64
            };
            if ((index >= end) || ((bytes[index as usize] as f64) < second_minimum))
                || ((bytes[index as usize] as f64) > second_maximum)
            {
                result.push_str(&(((UTF8_REPLACEMENT_CHARACTER).clone()).to_owned()));
                continue;
            }
            let second = (bytes[{
                index += 1.0;
                index
            } as usize] as f64);
            if (index >= end) || (!is_utf8_continuation((bytes[index as usize] as f64))) {
                result.push_str(&(((UTF8_REPLACEMENT_CHARACTER).clone()).to_owned()));
                continue;
            }
            let third = (bytes[{
                index += 1.0;
                index
            } as usize] as f64);
            result.push_str(
                &(__flight_string_from_char_code(&[(__flight_js_to_i32(
                    (__flight_js_to_i32(
                        __flight_js_to_i32(
                            (__flight_js_to_i32((first).clone()) & __flight_js_to_i32(15.0_f64))
                                as f64,
                        )
                        .wrapping_shl((__flight_js_to_u32(12.0_f64) & 31))
                            as f64,
                    ) | __flight_js_to_i32(
                        __flight_js_to_i32(
                            (__flight_js_to_i32((second).clone()) & __flight_js_to_i32(63.0_f64))
                                as f64,
                        )
                        .wrapping_shl((__flight_js_to_u32(6.0_f64) & 31))
                            as f64,
                    )) as f64,
                ) | __flight_js_to_i32(
                    (__flight_js_to_i32(third) & __flight_js_to_i32(63.0_f64)) as f64,
                )) as f64])),
            );
            continue;
        }
        if ((first).clone() >= 240.0_f64) && ((first).clone() <= 244.0_f64) {
            let second_minimum = if ((first).clone() == 240.0_f64) {
                144.0_f64
            } else {
                128.0_f64
            };
            let second_maximum = if ((first).clone() == 244.0_f64) {
                143.0_f64
            } else {
                191.0_f64
            };
            if ((index >= end) || ((bytes[index as usize] as f64) < second_minimum))
                || ((bytes[index as usize] as f64) > second_maximum)
            {
                result.push_str(&(((UTF8_REPLACEMENT_CHARACTER).clone()).to_owned()));
                continue;
            }
            let second = (bytes[{
                index += 1.0;
                index
            } as usize] as f64);
            if (index >= end) || (!is_utf8_continuation((bytes[index as usize] as f64))) {
                result.push_str(&(((UTF8_REPLACEMENT_CHARACTER).clone()).to_owned()));
                continue;
            }
            let third = (bytes[{
                index += 1.0;
                index
            } as usize] as f64);
            if (index >= end) || (!is_utf8_continuation((bytes[index as usize] as f64))) {
                result.push_str(&(((UTF8_REPLACEMENT_CHARACTER).clone()).to_owned()));
                continue;
            }
            let fourth = (bytes[{
                index += 1.0;
                index
            } as usize] as f64);
            let code_point = (__flight_js_to_i32(
                (__flight_js_to_i32(
                    (__flight_js_to_i32(
                        __flight_js_to_i32(
                            (__flight_js_to_i32((first).clone()) & __flight_js_to_i32(7.0_f64))
                                as f64,
                        )
                        .wrapping_shl((__flight_js_to_u32(18.0_f64) & 31))
                            as f64,
                    ) | __flight_js_to_i32(
                        __flight_js_to_i32(
                            (__flight_js_to_i32((second).clone()) & __flight_js_to_i32(63.0_f64))
                                as f64,
                        )
                        .wrapping_shl((__flight_js_to_u32(12.0_f64) & 31))
                            as f64,
                    )) as f64,
                ) | __flight_js_to_i32(
                    __flight_js_to_i32(
                        (__flight_js_to_i32(third) & __flight_js_to_i32(63.0_f64)) as f64,
                    )
                    .wrapping_shl((__flight_js_to_u32(6.0_f64) & 31)) as f64,
                )) as f64,
            ) | __flight_js_to_i32(
                (__flight_js_to_i32(fourth) & __flight_js_to_i32(63.0_f64)) as f64,
            )) as f64;
            let pair = (code_point - 65536.0_f64);
            result.push_str(
                &(__flight_string_from_char_code(&[
                    (__flight_js_to_i32(55296.0_f64)
                        | __flight_js_to_i32(
                            (__flight_js_to_i32(pair) >> (__flight_js_to_u32(10.0_f64) & 31))
                                as f64,
                        )) as f64,
                    (__flight_js_to_i32(56320.0_f64)
                        | __flight_js_to_i32(
                            (__flight_js_to_i32(pair) & __flight_js_to_i32(1023.0_f64)) as f64,
                        )) as f64,
                ])),
            );
            continue;
        }
        result.push_str(&(((UTF8_REPLACEMENT_CHARACTER).clone()).to_owned()));
    }
    return result;
}

// Source: upstream/packages/encoding/src/utf8.ts:76 (sha256:f3322dbe636df47e98826b433d4633f9d8435fb36f29ccbaed6d36a3b014692c)
pub fn encode_utf8(text: String) -> Vec<u8> {
    let __flight_utf16_text: std::sync::Arc<Vec<u16>> =
        std::sync::Arc::new(text.encode_utf16().collect());
    let mut result: Vec<u8> = vec![0_u8; (measure_utf8((text).clone())) as usize];
    let mut output_index = 0.0_f64;
    {
        let mut index = 0.0_f64;
        while (index < (__flight_utf16_text.len() as f64)) {
            let mut code_point = {
                let __flight_units: &[u16] = &__flight_utf16_text;
                let __flight_raw_index = index;
                let __flight_index = if __flight_raw_index.is_nan() {
                    0_i64
                } else if __flight_raw_index.is_finite() {
                    __flight_raw_index.trunc() as i64
                } else {
                    -1_i64
                };
                if __flight_index < 0 {
                    f64::NAN
                } else {
                    __flight_units
                        .get(__flight_index as usize)
                        .map_or(f64::NAN, |unit| f64::from(*unit))
                }
            };
            if (code_point >= 55296.0_f64) && (code_point <= 56319.0_f64) {
                let second = if ((index + 1.0_f64) < (__flight_utf16_text.len() as f64)) {
                    {
                        let __flight_units: &[u16] = &__flight_utf16_text;
                        let __flight_raw_index = (index + 1.0_f64);
                        let __flight_index = if __flight_raw_index.is_nan() {
                            0_i64
                        } else if __flight_raw_index.is_finite() {
                            __flight_raw_index.trunc() as i64
                        } else {
                            -1_i64
                        };
                        if __flight_index < 0 {
                            f64::NAN
                        } else {
                            __flight_units
                                .get(__flight_index as usize)
                                .map_or(f64::NAN, |unit| f64::from(*unit))
                        }
                    }
                } else {
                    0.0_f64
                };
                if (second >= 56320.0_f64) && (second <= 57343.0_f64) {
                    code_point = ((65536.0_f64
                        + __flight_js_to_i32((code_point - 55296.0_f64))
                            .wrapping_shl((__flight_js_to_u32(10.0_f64) & 31))
                            as f64)
                        + (second - 56320.0_f64));
                    {
                        index += 1.0;
                        index
                    };
                } else {
                    code_point = UTF8_REPLACEMENT_CODE_POINT;
                }
            } else {
                if (code_point >= 56320.0_f64) && (code_point <= 57343.0_f64) {
                    code_point = UTF8_REPLACEMENT_CODE_POINT;
                }
            }
            output_index = write_utf8_code_point(&mut result, output_index, code_point);
            {
                index += 1.0;
                index
            };
        }
    }
    return result;
}

// Source: upstream/packages/encoding/src/utf8.ts:97 (sha256:01ae647a3e2f33883c1609ed8f7428bde1bf6a7eacc7bd886276dc820e1328e3)
fn assert_utf8_window(byte_length: f64, offset: f64, length: f64) -> () {
    if ((((!(offset).is_finite() && (offset).fract() == 0.0_f64)
        || (!(length).is_finite() && (length).fract() == 0.0_f64))
        || (offset < 0.0_f64))
        || (length < 0.0_f64))
        || ((offset + length) > byte_length)
    {
        panic!("{}", "generated Flight function threw");
    }
}

// Source: upstream/packages/encoding/src/utf8.ts:109 (sha256:02ae8bafae900c47ea3c0e6b5f1d10d96adaf450a05b87b0ab515f1e58bdbb43)
fn is_utf8_continuation(byte: f64) -> bool {
    return (byte >= 128.0_f64) && (byte <= 191.0_f64);
}

// Source: upstream/packages/encoding/src/utf8.ts:113 (sha256:860cdee400899d32a9ac3ffeba27a7f9ee269fb3c633b23715755ab0c9d7c176)
fn measure_utf8(text: String) -> f64 {
    let __flight_utf16_text: std::sync::Arc<Vec<u16>> =
        std::sync::Arc::new(text.encode_utf16().collect());
    let mut byte_length = 0.0_f64;
    {
        let mut index = 0.0_f64;
        while (index < (__flight_utf16_text.len() as f64)) {
            let code_unit = {
                let __flight_units: &[u16] = &__flight_utf16_text;
                let __flight_raw_index = index;
                let __flight_index = if __flight_raw_index.is_nan() {
                    0_i64
                } else if __flight_raw_index.is_finite() {
                    __flight_raw_index.trunc() as i64
                } else {
                    -1_i64
                };
                if __flight_index < 0 {
                    f64::NAN
                } else {
                    __flight_units
                        .get(__flight_index as usize)
                        .map_or(f64::NAN, |unit| f64::from(*unit))
                }
            };
            if (code_unit <= 127.0_f64) {
                {
                    byte_length += 1.0;
                    byte_length
                };
            } else {
                if (code_unit <= 2047.0_f64) {
                    byte_length += 2.0_f64;
                } else {
                    if (code_unit >= 55296.0_f64) && (code_unit <= 56319.0_f64) {
                        let second = if ((index + 1.0_f64) < (__flight_utf16_text.len() as f64)) {
                            {
                                let __flight_units: &[u16] = &__flight_utf16_text;
                                let __flight_raw_index = (index + 1.0_f64);
                                let __flight_index = if __flight_raw_index.is_nan() {
                                    0_i64
                                } else if __flight_raw_index.is_finite() {
                                    __flight_raw_index.trunc() as i64
                                } else {
                                    -1_i64
                                };
                                if __flight_index < 0 {
                                    f64::NAN
                                } else {
                                    __flight_units
                                        .get(__flight_index as usize)
                                        .map_or(f64::NAN, |unit| f64::from(*unit))
                                }
                            }
                        } else {
                            0.0_f64
                        };
                        if (second >= 56320.0_f64) && (second <= 57343.0_f64) {
                            byte_length += 4.0_f64;
                            {
                                index += 1.0;
                                index
                            };
                        } else {
                            byte_length += 3.0_f64;
                        }
                    } else {
                        byte_length += 3.0_f64;
                    }
                }
            }
            {
                index += 1.0;
                index
            };
        }
    }
    return byte_length;
}

// Source: upstream/packages/encoding/src/utf8.ts:132 (sha256:480288ae2cf61222df9f0e192a4f2615b5cafb8bf51297242b88213184044b1a)
fn write_utf8_code_point(out: &mut Vec<u8>, mut offset: f64, code_point: f64) -> f64 {
    if (code_point <= 127.0_f64) {
        out[{
            offset += 1.0;
            offset
        } as usize] = (code_point) as u8;
    } else {
        if (code_point <= 2047.0_f64) {
            out[{
                offset += 1.0;
                offset
            } as usize] = ((__flight_js_to_i32(192.0_f64)
                | __flight_js_to_i32(
                    (__flight_js_to_i32(code_point) >> (__flight_js_to_u32(6.0_f64) & 31)) as f64,
                )) as f64) as u8;
            out[{
                offset += 1.0;
                offset
            } as usize] = ((__flight_js_to_i32(128.0_f64)
                | __flight_js_to_i32(
                    (__flight_js_to_i32(code_point) & __flight_js_to_i32(63.0_f64)) as f64,
                )) as f64) as u8;
        } else {
            if (code_point <= 65535.0_f64) {
                out[{
                    offset += 1.0;
                    offset
                } as usize] = ((__flight_js_to_i32(224.0_f64)
                    | __flight_js_to_i32(
                        (__flight_js_to_i32(code_point) >> (__flight_js_to_u32(12.0_f64) & 31))
                            as f64,
                    )) as f64) as u8;
                out[{
                    offset += 1.0;
                    offset
                } as usize] = ((__flight_js_to_i32(128.0_f64)
                    | __flight_js_to_i32(
                        (__flight_js_to_i32(
                            (__flight_js_to_i32(code_point) >> (__flight_js_to_u32(6.0_f64) & 31))
                                as f64,
                        ) & __flight_js_to_i32(63.0_f64)) as f64,
                    )) as f64) as u8;
                out[{
                    offset += 1.0;
                    offset
                } as usize] = ((__flight_js_to_i32(128.0_f64)
                    | __flight_js_to_i32(
                        (__flight_js_to_i32(code_point) & __flight_js_to_i32(63.0_f64)) as f64,
                    )) as f64) as u8;
            } else {
                out[{
                    offset += 1.0;
                    offset
                } as usize] = ((__flight_js_to_i32(240.0_f64)
                    | __flight_js_to_i32(
                        (__flight_js_to_i32(code_point) >> (__flight_js_to_u32(18.0_f64) & 31))
                            as f64,
                    )) as f64) as u8;
                out[{
                    offset += 1.0;
                    offset
                } as usize] = ((__flight_js_to_i32(128.0_f64)
                    | __flight_js_to_i32(
                        (__flight_js_to_i32(
                            (__flight_js_to_i32(code_point) >> (__flight_js_to_u32(12.0_f64) & 31))
                                as f64,
                        ) & __flight_js_to_i32(63.0_f64)) as f64,
                    )) as f64) as u8;
                out[{
                    offset += 1.0;
                    offset
                } as usize] = ((__flight_js_to_i32(128.0_f64)
                    | __flight_js_to_i32(
                        (__flight_js_to_i32(
                            (__flight_js_to_i32(code_point) >> (__flight_js_to_u32(6.0_f64) & 31))
                                as f64,
                        ) & __flight_js_to_i32(63.0_f64)) as f64,
                    )) as f64) as u8;
                out[{
                    offset += 1.0;
                    offset
                } as usize] = ((__flight_js_to_i32(128.0_f64)
                    | __flight_js_to_i32(
                        (__flight_js_to_i32(code_point) & __flight_js_to_i32(63.0_f64)) as f64,
                    )) as f64) as u8;
            }
        }
    }
    return offset;
}

// Source: upstream/packages/encoding/src/utf8.ts:151 (sha256:b628178aca34577b16f6be34065db639f47a20dfb4b2c80bd10d0be5ea3fc1f2)
const UTF8_REPLACEMENT_CHARACTER: &'static str = "�";

// Source: upstream/packages/encoding/src/utf8.ts:152 (sha256:23eece84de16c956bda2b7fd0d40e4624cfb4231449d1416d8f7ce944d454fb7)
const UTF8_REPLACEMENT_CODE_POINT: f64 = 65533.0_f64;
