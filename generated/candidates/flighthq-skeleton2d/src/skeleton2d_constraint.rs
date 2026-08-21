// @generated from upstream/packages/skeleton2d/src/skeleton2dConstraint.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::{
    Skeleton2D, Skeleton2DConstraint, Skeleton2DConstraintKind, Skeleton2DConstraintSolver,
};

// Source: upstream/packages/skeleton2d/src/skeleton2dConstraint.ts:10 (sha256:47db1cd0df285640efb855c6957e851f83a83a6f8cae9442e72d04d832f15698)
pub fn get_skeleton2_d_constraint_solver(
    kind: Skeleton2DConstraintKind,
) -> Option<Skeleton2DConstraintSolver> {
    return (*_SOLVERS.lock().unwrap())
        .iter()
        .find(|(entry_key, _)| entry_key == &(kind).clone())
        .map(|(_, value)| value.clone());
}

// Source: upstream/packages/skeleton2d/src/skeleton2dConstraint.ts:18 (sha256:efd21172f93f7e8e3a8d425dd8c5eb7a50d3fb093bbdbf5d05152228b45eedf8)
pub fn register_skeleton2_d_constraint_solver(
    kind: Skeleton2DConstraintKind,
    solve: Skeleton2DConstraintSolver,
) -> () {
    {
        let __flight_key = (kind).clone();
        let __flight_value = (solve).clone();
        if let Some((_, value)) = (*_SOLVERS.lock().unwrap())
            .iter_mut()
            .find(|(key, _)| key == &__flight_key)
        {
            *value = __flight_value;
        } else {
            (*_SOLVERS.lock().unwrap()).push((__flight_key, __flight_value));
        }
    };
}

// Source: upstream/packages/skeleton2d/src/skeleton2dConstraint.ts:36 (sha256:9c74871bafa1254d6728c87d77125f94829d0c0e07cb1755bbcb0fc83e6dbc3a)
pub fn solve_skeleton2_d_constraints(
    skeleton: &Skeleton2D,
    constraints: &Vec<Skeleton2DConstraint>,
) -> () {
    {
        let mut i = 0.0_f64;
        while (i < (constraints.len() as f64)) {
            let constraint = constraints[i as usize].clone();
            let solve = (*_SOLVERS.lock().unwrap())
                .iter()
                .find(|(entry_key, _)| entry_key == &(constraint.kind).clone())
                .map(|(_, value)| value.clone());
            if (solve).is_none() {
                {
                    i += 1.0;
                    i
                };
                continue;
            }
            solve.as_ref().unwrap().lock().unwrap()((*skeleton).clone(), (constraint).clone());
            {
                i += 1.0;
                i
            };
        }
    }
}

// Source: upstream/packages/skeleton2d/src/skeleton2dConstraint.ts:48 (sha256:cf0c77e697ae0a1e4fdbcd108579816b0950cd063e6910a3aa7f58cdb3ea1aae)
pub fn unregister_skeleton2_d_constraint_solver(kind: Skeleton2DConstraintKind) -> () {
    {
        let __flight_key = (kind).clone();
        if let Some(__flight_index) = (*_SOLVERS.lock().unwrap())
            .iter()
            .position(|(key, _)| key == &__flight_key)
        {
            (*_SOLVERS.lock().unwrap()).remove(__flight_index);
            true
        } else {
            false
        }
    };
}

// Source: upstream/packages/skeleton2d/src/skeleton2dConstraint.ts:52 (sha256:1cd010b7f91a5e4efa4a1454439f589bce52efe60ebec4cbc5df55933fe9687c)
static _SOLVERS: std::sync::LazyLock<
    std::sync::Mutex<Vec<(Skeleton2DConstraintKind, Skeleton2DConstraintSolver)>>,
> = std::sync::LazyLock::new(|| std::sync::Mutex::new(Vec::new()));
