// @generated from upstream/packages/image-codec/src/detectImageMimeType.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/image-codec/src/detectImageMimeType.ts:4 (sha256:4c84a6c3e49e993a18e486f13f1be6bb995cf3dc4da5fe9c1c2c9153421dde59)
pub fn detect_image_mime_type(data: &crate::FlightUnion2<Vec<u8>, Vec<u8>>) -> Option<String> {
    let b = if false {
        (*data).clone()
    } else {
        crate::FlightUnion2::<Vec<u8>, Vec<u8>>::A(vec![0_u8; (data) as usize])
    };
    if (b.byte_length < 4.0_f64) {
        return None;
    }
    if (((b[0.0_f64 as usize].clone() == 137.0_f64) && (b[1.0_f64 as usize].clone() == 80.0_f64))
        && (b[2.0_f64 as usize].clone() == 78.0_f64))
        && (b[3.0_f64 as usize].clone() == 71.0_f64)
    {
        return Some("image/png".to_owned());
    }
    if ((b[0.0_f64 as usize].clone() == 255.0_f64) && (b[1.0_f64 as usize].clone() == 216.0_f64))
        && (b[2.0_f64 as usize].clone() == 255.0_f64)
    {
        return Some("image/jpeg".to_owned());
    }
    if (((b[0.0_f64 as usize].clone() == 71.0_f64) && (b[1.0_f64 as usize].clone() == 73.0_f64))
        && (b[2.0_f64 as usize].clone() == 70.0_f64))
        && (b[3.0_f64 as usize].clone() == 56.0_f64)
    {
        return Some("image/gif".to_owned());
    }
    if ((((((((b.byte_length >= 12.0_f64) && (b[0.0_f64 as usize].clone() == 82.0_f64))
        && (b[1.0_f64 as usize].clone() == 73.0_f64))
        && (b[2.0_f64 as usize].clone() == 70.0_f64))
        && (b[3.0_f64 as usize].clone() == 70.0_f64))
        && (b[8.0_f64 as usize].clone() == 87.0_f64))
        && (b[9.0_f64 as usize].clone() == 69.0_f64))
        && (b[10.0_f64 as usize].clone() == 66.0_f64))
        && (b[11.0_f64 as usize].clone() == 80.0_f64)
    {
        return Some("image/webp".to_owned());
    }
    if is_avif_file_type_box(&b) {
        return Some("image/avif".to_owned());
    }
    if (((b[0.0_f64 as usize].clone() == 0.0_f64) && (b[1.0_f64 as usize].clone() == 0.0_f64))
        && (b[2.0_f64 as usize].clone() == 1.0_f64))
        && (b[3.0_f64 as usize].clone() == 0.0_f64)
    {
        return Some("image/x-icon".to_owned());
    }
    if ((((b[0.0_f64 as usize].clone() == 73.0_f64) && (b[1.0_f64 as usize].clone() == 73.0_f64))
        && (b[2.0_f64 as usize].clone() == 42.0_f64))
        && (b[3.0_f64 as usize].clone() == 0.0_f64))
        || ((((b[0.0_f64 as usize].clone() == 77.0_f64)
            && (b[1.0_f64 as usize].clone() == 77.0_f64))
            && (b[2.0_f64 as usize].clone() == 0.0_f64))
            && (b[3.0_f64 as usize].clone() == 42.0_f64))
    {
        return Some("image/tiff".to_owned());
    }
    if (b[0.0_f64 as usize].clone() == 66.0_f64) && (b[1.0_f64 as usize].clone() == 77.0_f64) {
        return Some("image/bmp".to_owned());
    }
    return None;
}

// Source: upstream/packages/image-codec/src/detectImageMimeType.ts:50 (sha256:a0225153759224401e5b923077ed998e21dbdbb36677832a60b34533d91ec1f1)
fn is_avif_file_type_box(bytes: &Vec<u8>) -> bool {
    if ((((bytes.byte_length < 16.0_f64) || ((bytes[4.0_f64 as usize] as f64) != 102.0_f64))
        || ((bytes[5.0_f64 as usize] as f64) != 116.0_f64))
        || ((bytes[6.0_f64 as usize] as f64) != 121.0_f64))
        || ((bytes[7.0_f64 as usize] as f64) != 112.0_f64)
    {
        return false;
    }
    let box_size = (((((bytes[0.0_f64 as usize] as f64) * 16777216.0_f64)
        + ((bytes[1.0_f64 as usize] as f64) * 65536.0_f64))
        + ((bytes[2.0_f64 as usize] as f64) * 256.0_f64))
        + (bytes[3.0_f64 as usize] as f64));
    if ((box_size < 16.0_f64) || (box_size > bytes.byte_length))
        || (((box_size - 16.0_f64) % 4.0_f64) != 0.0_f64)
    {
        return false;
    }
    if is_avif_brand(bytes, 8.0_f64) {
        return true;
    }
    {
        let mut offset = 16.0_f64;
        while (offset < box_size) {
            if is_avif_brand(bytes, offset) {
                return true;
            }
            {
                offset += 4.0_f64;
                offset.clone()
            };
        }
    }
    return false;
}

// Source: upstream/packages/image-codec/src/detectImageMimeType.ts:63 (sha256:5433917a6f92a04792819138ea57214780771070b50b25e5d61dfbd23dfa50da)
fn is_avif_brand(bytes: &Vec<u8>, offset: f64) -> bool {
    return ((((bytes[offset as usize] as f64) == 97.0_f64)
        && ((bytes[(offset + 1.0_f64) as usize] as f64) == 118.0_f64))
        && ((bytes[(offset + 2.0_f64) as usize] as f64) == 105.0_f64))
        && (((bytes[(offset + 3.0_f64) as usize] as f64) == 102.0_f64)
            || ((bytes[(offset + 3.0_f64) as usize] as f64) == 115.0_f64));
}
