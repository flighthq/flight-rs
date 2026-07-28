// @generated from upstream/packages/xml/src/xmlParse.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/xml/src/xmlParse.ts:7 (sha256:8413d922b6b9aacab00f8d6d95f3e14c96e5643e16fc094a3e965313f929e615)
#[derive(Clone)]
pub struct XmlElement {
    #[doc(hidden)]
    pub __flight_identity: std::sync::Arc<()>,
    pub attributes: Vec<(String, String)>,
    pub children: Vec<XmlElement>,
    pub name: String,
    pub text: String,
}
impl PartialEq for XmlElement {
    fn eq(&self, other: &Self) -> bool {
        std::sync::Arc::ptr_eq(&self.__flight_identity, &other.__flight_identity)
    }
}

// Source: upstream/packages/xml/src/xmlParse.ts:18 (sha256:5d3faebf7d0c254ea5522a2669360263c6aab9196a4015bcfba2079457bba066)
#[derive(Clone)]
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
        m = Some((re.exec)(attrs));
        m
    })
    .is_some()
    {
        let attr_name = m[1.0_f64 as usize].clone();
        let value = if (m[2.0_f64 as usize].clone()).is_some() {
            m[2.0_f64 as usize].clone()
        } else {
            (m[3.0_f64 as usize].clone()).unwrap_or("")
        };
        result
            .iter()
            .find(|(key, _)| key == &attr_name)
            .map(|(_, value)| value)
            .expect("TypeScript Record key was absent") = decode_xml_entities(value);
    }
    return (result).clone();
}

// Source: upstream/packages/xml/src/xmlParse.ts:34 (sha256:a7aa0aa5ab2be01d7e16430c29b0a95d3eb6d9fabda42b2df4c9aa07b329c349)
#[derive(Clone)]
struct ParseXmlDocumentRecord1 {
    __flight_identity: std::sync::Arc<()>,
    pos: f64,
}
impl PartialEq for ParseXmlDocumentRecord1 {
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
    .replace_all(&(strip_cdata(strip_xml_comments((xml).clone()))), "\n")
    .into_owned();
    src = ((regex::RegexBuilder::new("<!DOCTYPE[^>[]*(?:\\[[\\s\\S]*?\\][^>]*)?>")
        .case_insensitive(true)
        .multi_line(false)
        .dot_matches_new_line(false)
        .build()
        .expect("upstream TypeScript regular expression must be valid Rust regex syntax"))
    .replace_all(
        &((regex::RegexBuilder::new("<\\?[\\s\\S]*?\\?>")
            .case_insensitive(false)
            .multi_line(false)
            .dot_matches_new_line(false)
            .build()
            .expect("upstream TypeScript regular expression must be valid Rust regex syntax"))
        .replace_all(&(src), "")
        .into_owned()),
        "",
    )
    .into_owned())
    .trim()
    .to_owned();
    return parse_element(
        (src).clone(),
        &mut ParseState {
            __flight_identity: std::sync::Arc::new(()),
            pos: 0.0_f64,
        },
    );
}

// Source: upstream/packages/xml/src/xmlParse.ts:48 (sha256:9c67998340c6b093dfb9c8b8d29c8c1995935436f7e04c286d22775ca18ce3ee)
#[derive(Clone)]
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

// Source: upstream/packages/xml/src/xmlParse.ts:52 (sha256:4c1a4958644317ca043e154870018f0f08c7d60d4431d228485642dcd7c9586e)
static XML_ENTITIES: std::sync::LazyLock<Vec<(String, String)>> = std::sync::LazyLock::new(|| {
    let mut __flight_record = Vec::new();
    __flight_record.push(("amp".to_owned(), "&".to_owned()));
    __flight_record.push(("apos".to_owned(), "'".to_owned()));
    __flight_record.push(("gt".to_owned(), ">".to_owned()));
    __flight_record.push(("lt".to_owned(), "<".to_owned()));
    __flight_record.push(("quot".to_owned(), "\"".to_owned()));
    __flight_record
});

// Source: upstream/packages/xml/src/xmlParse.ts:60 (sha256:8811f841733eca8629e305c946ef832d20a2f999ceeda913887174f9667203a2)
fn decode_xml_entities(s: String) -> String {
    return {
        let mut __flight_replace = |_: String, dec: String, hex: String, name: String| -> String {
            if dec {
                return (string.from_code_point)(crate::host_value::<()>("host.call"));
            }
            if hex {
                return (string.from_code_point)(crate::host_value::<()>("host.call"));
            }
            return (XML_ENTITIES
                .iter()
                .find(|(key, _)| key == &(name).clone())
                .map(|(_, value)| value)
                .expect("TypeScript Record key was absent")
                .clone())
            .unwrap_or((_).clone());
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

// Source: upstream/packages/xml/src/xmlParse.ts:68 (sha256:2eec71b0fc6933bbc95bd293bffdfb962df6e919c9513a82c477b410974df169)
fn parse_element(src: String, state: &mut ParseState) -> Option<XmlElement> {
    skip_whitespace((src).clone(), state);
    if ((state.pos >= src.length) || (src[state.pos as usize].clone() != "<")) {
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
            src.length
        };
        return parse_element((src).clone(), state);
    }
    let name_start = state.pos;
    while ((state.pos < src.length)
        && (!(regex::RegexBuilder::new("[\\s>/]")
            .case_insensitive(false)
            .multi_line(false)
            .dot_matches_new_line(false)
            .build()
            .expect("upstream TypeScript regular expression must be valid Rust regex syntax"))
        .is_match(&(src[state.pos as usize].clone()))))
    {
        {
            state.pos += 1.0;
            state.pos
        };
    }
    let name = (src.slice)(name_start, state.pos);
    if (!name) {
        return None;
    }
    skip_whitespace((src).clone(), state);
    let mut attrs_str = "";
    let mut quote = "";
    while (state.pos < src.length) {
        let ch = src[state.pos as usize].clone();
        if quote {
            if (ch == quote) {
                quote = "".to_owned();
            }
        } else {
            if ((ch == "\"") || (ch == "'")) {
                quote = ch;
            } else {
                if ((ch == ">")
                    || ((ch == "/") && (src[(state.pos + 1.0_f64) as usize].clone() == ">")))
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
    let mut text = "";
    if (!self_closing) {
        while (state.pos < src.length) {
            skip_whitespace((src).clone(), state);
            if (state.pos >= src.length) {
                break;
            }
            if (src[state.pos as usize].clone() != "<") {
                let text_start = state.pos;
                while ((state.pos < src.length) && (src[state.pos as usize].clone() != "<")) {
                    {
                        state.pos += 1.0;
                        state.pos
                    };
                }
                text += decode_xml_entities(((src.slice)(text_start, state.pos).trim)());
                continue;
            }
            if (src[(state.pos + 1.0_f64) as usize].clone() == "/") {
                while ((state.pos < src.length) && (src[state.pos as usize].clone() != ">")) {
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
            }
        }
    }
    return Some(XmlElement {
        __flight_identity: std::sync::Arc::new(()),
        attributes: (attributes).clone(),
        children: (children).clone(),
        name: name,
        text: (text).clone(),
    });
}

// Source: upstream/packages/xml/src/xmlParse.ts:145 (sha256:8b165c706b68384ac4307222e85dcc19e0a696891ec44a55efdb1a50dacdcbda)
fn skip_whitespace(src: String, state: &mut ParseState) -> () {
    while ((state.pos < src.length)
        && (regex::RegexBuilder::new("\\s")
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

// Source: upstream/packages/xml/src/xmlParse.ts:149 (sha256:b2c29b1b97506f44d4752e1d1ade68734295aac2e3a260ec56abe80d87683558)
fn strip_cdata(xml: String) -> String {
    return {
        let mut __flight_replace =
            |m: String| -> String { (m.slice)(9.0_f64, (m.length - 3.0_f64)) };
        (regex::RegexBuilder::new("<!\\[CDATA\\[[\\s\\S]*?]]>")
            .case_insensitive(false)
            .multi_line(false)
            .dot_matches_new_line(false)
            .build()
            .expect("upstream TypeScript regular expression must be valid Rust regex syntax"))
        .replace_all(&(xml), |captures: &regex::Captures<'_>| {
            __flight_replace(
                captures
                    .get(0)
                    .map_or("", |matched| matched.as_str())
                    .to_owned(),
            )
        })
        .into_owned()
    };
}

// Source: upstream/packages/xml/src/xmlParse.ts:154 (sha256:343d495f2f22c546a51a41c548a6cb202b8343201c59b069d646af59bdb151ad)
fn strip_xml_comments(xml: String) -> String {
    return (regex::RegexBuilder::new("<!--[\\s\\S]*?-->")
        .case_insensitive(false)
        .multi_line(false)
        .dot_matches_new_line(false)
        .build()
        .expect("upstream TypeScript regular expression must be valid Rust regex syntax"))
    .replace_all(&(xml), "")
    .into_owned();
}
