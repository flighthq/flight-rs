// @generated from upstream/packages/lighting/src/lightIntensity.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::{
    CANDELA_LIGHT_UNIT as candela_light_unit_constant,
    LUMEN_LIGHT_UNIT as lumen_light_unit_constant, LUX_LIGHT_UNIT as lux_light_unit_constant,
    LightUnit, UNITLESS_LIGHT_UNIT as unitless_light_unit_constant,
};

// Source: upstream/packages/lighting/src/lightIntensity.ts:10 (sha256:6eeec312bceb6df097071359ea6174e8ba85571f4f4c02a1d6e79ca34104f120)
pub fn apply_light_exposure(intensity: f64, ev: f64) -> f64 {
    return (intensity * (2.0_f64).powf(ev));
}

// Source: upstream/packages/lighting/src/lightIntensity.ts:19 (sha256:6e294cacdc2232b16f6fc0f43fbb6924e20395a7a680a1062447342a0d75d823)
pub fn convert_light_intensity(from_unit: LightUnit, to_unit: LightUnit, value: f64) -> f64 {
    return (get_light_linear_intensity((from_unit).clone(), value)
        / (LINEAR_PER_UNIT
            .iter()
            .find(|(entry_key, _)| entry_key == &(to_unit).clone())
            .map(|(_, value)| value.clone()))
        .expect("TypeScript Record key was absent"));
}

// Source: upstream/packages/lighting/src/lightIntensity.ts:37 (sha256:f93d1d294f55dca88f7debdac46ef17ba0732edb141357ddcd721b6c63fc321e)
pub fn get_light_linear_intensity(unit: LightUnit, value: f64) -> f64 {
    return (value
        * (LINEAR_PER_UNIT
            .iter()
            .find(|(entry_key, _)| entry_key == &(unit).clone())
            .map(|(_, value)| value.clone()))
        .expect("TypeScript Record key was absent"));
}

// Source: upstream/packages/lighting/src/lightIntensity.ts:45 (sha256:93584cd63cb189de22b320b5d69f3fde4af0fbea717fdaf14514adcd3febe91f)
const REFERENCE_PHOTOMETRIC_LEVEL: f64 = 100000.0_f64;

// Source: upstream/packages/lighting/src/lightIntensity.ts:46 (sha256:8d7d73c811dd1fa92829d144862cfc48d9f773ce3d8f69a8b3482d32ab353e8c)
static LINEAR_PER_UNIT: std::sync::LazyLock<Vec<(LightUnit, f64)>> =
    std::sync::LazyLock::new(|| {
        let mut __flight_record = Vec::new();
        __flight_record.push((
            (candela_light_unit_constant).to_owned(),
            (1.0_f64 / REFERENCE_PHOTOMETRIC_LEVEL),
        ));
        __flight_record.push((
            (lumen_light_unit_constant).to_owned(),
            (1.0_f64 / ((REFERENCE_PHOTOMETRIC_LEVEL * 4.0_f64) * std::f64::consts::PI)),
        ));
        __flight_record.push((
            (lux_light_unit_constant).to_owned(),
            (1.0_f64 / REFERENCE_PHOTOMETRIC_LEVEL),
        ));
        __flight_record.push(((unitless_light_unit_constant).to_owned(), 1.0_f64));
        __flight_record
    });
