// @generated from upstream/packages/image-codec/src/decodeImage.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use crate::detect_image_mime_type;
use flighthq_types::{
    DecodedImage, HostImageDecodeCapabilities, HostImageDecodeFormatCapability, ImageDecodeOptions,
};

// Source: upstream/packages/image-codec/src/decodeImage.ts:9 (sha256:ee16f7b77d8682f6117427c8828dbbca5018988bd3f52ccce40a669e2cbafcf7)
pub fn decode_image(
    image_decode: HostImageDecodeCapabilities,
    bytes: Vec<u8>,
    mime_type: Option<String>,
) -> crate::FlightTask<Option<DecodedImage>> {
    crate::FlightTask::start(
        async move {
            let slot = resolve_decode_slot(&image_decode, &bytes, ((mime_type).clone()).clone());
            if (slot).is_none() {
                return Ok(None);
            }
            return {
                let __flight_callback = (slot.as_ref().unwrap().decode).clone();
                let __flight_result = __flight_callback.lock().unwrap()((bytes).clone());
                __flight_result
            }
            .await;
        },
        crate::FlightTaskOrigin {
            package: "@flighthq/image-codec",
            source: "upstream/packages/image-codec/src/decodeImage.ts",
            line: 9_u32,
            column: 1_u32,
            lexical_path: "decodeImage",
            fingerprint: "sha256:ee16f7b77d8682f6117427c8828dbbca5018988bd3f52ccce40a669e2cbafcf7",
        },
    )
}

// Source: upstream/packages/image-codec/src/decodeImage.ts:19 (sha256:0b55038799087b98bd47afc4140ba6696df7ff5f16676ef5d63452d2725a334d)
#[derive(Clone, Default)]
struct DecodeImagePremultipliedRecord1 {
    __flight_identity: std::sync::Arc<()>,
    premultiply_alpha: bool,
}
impl PartialEq for DecodeImagePremultipliedRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn decode_image_premultiplied(
    image_decode: HostImageDecodeCapabilities,
    bytes: Vec<u8>,
    mime_type: Option<String>,
) -> crate::FlightTask<Option<DecodedImage>> {
    crate::FlightTask::start(
        async move {
            let slot = resolve_decode_slot(&image_decode, &bytes, ((mime_type).clone()).clone());
            if (slot).is_none() {
                return Ok(None);
            }
            return {
                let __flight_callback = (slot.as_ref().unwrap().decode).clone();
                let __flight_result = __flight_callback.lock().unwrap()(
                    (bytes).clone(),
                    Some(ImageDecodeOptions {
                        __flight_identity: std::sync::Arc::new(()),
                        premultiply_alpha: Some(true),
                    }),
                );
                __flight_result
            }
            .await;
        },
        crate::FlightTaskOrigin {
            package: "@flighthq/image-codec",
            source: "upstream/packages/image-codec/src/decodeImage.ts",
            line: 19_u32,
            column: 1_u32,
            lexical_path: "decodeImagePremultiplied",
            fingerprint: "sha256:0b55038799087b98bd47afc4140ba6696df7ff5f16676ef5d63452d2725a334d",
        },
    )
}

// Source: upstream/packages/image-codec/src/decodeImage.ts:29 (sha256:d5d14ffa831cbef26aeb412d9fc609ecd980688eaaf02316f89f115bcdc23f4a)
fn resolve_decode_slot(
    image_decode: &HostImageDecodeCapabilities,
    bytes: &Vec<u8>,
    mime_type: Option<String>,
) -> Option<HostImageDecodeFormatCapability> {
    let type_ = (mime_type).or(detect_image_mime_type(
        &(crate::FlightUnion2::<Vec<u8>, Vec<u8>>::A((*bytes).clone())),
    ));
    if (type_).is_none() {
        return None;
    }
    return get_image_decode_slot(image_decode, (type_.as_ref().unwrap()).clone());
}

// Source: upstream/packages/image-codec/src/decodeImage.ts:39 (sha256:e1b5f8342a50a598e1dffe58272c0c65bc81b0aca1cc9f6c8a45b214ecb28315)
fn get_image_decode_slot(
    image_decode: &HostImageDecodeCapabilities,
    mime_type: String,
) -> Option<HostImageDecodeFormatCapability> {
    {
        let __switch_value = mime_type;
        let __flight_case = if __switch_value == "image/avif" {
            0_usize
        } else if __switch_value == "image/bmp" {
            1_usize
        } else if __switch_value == "image/gif" {
            2_usize
        } else if __switch_value == "image/jpeg" {
            3_usize
        } else if __switch_value == "image/png" {
            4_usize
        } else if __switch_value == "image/webp" {
            5_usize
        } else {
            6_usize
        };
        '__flight_switch: {
            if __flight_case <= 0_usize {
                return (image_decode.avif).clone();
            }
            if __flight_case <= 1_usize {
                return (image_decode.bmp).clone();
            }
            if __flight_case <= 2_usize {
                return (image_decode.gif).clone();
            }
            if __flight_case <= 3_usize {
                return (image_decode.jpeg).clone();
            }
            if __flight_case <= 4_usize {
                return (image_decode.png).clone();
            }
            if __flight_case <= 5_usize {
                return (image_decode.webp).clone();
            }
            if __flight_case <= 6_usize {
                return None;
            }
            unreachable!("exhaustive TypeScript switch completed without returning");
        }
    }
}
