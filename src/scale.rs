use bevy::{input::mouse::AccumulatedMouseMotion, prelude::*};

use crate::{AxisState, OrientationState, SelectedEntity};



pub(crate) fn scaling(
    mouse_motion: Res<AccumulatedMouseMotion>,
    mut selected_entity: ResMut<SelectedEntity>,
    parent_q: Query<&ChildOf>,
    mut t_ps: ParamSet<(
        Query<(&mut Transform, Option<&ChildOf>)>,
        TransformHelper,
    )>,
    window: Single<&Window>,
    camera_s: Single<(&Camera, &GlobalTransform)>,
    axis_state: Res<State<AxisState>>,
    orientation_state: Res<State<OrientationState>>,

){
    let parent_gt = if let Ok(parent) = parent_q.get(selected_entity.entity) {
        t_ps.p1().compute_global_transform(parent.0).unwrap_or_default()
    } else {
        GlobalTransform::default()
    };

    let mut current_transform = selected_entity.start_transform;


    let (_, camera_gt) = *camera_s;
    let centre = selected_entity.start_vp;
    let Some(cursor_pos) = window.physical_cursor_position() else {
        return;
    };
    let prev_cursor_pos = cursor_pos - mouse_motion.delta;

    let dist = (cursor_pos - centre).length();
    let prev_dist = (prev_cursor_pos - centre).length();

    // Avoid dividing by ~0 when the cursor is right on the centre
    if prev_dist < 0.1 {
        selected_entity.accum_delta.x *= 1.0;
    } else {
        selected_entity.accum_delta.x *= dist / prev_dist; 
    }



    let factor = selected_entity.accum_delta.x;

    let p_r = match orientation_state.get() {
        OrientationState::Local => current_transform.rotation, 
        OrientationState::Global => Quat::IDENTITY,
        OrientationState::View => camera_gt.rotation(),
    };

    let k = match axis_state.get() {
        AxisState::X => vec3(factor, 1.0, 1.0),
        AxisState::Y => vec3(1.0, factor, 1.0),
        AxisState::Z => vec3(1.0, 1.0, factor),
        AxisState::PlaneYZ => vec3(1.0, factor, factor),
        AxisState::PlaneXZ => vec3(factor, 1.0, factor),
        AxisState::PlaneXY => vec3(factor, factor, 1.0),
        _ => Vec3::splat(factor),
    };

    // World-space scaling matrix
    let f = Mat3::from_quat(p_r);
    let g = f * Mat3::from_diagonal(k) * f.transpose();

    // Object's current linear part
    let m = Mat3::from_quat(current_transform.rotation)
        * Mat3::from_diagonal(current_transform.scale);

    let m2 = g * m;
    let signs = current_transform.scale.signum(); 
    current_transform.scale = vec3(
        m2.x_axis.length(),
        m2.y_axis.length(),
        m2.z_axis.length(),
    ) * signs;

    let parent_affine = parent_gt.affine().inverse();
    let new_affine = parent_affine * current_transform.compute_affine();
    let new_t = Transform::from_matrix(
        Mat4::from_mat3_translation(new_affine.matrix3.into(), new_affine.translation.into())
    );

        
    if let Ok((mut t, _)) = t_ps.p0().get_mut(selected_entity.entity) {
        *t = new_t;
    }

}
