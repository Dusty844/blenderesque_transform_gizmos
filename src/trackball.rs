use bevy::{input::mouse::AccumulatedMouseMotion, prelude::*};

use crate::{SelectedEntity};



pub(crate) fn trackball_rotation(
    mouse_motion: Res<AccumulatedMouseMotion>,
    selected_entity: Res<SelectedEntity>,
    parent_q: Query<&ChildOf>,
    mut t_ps: ParamSet<(
        Query<(&mut Transform, Option<&ChildOf>)>,
        TransformHelper,
    )>,
){
    let parent_gt = if let Ok(parent) = parent_q.get(selected_entity.entity) {
        t_ps.p1().compute_global_transform(parent.0).unwrap_or_default()
    } else {
        GlobalTransform::default()
    };
    let mut current_transform = t_ps.p1().compute_global_transform(selected_entity.entity)
        .unwrap_or_default().compute_transform();

    current_transform.rotate_y(mouse_motion.delta.x * 0.01);
    current_transform.rotate_x(mouse_motion.delta.y * 0.01);

    let parent_affine = parent_gt.affine().inverse();
    let new_affine = parent_affine * current_transform.compute_affine();
    let new_t = Transform::from_matrix(
        Mat4::from_mat3_translation(new_affine.matrix3.into(), new_affine.translation.into())
    );

        
    if let Ok((mut t, _)) = t_ps.p0().get_mut(selected_entity.entity) {
        *t = new_t;
    }
        
}
