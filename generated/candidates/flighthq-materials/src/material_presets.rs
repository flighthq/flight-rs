// @generated from upstream/packages/materials/src/materialPresets.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{create_standard_pbr_material, create_transmission_volume_pbr_material};
use flighthq_types::{StandardPbrMaterial, TransmissionVolumePbrMaterial};

// Source: upstream/packages/materials/src/materialPresets.ts:15 (sha256:37bf6a37ffd0849714ad8499eb0e6b52ef5b1c6a8f276385256df8c490589610)
pub fn create_aluminum_standard_pbr_material(
    opts: Option<StandardPbrMaterial>,
) -> StandardPbrMaterial {
    return create_standard_pbr_material(Some(StandardPbrMaterial {
        base_color: 2964369663.0_f64,
        metallic: 1.0_f64,
        roughness: 0.35_f64,
        ..((opts).clone().unwrap()).clone()
    }));
}

// Source: upstream/packages/materials/src/materialPresets.ts:26 (sha256:8fad8dc882e56f36710e1f987034293853e63b686144e0d61ab7b4efb621c0ff)
pub fn create_carbon_standard_pbr_material(
    opts: Option<StandardPbrMaterial>,
) -> StandardPbrMaterial {
    return create_standard_pbr_material(Some(StandardPbrMaterial {
        base_color: 437918463.0_f64,
        metallic: 0.0_f64,
        roughness: 0.95_f64,
        ..((opts).clone().unwrap()).clone()
    }));
}

// Source: upstream/packages/materials/src/materialPresets.ts:37 (sha256:ae18ae07c254633d04c45d00d3e8b9beff00cd33a8383f9361971a63951c0297)
pub fn create_glass_transmission_volume_pbr_material(
    opts: Option<TransmissionVolumePbrMaterial>,
) -> TransmissionVolumePbrMaterial {
    return create_transmission_volume_pbr_material(Some(TransmissionVolumePbrMaterial {
        ior: 1.5_f64,
        transmission: 1.0_f64,
        ..((opts).clone().unwrap()).clone()
    }));
}

// Source: upstream/packages/materials/src/materialPresets.ts:49 (sha256:d98f9f1d62d29f41e2d5d19de7b0cb28609ad353bb45184ca449a58d7af5d370)
pub fn create_gold_standard_pbr_material(opts: Option<StandardPbrMaterial>) -> StandardPbrMaterial {
    return create_standard_pbr_material(Some(StandardPbrMaterial {
        base_color: 4292280575.0_f64,
        metallic: 1.0_f64,
        roughness: 0.25_f64,
        ..((opts).clone().unwrap()).clone()
    }));
}

// Source: upstream/packages/materials/src/materialPresets.ts:60 (sha256:107a10d2a6ef94f08211dd15c88d8ce086e4f396393dea1ada79f9a53dbd4139)
pub fn create_iron_standard_pbr_material(opts: Option<StandardPbrMaterial>) -> StandardPbrMaterial {
    return create_standard_pbr_material(Some(StandardPbrMaterial {
        base_color: 1145324799.0_f64,
        metallic: 1.0_f64,
        roughness: 0.7_f64,
        ..((opts).clone().unwrap()).clone()
    }));
}

// Source: upstream/packages/materials/src/materialPresets.ts:71 (sha256:491e857dd109b8011c2343482b72a1e95b4b38e617401ed58fcf03677af9bc5a)
pub fn create_marble_standard_pbr_material(
    opts: Option<StandardPbrMaterial>,
) -> StandardPbrMaterial {
    return create_standard_pbr_material(Some(StandardPbrMaterial {
        base_color: 4126537215.0_f64,
        metallic: 0.0_f64,
        roughness: 0.05_f64,
        ..((opts).clone().unwrap()).clone()
    }));
}

// Source: upstream/packages/materials/src/materialPresets.ts:82 (sha256:870a311b88865e43c684dea86b257a880568246fe8825ab79dffa10ef7a3a382)
pub fn create_plastic_standard_pbr_material(
    opts: Option<StandardPbrMaterial>,
) -> StandardPbrMaterial {
    return create_standard_pbr_material(Some(StandardPbrMaterial {
        base_color: 4294967295.0_f64,
        metallic: 0.0_f64,
        roughness: 0.05_f64,
        ..((opts).clone().unwrap()).clone()
    }));
}

// Source: upstream/packages/materials/src/materialPresets.ts:93 (sha256:a04bd6f96f8726b3e73a294602398a742949366ced747c5ce3ab071bb02f2c2e)
pub fn create_rubber_standard_pbr_material(
    opts: Option<StandardPbrMaterial>,
) -> StandardPbrMaterial {
    return create_standard_pbr_material(Some(StandardPbrMaterial {
        base_color: 471604479.0_f64,
        metallic: 0.0_f64,
        roughness: 0.9_f64,
        ..((opts).clone().unwrap()).clone()
    }));
}

// Source: upstream/packages/materials/src/materialPresets.ts:104 (sha256:a57f595677317b19c0f1c4b13ec257dceb02b01df554c311e346a167e5377d8e)
pub fn create_silver_standard_pbr_material(
    opts: Option<StandardPbrMaterial>,
) -> StandardPbrMaterial {
    return create_standard_pbr_material(Some(StandardPbrMaterial {
        base_color: 3233857791.0_f64,
        metallic: 1.0_f64,
        roughness: 0.1_f64,
        ..((opts).clone().unwrap()).clone()
    }));
}

// Source: upstream/packages/materials/src/materialPresets.ts:115 (sha256:6530a0ced19c503c71c857238735c33f55a792f056c13bcb0d4542e845537d0b)
pub fn create_skin_standard_pbr_material(opts: Option<StandardPbrMaterial>) -> StandardPbrMaterial {
    return create_standard_pbr_material(Some(StandardPbrMaterial {
        base_color: 4291598847.0_f64,
        metallic: 0.0_f64,
        roughness: 0.4_f64,
        ..((opts).clone().unwrap()).clone()
    }));
}

// Source: upstream/packages/materials/src/materialPresets.ts:126 (sha256:246fb70543d8587a017230d1fc082db9d37112b929d9cc869aafb2533cd44e99)
pub fn create_wood_standard_pbr_material(opts: Option<StandardPbrMaterial>) -> StandardPbrMaterial {
    return create_standard_pbr_material(Some(StandardPbrMaterial {
        base_color: 2337942527.0_f64,
        metallic: 0.0_f64,
        roughness: 0.8_f64,
        ..((opts).clone().unwrap()).clone()
    }));
}
