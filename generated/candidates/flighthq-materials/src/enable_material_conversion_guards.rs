// @generated from upstream/packages/materials/src/enableMaterialConversionGuards.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::set_material_conversion_guard;
use flighthq_log::log_once;
use flighthq_types::{LogData, LogDataProvider, LogLevel, MaterialConversionExplanation};

// Source: upstream/packages/materials/src/enableMaterialConversionGuards.ts:8 (sha256:ee9882574c9a4013d1c41006607c8ac6b2f6316fd0b614c126440c1bf0b298cd)
pub fn disable_material_conversion_guards() -> () {
    set_material_conversion_guard(&(None));
}

// Source: upstream/packages/materials/src/enableMaterialConversionGuards.ts:19 (sha256:80f27dfc34ce1d57188bb0a6fda3a09a7558896128a5be1cd6c282d6215bea4c)
pub fn enable_material_conversion_guards() -> () {
    set_material_conversion_guard(&(warn_material_conversion_drop));
}

// Source: upstream/packages/materials/src/enableMaterialConversionGuards.ts:23 (sha256:081c3fd283d98cae2b6c5ed42becaa5f4c2c720ae1761fada1081ee55b91f844)
fn warn_material_conversion_drop(
    explanation: &MaterialConversionExplanation,
    conversion: String,
) -> () {
    let maps = ((explanation.dropped_maps).clone())
        .iter()
        .map(|value| value.to_string())
        .collect::<Vec<_>>()
        .join((", ".to_owned()).as_str());
    log_once(
        format!("materials:conversion-dropped-map:{}", (conversion).clone()),
        LogLevel::Warn,
        &(crate::FlightUnion2::<LogData, LogDataProvider>::A(crate::FlightUnion2::<
            String,
            Vec<(String, crate::FlightValue)>,
        >::B({
            let mut __flight_record = Vec::new();
            __flight_record.push(("message".to_owned(), if ((explanation.reason).clone()) == Some("incompatible-channel-semantics".to_owned()) { { let __flight_portable_source = format!("{} discarded {}: its channels pack different quantities than the target slot, so no assignment preserves meaning. Bake the texture into the target's own layout and pass it explicitly.", (conversion).clone(), (maps).clone()); crate::FlightValue::String((&__flight_portable_source).clone()) } } else { { let __flight_portable_source = format!("{} discarded {}: the target material model has no slot for that quantity. Supply the nearest target map explicitly if the surface needs it.", (conversion).clone(), (maps).clone()); crate::FlightValue::String((&__flight_portable_source).clone()) } }));
            __flight_record
        }))),
        Some(("materials".to_owned()).clone()),
    );
}
