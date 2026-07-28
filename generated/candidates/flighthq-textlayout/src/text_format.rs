// @generated from upstream/packages/textlayout/src/textFormat.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::TextFormat;

// Source: upstream/packages/textlayout/src/textFormat.ts:3 (sha256:bc2273901aa571ab4bb9f5b1257f804ff0afa0bf278741682baa1b055d182120)
const DEFAULT_SIZE: f64 = 12.0_f64;

// Source: upstream/packages/textlayout/src/textFormat.ts:5 (sha256:629c355a2e7c4d3b79ddad24e8f73d78088668b47050c4d68da23e1df35c0ce4)
pub fn get_text_format_ascent(format: &TextFormat) -> f64 {
    return (format.size).unwrap_or(DEFAULT_SIZE);
}

// Source: upstream/packages/textlayout/src/textFormat.ts:9 (sha256:7b01e5279cfc8d74a95443e4cb3aa846ff372f323d39c7a9b4f144de0b28add7)
pub fn get_text_format_descent(format: &TextFormat) -> f64 {
    return ((format.size).unwrap_or(DEFAULT_SIZE) * 0.185_f64);
}

// Source: upstream/packages/textlayout/src/textFormat.ts:13 (sha256:e50acb9a1425d3105dfad44e1056e30d2f56b393cb683e007ef54bcb48a9ae50)
pub fn get_text_format_height(format: &TextFormat) -> f64 {
    return ((get_text_format_ascent(format) + get_text_format_descent(format))
        + get_text_format_leading(format));
}

// Source: upstream/packages/textlayout/src/textFormat.ts:17 (sha256:cbc5276667f22a83c52bf4e83baca83f6075de053724856220f14cb0c34be025)
pub fn get_text_format_leading(format: &TextFormat) -> f64 {
    return (format.leading).unwrap_or(0.0_f64);
}

// Source: upstream/packages/textlayout/src/textFormat.ts:21 (sha256:f972a195a8c46cf4a243c213ddce0f368dc524f6a4f101216ad2607d3b23b0fa)
pub fn merge_text_format(base: &TextFormat, override_: &TextFormat) -> TextFormat {
    let mut result: TextFormat = (base).clone();
    for key in (crate::host_value::<Vec<TextFormat>>("host.keys"))
        .iter()
        .cloned()
    {
        let value = override_[key as usize].clone();
        if (value).is_some() {
            result
                .iter()
                .find(|(key, _)| key == &key)
                .map(|(_, value)| value)
                .expect("TypeScript Record key was absent") = value;
        }
    }
    return result;
}
