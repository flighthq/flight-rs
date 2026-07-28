// @generated from upstream/packages/font/src/fontShorthand.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/font/src/fontShorthand.ts:1 (sha256:ffa28b0e96af6db552489387057b87e1b00a39fa95062ec772e5ba90d4fb991d)
pub fn get_font_shorthand(family: String, style: Option<String>) -> String {
    let quoted = format!(
        "'{}'",
        (regex::RegexBuilder::new("[\\\\']")
            .case_insensitive(false)
            .multi_line(false)
            .dot_matches_new_line(false)
            .build()
            .expect("upstream TypeScript regular expression must be valid Rust regex syntax"))
        .replace_all(&(family), "\\$&")
        .into_owned()
    );
    return if ((style).is_some()) && (!((style) == Some("".to_owned()))) {
        format!("{} 1em {}", style, quoted)
    } else {
        format!("1em {}", quoted)
    };
}
