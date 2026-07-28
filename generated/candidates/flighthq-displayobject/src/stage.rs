// @generated from upstream/packages/displayobject/src/stage.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::create_display_object;
use flighthq_node::{create_viewport, get_node_root, get_node_runtime};
use flighthq_signals::{create_signal, emit_signal};
use flighthq_types::{
    DisplayObject, DisplayObjectTraits, Stage, StageRuntime, StageSignals, Viewport,
};

// Source: upstream/packages/displayobject/src/stage.ts:20 (sha256:da2da113b8caf0d17a4879547f07ddebbab7c2bd57237a666358c0fb0e300e6f)
pub fn create_stage(obj: Option<Stage>) -> Stage {
    let root = create_display_object(None);
    let mut stage = create_viewport(Some(Viewport::<DisplayObjectTraits> {
        __flight_identity: std::sync::Arc::new(()),
        align: (obj.as_ref().map(|value| (value.align).clone())).unwrap(),
        root: root,
        scale_mode: (obj.as_ref().map(|value| (value.scale_mode).clone())).unwrap(),
    }));
    stage.color = obj.as_ref().and_then(|value| value.color);
    stage.stage_height = (obj.as_ref().map(|value| value.stage_height)).unwrap_or(550.0_f64);
    stage.stage_width = (obj.as_ref().map(|value| value.stage_width)).unwrap_or(400.0_f64);
    get_node_runtime(&root).stage = Some((stage).clone());
    return (stage).clone();
}

// Source: upstream/packages/displayobject/src/stage.ts:32 (sha256:d0afe852919bc94716fa0e218a45da19d3a7c50ecc4bfc997edc1c6d3da9b07b)
pub fn create_stage_runtime() -> StageRuntime {
    return StageRuntime {
        __flight_identity: std::sync::Arc::new(()),
        binding: None,
        stage_signals: None,
    };
}

// Source: upstream/packages/displayobject/src/stage.ts:39 (sha256:29e05d8466740288524a3ffc0101563bed174d016b3afbc49e86498748a5e9cb)
pub fn create_stage_signals() -> StageSignals {
    return StageSignals {
        __flight_identity: std::sync::Arc::new(()),
        on_fullscreen_changed: create_signal(),
        on_orientation_changed: create_signal(),
        on_resize: create_signal(),
    };
}

// Source: upstream/packages/displayobject/src/stage.ts:47 (sha256:8f44077bdba5077c06cdffbabfe373825cb9e73273f9b7977a8f024410ffad6a)
pub fn enable_stage_signals(source: &mut Stage) -> StageSignals {
    let mut runtime = ensure_stage_runtime(source);
    return {
        runtime.stage_signals?? = Some(create_stage_signals());
        runtime.stage_signals
    };
}

// Source: upstream/packages/displayobject/src/stage.ts:54 (sha256:17be1b0878e798f0a77f96d645f971f81534b961524296b58053d9c87deb44c3)
pub fn get_display_object_stage(source: &mut DisplayObject) -> Option<Stage> {
    let root = get_node_root(source);
    return (get_node_runtime(&root).stage).clone();
}

// Source: upstream/packages/displayobject/src/stage.ts:61 (sha256:0b9c2d84fc0cb931b337fa571702891c440a22703e738643ebb864e84e452cbf)
pub fn get_stage_runtime(source: &mut Stage) -> StageRuntime {
    return ensure_stage_runtime(&mut (*source).clone());
}

// Source: upstream/packages/displayobject/src/stage.ts:65 (sha256:d73d56adcdeef33c175e91201573449d725352252c6d658aaf412702cfb59586)
pub fn get_stage_signals(source: &Stage) -> Option<StageSignals> {
    let runtime = panic!("entity runtime storage requires the generated native entity trait");
    return runtime
        .as_ref()
        .and_then(|value| (value.stage_signals).clone());
}

// Source: upstream/packages/displayobject/src/stage.ts:70 (sha256:53e699b0818e59446648ed2b595e699ba69afbf416e121f633142b3ce6fb1270)
pub fn set_stage_size(source: &mut Stage, width: f64, height: f64) -> () {
    if ((source.stage_width == width) && (source.stage_height == height)) {
        return;
    }
    source.stage_width = width;
    source.stage_height = height;
    let runtime = panic!("entity runtime storage requires the generated native entity trait");
    if (runtime
        .as_ref()
        .and_then(|value| (value.stage_signals).clone()))
    .is_some()
    {
        emit_signal(
            (runtime
                .as_ref()
                .unwrap()
                .stage_signals
                .as_ref()
                .unwrap()
                .on_resize)
                .clone(),
            (),
        );
    }
}

// Source: upstream/packages/displayobject/src/stage.ts:78 (sha256:1a05e17c0525eeffdb91b14d2c230cc60f78361bc745327ce885abf240ec08ff)
fn ensure_stage_runtime(source: &mut Stage) -> StageRuntime {
    let existing = panic!("entity runtime storage requires the generated native entity trait");
    if (existing).is_some() {
        return (existing.as_ref().unwrap()).clone();
    }
    let runtime = create_stage_runtime();
    ();
    return (runtime).clone();
}
