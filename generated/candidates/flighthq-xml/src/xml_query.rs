// @generated from upstream/packages/xml/src/xmlQuery.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::XmlElement;

#[inline]

fn __flight_number_from_string(value: &str) -> f64 {
    let value = value.trim();
    if value.is_empty() {
        return 0.0_f64;
    }
    match value {
        "Infinity" | "+Infinity" => return f64::INFINITY,
        "-Infinity" => return f64::NEG_INFINITY,
        _ => {}
    }
    let prefixed = if let Some(digits) = value
        .strip_prefix("0x")
        .or_else(|| value.strip_prefix("0X"))
    {
        Some((digits, 16_u32))
    } else if let Some(digits) = value
        .strip_prefix("0b")
        .or_else(|| value.strip_prefix("0B"))
    {
        Some((digits, 2_u32))
    } else {
        value
            .strip_prefix("0o")
            .or_else(|| value.strip_prefix("0O"))
            .map(|digits| (digits, 8_u32))
    };
    if let Some((digits, radix)) = prefixed {
        return u64::from_str_radix(digits, radix).map_or(f64::NAN, |number| number as f64);
    }
    value.parse::<f64>().unwrap_or(f64::NAN)
}

// Source: upstream/packages/xml/src/xmlQuery.ts:8 (sha256:04ad11a478cb8161a5f12d68585eee4444c7ff8c6375551503f78d84c8666389)
pub fn get_xml_element_attribute(element: &XmlElement, name: String) -> Option<String> {
    let value = element
        .attributes
        .iter()
        .find(|(entry_key, _)| entry_key == &(name).clone())
        .map(|(_, value)| value.clone())
        .clone();
    return if (value).is_some() {
        (value).clone()
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
        .map(|(_, value)| value.clone())
        .clone();
    if ((value).is_none()) || ((value.as_ref().unwrap()).trim().to_owned() == "") {
        return None;
    }
    let parsed = __flight_number_from_string(&((value.as_ref().unwrap()).clone()));
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
    return {
        let mut __flight_filter = |child: XmlElement| -> bool { ((child.name).clone() == name) };
        ((element.children).clone())
            .iter()
            .cloned()
            .filter(|value| __flight_filter(value.clone()))
            .collect::<Vec<_>>()
    };
}
