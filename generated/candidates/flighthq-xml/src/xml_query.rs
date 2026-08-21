// @generated from upstream/packages/xml/src/xmlQuery.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::XmlElement;

// Source: upstream/packages/xml/src/xmlQuery.ts:8 (sha256:04ad11a478cb8161a5f12d68585eee4444c7ff8c6375551503f78d84c8666389)
pub fn get_xml_element_attribute(element: &XmlElement, name: String) -> Option<String> {
    let value = element
        .attributes
        .iter()
        .find(|(entry_key, _)| entry_key == &(name).clone())
        .map(|(_, value)| value)
        .expect("TypeScript Record key was absent")
        .clone();
    return if (value).is_some() {
        Some((value).clone())
    } else {
        None
    };
}

// Source: upstream/packages/xml/src/xmlQuery.ts:15 (sha256:8e632156f6b6895e26bb60763596f61a051eebf21cbb055740fe7673e4fe5069)
pub fn get_xml_element_attribute_number(element: &XmlElement, name: String) -> Option<f64> {
    let value = element
        .attributes
        .iter()
        .find(|(entry_key, _)| entry_key == &(name).clone())
        .map(|(_, value)| value)
        .expect("TypeScript Record key was absent")
        .clone();
    if ((value).is_none()) || ((value).trim().to_owned() == "") {
        return None;
    }
    let parsed = number(value);
    return if (parsed).is_finite() {
        Some(parsed)
    } else {
        None
    };
}

// Source: upstream/packages/xml/src/xmlQuery.ts:23 (sha256:8fb8eff0f83e607f74211189fe3a029b061143e836f4057e17a1f1460265450a)
pub fn get_xml_element_child_by_name(element: &XmlElement, name: String) -> Option<XmlElement> {
    for child in ((element.children).clone()).iter().cloned() {
        if ((child.name).clone() == name) {
            return Some((child).clone());
        }
    }
    return None;
}

// Source: upstream/packages/xml/src/xmlQuery.ts:31 (sha256:9f29cf701e1e5345f5dc30d679c8e7f9324d37e68ef969b99908eb17205be038)
pub fn get_xml_element_children_by_name(element: &XmlElement, name: String) -> Vec<XmlElement> {
    return (element.children.filter)(std::sync::Arc::new(std::sync::Mutex::new(Box::new(
        move |child: crate::OpaqueHostValue| -> f64 {
            (crate::host_value::<String>("host.name") == name)
        },
    )
        as Box<dyn FnMut(crate::OpaqueHostValue) -> f64 + Send + 'static>)));
}
