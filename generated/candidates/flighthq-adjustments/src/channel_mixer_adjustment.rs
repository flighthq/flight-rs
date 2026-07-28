// @generated from upstream/packages/adjustments/src/channelMixerAdjustment.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::create_channel_mixer_color_matrix;
use flighthq_types::ChannelMixerAdjustment;

#[derive(Clone)]
pub struct FlightOmitRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub matrix: Vec<f64>,
}
impl PartialEq for FlightOmitRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/adjustments/src/channelMixerAdjustment.ts:9 (sha256:9cc9895ee668e174188dd1fa6483a16626d324d8d10b009d95cba5b40e1da4a8)
pub fn create_channel_mixer_adjustment(
    options: Option<FlightOmitRecord1>,
) -> ChannelMixerAdjustment {
    let options = options.unwrap_or(FlightOmitRecord1 {
        __flight_identity: std::sync::Arc::new(()),
        matrix: ((*IDENTITY_CHANNEL_MIXER).clone()).clone(),
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
        let __flight_value = ({
            let __flight_callback = (m).clone();
            let __flight_result = __flight_callback.lock().unwrap()(3.0_f64);
            __flight_result
        } * 255.0_f64);
        if __flight_index == color_matrix.len() {
            color_matrix.push(__flight_value);
        } else {
            color_matrix[__flight_index] = __flight_value;
        }
    };
    {
        let __flight_index = (9.0_f64) as usize;
        let __flight_value = ({
            let __flight_callback = (m).clone();
            let __flight_result = __flight_callback.lock().unwrap()(7.0_f64);
            __flight_result
        } * 255.0_f64);
        if __flight_index == color_matrix.len() {
            color_matrix.push(__flight_value);
        } else {
            color_matrix[__flight_index] = __flight_value;
        }
    };
    {
        let __flight_index = (14.0_f64) as usize;
        let __flight_value = ({
            let __flight_callback = (m).clone();
            let __flight_result = __flight_callback.lock().unwrap()(11.0_f64);
            __flight_result
        } * 255.0_f64);
        if __flight_index == color_matrix.len() {
            color_matrix.push(__flight_value);
        } else {
            color_matrix[__flight_index] = __flight_value;
        }
    };
    return {
        let __flight_spread_1 = options;
        ChannelMixerAdjustment {
            __flight_identity: std::sync::Arc::new(()),
            kind: "ChannelMixerAdjustment".to_owned(),
            color_matrix: (color_matrix).clone(),
            matrix: (matrix).clone(),
        }
    };
}

// Source: upstream/packages/adjustments/src/channelMixerAdjustment.ts:21 (sha256:4cd14e404ff62b7643db57e9fab64819e517471926cb375ba34ec695c8f6e0a4)
static IDENTITY_CHANNEL_MIXER: std::sync::LazyLock<Vec<f64>> = std::sync::LazyLock::new(|| {
    vec![
        1.0_f64, 0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64, 1.0_f64, 0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64,
        1.0_f64, 0.0_f64,
    ]
});
