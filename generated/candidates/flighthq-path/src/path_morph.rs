// @generated from upstream/packages/path/src/pathMorph.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::build_path_morph;
use flighthq_types::Path;
pub use flighthq_types::PathMorph;

// Source: upstream/packages/path/src/pathMorph.ts:12 (sha256:06c1f4dce3d6f03861c8d309eed3638972cef24d2a7854067cb9b876d761798f)
pub fn create_path_morph(start: &Path, end: &Path) -> Option<PathMorph> {
    return (build_path_morph(start, end).morph).clone();
}

// Source: upstream/packages/path/src/pathMorph.ts:19 (sha256:437fec28f463af3f321bda2c7676c2e21d16f5d373f1c5e9879341d0e4b33cd4)
pub fn sample_path_morph(out: &mut Path, morph: &PathMorph, progress: f64) -> () {
    out.commands
        .truncate((morph.commands.len() as f64) as usize);
    {
        let mut i = 0.0_f64;
        while (i < (morph.commands.len() as f64)) {
            {
                let __flight_index = (i) as usize;
                let __flight_value = morph.commands[i as usize].clone();
                if __flight_index == out.commands.len() {
                    out.commands.push(__flight_value);
                } else {
                    out.commands[__flight_index] = __flight_value;
                }
            };
            {
                i += 1.0;
                i
            };
        }
    }
    out.data.truncate((morph.start_data.len() as f64) as usize);
    if (progress == 0.0_f64) || (progress == 1.0_f64) {
        let endpoint = if (progress == 0.0_f64) {
            (morph.start_data).clone()
        } else {
            (morph.end_data).clone()
        };
        {
            let mut i = 0.0_f64;
            while (i < (endpoint.len() as f64)) {
                {
                    let __flight_index = (i) as usize;
                    let __flight_value = endpoint[i as usize].clone();
                    if __flight_index == out.data.len() {
                        out.data.push(__flight_value);
                    } else {
                        out.data[__flight_index] = __flight_value;
                    }
                };
                {
                    i += 1.0;
                    i
                };
            }
        }
        out.winding = (morph.winding).clone();
        return;
    }
    {
        let mut i = 0.0_f64;
        while (i < (morph.start_data.len() as f64)) {
            let start = morph.start_data[i as usize].clone();
            let end = morph.end_data[i as usize].clone();
            {
                let __flight_index = (i) as usize;
                let __flight_value = (start + ((end - start) * progress));
                if __flight_index == out.data.len() {
                    out.data.push(__flight_value);
                } else {
                    out.data[__flight_index] = __flight_value;
                }
            };
            {
                i += 1.0;
                i
            };
        }
    }
    out.winding = (morph.winding).clone();
}
