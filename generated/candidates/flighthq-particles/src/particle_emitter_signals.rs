// @generated from upstream/packages/particles/src/particleEmitterSignals.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_signals::create_signal;
use flighthq_types::ParticleEmitterSignals;

// Source: upstream/packages/particles/src/particleEmitterSignals.ts:6 (sha256:d5f1a6c464d997adabd0656cc89fee92dcaa9d77243b0e8290a708584832fd24)
pub fn create_particle_emitter_signals() -> ParticleEmitterSignals {
    return ParticleEmitterSignals {
        __flight_identity: std::sync::Arc::new(()),
        on_emitter_complete: create_signal(),
        on_particle_death: create_signal(),
        on_particle_spawn: create_signal(),
    };
}

// Source: upstream/packages/particles/src/particleEmitterSignals.ts:24 (sha256:9859b11f2926c1647db71ff5bc9c951f09eca6550a6c2c2fee5c16c7bd8cc011)
pub fn enable_particle_emitter_signals(state: crate::OpaqueHostValue) -> ParticleEmitterSignals {
    let mut s = crate::host_value::<Vec<(crate::FlightSymbol, Option<ParticleEmitterSignals>)>>(
        "host.cast",
    );
    return {
        s.iter()
            .find(|(entry_key, _)| entry_key == &*SIGNALS_SLOT)
            .map(|(_, value)| value)
            .expect("TypeScript Record key was absent")?? = Some(create_particle_emitter_signals());
        s.iter()
            .find(|(entry_key, _)| entry_key == &*SIGNALS_SLOT)
            .map(|(_, value)| value)
            .expect("TypeScript Record key was absent")
    };
}

// Source: upstream/packages/particles/src/particleEmitterSignals.ts:31 (sha256:ee8c99b43ce7d0fa65da5a7f44898bc12bcf7c48597700f05104b381312832d0)
pub fn get_particle_emitter_signals(
    state: crate::OpaqueHostValue,
) -> Option<ParticleEmitterSignals> {
    return crate::host_value::<crate::OpaqueHostValue>("host.index");
}

// Source: upstream/packages/particles/src/particleEmitterSignals.ts:37 (sha256:96e83ae56c18de3bf5fd3e8ba9ea7134eac3425cf70c7de9f9fffe0650adb77d)
static SIGNALS_SLOT: std::sync::LazyLock<crate::FlightSymbol> =
    std::sync::LazyLock::new(|| crate::FlightSymbol::new());
