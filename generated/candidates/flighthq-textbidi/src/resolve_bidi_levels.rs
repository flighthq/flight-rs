// @generated from upstream/packages/textbidi/src/resolveBidiLevels.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::get_bidi_class_backend;
use flighthq_types::{BidiClass, BidiDirection};

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

// Source: upstream/packages/textbidi/src/resolveBidiLevels.ts:22 (sha256:ff3bb02a6da9b247009d289bc8ab14be1f5a42e0893d6ed6131852f8c934690a)
pub fn resolve_bidi_levels(text: String, base_direction: BidiDirection) -> Vec<u8> {
    let __flight_utf16_text: std::sync::Arc<Vec<u16>> =
        std::sync::Arc::new(text.encode_utf16().collect());
    let length = (__flight_utf16_text.len() as f64);
    let mut levels: Vec<u8> = vec![0_u8; (length) as usize];
    if (length == 0.0_f64) {
        return levels;
    }
    let backend = get_bidi_class_backend();
    let mut original: Vec<BidiClass> = vec![Default::default(); (length) as usize];
    {
        let mut i = 0.0_f64;
        while (i < length) {
            let codepoint = {
                let __flight_units: &[u16] = &__flight_utf16_text;
                let __flight_raw_index = i;
                let __flight_index = if __flight_raw_index.is_nan() {
                    0_i64
                } else if __flight_raw_index.is_finite() {
                    __flight_raw_index.trunc() as i64
                } else {
                    -1_i64
                };
                if __flight_index < 0 {
                    f64::NAN
                } else if let Some(&__flight_first) = __flight_units.get(__flight_index as usize) {
                    let __flight_first = u32::from(__flight_first);
                    if (0xD800_u32..=0xDBFF_u32).contains(&__flight_first) {
                        if let Some(&__flight_second) =
                            __flight_units.get(__flight_index as usize + 1)
                        {
                            let __flight_second = u32::from(__flight_second);
                            if (0xDC00_u32..=0xDFFF_u32).contains(&__flight_second) {
                                (((__flight_first - 0xD800_u32) << 10)
                                    + (__flight_second - 0xDC00_u32)
                                    + 0x10000_u32) as f64
                            } else {
                                __flight_first as f64
                            }
                        } else {
                            __flight_first as f64
                        }
                    } else {
                        __flight_first as f64
                    }
                } else {
                    f64::NAN
                }
            };
            let cls = {
                let __flight_callback = (backend.get_bidi_class).clone();
                let __flight_result = __flight_callback.lock().unwrap()(codepoint);
                __flight_result
            };
            {
                let __flight_index = (i) as usize;
                let __flight_value = (cls).clone();
                if __flight_index == original.len() {
                    original.push(__flight_value);
                } else {
                    original[__flight_index] = __flight_value;
                }
            };
            if (codepoint > 65535.0_f64) {
                {
                    let __flight_index = (i + 1.0_f64) as usize;
                    let __flight_value = (cls).clone();
                    if __flight_index == original.len() {
                        original.push(__flight_value);
                    } else {
                        original[__flight_index] = __flight_value;
                    }
                };
                {
                    i += 1.0;
                    i
                };
            }
            {
                i += 1.0;
                i
            };
        }
    }
    let paragraph_level = if (base_direction == "ltr") {
        0.0_f64
    } else {
        if (base_direction == "rtl") {
            1.0_f64
        } else {
            compute_paragraph_level(&original, 0.0_f64, length)
        }
    };
    let mut matching_pdi = {
        let mut __flight_collection = vec![0_i32; (length) as usize];
        let __flight_value = (length) as i32;
        __flight_collection.fill(__flight_value);
        __flight_collection
    };
    let mut matching_initiator = {
        let mut __flight_collection = vec![0_i32; (length) as usize];
        let __flight_value = (-1.0_f64) as i32;
        __flight_collection.fill(__flight_value);
        __flight_collection
    };
    pair_isolates(&original, &mut matching_pdi, &mut matching_initiator);
    let mut working = (original).clone();
    let mut level_array: Vec<f64> = vec![Default::default(); (length) as usize];
    apply_explicit_levels(
        &original,
        &mut working,
        &mut level_array,
        &matching_pdi,
        paragraph_level,
    );
    resolve_isolating_run_sequences(
        &original,
        &working,
        &mut level_array,
        &matching_pdi,
        &matching_initiator,
        paragraph_level,
    );
    apply_line_reset(&original, &mut level_array, paragraph_level);
    {
        let mut i = 0.0_f64;
        while (i < length) {
            levels[i as usize] = (level_array[i as usize].clone()) as u8;
            {
                i += 1.0;
                i
            };
        }
    }
    return levels;
}

// Source: upstream/packages/textbidi/src/resolveBidiLevels.ts:61 (sha256:e6a3a4bc0f59585b0e6abd39737d1b527d2c916a8506271e203487a0e3bf44ed)
fn compute_paragraph_level(types: &Vec<BidiClass>, start: f64, end: f64) -> f64 {
    let mut isolate_depth = 0.0_f64;
    {
        let mut i = start;
        while (i < end) {
            let t = types[i as usize].clone();
            if ((t == "LRI") || (t == "RLI")) || (t == "FSI") {
                {
                    isolate_depth += 1.0;
                    isolate_depth
                };
            } else {
                if (t == "PDI") {
                    if (isolate_depth > 0.0_f64) {
                        {
                            isolate_depth -= 1.0;
                            isolate_depth
                        };
                    }
                } else {
                    if (isolate_depth == 0.0_f64) {
                        if (t == "L") {
                            return 0.0_f64;
                        }
                        if (t == "R") || (t == "AL") {
                            return 1.0_f64;
                        }
                    }
                }
            }
            {
                i += 1.0;
                i
            };
        }
    }
    return 0.0_f64;
}

// Source: upstream/packages/textbidi/src/resolveBidiLevels.ts:79 (sha256:7421e1e797c1380a5dadf77e0610ad23622be0a0427117385da4210ce94af78c)
fn pair_isolates(
    types: &Vec<BidiClass>,
    matching_pdi: &mut Vec<i32>,
    matching_initiator: &mut Vec<i32>,
) -> () {
    let mut stack: Vec<f64> = vec![];
    {
        let mut i = 0.0_f64;
        while (i < (types.len() as f64)) {
            let t = types[i as usize].clone();
            if ((t == "LRI") || (t == "RLI")) || (t == "FSI") {
                stack.push(i);
            } else {
                if (t == "PDI") && ((stack.len() as f64) > 0.0_f64) {
                    let initiator = stack.pop().unwrap();
                    matching_pdi[initiator as usize] = (i) as i32;
                    matching_initiator[i as usize] = (initiator) as i32;
                }
            }
            {
                i += 1.0;
                i
            };
        }
    }
}

// Source: upstream/packages/textbidi/src/resolveBidiLevels.ts:96 (sha256:c59ffc287557f53ca369a350b10cb2766bf751e52901fda5862c4cd81b54a07f)
fn apply_explicit_levels(
    original: &Vec<BidiClass>,
    working: &mut Vec<BidiClass>,
    level_array: &mut Vec<f64>,
    matching_pdi: &Vec<i32>,
    paragraph_level: f64,
) -> () {
    let max_depth = 125.0_f64;
    let mut stack_level: Vec<f64> = vec![paragraph_level];
    let mut stack_override: Vec<Option<BidiClass>> = vec![None];
    let mut stack_isolate: Vec<bool> = vec![false];
    let mut overflow_isolate = 0.0_f64;
    let mut overflow_embedding = 0.0_f64;
    let mut valid_isolate = 0.0_f64;
    {
        let mut i = 0.0_f64;
        while (i < (original.len() as f64)) {
            let t = original[i as usize].clone();
            let top = ((stack_level.len() as f64) - 1.0_f64);
            {
                let __switch_value = (t).clone();
                let __flight_case = if __switch_value == "RLE" {
                    0_usize
                } else if __switch_value == "LRE" {
                    1_usize
                } else if __switch_value == "RLO" {
                    2_usize
                } else if __switch_value == "LRO" {
                    3_usize
                } else if __switch_value == "RLI" {
                    4_usize
                } else if __switch_value == "LRI" {
                    5_usize
                } else if __switch_value == "FSI" {
                    6_usize
                } else if __switch_value == "PDI" {
                    7_usize
                } else if __switch_value == "PDF" {
                    8_usize
                } else if __switch_value == "B" {
                    9_usize
                } else if __switch_value == "BN" {
                    10_usize
                } else {
                    11_usize
                };
                '__flight_switch: {
                    if __flight_case <= 0_usize {}
                    if __flight_case <= 1_usize {}
                    if __flight_case <= 2_usize {}
                    if __flight_case <= 3_usize {
                        {
                            {
                                let __flight_index = (i) as usize;
                                let __flight_value = stack_level[top as usize].clone();
                                if __flight_index == level_array.len() {
                                    level_array.push(__flight_value);
                                } else {
                                    level_array[__flight_index] = __flight_value;
                                }
                            };
                            {
                                let __flight_index = (i) as usize;
                                let __flight_value = "BN".to_owned();
                                if __flight_index == working.len() {
                                    working.push(__flight_value);
                                } else {
                                    working[__flight_index] = __flight_value;
                                }
                            };
                            let new_level = if (t == "RLE") || (t == "RLO") {
                                next_odd(stack_level[top as usize].clone())
                            } else {
                                next_even(stack_level[top as usize].clone())
                            };
                            if ((new_level <= max_depth) && (overflow_isolate == 0.0_f64))
                                && (overflow_embedding == 0.0_f64)
                            {
                                {
                                    stack_level.push(new_level);
                                    stack_override.push(if (t == "RLO") {
                                        Some("R".to_owned())
                                    } else {
                                        if (t == "LRO") {
                                            Some("L".to_owned())
                                        } else {
                                            None
                                        }
                                    });
                                    stack_isolate.push(false);
                                }
                            } else {
                                if (overflow_isolate == 0.0_f64) {
                                    {
                                        {
                                            overflow_embedding += 1.0;
                                            overflow_embedding
                                        };
                                    }
                                }
                            }
                            break '__flight_switch;
                        }
                    }
                    if __flight_case <= 4_usize {}
                    if __flight_case <= 5_usize {}
                    if __flight_case <= 6_usize {
                        {
                            {
                                let __flight_index = (i) as usize;
                                let __flight_value = stack_level[top as usize].clone();
                                if __flight_index == level_array.len() {
                                    level_array.push(__flight_value);
                                } else {
                                    level_array[__flight_index] = __flight_value;
                                }
                            };
                            if !(stack_override.get((top) as usize).is_none()) {
                                {
                                    let __flight_index = (i) as usize;
                                    let __flight_value =
                                        stack_override[top as usize].clone().unwrap();
                                    if __flight_index == working.len() {
                                        working.push(__flight_value);
                                    } else {
                                        working[__flight_index] = __flight_value;
                                    }
                                };
                            }
                            let mut as_rtl = (t == "RLI");
                            if (t == "FSI") {
                                as_rtl = (compute_paragraph_level(
                                    original,
                                    (i + 1.0_f64),
                                    (matching_pdi[i as usize] as f64),
                                ) == 1.0_f64);
                            }
                            let new_level = if as_rtl {
                                next_odd(stack_level[top as usize].clone())
                            } else {
                                next_even(stack_level[top as usize].clone())
                            };
                            if ((new_level <= max_depth) && (overflow_isolate == 0.0_f64))
                                && (overflow_embedding == 0.0_f64)
                            {
                                {
                                    {
                                        valid_isolate += 1.0;
                                        valid_isolate
                                    };
                                    stack_level.push(new_level);
                                    stack_override.push(None);
                                    stack_isolate.push(true);
                                }
                            } else {
                                {
                                    {
                                        overflow_isolate += 1.0;
                                        overflow_isolate
                                    };
                                }
                            }
                            break '__flight_switch;
                        }
                    }
                    if __flight_case <= 7_usize {
                        {
                            if (overflow_isolate > 0.0_f64) {
                                {
                                    {
                                        overflow_isolate -= 1.0;
                                        overflow_isolate
                                    };
                                }
                            } else {
                                if (valid_isolate > 0.0_f64) {
                                    {
                                        overflow_embedding = 0.0_f64;
                                        while (!stack_isolate
                                            [((stack_level.len() as f64) - 1.0_f64) as usize]
                                            .clone())
                                        {
                                            stack_level
                                                .pop()
                                                .expect("TypeScript Array.pop returned undefined");
                                            stack_override
                                                .pop()
                                                .expect("TypeScript Array.pop returned undefined");
                                            stack_isolate
                                                .pop()
                                                .expect("TypeScript Array.pop returned undefined");
                                        }
                                        stack_level
                                            .pop()
                                            .expect("TypeScript Array.pop returned undefined");
                                        stack_override
                                            .pop()
                                            .expect("TypeScript Array.pop returned undefined");
                                        stack_isolate
                                            .pop()
                                            .expect("TypeScript Array.pop returned undefined");
                                        {
                                            valid_isolate -= 1.0;
                                            valid_isolate
                                        };
                                    }
                                }
                            }
                            let new_top = ((stack_level.len() as f64) - 1.0_f64);
                            {
                                let __flight_index = (i) as usize;
                                let __flight_value = stack_level[new_top as usize].clone();
                                if __flight_index == level_array.len() {
                                    level_array.push(__flight_value);
                                } else {
                                    level_array[__flight_index] = __flight_value;
                                }
                            };
                            if !(stack_override.get((new_top) as usize).is_none()) {
                                {
                                    let __flight_index = (i) as usize;
                                    let __flight_value =
                                        stack_override[new_top as usize].clone().unwrap();
                                    if __flight_index == working.len() {
                                        working.push(__flight_value);
                                    } else {
                                        working[__flight_index] = __flight_value;
                                    }
                                };
                            }
                            break '__flight_switch;
                        }
                    }
                    if __flight_case <= 8_usize {
                        {
                            if (overflow_isolate > 0.0_f64) {
                                {}
                            } else {
                                if (overflow_embedding > 0.0_f64) {
                                    {
                                        {
                                            overflow_embedding -= 1.0;
                                            overflow_embedding
                                        };
                                    }
                                } else {
                                    if (!stack_isolate[top as usize].clone())
                                        && ((stack_level.len() as f64) >= 2.0_f64)
                                    {
                                        {
                                            stack_level
                                                .pop()
                                                .expect("TypeScript Array.pop returned undefined");
                                            stack_override
                                                .pop()
                                                .expect("TypeScript Array.pop returned undefined");
                                            stack_isolate
                                                .pop()
                                                .expect("TypeScript Array.pop returned undefined");
                                        }
                                    }
                                }
                            }
                            {
                                let __flight_index = (i) as usize;
                                let __flight_value = stack_level
                                    [((stack_level.len() as f64) - 1.0_f64) as usize]
                                    .clone();
                                if __flight_index == level_array.len() {
                                    level_array.push(__flight_value);
                                } else {
                                    level_array[__flight_index] = __flight_value;
                                }
                            };
                            {
                                let __flight_index = (i) as usize;
                                let __flight_value = "BN".to_owned();
                                if __flight_index == working.len() {
                                    working.push(__flight_value);
                                } else {
                                    working[__flight_index] = __flight_value;
                                }
                            };
                            break '__flight_switch;
                        }
                    }
                    if __flight_case <= 9_usize {
                        {
                            stack_level.truncate((1.0_f64) as usize);
                            stack_override.truncate((1.0_f64) as usize);
                            stack_isolate.truncate((1.0_f64) as usize);
                            overflow_isolate = 0.0_f64;
                            overflow_embedding = 0.0_f64;
                            valid_isolate = 0.0_f64;
                            {
                                let __flight_index = (i) as usize;
                                let __flight_value = paragraph_level;
                                if __flight_index == level_array.len() {
                                    level_array.push(__flight_value);
                                } else {
                                    level_array[__flight_index] = __flight_value;
                                }
                            };
                            break '__flight_switch;
                        }
                    }
                    if __flight_case <= 10_usize {
                        {
                            {
                                let __flight_index = (i) as usize;
                                let __flight_value = stack_level[top as usize].clone();
                                if __flight_index == level_array.len() {
                                    level_array.push(__flight_value);
                                } else {
                                    level_array[__flight_index] = __flight_value;
                                }
                            };
                            break '__flight_switch;
                        }
                    }
                    if __flight_case <= 11_usize {
                        {
                            {
                                let __flight_index = (i) as usize;
                                let __flight_value = stack_level[top as usize].clone();
                                if __flight_index == level_array.len() {
                                    level_array.push(__flight_value);
                                } else {
                                    level_array[__flight_index] = __flight_value;
                                }
                            };
                            if !(stack_override.get((top) as usize).is_none()) {
                                {
                                    let __flight_index = (i) as usize;
                                    let __flight_value =
                                        stack_override[top as usize].clone().unwrap();
                                    if __flight_index == working.len() {
                                        working.push(__flight_value);
                                    } else {
                                        working[__flight_index] = __flight_value;
                                    }
                                };
                            }
                            break '__flight_switch;
                        }
                    }
                    unreachable!("exhaustive TypeScript switch completed without exiting");
                }
            }
            {
                i += 1.0;
                i
            };
        }
    }
}

// Source: upstream/packages/textbidi/src/resolveBidiLevels.ts:211 (sha256:5e5bebfd7380a306e78bda85f7c5a6bbbbb8085d5f8373e3f86a3a1a683be45d)
#[derive(Clone, Default)]
struct ResolveIsolatingRunSequencesRecord1 {
    __flight_identity: std::sync::Arc<()>,
    indices: Vec<f64>,
    kept_start: f64,
    kept_end: f64,
}
impl PartialEq for ResolveIsolatingRunSequencesRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

fn resolve_isolating_run_sequences(
    original: &Vec<BidiClass>,
    working: &Vec<BidiClass>,
    level_array: &mut Vec<f64>,
    matching_pdi: &Vec<i32>,
    matching_initiator: &Vec<i32>,
    paragraph_level: f64,
) -> () {
    let length = (original.len() as f64);
    let mut kept: Vec<f64> = vec![];
    {
        let mut i = 0.0_f64;
        while (i < length) {
            if (working[i as usize].clone() != "BN") {
                kept.push(i);
            }
            {
                i += 1.0;
                i
            };
        }
    }
    if ((kept.len() as f64) == 0.0_f64) {
        return;
    }
    let mut runs: Vec<ResolveIsolatingRunSequencesRecord1> = vec![];
    let mut run_start = 0.0_f64;
    {
        let mut k = 1.0_f64;
        while (k <= (kept.len() as f64)) {
            if (k == (kept.len() as f64))
                || (level_array[kept[k as usize].clone() as usize].clone()
                    != level_array[kept[run_start as usize].clone() as usize].clone())
            {
                let mut indices: Vec<f64> = vec![];
                {
                    let mut m = run_start;
                    while (m < k) {
                        indices.push(kept[m as usize].clone());
                        {
                            m += 1.0;
                            m
                        };
                    }
                }
                runs.push(ResolveIsolatingRunSequencesRecord1 {
                    __flight_identity: std::sync::Arc::new(()),
                    indices: (indices).clone(),
                    kept_start: run_start,
                    kept_end: k,
                });
                run_start = k;
            }
            {
                k += 1.0;
                k
            };
        }
    }
    let mut run_by_first: Vec<(f64, f64)> = Vec::new();
    {
        let mut r = 0.0_f64;
        while (r < (runs.len() as f64)) {
            {
                let __flight_key = runs[r as usize].indices[0.0_f64 as usize].clone();
                let __flight_value = r;
                if let Some((_, value)) = run_by_first
                    .iter_mut()
                    .find(|(key, _)| key == &__flight_key)
                {
                    *value = __flight_value;
                } else {
                    run_by_first.push((__flight_key, __flight_value));
                }
            };
            {
                r += 1.0;
                r
            };
        }
    }
    {
        let mut r = 0.0_f64;
        while (r < (runs.len() as f64)) {
            let first_idx = runs[r as usize].indices[0.0_f64 as usize].clone();
            if (original[first_idx as usize].clone() == "PDI")
                && ((matching_initiator[first_idx as usize] as f64) != (-1.0_f64))
            {
                {
                    r += 1.0;
                    r
                };
                continue;
            }
            let mut sequence: Vec<f64> = vec![];
            let mut kept_start = runs[r as usize].kept_start;
            let mut kept_end = runs[r as usize].kept_end;
            let mut current = r;
            {
                while true {
                    let run = runs[current as usize].clone();
                    {
                        let mut m = 0.0_f64;
                        while (m < (run.indices.len() as f64)) {
                            sequence.push(run.indices[m as usize].clone());
                            {
                                m += 1.0;
                                m
                            };
                        }
                    }
                    kept_end = run.kept_end;
                    let last_idx =
                        run.indices[((run.indices.len() as f64) - 1.0_f64) as usize].clone();
                    let last_type = original[last_idx as usize].clone();
                    if (((last_type == "LRI") || (last_type == "RLI")) || (last_type == "FSI"))
                        && ((matching_pdi[last_idx as usize] as f64) < length)
                    {
                        let next = run_by_first
                            .iter()
                            .find(|(entry_key, _)| {
                                entry_key == &(matching_pdi[last_idx as usize] as f64)
                            })
                            .map(|(_, value)| value.clone());
                        if (next).is_none() {
                            break;
                        }
                        current = (next).clone().unwrap();
                    } else {
                        break;
                    }
                }
            }
            resolve_sequence(
                original,
                working,
                level_array,
                &kept,
                &sequence,
                kept_start,
                kept_end,
                matching_pdi,
                paragraph_level,
            );
            {
                r += 1.0;
                r
            };
        }
    }
}

// Source: upstream/packages/textbidi/src/resolveBidiLevels.ts:274 (sha256:76ffa06470ec4557cec6c89b24cb8370c8ce0336515ccbf7b5b38677ed0735d6)
fn resolve_sequence(
    original: &Vec<BidiClass>,
    working: &Vec<BidiClass>,
    level_array: &mut Vec<f64>,
    kept: &Vec<f64>,
    sequence: &Vec<f64>,
    kept_start: f64,
    kept_end: f64,
    matching_pdi: &Vec<i32>,
    paragraph_level: f64,
) -> () {
    let mut seq_level = level_array[sequence[0.0_f64 as usize].clone() as usize].clone();
    let prev_level = if (kept_start > 0.0_f64) {
        level_array[kept[(kept_start - 1.0_f64) as usize].clone() as usize].clone()
    } else {
        paragraph_level
    };
    let mut sos = if (((seq_level).max(prev_level) % 2.0_f64) == 1.0_f64) {
        "R".to_owned()
    } else {
        "L".to_owned()
    };
    let last_idx = sequence[((sequence.len() as f64) - 1.0_f64) as usize].clone();
    let last_type = original[last_idx as usize].clone();
    let ends_unmatched_isolate = (((last_type == "LRI") || (last_type == "RLI"))
        || (last_type == "FSI"))
        && ((matching_pdi[last_idx as usize] as f64) >= (original.len() as f64));
    let next_level = if ends_unmatched_isolate {
        paragraph_level
    } else {
        if (kept_end < (kept.len() as f64)) {
            level_array[kept[kept_end as usize].clone() as usize].clone()
        } else {
            paragraph_level
        }
    };
    let eos = if (((seq_level).max(next_level) % 2.0_f64) == 1.0_f64) {
        "R".to_owned()
    } else {
        "L".to_owned()
    };
    let len = (sequence.len() as f64);
    let mut ty: Vec<BidiClass> = vec![Default::default(); (len) as usize];
    {
        let mut k = 0.0_f64;
        while (k < len) {
            {
                let __flight_index = (k) as usize;
                let __flight_value = working[sequence[k as usize].clone() as usize].clone();
                if __flight_index == ty.len() {
                    ty.push(__flight_value);
                } else {
                    ty[__flight_index] = __flight_value;
                }
            };
            {
                k += 1.0;
                k
            };
        }
    }
    let mut prev: BidiClass = (sos).clone();
    {
        let mut k = 0.0_f64;
        while (k < len) {
            if (ty[k as usize].clone() == "NSM") {
                {
                    let __flight_index = (k) as usize;
                    let __flight_value = if ((((prev).clone() == "LRI")
                        || ((prev).clone() == "RLI"))
                        || ((prev).clone() == "FSI"))
                        || ((prev).clone() == "PDI")
                    {
                        "ON".to_owned()
                    } else {
                        (prev).clone()
                    };
                    if __flight_index == ty.len() {
                        ty.push(__flight_value);
                    } else {
                        ty[__flight_index] = __flight_value;
                    }
                };
            }
            prev = ty[k as usize].clone();
            {
                k += 1.0;
                k
            };
        }
    }
    let mut strong: BidiClass = (sos).clone();
    {
        let mut k = 0.0_f64;
        while (k < len) {
            let c = ty[k as usize].clone();
            if (((c).clone() == "L") || ((c).clone() == "R")) || ((c).clone() == "AL") {
                strong = (c).clone();
            } else {
                if ((c).clone() == "EN") && (strong == "AL") {
                    {
                        let __flight_index = (k) as usize;
                        let __flight_value = "AN".to_owned();
                        if __flight_index == ty.len() {
                            ty.push(__flight_value);
                        } else {
                            ty[__flight_index] = __flight_value;
                        }
                    };
                }
            }
            {
                k += 1.0;
                k
            };
        }
    }
    {
        let mut k = 0.0_f64;
        while (k < len) {
            if (ty[k as usize].clone() == "AL") {
                {
                    let __flight_index = (k) as usize;
                    let __flight_value = "R".to_owned();
                    if __flight_index == ty.len() {
                        ty.push(__flight_value);
                    } else {
                        ty[__flight_index] = __flight_value;
                    }
                };
            }
            {
                k += 1.0;
                k
            };
        }
    }
    {
        let mut k = 1.0_f64;
        while (k < (len - 1.0_f64)) {
            let c = ty[k as usize].clone();
            if ((c == "ES") && (ty[(k - 1.0_f64) as usize].clone() == "EN"))
                && (ty[(k + 1.0_f64) as usize].clone() == "EN")
            {
                {
                    let __flight_index = (k) as usize;
                    let __flight_value = "EN".to_owned();
                    if __flight_index == ty.len() {
                        ty.push(__flight_value);
                    } else {
                        ty[__flight_index] = __flight_value;
                    }
                };
            } else {
                if (c == "CS") {
                    if (ty[(k - 1.0_f64) as usize].clone() == "EN")
                        && (ty[(k + 1.0_f64) as usize].clone() == "EN")
                    {
                        {
                            let __flight_index = (k) as usize;
                            let __flight_value = "EN".to_owned();
                            if __flight_index == ty.len() {
                                ty.push(__flight_value);
                            } else {
                                ty[__flight_index] = __flight_value;
                            }
                        };
                    } else {
                        if (ty[(k - 1.0_f64) as usize].clone() == "AN")
                            && (ty[(k + 1.0_f64) as usize].clone() == "AN")
                        {
                            {
                                let __flight_index = (k) as usize;
                                let __flight_value = "AN".to_owned();
                                if __flight_index == ty.len() {
                                    ty.push(__flight_value);
                                } else {
                                    ty[__flight_index] = __flight_value;
                                }
                            };
                        }
                    }
                }
            }
            {
                k += 1.0;
                k
            };
        }
    }
    {
        let mut k = 0.0_f64;
        while (k < len) {
            if (ty[k as usize].clone() == "ET") {
                let mut j = k;
                while (j < len) && (ty[j as usize].clone() == "ET") {
                    {
                        j += 1.0;
                        j
                    };
                }
                let before = if (k > 0.0_f64) {
                    ty[(k - 1.0_f64) as usize].clone()
                } else {
                    (sos).clone()
                };
                let after = if (j < len) {
                    ty[j as usize].clone()
                } else {
                    (eos).clone()
                };
                if (before == "EN") || (after == "EN") {
                    {
                        let mut m = k;
                        while (m < j) {
                            {
                                let __flight_index = (m) as usize;
                                let __flight_value = "EN".to_owned();
                                if __flight_index == ty.len() {
                                    ty.push(__flight_value);
                                } else {
                                    ty[__flight_index] = __flight_value;
                                }
                            };
                            {
                                m += 1.0;
                                m
                            };
                        }
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
        let mut k = 0.0_f64;
        while (k < len) {
            if ((ty[k as usize].clone() == "ES") || (ty[k as usize].clone() == "ET"))
                || (ty[k as usize].clone() == "CS")
            {
                {
                    let __flight_index = (k) as usize;
                    let __flight_value = "ON".to_owned();
                    if __flight_index == ty.len() {
                        ty.push(__flight_value);
                    } else {
                        ty[__flight_index] = __flight_value;
                    }
                };
            }
            {
                k += 1.0;
                k
            };
        }
    }
    strong = (sos).clone();
    {
        let mut k = 0.0_f64;
        while (k < len) {
            let c = ty[k as usize].clone();
            if ((c).clone() == "L") || ((c).clone() == "R") {
                strong = (c).clone();
            } else {
                if ((c).clone() == "EN") && (strong == "L") {
                    {
                        let __flight_index = (k) as usize;
                        let __flight_value = "L".to_owned();
                        if __flight_index == ty.len() {
                            ty.push(__flight_value);
                        } else {
                            ty[__flight_index] = __flight_value;
                        }
                    };
                }
            }
            {
                k += 1.0;
                k
            };
        }
    }
    let embedding_dir: BidiClass = if ((seq_level % 2.0_f64) == 1.0_f64) {
        "R".to_owned()
    } else {
        "L".to_owned()
    };
    {
        let mut k = 0.0_f64;
        while (k < len) {
            if is_neutral_or_isolate(ty[k as usize].clone()) {
                let mut j = k;
                while (j < len) && (is_neutral_or_isolate(ty[j as usize].clone())) {
                    {
                        j += 1.0;
                        j
                    };
                }
                let before = if (k > 0.0_f64) {
                    neutral_direction(ty[(k - 1.0_f64) as usize].clone())
                } else {
                    (sos).clone()
                };
                let after = if (j < len) {
                    neutral_direction(ty[j as usize].clone())
                } else {
                    (eos).clone()
                };
                let resolved = if ((before).clone() == after) {
                    (before).clone()
                } else {
                    (embedding_dir).clone()
                };
                {
                    let mut m = k;
                    while (m < j) {
                        {
                            let __flight_index = (m) as usize;
                            let __flight_value = (resolved).clone();
                            if __flight_index == ty.len() {
                                ty.push(__flight_value);
                            } else {
                                ty[__flight_index] = __flight_value;
                            }
                        };
                        {
                            m += 1.0;
                            m
                        };
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
    let even = ((seq_level % 2.0_f64) == 0.0_f64);
    {
        let mut k = 0.0_f64;
        while (k < len) {
            let c = ty[k as usize].clone();
            let mut lvl = seq_level;
            if even {
                if (c == "R") {
                    lvl = (seq_level + 1.0_f64);
                } else {
                    if (c == "AN") || (c == "EN") {
                        lvl = (seq_level + 2.0_f64);
                    }
                }
            } else {
                if ((c == "L") || (c == "EN")) || (c == "AN") {
                    lvl = (seq_level + 1.0_f64);
                }
            }
            {
                let __flight_index = (sequence[k as usize].clone()) as usize;
                let __flight_value = lvl;
                if __flight_index == level_array.len() {
                    level_array.push(__flight_value);
                } else {
                    level_array[__flight_index] = __flight_value;
                }
            };
            {
                k += 1.0;
                k
            };
        }
    }
}

// Source: upstream/packages/textbidi/src/resolveBidiLevels.ts:396 (sha256:976691bcce6a7b809f105f3cb98a2814fcb05320dc2fb236053254279b3049a8)
fn apply_line_reset(
    original: &Vec<BidiClass>,
    level_array: &mut Vec<f64>,
    paragraph_level: f64,
) -> () {
    let length = (original.len() as f64);
    {
        let mut i = 0.0_f64;
        while (i < length) {
            let t = original[i as usize].clone();
            if (t == "B") || (t == "S") {
                {
                    let __flight_index = (i) as usize;
                    let __flight_value = paragraph_level;
                    if __flight_index == level_array.len() {
                        level_array.push(__flight_value);
                    } else {
                        level_array[__flight_index] = __flight_value;
                    }
                };
                {
                    let mut j = (i - 1.0_f64);
                    while (j >= 0.0_f64) && (is_reset_type(original[j as usize].clone())) {
                        {
                            let __flight_index = (j) as usize;
                            let __flight_value = paragraph_level;
                            if __flight_index == level_array.len() {
                                level_array.push(__flight_value);
                            } else {
                                level_array[__flight_index] = __flight_value;
                            }
                        };
                        {
                            j -= 1.0;
                            j
                        };
                    }
                }
            }
            {
                i += 1.0;
                i
            };
        }
    }
    {
        let mut j = (length - 1.0_f64);
        while (j >= 0.0_f64) && (is_reset_type(original[j as usize].clone())) {
            {
                let __flight_index = (j) as usize;
                let __flight_value = paragraph_level;
                if __flight_index == level_array.len() {
                    level_array.push(__flight_value);
                } else {
                    level_array[__flight_index] = __flight_value;
                }
            };
            {
                j -= 1.0;
                j
            };
        }
    }
}

// Source: upstream/packages/textbidi/src/resolveBidiLevels.ts:409 (sha256:9a2953e86e5d7423b133ec2dd4604a5c4882107ada938fa33f767f6f09a62c2e)
fn is_neutral_or_isolate(t: BidiClass) -> bool {
    return (((((((t == "B") || (t == "S")) || (t == "WS")) || (t == "ON")) || (t == "FSI"))
        || (t == "LRI"))
        || (t == "RLI"))
        || (t == "PDI");
}

// Source: upstream/packages/textbidi/src/resolveBidiLevels.ts:415 (sha256:f6998b0c290e2f3f3652d1e92f530f560c3659d404a860c5865389303a874a84)
fn is_reset_type(t: BidiClass) -> bool {
    return ((((((((((t == "WS") || (t == "LRI")) || (t == "RLI")) || (t == "FSI"))
        || (t == "PDI"))
        || (t == "LRE"))
        || (t == "RLE"))
        || (t == "LRO"))
        || (t == "RLO"))
        || (t == "PDF"))
        || (t == "BN");
}

// Source: upstream/packages/textbidi/src/resolveBidiLevels.ts:433 (sha256:f6671c440a296df5e2d401cc2d1db9a6972ecd2468ec874a290f75ec19db0b78)
fn neutral_direction(t: BidiClass) -> String {
    return if (t == "L") {
        "L".to_owned()
    } else {
        "R".to_owned()
    };
}

// Source: upstream/packages/textbidi/src/resolveBidiLevels.ts:438 (sha256:156a19df64b8916c5a4606f79d87089f011d31d0bd9ef5ef110b67ad4bf09fc1)
fn next_even(level: f64) -> f64 {
    return (__flight_js_to_i32((level + 2.0_f64))
        & __flight_js_to_i32((!__flight_js_to_i32(1.0_f64)) as f64)) as f64;
}

// Source: upstream/packages/textbidi/src/resolveBidiLevels.ts:443 (sha256:fb3af4f5d8c26d90b7d3e3936d855f595067be72016359059516acfc60e69eb3)
fn next_odd(level: f64) -> f64 {
    return (__flight_js_to_i32((level + 1.0_f64)) | __flight_js_to_i32(1.0_f64)) as f64;
}
