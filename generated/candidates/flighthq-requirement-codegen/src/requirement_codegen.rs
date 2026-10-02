// @generated from upstream/packages/requirement-codegen/src/requirementCodegen.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_entity::{allocate_entity, finish_entity};
use flighthq_requirement_catalog::find_requirement_catalog_entries;
use flighthq_types::{
    EntityConstruction, Requirement, RequirementCatalog, RequirementCatalogEntry,
    RequirementDeclination, RequirementDisposition, RequirementSet,
};

#[derive(Clone, Default)]
pub struct SharedStructuralRecord1 {
    pub __flight_identity: std::sync::Arc<()>,
    pub backend: String,
    pub declined: Vec<RequirementDeclination>,
    pub entries: Vec<RequirementCatalogEntry>,
    pub unresolved: Vec<Requirement>,
}
impl PartialEq for SharedStructuralRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/requirement-codegen/src/requirementCodegen.ts:14 (sha256:69438484aecb1b42b3b87b69921696de13bb1285141a63a3adb1ea9c2b7d7cfd)
pub fn create_requirement_codegen_plan(
    catalog: &RequirementCatalog,
    requirements: &RequirementSet,
    backend: String,
) -> SharedStructuralRecord1 {
    let mut out = allocate_entity();
    initialize_requirement_codegen_plan((out).clone(), catalog, requirements, (backend).clone());
    return finish_entity((out).clone());
}

// Source: upstream/packages/requirement-codegen/src/requirementCodegen.ts:26 (sha256:6bab4e913bb6e5f820faf2b2c9e2a9e0fc561d7145d3dd9b30a012cea5a756a7)
pub fn initialize_requirement_codegen_plan(
    out: EntityConstruction<SharedStructuralRecord1>,
    catalog: &RequirementCatalog,
    requirements: &RequirementSet,
    backend: String,
) -> () {
    let mut entries: Vec<RequirementCatalogEntry> = vec![];
    let mut declined: Vec<RequirementDeclination> = vec![];
    let mut unresolved: Vec<Requirement> = vec![];
    let mut seen: Vec<String> = Vec::new();
    for requirement in (expand_requirements(catalog, &requirements.requirements))
        .iter()
        .cloned()
    {
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
        let matches = find_requirement_catalog_entries(
            catalog,
            (backend).clone(),
            (requirement.facet).clone(),
            (requirement.key).clone(),
        );
        if ((matches.len() as f64) > 0.0_f64) {
            {
                entries.extend(((matches).clone()).iter().cloned());
                entries.len() as f64
            };
            continue;
        }
        let disposition = (catalog.dispositions.as_ref().unwrap())
            .iter()
            .find(|value| {
                (|candidate: RequirementDisposition| -> bool {
                    (((candidate.backend).clone() == backend)
                        && ((candidate.facet).clone() == (requirement.facet).clone()))
                        && ((candidate.kind).clone() == (requirement.key).clone())
                })((*value).clone())
            })
            .cloned();
        if (disposition).is_none() {
            unresolved.push(Requirement {
                __flight_identity: std::sync::Arc::new(()),
                facet: (requirement.facet).clone(),
                key: (requirement.key).clone(),
            });
        } else {
            declined.push(RequirementDeclination {
                __flight_identity: std::sync::Arc::new(()),
                reason: (disposition.as_ref().unwrap().reason).clone(),
                requirement: Requirement {
                    __flight_identity: std::sync::Arc::new(()),
                    facet: (requirement.facet).clone(),
                    key: (requirement.key).clone(),
                },
            });
        }
    }
    crate::host_set("host.backend", backend);
    crate::host_set("host.declined", declined);
    crate::host_set("host.entries", entries);
    crate::host_set("host.unresolved", unresolved);
}

// Source: upstream/packages/requirement-codegen/src/requirementCodegen.ts:80 (sha256:49ccab921517535b774b254364f1486ea4c528eb05f3863895efc0a67dff6b57)
fn expand_requirements(
    catalog: &RequirementCatalog,
    requirements: &Vec<Requirement>,
) -> Vec<Requirement> {
    let translations = (catalog.translations).clone();
    if ((translations).is_none()) || ((translations.as_ref().unwrap().len() as f64) == 0.0_f64) {
        return requirements.clone();
    }
    let mut expanded: Vec<Requirement> = {
        let mut __flight_array = Vec::new();
        __flight_array.extend((requirements).iter().cloned());
        __flight_array
    };
    for requirement in (requirements).iter().cloned() {
        let namespace = if (requirement.key.includes)(".") {
            (requirement.key.slice)(0.0_f64, (requirement.key.index_of)("."))
        } else {
            None
        };
        for translation in (translations.as_ref().unwrap()).iter().cloned() {
            if ((translation.from.facet).clone() != (requirement.facet).clone()) {
                continue;
            }
            let matches = ((translation.from.key).clone() == (requirement.key).clone())
                || (((namespace).is_some()) && ((translation.from.key).clone() == namespace));
            if matches {
                {
                    expanded.extend(((translation.to).clone()).iter().cloned());
                    expanded.len() as f64
                };
            }
        }
    }
    return expanded;
}

// Source: upstream/packages/requirement-codegen/src/requirementCodegen.ts:99 (sha256:97cf5b63c838239b51a62d055fc21c4f8da65b3bf87d6c33a99f626773faf06a)
fn requirement_identity(requirement: &Requirement) -> String {
    return format!(
        "{}\u{0000}{}",
        (requirement.facet).clone(),
        (requirement.key).clone()
    );
}
