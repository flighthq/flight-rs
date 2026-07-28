// @generated from upstream/packages/easing/src/getEasingDerivative.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

// Source: upstream/packages/easing/src/getEasingDerivative.ts:15 (sha256:ffeaee626b1cb6d62284a0bec434ffb6782394e849c35683da5f38d2a5876531)
pub fn get_easing_derivative(
    ease: &mut impl FnMut(f64) -> f64,
    t: f64,
    epsilon: Option<f64>,
) -> f64 {
    let epsilon = epsilon.unwrap_or(DEFAULT_EPSILON);
    if (t <= epsilon) {
        return ((ease((epsilon * 2.0_f64)) - ease(0.0_f64)) / (epsilon * 2.0_f64));
    }
    if (t >= (1.0_f64 - epsilon)) {
        return ((ease(1.0_f64) - ease((1.0_f64 - (epsilon * 2.0_f64)))) / (epsilon * 2.0_f64));
    }
    return ((ease((t + epsilon)) - ease((t - epsilon))) / (2.0_f64 * epsilon));
}

// Source: upstream/packages/easing/src/getEasingDerivative.ts:30 (sha256:06ddd209d98231e3261db52aa12243efdb7ccc40d3be8d92d24f2fbd80b304ac)
const DEFAULT_EPSILON: f64 = 0.000001_f64;
