use bevy::{input::mouse::AccumulatedMouseMotion, prelude::*};

use crate::{BlenderTransformKeybinds, BlenderTransformSettings, SelectedEntity};



pub(crate) fn trackball_rotation(
    mouse_motion: Res<AccumulatedMouseMotion>,
    mut selected_entity: ResMut<SelectedEntity>,
    parent_q: Query<&ChildOf>,
    mut t_ps: ParamSet<(
        Query<(&mut Transform, Option<&ChildOf>)>,
        TransformHelper,
    )>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    settings: Res<BlenderTransformSettings>,
    keybinds: Res<BlenderTransformKeybinds>,

){
    let parent_gt = if let Ok(parent) = parent_q.get(selected_entity.entity) {
        t_ps.p1().compute_global_transform(parent.0).unwrap_or_default()
    } else {
        GlobalTransform::default()
    };

    let mut current_transform = selected_entity.start_transform;

    selected_entity.accum_delta += mouse_motion.delta * 0.05;

    let mut angle = selected_entity.accum_delta;

    if keyboard_input.pressed(keybinds.grid_snap){
        let snap = settings.trackball_grid_snap.to_radians();

        angle = (angle / snap).round() * snap;
    }

    current_transform.rotate_y(angle.x);
    current_transform.rotate_x(angle.y);

    let parent_affine = parent_gt.affine().inverse();
    let new_affine = parent_affine * current_transform.compute_affine();
    let new_t = Transform::from_matrix(
        Mat4::from_mat3_translation(new_affine.matrix3.into(), new_affine.translation.into())
    );

        
    if let Ok((mut t, _)) = t_ps.p0().get_mut(selected_entity.entity) {
        *t = new_t;
    }
        
}
