// @generated from upstream/packages/image-codec/src/encodeImage.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::{
    DecodedImage, HostImageEncodeCapabilities, HostImageEncodeFormatCapability, ImageEncodeOptions,
};

// Source: upstream/packages/image-codec/src/encodeImage.ts:8 (sha256:fb9ae9aac43329c51af8d5b11cc5499be1837b555c6c04b381396cebe2b142c8)
pub fn encode_image(
    image_encode: HostImageEncodeCapabilities,
    image: DecodedImage,
    mime_type: String,
    options: Option<ImageEncodeOptions>,
) -> crate::FlightTask<Option<Vec<u8>>> {
    crate::FlightTask::start(
        async move {
            let slot = get_image_encode_slot(&image_encode, (mime_type).clone());
            if (slot).is_none() {
                return Ok(None);
            }
            return {
                let __flight_callback = (slot.as_ref().unwrap().encode).clone();
                let __flight_result =
                    __flight_callback.lock().unwrap()((image).clone(), (options).clone());
                __flight_result
            }
            .await;
        },
        crate::FlightTaskOrigin {
            package: "@flighthq/image-codec",
            source: "upstream/packages/image-codec/src/encodeImage.ts",
            line: 8_u32,
            column: 1_u32,
            lexical_path: "encodeImage",
            fingerprint: "sha256:fb9ae9aac43329c51af8d5b11cc5499be1837b555c6c04b381396cebe2b142c8",
        },
    )
}

// Source: upstream/packages/image-codec/src/encodeImage.ts:19 (sha256:b2ae81eca1cb7f1dc5a30663d8d90d643d32885732af15a2d7be9bf26ac070ba)
fn get_image_encode_slot(
    image_encode: &HostImageEncodeCapabilities,
    mime_type: String,
) -> Option<HostImageEncodeFormatCapability> {
    {
        let __switch_value = mime_type;
        let __flight_case = if __switch_value == "image/jpeg" {
            0_usize
        } else if __switch_value == "image/png" {
            1_usize
        } else if __switch_value == "image/webp" {
            2_usize
        } else {
            3_usize
        };
        '__flight_switch: {
            if __flight_case <= 0_usize {
                return (image_encode.jpeg).clone();
            }
            if __flight_case <= 1_usize {
                return (image_encode.png).clone();
            }
            if __flight_case <= 2_usize {
                return (image_encode.webp).clone();
            }
            if __flight_case <= 3_usize {
                return None;
            }
            unreachable!("exhaustive TypeScript switch completed without returning");
        }
    }
}
