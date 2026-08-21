// @generated from upstream/packages/spritesheet/src/spritesheetValidation.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::SpritesheetData;
use flighthq_types::{
    Spritesheet, SpritesheetData, SpritesheetValidationDiagnostic, TextureAtlasRegion,
};

// Source: upstream/packages/spritesheet/src/spritesheetValidation.ts:10 (sha256:4e31626c24a112007a5a01ba739b08e9601a16e55b8aec8fb7fd8d96c7618763)
pub fn validate_spritesheet(
    spritesheet: &Spritesheet,
) -> Option<Vec<SpritesheetValidationDiagnostic>> {
    let mut diagnostics: Vec<SpritesheetValidationDiagnostic> = vec![];
    let atlas = (spritesheet.atlas).clone();
    if (atlas).is_some() {
        let region_ids: Vec<crate::OpaqueHostValue> = {
            let mut __flight_set = Vec::new();
            for __flight_value in ((atlas.as_ref().unwrap().regions).clone())
                .iter()
                .cloned()
                .map(|r: TextureAtlasRegion| -> crate::OpaqueHostValue {
                    {
                        let __flight_portable_source = r.id;
                        crate::FlightValue::Number(*(&__flight_portable_source) as f64)
                    }
                })
                .collect::<Vec<_>>()
            {
                if !__flight_set.contains(&__flight_value) {
                    __flight_set.push(__flight_value);
                }
            }
            __flight_set
        };
        {
            let mut fi = 0.0_f64;
            while (fi < (spritesheet.frames.len() as f64)) {
                if (!region_ids.iter().any(|item| {
                    item == &{
                        let __flight_portable_source = spritesheet.frames[fi as usize].id;
                        crate::FlightValue::Number(*(&__flight_portable_source) as f64)
                    }
                })) {
                    diagnostics.push(SpritesheetValidationDiagnostic {
            __flight_identity: std::sync::Arc::new(()),
            animation_name: None,
            frame_index: Some(fi),
            message: format!("Frame {} references atlas region id {} which does not exist in the atlas.", fi, spritesheet.frames[fi as usize].id),
            severity: "error".to_owned(),
          });
                }
                {
                    fi += 1.0;
                    fi
                };
            }
        }
    }
    for __iteration1 in (((spritesheet.animations).clone()).clone()).iter().cloned() {
        let anim_name = __iteration1.0.clone();
        let anim = __iteration1.1.clone();
        if ((anim.frames.len() as f64) == 0.0_f64) {
            diagnostics.push(SpritesheetValidationDiagnostic {
                __flight_identity: std::sync::Arc::new(()),
                animation_name: Some((anim_name).clone()),
                frame_index: None,
                message: format!("Animation \"{}\" has no frames.", (anim_name).clone()),
                severity: "warning".to_owned(),
            });
        }
        {
            let mut ai = 0.0_f64;
            while (ai < (anim.frames.len() as f64)) {
                let frame_ref = anim.frames[ai as usize].clone();
                if (frame_ref < 0.0_f64) || (frame_ref >= (spritesheet.frames.len() as f64)) {
                    diagnostics.push(SpritesheetValidationDiagnostic {
            __flight_identity: std::sync::Arc::new(()),
            animation_name: Some((anim_name).clone()),
            frame_index: Some(ai),
            message: format!("Animation \"{}\" references frame index {} which is out of range (sheet has {} frames).", (anim_name).clone(), frame_ref, (spritesheet.frames.len() as f64)),
            severity: "error".to_owned(),
          });
                }
                {
                    ai += 1.0;
                    ai
                };
            }
        }
    }
    return if ((diagnostics.len() as f64) > 0.0_f64) {
        Some((diagnostics).clone())
    } else {
        None
    };
}

// Source: upstream/packages/spritesheet/src/spritesheetValidation.ts:58 (sha256:c52e266dccb2d76c318e481dfd3d3ea290d8a39b90151c832d801adf956c901d)
pub fn validate_spritesheet_data(
    data: &SpritesheetData,
) -> Option<Vec<SpritesheetValidationDiagnostic>> {
    let mut diagnostics: Vec<SpritesheetValidationDiagnostic> = vec![];
    let mut frame_name_set: Vec<String> = Vec::new();
    for fd in (data.frames).iter().cloned() {
        if ((fd.name).clone() != "") {
            {
                let __flight_value = (fd.name).clone();
                if !frame_name_set.contains(&__flight_value) {
                    frame_name_set.push(__flight_value);
                }
            };
        }
    }
    for ad in (data.animations).iter().cloned() {
        if ((ad.frame_names.len() as f64) == 0.0_f64) {
            diagnostics.push(SpritesheetValidationDiagnostic {
        __flight_identity: std::sync::Arc::new(()),
        animation_name: Some((ad.name).clone()),
        frame_index: None,
        message: format!("Animation \"{}\" has no frameNames — all sheet frames will be used as the frame list.", (ad.name).clone()),
        severity: "warning".to_owned(),
      });
        } else {
            {
                let mut ai = 0.0_f64;
                while (ai < (ad.frame_names.len() as f64)) {
                    let fname = ad.frame_names[ai as usize].clone();
                    if (!frame_name_set.iter().any(|item| item == &(fname).clone())) {
                        diagnostics.push(SpritesheetValidationDiagnostic {
              __flight_identity: std::sync::Arc::new(()),
              animation_name: Some((ad.name).clone()),
              frame_index: Some(ai),
              message: format!("Animation \"{}\" references frame name \"{}\" which is not present in the data frame list.", (ad.name).clone(), (fname).clone()),
              severity: "error".to_owned(),
            });
                    }
                    {
                        ai += 1.0;
                        ai
                    };
                }
            }
            if (((ad.frame_durations).clone()).is_some())
                && ((ad.frame_durations.as_ref().unwrap().len() as f64)
                    != (ad.frame_names.len() as f64))
            {
                diagnostics.push(SpritesheetValidationDiagnostic {
          __flight_identity: std::sync::Arc::new(()),
          animation_name: Some((ad.name).clone()),
          frame_index: None,
          message: format!("Animation \"{}\" has {} frameDurations but {} frameNames — lengths should match.", (ad.name).clone(), (ad.frame_durations.as_ref().unwrap().len() as f64), (ad.frame_names.len() as f64)),
          severity: "warning".to_owned(),
        });
            }
        }
    }
    return if ((diagnostics.len() as f64) > 0.0_f64) {
        Some((diagnostics).clone())
    } else {
        None
    };
}
