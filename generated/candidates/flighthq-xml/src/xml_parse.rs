// @generated from upstream/packages/xml/src/xmlParse.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::XmlElement;

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
    let mut m: Option<crate::OpaqueHostValue>;
    while ({
        m = {
            let __flight_regex = re;
            __flight_regex.captures(&((attrs).clone())).map(|captures| {
                (0..captures.len())
                    .map(|index| {
                        captures
                            .get(index)
                            .map_or("", |matched| matched.as_str())
                            .to_owned()
                    })
                    .collect::<Vec<_>>()
            })
        };
        m
    })
    .is_some()
    {
        let attr_name = crate::host_value::<crate::OpaqueHostValue>("host.index");
        let value = if (crate::host_value::<crate::OpaqueHostValue>("host.index")).is_some() {
            crate::host_value::<crate::OpaqueHostValue>("host.index")
        } else {
            crate::host_value::<crate::OpaqueHostValue>("host.index")
        };
        result
            .iter()
            .find(|(key, _)| key == &attr_name)
            .map(|(_, value)| value)
            .expect("TypeScript Record key was absent") = decode_xml_entities(value);
    }
    return result;
}

// Source: upstream/packages/xml/src/xmlParse.ts:28 (sha256:c2ec91262260625cea9bdb958980ce529d2126f6fd32b2f3b254f162bf066abb)
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
    .replace_all(&(strip_xml_comments((xml).clone())), "\n")
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
        .replace_all(&(src), "")
        .into_owned(),
        &mut entities,
    ))
    .trim()
    .to_owned();
    return parse_element(
        expand_xml_entities((src).clone(), (entities).clone()),
        &mut ParseState {
            __flight_identity: std::sync::Arc::new(()),
            pos: 0.0_f64,
        },
    );
}

// Source: upstream/packages/xml/src/xmlParse.ts:42 (sha256:9c67998340c6b093dfb9c8b8d29c8c1995935436f7e04c286d22775ca18ce3ee)
#[derive(Clone, Default)]
struct ParseState {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub pos: f64,
}
impl PartialEq for ParseState {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/xml/src/xmlParse.ts:47 (sha256:91c52a33034f8aa1e5130a26ef4eb6be0ae2b982174acfb791248c4a7e00b11f)
const MAX_XML_ENTITY_PASSES: f64 = 8.0_f64;

// Source: upstream/packages/xml/src/xmlParse.ts:48 (sha256:ca8cc244421066cc8cdd8166f8f04c4dc862f3649453760ea571e8979017ea61)
const MAX_XML_ENTITY_GROWTH: f64 = 16.0_f64;

// Source: upstream/packages/xml/src/xmlParse.ts:49 (sha256:3d0e7fc32770b869571e1fa579292d08a16401c068942338a0ad6887c7889f0c)
const MAX_XML_ENTITY_BUDGET: f64 = 65536.0_f64;

// Source: upstream/packages/xml/src/xmlParse.ts:51 (sha256:4c1a4958644317ca043e154870018f0f08c7d60d4431d228485642dcd7c9586e)
static XML_ENTITIES: std::sync::LazyLock<Vec<(String, String)>> = std::sync::LazyLock::new(|| {
    let mut __flight_record = Vec::new();
    __flight_record.push(("amp".to_owned(), "&".to_owned()));
    __flight_record.push(("apos".to_owned(), "'".to_owned()));
    __flight_record.push(("gt".to_owned(), ">".to_owned()));
    __flight_record.push(("lt".to_owned(), "<".to_owned()));
    __flight_record.push(("quot".to_owned(), "\"".to_owned()));
    __flight_record
});

// Source: upstream/packages/xml/src/xmlParse.ts:71 (sha256:63809e1d1109649f96670ef219d1f6f45fb2a9e67901329f0b5aa17534397655)
fn expand_xml_entities(src: String, entities: Vec<(String, String)>) -> String {
    let mut output = src;
    let budget =
        (((src.encode_utf16().count() as f64) * MAX_XML_ENTITY_GROWTH) + MAX_XML_ENTITY_BUDGET);
    {
        let mut pass = 0.0_f64;
        while (pass < MAX_XML_ENTITY_PASSES) {
            let expanded: std::sync::Arc<std::sync::Mutex<bool>> =
                std::sync::Arc::new(std::sync::Mutex::new(false));
            let next = {
                let mut __flight_replace = |reference: String, name: String| -> String {
                    let replacement = entities
                        .iter()
                        .find(|(key, _)| key == &(name).clone())
                        .map(|(_, value)| value)
                        .expect("TypeScript Record key was absent")
                        .clone();
                    if (replacement).is_none() {
                        return reference;
                    }
                    (*expanded.lock().unwrap()) = true;
                    return replacement;
                };
                (regex::RegexBuilder::new("&([\\w:.-]+);")
                    .case_insensitive(false)
                    .multi_line(false)
                    .dot_matches_new_line(false)
                    .build()
                    .expect(
                        "upstream TypeScript regular expression must be valid Rust regex syntax",
                    ))
                .replace_all(&(output), |captures: &regex::Captures<'_>| {
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

// Source: upstream/packages/xml/src/xmlParse.ts:88 (sha256:250cafe58791dc2518324d175e916c0e25f718a32d57c407bafde6272c8baf88)
fn decode_xml_entities(s: String) -> String {
    return {
        let mut __flight_replace =
            |reference: String, dec: String, hex: String, name: String| -> String {
                let numeric = dec;
                if (numeric).is_some() {
                    let codepoint = {
                        let __flight_value = (numeric).clone();
                        let __flight_radix =
                            (if (dec).is_some() { 10.0_f64 } else { 16.0_f64 }) as u32;
                        i64::from_str_radix(__flight_value.trim(), __flight_radix)
                            .map_or(f64::NAN, |value| value as f64)
                    };
                    if (codepoint > 1114111.0_f64)
                        || ((codepoint >= 55296.0_f64) && (codepoint <= 57343.0_f64))
                    {
                        return reference;
                    }
                    return (string.from_code_point)(codepoint);
                }
                return XML_ENTITIES
                    .iter()
                    .find(|(key, _)| key == &(name).clone())
                    .map(|(_, value)| value)
                    .expect("TypeScript Record key was absent")
                    .clone();
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
                captures
                    .get(1)
                    .map_or("", |matched| matched.as_str())
                    .to_owned(),
                captures
                    .get(2)
                    .map_or("", |matched| matched.as_str())
                    .to_owned(),
                captures
                    .get(3)
                    .map_or("", |matched| matched.as_str())
                    .to_owned(),
            )
        })
        .into_owned()
    };
}

// Source: upstream/packages/xml/src/xmlParse.ts:100 (sha256:6da049cc35a4d4c594444a8125ae0901e9e7003789feb1b7025785329dbd7a82)
fn parse_element(src: String, state: &mut ParseState) -> Option<XmlElement> {
    skip_whitespace((src).clone(), state);
    if (state.pos >= (src.encode_utf16().count() as f64))
        || (src[state.pos as usize].clone() != "<")
    {
        return None;
    }
    {
        state.pos += 1.0;
        state.pos
    };
    if (src[state.pos as usize].clone() == "?") {
        let end = (src.index_of)("?>", state.pos);
        state.pos = if (end >= 0.0_f64) {
            (end + 2.0_f64)
        } else {
            (src.encode_utf16().count() as f64)
        };
        return parse_element((src).clone(), state);
    }
    let name_start = state.pos;
    while (state.pos < (src.encode_utf16().count() as f64))
        && (!(regex::RegexBuilder::new("[\\s>/]")
            .case_insensitive(false)
            .multi_line(false)
            .dot_matches_new_line(false)
            .build()
            .expect("upstream TypeScript regular expression must be valid Rust regex syntax"))
        .is_match(&(src[state.pos as usize].clone())))
    {
        {
            state.pos += 1.0;
            state.pos
        };
    }
    let name = String::from_utf16_lossy(
        &(src)
            .encode_utf16()
            .skip((name_start) as usize)
            .take(((state.pos) as usize).saturating_sub((name_start) as usize))
            .collect::<Vec<u16>>(),
    );
    if (!name) {
        return None;
    }
    skip_whitespace((src).clone(), state);
    let mut attrs_str = "";
    let mut quote = "";
    while (state.pos < (src.encode_utf16().count() as f64)) {
        let ch = src[state.pos as usize].clone();
        if quote {
            if (ch == quote) {
                quote = "".to_owned();
            }
        } else {
            if (ch == "\"") || (ch == "'") {
                quote = ch;
            } else {
                if (ch == ">")
                    || ((ch == "/") && (src[(state.pos + 1.0_f64) as usize].clone() == ">"))
                {
                    break;
                }
            }
        }
        attrs_str += ch;
        {
            state.pos += 1.0;
            state.pos
        };
    }
    let self_closing = (src[state.pos as usize].clone() == "/");
    state.pos += if self_closing { 2.0_f64 } else { 1.0_f64 };
    let attributes = parse_xml_attributes((attrs_str).clone());
    let mut children: Vec<XmlElement> = vec![];
    let mut content: Vec<crate::FlightUnion2<String, XmlElement>> = vec![];
    let mut text = "";
    if (!self_closing) {
        while (state.pos < (src.encode_utf16().count() as f64)) {
            if (state.pos >= (src.encode_utf16().count() as f64)) {
                break;
            }
            if (src[state.pos as usize].clone() != "<") {
                let text_start = state.pos;
                while (state.pos < (src.encode_utf16().count() as f64))
                    && (src[state.pos as usize].clone() != "<")
                {
                    {
                        state.pos += 1.0;
                        state.pos
                    };
                }
                let decoded = decode_xml_entities(String::from_utf16_lossy(
                    &(src)
                        .encode_utf16()
                        .skip((text_start) as usize)
                        .take(((state.pos) as usize).saturating_sub((text_start) as usize))
                        .collect::<Vec<u16>>(),
                ));
                text += (decoded).trim().to_owned();
                if (decoded != "") {
                    content.push(
                        (crate::FlightUnion2::<String, XmlElement>::A((decoded).clone())).clone(),
                    );
                }
                continue;
            }
            if (String::from_utf16_lossy(
                &(src)
                    .encode_utf16()
                    .skip((state.pos) as usize)
                    .take(((state.pos + 9.0_f64) as usize).saturating_sub((state.pos) as usize))
                    .collect::<Vec<u16>>(),
            ) == "<![CDATA[")
            {
                let cdata_start = (state.pos + 9.0_f64);
                let cdata_end = (src.index_of)("]]>", cdata_start);
                let content_end = if (cdata_end >= 0.0_f64) {
                    cdata_end
                } else {
                    (src.encode_utf16().count() as f64)
                };
                let cdata = String::from_utf16_lossy(
                    &(src)
                        .encode_utf16()
                        .skip((cdata_start) as usize)
                        .take(((content_end) as usize).saturating_sub((cdata_start) as usize))
                        .collect::<Vec<u16>>(),
                );
                text += (cdata.trim)();
                if (cdata != "") {
                    content.push(cdata);
                }
                state.pos = if (cdata_end >= 0.0_f64) {
                    (cdata_end + 3.0_f64)
                } else {
                    (src.encode_utf16().count() as f64)
                };
                continue;
            }
            if (src[(state.pos + 1.0_f64) as usize].clone() == "/") {
                while (state.pos < (src.encode_utf16().count() as f64))
                    && (src[state.pos as usize].clone() != ">")
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
            let child = parse_element((src).clone(), state);
            if (child).is_some() {
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
        name: name,
        text: (text).clone(),
    });
}

// Source: upstream/packages/xml/src/xmlParse.ts:193 (sha256:8b165c706b68384ac4307222e85dcc19e0a696891ec44a55efdb1a50dacdcbda)
fn skip_whitespace(src: String, state: &mut ParseState) -> () {
    while (state.pos < (src.encode_utf16().count() as f64))
        && ((regex::RegexBuilder::new("\\s")
            .case_insensitive(false)
            .multi_line(false)
            .dot_matches_new_line(false)
            .build()
            .expect("upstream TypeScript regular expression must be valid Rust regex syntax"))
        .is_match(&(src[state.pos as usize].clone())))
    {
        {
            state.pos += 1.0;
            state.pos
        };
    }
}

// Source: upstream/packages/xml/src/xmlParse.ts:197 (sha256:7a46a3c2689e4a2acfb51e54dcb2ace37d130350ae6633e3a86b6ed6c9360711)
fn strip_xml_comments(xml: String) -> String {
    let mut copy_start = 0.0_f64;
    let mut output = "";
    let mut pos = 0.0_f64;
    while (pos < (xml.encode_utf16().count() as f64)) {
        if (String::from_utf16_lossy(
            &(xml)
                .encode_utf16()
                .skip((pos) as usize)
                .take(((pos + 9.0_f64) as usize).saturating_sub((pos) as usize))
                .collect::<Vec<u16>>(),
        ) == "<![CDATA[")
        {
            let cdata_end = (xml.index_of)("]]>", (pos + 9.0_f64));
            pos = if (cdata_end >= 0.0_f64) {
                (cdata_end + 3.0_f64)
            } else {
                (xml.encode_utf16().count() as f64)
            };
            continue;
        }
        if (String::from_utf16_lossy(
            &(xml)
                .encode_utf16()
                .skip((pos) as usize)
                .take(((pos + 4.0_f64) as usize).saturating_sub((pos) as usize))
                .collect::<Vec<u16>>(),
        ) != "<!--")
        {
            {
                pos += 1.0;
                pos
            };
            continue;
        }
        output += String::from_utf16_lossy(
            &(xml)
                .encode_utf16()
                .skip((copy_start) as usize)
                .take(((pos) as usize).saturating_sub((copy_start) as usize))
                .collect::<Vec<u16>>(),
        );
        let comment_end = (xml.index_of)("-->", (pos + 4.0_f64));
        pos = if (comment_end >= 0.0_f64) {
            (comment_end + 3.0_f64)
        } else {
            (xml.encode_utf16().count() as f64)
        };
        copy_start = pos;
    }
    return (output
        + String::from_utf16_lossy(
            &(xml)
                .encode_utf16()
                .skip((copy_start) as usize)
                .collect::<Vec<u16>>(),
        ));
}

// Source: upstream/packages/xml/src/xmlParse.ts:229 (sha256:954be5d30230e6ab51775a59a19d79af69864b0600682453ef64b8b2a0741f71)
fn strip_xml_doctypes(xml: String, out: &mut Vec<(String, String)>) -> String {
    let mut copy_start = 0.0_f64;
    let mut output = "";
    let mut pos = 0.0_f64;
    while (pos < (xml.encode_utf16().count() as f64)) {
        if (xml[pos as usize].clone() != "<")
            || ((String::from_utf16_lossy(
                &(xml)
                    .encode_utf16()
                    .skip((pos) as usize)
                    .take(((pos + 9.0_f64) as usize).saturating_sub((pos) as usize))
                    .collect::<Vec<u16>>(),
            )
            .to_lower_case)()
                != "<!doctype")
        {
            {
                pos += 1.0;
                pos
            };
            continue;
        }
        output += String::from_utf16_lossy(
            &(xml)
                .encode_utf16()
                .skip((copy_start) as usize)
                .take(((pos) as usize).saturating_sub((copy_start) as usize))
                .collect::<Vec<u16>>(),
        );
        let doctype_start = pos;
        pos += 9.0_f64;
        let mut internal_subset_depth = 0.0_f64;
        let mut quote = "";
        while (pos < (xml.encode_utf16().count() as f64)) {
            let ch = xml[pos as usize].clone();
            if quote {
                if (ch == quote) {
                    quote = "".to_owned();
                }
            } else {
                if (ch == "\"") || (ch == "'") {
                    quote = ch;
                } else {
                    if (ch == "[") {
                        {
                            internal_subset_depth += 1.0;
                            internal_subset_depth
                        };
                    } else {
                        if (ch == "]") && (internal_subset_depth > 0.0_f64) {
                            {
                                internal_subset_depth -= 1.0;
                                internal_subset_depth
                            };
                        } else {
                            if (ch == ">") && (internal_subset_depth == 0.0_f64) {
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
            String::from_utf16_lossy(
                &(xml)
                    .encode_utf16()
                    .skip((doctype_start) as usize)
                    .take(((pos) as usize).saturating_sub((doctype_start) as usize))
                    .collect::<Vec<u16>>(),
            ),
            out,
        );
        copy_start = pos;
    }
    return (output
        + String::from_utf16_lossy(
            &(xml)
                .encode_utf16()
                .skip((copy_start) as usize)
                .collect::<Vec<u16>>(),
        ));
}

// Source: upstream/packages/xml/src/xmlParse.ts:273 (sha256:138254e36b8b51442f232967462a2bae0ed911daab5cb6cc412cd2b508085b59)
fn collect_xml_entity_declarations(doctype: String, out: &mut Vec<(String, String)>) -> () {
    let declaration =
        regex::RegexBuilder::new("<!ENTITY\\s+([\\w:.-]+)\\s*(?:\"([^\"]*)\"|'([^']*)')\\s*>")
            .case_insensitive(false)
            .multi_line(false)
            .dot_matches_new_line(false)
            .build()
            .expect("upstream TypeScript regular expression must be valid Rust regex syntax");
    let mut match_: Option<crate::OpaqueHostValue>;
    while ({
        match_ = {
            let __flight_regex = declaration;
            __flight_regex
                .captures(&((doctype).clone()))
                .map(|captures| {
                    (0..captures.len())
                        .map(|index| {
                            captures
                                .get(index)
                                .map_or("", |matched| matched.as_str())
                                .to_owned()
                        })
                        .collect::<Vec<_>>()
                })
        };
        match_
    })
    .is_some()
    {
        out.iter()
            .find(|(key, _)| key == &crate::host_value::<String>("host.index"))
            .map(|(_, value)| value)
            .expect("TypeScript Record key was absent") =
            crate::host_value::<crate::OpaqueHostValue>("host.index");
    }
}
