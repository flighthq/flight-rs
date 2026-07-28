// @generated from upstream/packages/effects/src/contactShadowsEffect.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::ContactShadowsEffect;

// Source: upstream/packages/effects/src/contactShadowsEffect.ts:3 (sha256:4bc30aa21b51ac6d1144ef6e52410b9010dfd548130f08c88f4b071dbdaa3dd3)
#[derive(Clone)]
struct CreateContactShadowsEffectRecord1 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for CreateContactShadowsEffectRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn create_contact_shadows_effect(
    options: Option<ContactShadowsEffect>,
) -> ContactShadowsEffect {
    let options = options.unwrap_or(ContactShadowsEffect {
        __flight_identity: std::sync::Arc::new(()),
        distance: None,
        opacity: None,
        samples: None,
        smoothness: None,
    });
    return ContactShadowsEffect {
        kind: "ContactShadowsEffect".to_owned(),
        ..((options).clone()).clone()
    };
}
