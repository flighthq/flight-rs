// @generated from upstream/packages/importdiagnostics/src/importDiagnosticText.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::ImportDiagnostic;

// Source: upstream/packages/importdiagnostics/src/importDiagnosticText.ts:10 (sha256:476f9fed9cbe921c1c7673fdb63251d6f2af576c062a18fb5f40d47637979ed6)
pub fn format_import_diagnostic(diagnostic: &ImportDiagnostic) -> String {
    let detail = (diagnostic.detail).clone();
    let kind = (diagnostic.kind).clone();
    let origin = (diagnostic.origin).clone();
    let severity = (diagnostic.severity).clone();
    let mut detail_text = "".to_owned();
    if (detail).is_some() {
        let keys = {
            let mut __flight_values = detail
                .as_ref()
                .unwrap()
                .iter()
                .map(|(entry_key, _)| entry_key.clone())
                .collect::<Vec<_>>();
            __flight_values.sort_by(|left, right| {
                left.partial_cmp(right).unwrap_or(std::cmp::Ordering::Equal)
            });
            __flight_values
        };
        for key in (keys).iter().cloned() {
            detail_text.push_str(
                &(format!(
                    " {}={}",
                    (key).clone(),
                    (detail
                        .as_ref()
                        .unwrap()
                        .iter()
                        .find(|(entry_key, _)| entry_key == &(key).clone())
                        .map(|(_, value)| value.clone())
                        .clone())
                    .as_ref()
                    .map_or_else(|| "undefined".to_owned(), |value| value.to_string())
                )),
            );
        }
    }
    return format!(
        "{} {}: {}{}",
        (severity).clone(),
        (origin).clone(),
        (kind).clone(),
        (detail_text).clone()
    );
}
