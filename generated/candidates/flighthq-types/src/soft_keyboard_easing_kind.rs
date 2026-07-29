// @generated from upstream/packages/types/src/SoftKeyboardEasingKind.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/types/src/SoftKeyboardEasingKind.ts:1 (sha256:b0437a253fa4fc4abe9a216d4a029cabe17499d12a045754d57fbe75b379375a)
// TypeScript value namespace SoftKeyboardEasingDefaultKind is represented by its generated Rust type.

// Source: upstream/packages/types/src/SoftKeyboardEasingKind.ts:2 (sha256:47faa01c334b4c2eee83b9a46d2faaee4ecd43b25b0b41e05ec5cef0e24a6f68)
pub type SoftKeyboardEasingDefaultKind = crate::OpaqueHostValue;

// Source: upstream/packages/types/src/SoftKeyboardEasingKind.ts:3 (sha256:356e0e45ee757527c007d504a7d516520ee95953d56b5efcde5d001ad360a4f8)
// TypeScript value namespace SoftKeyboardEasingEaseInKind is represented by its generated Rust type.

// Source: upstream/packages/types/src/SoftKeyboardEasingKind.ts:4 (sha256:b862a661f691f6dc784cd516cf26c45397229d387512af31758b1fc67e9db7da)
pub type SoftKeyboardEasingEaseInKind = crate::OpaqueHostValue;

// Source: upstream/packages/types/src/SoftKeyboardEasingKind.ts:5 (sha256:a8a2f647328597e0d2251c4ac70b0af05243f7c046d8f9f4a8c9398f4e813123)
// TypeScript value namespace SoftKeyboardEasingEaseOutKind is represented by its generated Rust type.

// Source: upstream/packages/types/src/SoftKeyboardEasingKind.ts:6 (sha256:7ad088d5794809870e3517c53feda9f78c53a7e2709b9138ff3c2efbf93da194)
pub type SoftKeyboardEasingEaseOutKind = crate::OpaqueHostValue;

// Source: upstream/packages/types/src/SoftKeyboardEasingKind.ts:7 (sha256:f7c48de53598ee3af6efb93bdfb2ea6e94787d49a26959516c98e13378c565aa)
// TypeScript value namespace SoftKeyboardEasingLinearKind is represented by its generated Rust type.

// Source: upstream/packages/types/src/SoftKeyboardEasingKind.ts:8 (sha256:9fde60da5cd704e6523bc22e8ff32d54f04d5bebc3776b0cdfa87725fc7431e0)
pub type SoftKeyboardEasingLinearKind = crate::OpaqueHostValue;

// Source: upstream/packages/types/src/SoftKeyboardEasingKind.ts:9 (sha256:9e0a2c92a944090780c3ad3395339268ae966bf75a7825c217182c83a9759eeb)
// TypeScript value namespace SoftKeyboardEasingKeyboardDefaultKind is represented by its generated Rust type.

// Source: upstream/packages/types/src/SoftKeyboardEasingKind.ts:10 (sha256:3e80e252f50e47fd96d527c04c232d78d2708ea9219cf94f86fd920fe1624995)
pub type SoftKeyboardEasingKeyboardDefaultKind = crate::OpaqueHostValue;

// Source: upstream/packages/types/src/SoftKeyboardEasingKind.ts:11 (sha256:ecad8adae116529ef97375ebe808fc4c51f2faeb8a486984438c965531d5df2e)
pub type SoftKeyboardEasingKind = crate::FlightUnion2<
    SoftKeyboardEasingDefaultKind,
    crate::FlightUnion2<
        SoftKeyboardEasingEaseInKind,
        crate::FlightUnion2<
            SoftKeyboardEasingEaseOutKind,
            crate::FlightUnion2<
                SoftKeyboardEasingLinearKind,
                SoftKeyboardEasingKeyboardDefaultKind,
            >,
        >,
    >,
>;
