// @generated from upstream/packages/skeleton3d/src/updateMeshSkin.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_types::Mesh;

// Source: upstream/packages/skeleton3d/src/updateMeshSkin.ts:19 (sha256:aa2ccb7aeaccc9503d2bd0f2a747e5ee60be56dd097910b0546b007e7f6fac1e)
pub fn update_mesh_skin(mesh: &Mesh) -> () {
    let skin = (mesh.skin).clone();
    if (skin == None).is_some() {
        return;
    }
    compute_skeleton3_d_joint_matrices(skin.skeleton);
    let geometry = &mesh.geometry;
    let mut bind_pose = get_mesh_geometry_skin_bind_pose(geometry);
    if (bind_pose).is_none() {
        bind_pose = capture_mesh_skin_bind_pose(geometry);
        set_mesh_geometry_skin_bind_pose(geometry, bind_pose);
    }
    skin_mesh_geometry(geometry, skin.skeleton, bind_pose);
}
