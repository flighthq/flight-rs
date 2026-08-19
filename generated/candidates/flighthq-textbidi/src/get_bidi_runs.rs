// @generated from upstream/packages/textbidi/src/getBidiRuns.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::resolve_bidi_levels;
use flighthq_types::{BidiDirection, BidiRun};

// Source: upstream/packages/textbidi/src/getBidiRuns.ts:10 (sha256:9c1d0f7d5edf06e0ec4531595002ee0cc338d787af2be1d594bd8e4350a9840d)
pub fn get_bidi_runs(text: String, base_direction: BidiDirection) -> Vec<BidiRun> {
    let levels = resolve_bidi_levels((text).clone(), (base_direction).clone());
    let mut runs: Vec<BidiRun> = vec![];
    let length = (levels.len() as f64);
    let mut start = 0.0_f64;
    {
        let mut i = 1.0_f64;
        while (i <= length) {
            if (i == length) || ((levels[i as usize] as f64) != (levels[start as usize] as f64)) {
                let level = (levels[start as usize] as f64);
                runs.push(BidiRun {
                    __flight_identity: std::sync::Arc::new(()),
                    start: start,
                    end: i,
                    level: (level).clone(),
                    direction: if (((level).clone() % 2.0_f64) == 0.0_f64) {
                        "ltr".to_owned()
                    } else {
                        "rtl".to_owned()
                    },
                });
                start = i;
            }
            {
                i += 1.0;
                i
            };
        }
    }
    return runs;
}
