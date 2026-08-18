// @generated from upstream/packages/types/src/ShortcutKeyName.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/ShortcutKeyName.ts:10 (sha256:b068fb2b7c7b0ff77826a82786b2c69302cb02f43b677ba94fb4d828f3ca5ca7)
pub type ShortcutKeyName = crate::FlightUnion2<
    ShortcutDigitKeyName,
    crate::FlightUnion2<
        ShortcutEditingKeyName,
        crate::FlightUnion2<
            ShortcutFunctionKeyName,
            crate::FlightUnion2<
                ShortcutLetterKeyName,
                crate::FlightUnion2<
                    ShortcutLockKeyName,
                    crate::FlightUnion2<
                        ShortcutMediaKeyName,
                        crate::FlightUnion2<
                            ShortcutNavigationKeyName,
                            crate::FlightUnion2<ShortcutNumpadKeyName, ShortcutPunctuationKeyName>,
                        >,
                    >,
                >,
            >,
        >,
    >,
>;

// Source: upstream/packages/types/src/ShortcutKeyName.ts:21 (sha256:2f3dd29c86004f7d44a1cfffc2632d8b1435a6594d0b4488cdf98576c10ac5cd)
pub type ShortcutDigitKeyName = String;

// Source: upstream/packages/types/src/ShortcutKeyName.ts:23 (sha256:22520b0324251304f66a3f2020132b6f054f8136056c151913b461edf2db46eb)
pub type ShortcutEditingKeyName = String;

// Source: upstream/packages/types/src/ShortcutKeyName.ts:25 (sha256:42a3220c317303a1018e05f9d04c9c86065c2240f37cf239803ab87ce724d429)
pub type ShortcutFunctionKeyName = String;

// Source: upstream/packages/types/src/ShortcutKeyName.ts:51 (sha256:3d1bd706d20380e37467ef5cb177cd7124d23e80f3f5d47401f124281801003a)
pub type ShortcutLetterKeyName = String;

// Source: upstream/packages/types/src/ShortcutKeyName.ts:79 (sha256:f9423159b13aa40d73e361b79fcf867f75f9289175fe3122be680989f86b33de)
pub type ShortcutLockKeyName = String;

// Source: upstream/packages/types/src/ShortcutKeyName.ts:81 (sha256:efec2b2d01fee872698a486a3895f60f18bd54c60c24db63c8ab6be587cfe497)
pub type ShortcutMediaKeyName = String;

// Source: upstream/packages/types/src/ShortcutKeyName.ts:90 (sha256:f8aff1c690ff2ba874d6a596856fdd5e088a627b07645283cf2ff27e6db539e5)
pub type ShortcutNavigationKeyName = String;

// Source: upstream/packages/types/src/ShortcutKeyName.ts:100 (sha256:df33bb28b508f881bb2b9a2401ad56094ae0e0c8f4b2c28647c8230d17f07fb0)
pub type ShortcutNumpadKeyName = String;

// Source: upstream/packages/types/src/ShortcutKeyName.ts:118 (sha256:1f00a37fc3c524ce675513c6d1c47148fc24c992f5426d3c763cb6863f327dea)
pub type ShortcutPunctuationKeyName = String;
