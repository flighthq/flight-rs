// @generated from upstream/packages/xml/src/xmlParse.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::XmlElement;

#[inline]

fn __flight_string_index_of(value: &str, search: &str, position: f64) -> f64 {
    let value: Vec<u16> = value.encode_utf16().collect();
    let search: Vec<u16> = search.encode_utf16().collect();
    let start = if position.is_nan() || position <= 0.0_f64 {
        0_usize
    } else if position >= value.len() as f64 {
        value.len()
    } else {
        position.trunc() as usize
    };
    if search.is_empty() {
        return start as f64;
    }
    value[start..]
        .windows(search.len())
        .position(|window| window == search)
        .map_or(-1.0_f64, |index| (start + index) as f64)
}

#[inline]

fn __flight_string_slice(value: &str, start: f64, end: Option<f64>) -> String {
    let value: Vec<u16> = value.encode_utf16().collect();
    let length = value.len();
    let relative = |index: f64| -> usize {
        if index.is_nan() {
            0
        } else if index < 0.0_f64 {
            length.saturating_sub((-index.trunc()) as usize)
        } else {
            (index.trunc() as usize).min(length)
        }
    };
    let start = relative(start);
    let end = end.map_or(length, relative);
    String::from_utf16_lossy(&value[start..end.max(start)])
}

#[inline]

fn __flight_string_from_code_point(value: f64) -> String {
    assert!(
        value.is_finite()
            && value.fract() == 0.0_f64
            && (0.0_f64..=0x10FFFF_u32 as f64).contains(&value),
        "String.fromCodePoint received an invalid code point"
    );
    char::from_u32(value as u32)
        .expect("Rust strings cannot represent surrogate code points")
        .to_string()
}

// Source: upstream/packages/xml/src/xmlParse.ts:12 (sha256:5d3faebf7d0c254ea5522a2669360263c6aab9196a4015bcfba2079457bba066)
#[derive(Clone, Default)]
struct ParseXmlAttributesRecord1 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for ParseXmlAttributesRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn parse_xml_attributes(attrs: String) -> Vec<(String, String)> {
    let mut result: Vec<(String, String)> = {
        let mut __flight_record = Vec::new();
        __flight_record
    };
    let re = regex::RegexBuilder::new("([\\w:.-]+)\\s*=\\s*(?:\"([^\"]*)\"|'([^']*)')")
        .case_insensitive(false)
        .multi_line(false)
        .dot_matches_new_line(false)
        .build()
        .expect("upstream TypeScript regular expression must be valid Rust regex syntax");
    let mut m: Option<Vec<Option<String>>>;
    while ({
        m = {
            let __flight_regex = &(re);
            __flight_regex.captures(&((attrs).clone())).map(|captures| {
                (0..captures.len())
                    .map(|index| {
                        captures
                            .get(index)
                            .map(|matched| matched.as_str().to_owned())
                    })
                    .collect::<Vec<_>>()
            })
        };
        m.clone()
    })
    .is_some()
    {
        let attr_name = m.as_mut().unwrap()[1.0_f64 as usize].clone();
        let value = if (m.as_mut().unwrap()[2.0_f64 as usize].clone()).is_some() {
            m.as_mut().unwrap()[2.0_f64 as usize].clone()
        } else {
            Some(
                (m.as_mut().unwrap()[3.0_f64 as usize].clone())
                    .clone()
                    .unwrap_or("".to_owned()),
            )
        };
        {
            let __flight_key = (attr_name).clone().unwrap();
            let __flight_value = decode_xml_entities((((value).clone()).clone().unwrap()).clone());
            if let Some((_, value)) = result.iter_mut().find(|(key, _)| key == &__flight_key) {
                *value = __flight_value;
            } else {
                result.push((__flight_key, __flight_value));
            }
        };
    }
    return result;
}

// Source: upstream/packages/xml/src/xmlParse.ts:28 (sha256:36866d62641b276cbeb91da3908318c5be1c75084f808948d9c6608a6539fec5)
#[derive(Clone, Default)]
struct ParseXmlDocumentRecord1 {
    __flight_identity: std::sync::Arc<()>,
}
impl PartialEq for ParseXmlDocumentRecord1 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

#[derive(Clone, Default)]
struct ParseXmlDocumentRecord2 {
    __flight_identity: std::sync::Arc<()>,
    depth: f64,
    depth_exceeded: bool,
    pos: f64,
}
impl PartialEq for ParseXmlDocumentRecord2 {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

pub fn parse_xml_document(xml: String) -> Option<XmlElement> {
    let mut src = (regex::RegexBuilder::new("\\r\\n?")
        .case_insensitive(false)
        .multi_line(false)
        .dot_matches_new_line(false)
        .build()
        .expect("upstream TypeScript regular expression must be valid Rust regex syntax"))
    .replace_all(&(strip_xml_comments((xml).clone())), "\n".to_owned())
    .into_owned();
    let mut entities: Vec<(String, String)> = {
        let mut __flight_record = Vec::new();
        __flight_record
    };
    src = (strip_xml_doctypes(
        (regex::RegexBuilder::new("<\\?[\\s\\S]*?\\?>")
            .case_insensitive(false)
            .multi_line(false)
            .dot_matches_new_line(false)
            .build()
            .expect("upstream TypeScript regular expression must be valid Rust regex syntax"))
        .replace_all(&(src), "".to_owned())
        .into_owned(),
        &mut entities,
    ))
    .trim()
    .to_owned();
    return parse_element(
        expand_xml_entities((src).clone(), (entities).clone()),
        &mut ParseState {
            __flight_identity: std::sync::Arc::new(()),
            depth: 0.0_f64,
            depth_exceeded: false,
            pos: 0.0_f64,
        },
    );
}

// Source: upstream/packages/xml/src/xmlParse.ts:42 (sha256:d7c842382121a49f2a2bc98fa36aa2bb36359af6c889578171aa6ca444505e68)
#[derive(Clone, Default)]
pub(crate) struct ParseState {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub depth: f64,
    pub depth_exceeded: bool,
    pub pos: f64,
}
impl PartialEq for ParseState {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/xml/src/xmlParse.ts:49 (sha256:91c52a33034f8aa1e5130a26ef4eb6be0ae2b982174acfb791248c4a7e00b11f)
const MAX_XML_ENTITY_PASSES: f64 = 8.0_f64;

// Source: upstream/packages/xml/src/xmlParse.ts:50 (sha256:ca8cc244421066cc8cdd8166f8f04c4dc862f3649453760ea571e8979017ea61)
const MAX_XML_ENTITY_GROWTH: f64 = 16.0_f64;

// Source: upstream/packages/xml/src/xmlParse.ts:51 (sha256:3d0e7fc32770b869571e1fa579292d08a16401c068942338a0ad6887c7889f0c)
const MAX_XML_ENTITY_BUDGET: f64 = 65536.0_f64;

// Source: upstream/packages/xml/src/xmlParse.ts:55 (sha256:5fedcf39ac246c9a0f25087290beb99df54d9a80003ae08838298cdb8e4913c0)
const MAX_XML_ELEMENT_DEPTH: f64 = 256.0_f64;

// Source: upstream/packages/xml/src/xmlParse.ts:57 (sha256:4c1a4958644317ca043e154870018f0f08c7d60d4431d228485642dcd7c9586e)
static XML_ENTITIES: std::sync::LazyLock<Vec<(String, String)>> = std::sync::LazyLock::new(|| {
    let mut __flight_record = Vec::new();
    __flight_record.push(("amp".to_owned(), "&".to_owned()));
    __flight_record.push(("apos".to_owned(), "'".to_owned()));
    __flight_record.push(("gt".to_owned(), ">".to_owned()));
    __flight_record.push(("lt".to_owned(), "<".to_owned()));
    __flight_record.push(("quot".to_owned(), "\"".to_owned()));
    __flight_record
});

// Source: upstream/packages/xml/src/xmlParse.ts:77 (sha256:63809e1d1109649f96670ef219d1f6f45fb2a9e67901329f0b5aa17534397655)
fn expand_xml_entities(src: String, entities: Vec<(String, String)>) -> String {
    let __flight_utf16_src: std::sync::Arc<Vec<u16>> =
        std::sync::Arc::new(src.encode_utf16().collect());
    let mut output = (src).clone();
    let budget =
        (((__flight_utf16_src.len() as f64) * MAX_XML_ENTITY_GROWTH) + MAX_XML_ENTITY_BUDGET);
    {
        let mut pass = 0.0_f64;
        while (pass < MAX_XML_ENTITY_PASSES) {
            let expanded: std::sync::Arc<std::sync::Mutex<bool>> =
                std::sync::Arc::new(std::sync::Mutex::new(false));
            let next = {
                let mut __flight_replace = |reference: String, name: String| -> String {
                    let replacement = entities
                        .iter()
                        .find(|(entry_key, _)| entry_key == &(name).clone())
                        .map(|(_, value)| value.clone())
                        .clone();
                    if (replacement).is_none() {
                        return reference;
                    }
                    (*expanded.lock().unwrap()) = true;
                    return ((replacement.as_ref().unwrap()).clone()).clone();
                };
                (regex::RegexBuilder::new("&([\\w:.-]+);")
                    .case_insensitive(false)
                    .multi_line(false)
                    .dot_matches_new_line(false)
                    .build()
                    .expect(
                        "upstream TypeScript regular expression must be valid Rust regex syntax",
                    ))
                .replace_all(&((output).clone()), |captures: &regex::Captures<'_>| {
                    __flight_replace(
                        captures
                            .get(0)
                            .map_or("", |matched| matched.as_str())
                            .to_owned(),
                        captures
                            .get(1)
                            .map_or("", |matched| matched.as_str())
                            .to_owned(),
                    )
                })
                .into_owned()
            };
            if (!(*expanded.lock().unwrap()).clone())
                || ((next.encode_utf16().count() as f64) > budget)
            {
                return output;
            }
            output = (next).clone();
            {
                pass += 1.0;
                pass
            };
        }
    }
    return output;
}

// Source: upstream/packages/xml/src/xmlParse.ts:94 (sha256:250cafe58791dc2518324d175e916c0e25f718a32d57c407bafde6272c8baf88)
fn decode_xml_entities(s: String) -> String {
    return {
        let mut __flight_replace = |reference: String,
                                    dec: Option<String>,
                                    hex: Option<String>,
                                    name: Option<String>|
         -> String {
            let numeric = (dec).clone().or((hex).clone());
            if (numeric).is_some() {
                let codepoint = {
                    let __flight_value = (numeric.as_ref().unwrap()).clone();
                    let __flight_radix = (if (dec).is_some() { 10.0_f64 } else { 16.0_f64 }) as u32;
                    i64::from_str_radix(__flight_value.trim(), __flight_radix)
                        .map_or(f64::NAN, |value| value as f64)
                };
                if (codepoint > 1114111.0_f64)
                    || ((codepoint >= 55296.0_f64) && (codepoint <= 57343.0_f64))
                {
                    return reference;
                }
                return __flight_string_from_code_point(codepoint);
            }
            return (XML_ENTITIES
                .iter()
                .find(|(entry_key, _)| entry_key == &(name).clone().unwrap())
                .map(|(_, value)| value.clone())
                .clone())
            .clone()
            .unwrap_or((reference).clone());
        };
        (regex::RegexBuilder::new("&(?:#(\\d+)|#x([\\da-fA-F]+)|(\\w+));")
            .case_insensitive(false)
            .multi_line(false)
            .dot_matches_new_line(false)
            .build()
            .expect("upstream TypeScript regular expression must be valid Rust regex syntax"))
        .replace_all(&(s), |captures: &regex::Captures<'_>| {
            __flight_replace(
                captures
                    .get(0)
                    .map_or("", |matched| matched.as_str())
                    .to_owned(),
                captures.get(1).map(|matched| matched.as_str().to_owned()),
                captures.get(2).map(|matched| matched.as_str().to_owned()),
                captures.get(3).map(|matched| matched.as_str().to_owned()),
            )
        })
        .into_owned()
    };
}

// Source: upstream/packages/xml/src/xmlParse.ts:106 (sha256:c7ece3c90f4a0fee6fba779e7752d8831ec8b4a0734d8f2da72f995992043c22)
fn parse_element(src: String, state: &mut ParseState) -> Option<XmlElement> {
    let __flight_utf16_src: std::sync::Arc<Vec<u16>> =
        std::sync::Arc::new(src.encode_utf16().collect());
    if (state.depth >= MAX_XML_ELEMENT_DEPTH) {
        state.depth_exceeded = true;
        return None;
    }
    skip_whitespace((src).clone(), state);
    if (state.pos >= (__flight_utf16_src.len() as f64))
        || ({
            let __flight_units: &[u16] = &__flight_utf16_src;
            let __flight_raw_index = state.pos;
            if __flight_raw_index.is_finite()
                && __flight_raw_index >= 0.0_f64
                && __flight_raw_index.fract() == 0.0_f64
            {
                __flight_units
                    .get(__flight_raw_index as usize)
                    .map_or_else(String::new, |unit| String::from_utf16_lossy(&[*unit]))
            } else {
                String::new()
            }
        } != "<")
    {
        return None;
    }
    {
        state.pos += 1.0;
        state.pos
    };
    if ({
        let __flight_units: &[u16] = &__flight_utf16_src;
        let __flight_raw_index = state.pos;
        if __flight_raw_index.is_finite()
            && __flight_raw_index >= 0.0_f64
            && __flight_raw_index.fract() == 0.0_f64
        {
            __flight_units
                .get(__flight_raw_index as usize)
                .map_or_else(String::new, |unit| String::from_utf16_lossy(&[*unit]))
        } else {
            String::new()
        }
    } == "?")
    {
        let end = __flight_string_index_of(&((src).clone()), &("?>".to_owned()), state.pos);
        state.pos = if (end >= 0.0_f64) {
            (end + 2.0_f64)
        } else {
            (__flight_utf16_src.len() as f64)
        };
        return parse_element((src).clone(), state);
    }
    let name_start = state.pos;
    while (state.pos < (__flight_utf16_src.len() as f64))
        && (!(regex::RegexBuilder::new("[\\s>/]")
            .case_insensitive(false)
            .multi_line(false)
            .dot_matches_new_line(false)
            .build()
            .expect("upstream TypeScript regular expression must be valid Rust regex syntax"))
        .is_match(
            &({
                let __flight_units: &[u16] = &__flight_utf16_src;
                let __flight_raw_index = state.pos;
                if __flight_raw_index.is_finite()
                    && __flight_raw_index >= 0.0_f64
                    && __flight_raw_index.fract() == 0.0_f64
                {
                    __flight_units
                        .get(__flight_raw_index as usize)
                        .map_or_else(String::new, |unit| String::from_utf16_lossy(&[*unit]))
                } else {
                    String::new()
                }
            }),
        ))
    {
        {
            state.pos += 1.0;
            state.pos
        };
    }
    let name = __flight_string_slice(&(src), name_start, Some(state.pos));
    if (name).is_empty() {
        return None;
    }
    skip_whitespace((src).clone(), state);
    let mut attrs_str = "".to_owned();
    let mut quote = "".to_owned();
    while (state.pos < (__flight_utf16_src.len() as f64)) {
        let ch = {
            let __flight_units: &[u16] = &__flight_utf16_src;
            let __flight_raw_index = state.pos;
            if __flight_raw_index.is_finite()
                && __flight_raw_index >= 0.0_f64
                && __flight_raw_index.fract() == 0.0_f64
            {
                __flight_units
                    .get(__flight_raw_index as usize)
                    .map_or_else(String::new, |unit| String::from_utf16_lossy(&[*unit]))
            } else {
                String::new()
            }
        };
        if !(quote).is_empty() {
            if ((ch).clone() == quote) {
                quote = "".to_owned();
            }
        } else {
            if ((ch).clone() == "\"") || ((ch).clone() == "'") {
                quote = (ch).clone();
            } else {
                if ((ch).clone() == ">")
                    || (((ch).clone() == "/")
                        && ({
                            let __flight_units: &[u16] = &__flight_utf16_src;
                            let __flight_raw_index = (state.pos + 1.0_f64);
                            if __flight_raw_index.is_finite()
                                && __flight_raw_index >= 0.0_f64
                                && __flight_raw_index.fract() == 0.0_f64
                            {
                                __flight_units
                                    .get(__flight_raw_index as usize)
                                    .map_or_else(String::new, |unit| {
                                        String::from_utf16_lossy(&[*unit])
                                    })
                            } else {
                                String::new()
                            }
                        } == ">"))
                {
                    break;
                }
            }
        }
        attrs_str.push_str(&((ch).clone()));
        {
            state.pos += 1.0;
            state.pos
        };
    }
    let self_closing = ({
        let __flight_units: &[u16] = &__flight_utf16_src;
        let __flight_raw_index = state.pos;
        if __flight_raw_index.is_finite()
            && __flight_raw_index >= 0.0_f64
            && __flight_raw_index.fract() == 0.0_f64
        {
            __flight_units
                .get(__flight_raw_index as usize)
                .map_or_else(String::new, |unit| String::from_utf16_lossy(&[*unit]))
        } else {
            String::new()
        }
    } == "/");
    state.pos += if self_closing { 2.0_f64 } else { 1.0_f64 };
    let attributes = parse_xml_attributes((attrs_str).clone());
    let mut children: Vec<XmlElement> = vec![];
    let mut content: Vec<crate::FlightUnion2<String, XmlElement>> = vec![];
    let mut text = "".to_owned();
    if (!self_closing) {
        while (state.pos < (__flight_utf16_src.len() as f64)) {
            if (state.pos >= (__flight_utf16_src.len() as f64)) {
                break;
            }
            if ({
                let __flight_units: &[u16] = &__flight_utf16_src;
                let __flight_raw_index = state.pos;
                if __flight_raw_index.is_finite()
                    && __flight_raw_index >= 0.0_f64
                    && __flight_raw_index.fract() == 0.0_f64
                {
                    __flight_units
                        .get(__flight_raw_index as usize)
                        .map_or_else(String::new, |unit| String::from_utf16_lossy(&[*unit]))
                } else {
                    String::new()
                }
            } != "<")
            {
                let text_start = state.pos;
                while (state.pos < (__flight_utf16_src.len() as f64))
                    && ({
                        let __flight_units: &[u16] = &__flight_utf16_src;
                        let __flight_raw_index = state.pos;
                        if __flight_raw_index.is_finite()
                            && __flight_raw_index >= 0.0_f64
                            && __flight_raw_index.fract() == 0.0_f64
                        {
                            __flight_units
                                .get(__flight_raw_index as usize)
                                .map_or_else(String::new, |unit| String::from_utf16_lossy(&[*unit]))
                        } else {
                            String::new()
                        }
                    } != "<")
                {
                    {
                        state.pos += 1.0;
                        state.pos
                    };
                }
                let decoded = decode_xml_entities(__flight_string_slice(
                    &((src).clone()),
                    text_start,
                    Some(state.pos),
                ));
                text.push_str(&(((decoded).clone()).trim().to_owned()));
                if ((decoded).clone() != "") {
                    content.push(
                        (crate::FlightUnion2::<String, XmlElement>::A((decoded).clone())).clone(),
                    );
                }
                continue;
            }
            if (__flight_string_slice(&((src).clone()), state.pos, Some((state.pos + 9.0_f64)))
                == "<![CDATA[")
            {
                let cdata_start = (state.pos + 9.0_f64);
                let cdata_end =
                    __flight_string_index_of(&((src).clone()), &("]]>".to_owned()), cdata_start);
                let content_end = if (cdata_end >= 0.0_f64) {
                    cdata_end
                } else {
                    (__flight_utf16_src.len() as f64)
                };
                let cdata = __flight_string_slice(&((src).clone()), cdata_start, Some(content_end));
                text.push_str(&(((cdata).clone()).trim().to_owned()));
                if ((cdata).clone() != "") {
                    content.push(
                        (crate::FlightUnion2::<String, XmlElement>::A((cdata).clone())).clone(),
                    );
                }
                state.pos = if (cdata_end >= 0.0_f64) {
                    (cdata_end + 3.0_f64)
                } else {
                    (__flight_utf16_src.len() as f64)
                };
                continue;
            }
            if ({
                let __flight_units: &[u16] = &__flight_utf16_src;
                let __flight_raw_index = (state.pos + 1.0_f64);
                if __flight_raw_index.is_finite()
                    && __flight_raw_index >= 0.0_f64
                    && __flight_raw_index.fract() == 0.0_f64
                {
                    __flight_units
                        .get(__flight_raw_index as usize)
                        .map_or_else(String::new, |unit| String::from_utf16_lossy(&[*unit]))
                } else {
                    String::new()
                }
            } == "/")
            {
                while (state.pos < (__flight_utf16_src.len() as f64))
                    && ({
                        let __flight_units: &[u16] = &__flight_utf16_src;
                        let __flight_raw_index = state.pos;
                        if __flight_raw_index.is_finite()
                            && __flight_raw_index >= 0.0_f64
                            && __flight_raw_index.fract() == 0.0_f64
                        {
                            __flight_units
                                .get(__flight_raw_index as usize)
                                .map_or_else(String::new, |unit| String::from_utf16_lossy(&[*unit]))
                        } else {
                            String::new()
                        }
                    } != ">")
                {
                    {
                        state.pos += 1.0;
                        state.pos
                    };
                }
                {
                    state.pos += 1.0;
                    state.pos
                };
                break;
            }
            {
                state.depth += 1.0;
                state.depth
            };
            let child = parse_element((src).clone(), state);
            {
                state.depth -= 1.0;
                state.depth
            };
            if state.depth_exceeded {
                return None;
            }
            if ((child).clone()).is_some() {
                children.push(((child.as_ref().unwrap()).clone()).clone());
                content.push(
                    (crate::FlightUnion2::<String, XmlElement>::B(
                        (child.as_ref().unwrap()).clone(),
                    ))
                    .clone(),
                );
            }
        }
    }
    return Some(XmlElement {
        __flight_identity: std::sync::Arc::new(()),
        attributes: (attributes).clone(),
        children: (children).clone(),
        content: (content).clone(),
        name: (name).clone(),
        text: (text).clone(),
    });
}

// Source: upstream/packages/xml/src/xmlParse.ts:206 (sha256:8b165c706b68384ac4307222e85dcc19e0a696891ec44a55efdb1a50dacdcbda)
fn skip_whitespace(src: String, state: &mut ParseState) -> () {
    let __flight_utf16_src: std::sync::Arc<Vec<u16>> =
        std::sync::Arc::new(src.encode_utf16().collect());
    while (state.pos < (__flight_utf16_src.len() as f64))
        && ((regex::RegexBuilder::new("\\s")
            .case_insensitive(false)
            .multi_line(false)
            .dot_matches_new_line(false)
            .build()
            .expect("upstream TypeScript regular expression must be valid Rust regex syntax"))
        .is_match(
            &({
                let __flight_units: &[u16] = &__flight_utf16_src;
                let __flight_raw_index = state.pos;
                if __flight_raw_index.is_finite()
                    && __flight_raw_index >= 0.0_f64
                    && __flight_raw_index.fract() == 0.0_f64
                {
                    __flight_units
                        .get(__flight_raw_index as usize)
                        .map_or_else(String::new, |unit| String::from_utf16_lossy(&[*unit]))
                } else {
                    String::new()
                }
            }),
        ))
    {
        {
            state.pos += 1.0;
            state.pos
        };
    }
}

// Source: upstream/packages/xml/src/xmlParse.ts:210 (sha256:7a46a3c2689e4a2acfb51e54dcb2ace37d130350ae6633e3a86b6ed6c9360711)
fn strip_xml_comments(xml: String) -> String {
    let __flight_utf16_xml: std::sync::Arc<Vec<u16>> =
        std::sync::Arc::new(xml.encode_utf16().collect());
    let mut copy_start = 0.0_f64;
    let mut output = "".to_owned();
    let mut pos = 0.0_f64;
    while (pos < (__flight_utf16_xml.len() as f64)) {
        if (__flight_string_slice(&(xml), pos, Some((pos + 9.0_f64))) == "<![CDATA[") {
            let cdata_end = __flight_string_index_of(&(xml), &("]]>".to_owned()), (pos + 9.0_f64));
            pos = if (cdata_end >= 0.0_f64) {
                (cdata_end + 3.0_f64)
            } else {
                (__flight_utf16_xml.len() as f64)
            };
            continue;
        }
        if (__flight_string_slice(&(xml), pos, Some((pos + 4.0_f64))) != "<!--") {
            {
                pos += 1.0;
                pos
            };
            continue;
        }
        output.push_str(&(__flight_string_slice(&(xml), copy_start, Some(pos))));
        let comment_end = __flight_string_index_of(&(xml), &("-->".to_owned()), (pos + 4.0_f64));
        pos = if (comment_end >= 0.0_f64) {
            (comment_end + 3.0_f64)
        } else {
            (__flight_utf16_xml.len() as f64)
        };
        copy_start = pos;
    }
    return format!(
        "{}{}",
        output,
        __flight_string_slice(&(xml), copy_start, None)
    );
}

// Source: upstream/packages/xml/src/xmlParse.ts:242 (sha256:954be5d30230e6ab51775a59a19d79af69864b0600682453ef64b8b2a0741f71)
fn strip_xml_doctypes(xml: String, out: &mut Vec<(String, String)>) -> String {
    let __flight_utf16_xml: std::sync::Arc<Vec<u16>> =
        std::sync::Arc::new(xml.encode_utf16().collect());
    let mut copy_start = 0.0_f64;
    let mut output = "".to_owned();
    let mut pos = 0.0_f64;
    while (pos < (__flight_utf16_xml.len() as f64)) {
        if ({
            let __flight_units: &[u16] = &__flight_utf16_xml;
            let __flight_raw_index = pos;
            if __flight_raw_index.is_finite()
                && __flight_raw_index >= 0.0_f64
                && __flight_raw_index.fract() == 0.0_f64
            {
                __flight_units
                    .get(__flight_raw_index as usize)
                    .map_or_else(String::new, |unit| String::from_utf16_lossy(&[*unit]))
            } else {
                String::new()
            }
        } != "<")
            || ((__flight_string_slice(&(xml), pos, Some((pos + 9.0_f64)))).to_lowercase()
                != "<!doctype")
        {
            {
                pos += 1.0;
                pos
            };
            continue;
        }
        output.push_str(&(__flight_string_slice(&(xml), copy_start, Some(pos))));
        let doctype_start = pos;
        pos += 9.0_f64;
        let mut internal_subset_depth = 0.0_f64;
        let mut quote = "".to_owned();
        while (pos < (__flight_utf16_xml.len() as f64)) {
            let ch = {
                let __flight_units: &[u16] = &__flight_utf16_xml;
                let __flight_raw_index = pos;
                if __flight_raw_index.is_finite()
                    && __flight_raw_index >= 0.0_f64
                    && __flight_raw_index.fract() == 0.0_f64
                {
                    __flight_units
                        .get(__flight_raw_index as usize)
                        .map_or_else(String::new, |unit| String::from_utf16_lossy(&[*unit]))
                } else {
                    String::new()
                }
            };
            if !(quote).is_empty() {
                if ((ch).clone() == quote) {
                    quote = "".to_owned();
                }
            } else {
                if ((ch).clone() == "\"") || ((ch).clone() == "'") {
                    quote = (ch).clone();
                } else {
                    if ((ch).clone() == "[") {
                        {
                            internal_subset_depth += 1.0;
                            internal_subset_depth
                        };
                    } else {
                        if ((ch).clone() == "]") && (internal_subset_depth > 0.0_f64) {
                            {
                                internal_subset_depth -= 1.0;
                                internal_subset_depth
                            };
                        } else {
                            if ((ch).clone() == ">") && (internal_subset_depth == 0.0_f64) {
                                {
                                    pos += 1.0;
                                    pos
                                };
                                break;
                            }
                        }
                    }
                }
            }
            {
                pos += 1.0;
                pos
            };
        }
        collect_xml_entity_declarations(
            __flight_string_slice(&(xml), doctype_start, Some(pos)),
            out,
        );
        copy_start = pos;
    }
    return format!(
        "{}{}",
        output,
        __flight_string_slice(&(xml), copy_start, None)
    );
}

// Source: upstream/packages/xml/src/xmlParse.ts:286 (sha256:138254e36b8b51442f232967462a2bae0ed911daab5cb6cc412cd2b508085b59)
fn collect_xml_entity_declarations(doctype: String, out: &mut Vec<(String, String)>) -> () {
    let declaration =
        regex::RegexBuilder::new("<!ENTITY\\s+([\\w:.-]+)\\s*(?:\"([^\"]*)\"|'([^']*)')\\s*>")
            .case_insensitive(false)
            .multi_line(false)
            .dot_matches_new_line(false)
            .build()
            .expect("upstream TypeScript regular expression must be valid Rust regex syntax");
    let mut match_: Option<Vec<Option<String>>>;
    while ({
        match_ = {
            let __flight_regex = &(declaration);
            __flight_regex
                .captures(&((doctype).clone()))
                .map(|captures| {
                    (0..captures.len())
                        .map(|index| {
                            captures
                                .get(index)
                                .map(|matched| matched.as_str().to_owned())
                        })
                        .collect::<Vec<_>>()
                })
        };
        match_.clone()
    })
    .is_some()
    {
        {
            let __flight_key = match_.as_mut().unwrap()[1.0_f64 as usize].clone().unwrap();
            let __flight_value = ((match_.as_mut().unwrap()[2.0_f64 as usize].clone())
                .clone()
                .or(match_.as_mut().unwrap()[3.0_f64 as usize].clone()))
            .clone()
            .unwrap_or("".to_owned());
            if let Some((_, value)) = out.iter_mut().find(|(key, _)| key == &__flight_key) {
                *value = __flight_value;
            } else {
                out.push((__flight_key, __flight_value));
            }
        };
    }
}
