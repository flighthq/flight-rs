// @generated from upstream/packages/adjustments/src/channelMixerAdjustment.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::{create_channel_mixer_color_matrix, initialize_color_matrix_adjustment};
use flighthq_entity::{allocate_entity, finish_entity};
use flighthq_types::{
    ChannelMixerAdjustment, ColorBlindType, ColorScaleBiasLike, EntityConstruction,
};

#[derive(Clone, Default)]
pub struct FlightOmitRecord2968336371 {
    pub __flight_identity: std::sync::Arc<()>,
    pub intensity: Option<f64>,
    pub exposure: Option<f64>,
    pub color_scale_bias: ColorScaleBiasLike,
    pub type_: Option<ColorBlindType>,
    pub matrix: Vec<f64>,
    pub brightness: Option<f64>,
    pub contrast: Option<f64>,
}
impl PartialEq for FlightOmitRecord2968336371 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/adjustments/src/channelMixerAdjustment.ts:7 (sha256:f4a4e6380e73d07d42d3c892ead0ba42d8ed4fb745fe8196c6277206006abe7d)
pub fn create_channel_mixer_adjustment(
    options: Option<FlightOmitRecord2968336371>,
) -> ChannelMixerAdjustment {
    let options = options.unwrap_or(FlightOmitRecord2968336371 {
        __flight_identity: std::sync::Arc::new(()),
        matrix: ((*IDENTITY_CHANNEL_MIXER).clone()).clone(),
        intensity: None,
        exposure: None,
        color_scale_bias: Default::default(),
        type_: None,
        brightness: None,
        contrast: None,
    });
    let mut out = allocate_entity();
    initialize_channel_mixer_adjustment(
        (out).clone(),
        Some({
            let __flight_source = &((options).clone());
            FlightOmitRecord2968336371 {
                __flight_identity: std::sync::Arc::clone(&__flight_source.__flight_identity),
                intensity: __flight_source.intensity,
                exposure: __flight_source.exposure,
                color_scale_bias: (__flight_source.color_scale_bias).clone(),
                type_: (__flight_source.type_).clone(),
                matrix: (__flight_source.matrix).clone(),
                brightness: __flight_source.brightness,
                contrast: __flight_source.contrast,
            }
        }),
    );
    return finish_entity((out).clone());
}

// Source: upstream/packages/adjustments/src/channelMixerAdjustment.ts:21 (sha256:33ac6e036993c2ca0a54e55cf9ae6a3dc79848f5b4578abc8b8212abce7387ee)
pub fn initialize_channel_mixer_adjustment(
    out: EntityConstruction<ChannelMixerAdjustment>,
    options: Option<FlightOmitRecord2968336371>,
) -> () {
    let options = options.unwrap_or(FlightOmitRecord2968336371 {
        __flight_identity: std::sync::Arc::new(()),
        matrix: ((*IDENTITY_CHANNEL_MIXER).clone()).clone(),
        intensity: None,
        exposure: None,
        color_scale_bias: Default::default(),
        type_: None,
        brightness: None,
        contrast: None,
    });
    let matrix = (options.matrix).clone();
    let mut m: std::sync::Arc<std::sync::Mutex<Box<dyn FnMut(f64) -> f64 + Send + 'static>>> =
        std::sync::Arc::new(std::sync::Mutex::new(Box::new({
            let matrix = matrix.clone();
            move |i: f64| -> f64 { matrix[i as usize].clone() }
        })
            as Box<dyn FnMut(f64) -> f64 + Send + 'static>));
    let mut color_matrix = create_channel_mixer_color_matrix(
        &vec![
            {
                let __flight_callback = (m).clone();
                let __flight_result = __flight_callback.lock().unwrap()(0.0_f64);
                __flight_result
            },
            {
                let __flight_callback = (m).clone();
                let __flight_result = __flight_callback.lock().unwrap()(1.0_f64);
                __flight_result
            },
            {
                let __flight_callback = (m).clone();
                let __flight_result = __flight_callback.lock().unwrap()(2.0_f64);
                __flight_result
            },
        ],
        &vec![
            {
                let __flight_callback = (m).clone();
                let __flight_result = __flight_callback.lock().unwrap()(4.0_f64);
                __flight_result
            },
            {
                let __flight_callback = (m).clone();
                let __flight_result = __flight_callback.lock().unwrap()(5.0_f64);
                __flight_result
            },
            {
                let __flight_callback = (m).clone();
                let __flight_result = __flight_callback.lock().unwrap()(6.0_f64);
                __flight_result
            },
        ],
        &vec![
            {
                let __flight_callback = (m).clone();
                let __flight_result = __flight_callback.lock().unwrap()(8.0_f64);
                __flight_result
            },
            {
                let __flight_callback = (m).clone();
                let __flight_result = __flight_callback.lock().unwrap()(9.0_f64);
                __flight_result
            },
            {
                let __flight_callback = (m).clone();
                let __flight_result = __flight_callback.lock().unwrap()(10.0_f64);
                __flight_result
            },
        ],
    );
    {
        let __flight_index = (4.0_f64) as usize;
        let __flight_value = {
            let __flight_callback = (m).clone();
            let __flight_result = __flight_callback.lock().unwrap()(3.0_f64);
            __flight_result
        };
        if __flight_index == color_matrix.len() {
            color_matrix.push(__flight_value);
        } else {
            color_matrix[__flight_index] = __flight_value;
        }
    };
    {
        let __flight_index = (9.0_f64) as usize;
        let __flight_value = {
            let __flight_callback = (m).clone();
            let __flight_result = __flight_callback.lock().unwrap()(7.0_f64);
            __flight_result
        };
        if __flight_index == color_matrix.len() {
            color_matrix.push(__flight_value);
        } else {
            color_matrix[__flight_index] = __flight_value;
        }
    };
    {
        let __flight_index = (14.0_f64) as usize;
        let __flight_value = {
            let __flight_callback = (m).clone();
            let __flight_result = __flight_callback.lock().unwrap()(11.0_f64);
            __flight_result
        };
        if __flight_index == color_matrix.len() {
            color_matrix.push(__flight_value);
        } else {
            color_matrix[__flight_index] = __flight_value;
        }
    };
    initialize_color_matrix_adjustment(
        (out).clone(),
        "ChannelMixerAdjustment".to_owned(),
        &color_matrix,
    );
    crate::host_set("host.matrix", matrix);
}

// Source: upstream/packages/adjustments/src/channelMixerAdjustment.ts:37 (sha256:4cd14e404ff62b7643db57e9fab64819e517471926cb375ba34ec695c8f6e0a4)
static IDENTITY_CHANNEL_MIXER: std::sync::LazyLock<Vec<f64>> = std::sync::LazyLock::new(|| {
    vec![
        1.0_f64, 0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64, 1.0_f64, 0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64,
        1.0_f64, 0.0_f64,
    ]
});
