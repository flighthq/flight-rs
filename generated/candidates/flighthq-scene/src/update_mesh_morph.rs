// @generated from upstream/packages/scene/src/updateMeshMorph.ts; do not edit.
#![allow(clippy::excessive_precision)]
#![allow(non_upper_case_globals)]
#![allow(unused_braces)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_parens)]

use flighthq_mesh::{
    blend_mesh_geometry_morph, capture_mesh_morph_bind_pose, get_mesh_geometry_morph_bind_pose,
    set_mesh_geometry_morph_bind_pose,
};
use flighthq_types::Mesh;

// Source: upstream/packages/scene/src/updateMeshMorph.ts:21 (sha256:23716297f51b6f47179198b6180c77c4e4d872891d904412b5e7ba2361059a84)
pub fn update_mesh_morph(mesh: &mut Mesh) -> () {
    let morph = (mesh.morph).clone();
    if (morph).is_none() {
        return;
    }
    let mut bind_pose = get_mesh_geometry_morph_bind_pose(&mesh.geometry);
    if (bind_pose).is_none() {
        bind_pose = Some(capture_mesh_morph_bind_pose(&mesh.geometry));
        set_mesh_geometry_morph_bind_pose(&mut mesh.geometry, ((bind_pose).clone()).clone());
    }
    blend_mesh_geometry_morph(
        &mut mesh.geometry,
        morph.as_ref().unwrap(),
        bind_pose.as_mut().unwrap(),
    );
}
