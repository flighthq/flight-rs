// @generated from upstream/packages/registry-codegen/src/registryCodegen.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_registry_catalog::find_registry_catalog_entries;
use flighthq_types::{
    RegistryCatalog, RegistryCatalogEntry, RegistryCodegenPlan, Requirement, RequirementSet,
};

// Source: upstream/packages/registry-codegen/src/registryCodegen.ts:12 (sha256:f9503f0fab22fb556387b920320672072368fc98f8eac0a39ad4e4e7908b65dc)
pub fn create_registry_codegen_plan(
    catalog: &RegistryCatalog,
    requirements: &RequirementSet,
    backend: String,
) -> RegistryCodegenPlan {
    let mut entries: Vec<RegistryCatalogEntry> = vec![];
    let mut unresolved: Vec<Requirement> = vec![];
    let mut seen: Vec<String> = Vec::new();
    for requirement in ((requirements.requirements).clone()).iter().cloned() {
        let identity = requirement_identity(&requirement);
        if seen.iter().any(|item| item == &(identity).clone()) {
            continue;
        }
        {
            let __flight_value = (identity).clone();
            if !seen.contains(&__flight_value) {
                seen.push(__flight_value);
            }
        };
        let matches = find_registry_catalog_entries(
            catalog,
            (backend).clone(),
            (requirement.facet).clone(),
            (requirement.key).clone(),
        );
        if ((matches.len() as f64) == 0.0_f64) {
            unresolved.push(Requirement {
                __flight_identity: std::sync::Arc::new(()),
                facet: (requirement.facet).clone(),
                key: (requirement.key).clone(),
            });
        } else {
            {
                entries.extend(((matches).clone()).iter().cloned());
                entries.len() as f64
            };
        }
    }
    return RegistryCodegenPlan {
        __flight_identity: std::sync::Arc::new(()),
        backend: (backend).clone(),
        entries: (entries).clone(),
        unresolved: (unresolved).clone(),
    };
}

// Source: upstream/packages/registry-codegen/src/registryCodegen.ts:37 (sha256:97cf5b63c838239b51a62d055fc21c4f8da65b3bf87d6c33a99f626773faf06a)
fn requirement_identity(requirement: &Requirement) -> String {
    return format!(
        "{}\u{0000}{}",
        (requirement.facet).clone(),
        (requirement.key).clone()
    );
}
