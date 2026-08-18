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
    let mut detail_text = "";
    if (detail).is_some() {
        let keys = (crate::host_value::<()>("host.keys").sort)();
        for key in (keys).iter().cloned() {
            detail_text += format!(
                " {}={}",
                key,
                detail
                    .as_ref()
                    .unwrap()
                    .iter()
                    .find(|(key, _)| key == &key)
                    .map(|(_, value)| value)
                    .expect("TypeScript Record key was absent")
                    .clone()
            );
        }
    }
    return format!("{} {}: {}{}", severity, origin, kind, detail_text);
}
