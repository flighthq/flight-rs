// @generated from upstream/packages/importdiagnostics/src/importDiagnosticCollector.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::{ImportDiagnostic, ImportDiagnosticSeverity};

// Source: upstream/packages/importdiagnostics/src/importDiagnosticCollector.ts:18 (sha256:2481e49965b60d65cfcd57d00ac9899c8dbcd9b2fb1ad9c572bafeac06284747)
pub fn collect_import_diagnostics(
    run: &mut impl FnMut(Vec<ImportDiagnostic>) -> (),
) -> Vec<ImportDiagnostic> {
    let diagnostics: Vec<ImportDiagnostic> = vec![];
    run((diagnostics).clone());
    return diagnostics;
}

// Source: upstream/packages/importdiagnostics/src/importDiagnosticCollector.ts:36 (sha256:cc03bd6618c3b38da48611ce37104e816d07c7447c267cddcc109075c3ddea8b)
pub fn report_import_diagnostic(
    sink: &mut Option<Vec<ImportDiagnostic>>,
    severity: ImportDiagnosticSeverity,
    kind: String,
    origin: String,
    detail: Option<
        Vec<(
            String,
            crate::FlightUnion2<bool, crate::FlightUnion2<f64, String>>,
        )>,
    >,
) -> () {
    if (sink).is_none() {
        return;
    }
    sink.as_mut().unwrap().push(ImportDiagnostic {
        __flight_identity: std::sync::Arc::new(()),
        detail: Some((detail).clone().unwrap()),
        kind: (kind).clone(),
        origin: (origin).clone(),
        severity: (severity).clone(),
    });
}
