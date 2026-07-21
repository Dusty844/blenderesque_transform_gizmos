use bevy::{input::mouse::AccumulatedMouseMotion, prelude::*};

use crate::{AxisState, OrientationState, SelectedEntity, helper::{find_best_dir, orientation_rotation}};



pub(crate) fn rotation(
    mouse_motion: Res<AccumulatedMouseMotion>,
    selected_entity: Res<SelectedEntity>,
    parent_q: Query<&ChildOf>,
    mut t_ps: ParamSet<(
        Query<(&mut Transform, Option<&ChildOf>)>,
        TransformHelper,
    )>,
    camera_s: Single<(&Camera, &GlobalTransform)>,
    axis_state: Res<State<AxisState>>,
    orientation_state: Res<State<OrientationState>>,
    window: Single<&Window>,
) {
    let parent_gt = if let Ok(parent) = parent_q.get(selected_entity.0) {
        t_ps.p1().compute_global_transform(parent.0).unwrap_or_default()
    } else {
        GlobalTransform::default()
    };

    let mut current_transform = t_ps.p1().compute_global_transform(selected_entity.0)
    .unwrap_or_default().compute_transform();

    
    let (camera, camera_gt) = *camera_s;
    let Ok(centre) = camera.world_to_viewport(camera_gt, current_transform.translation) else {return; };
    let Some(cursor_pos) = window.physical_cursor_position() else {
        return;
    };
    let prev_cursor_pos = cursor_pos - mouse_motion.delta;

    let v0 = prev_cursor_pos - centre;
    let v1 = cursor_pos - centre;

    let a0 = v0.y.atan2(v0.x);
    let a1 = v1.y.atan2(v1.x);

    let delta = a1 - a0;

    let orient_rot = orientation_rotation(orientation_state.get(), &parent_gt, camera_gt);

    let axis = match axis_state.get() {
        AxisState::Camera => camera_gt.forward(),
        AxisState::X | AxisState::PlaneYZ => {
            let candidates = [orient_rot * Dir3::X, orient_rot * Dir3::NEG_X];
            find_best_dir(camera_gt.forward(), candidates.to_vec())
        }
        AxisState::Y | AxisState::PlaneXZ => {
            let candidates = [orient_rot * Dir3::Y, orient_rot * Dir3::NEG_Y];
            find_best_dir(camera_gt.forward(), candidates.to_vec())
        }
        AxisState::Z | AxisState::PlaneXY => {
            let candidates = [orient_rot * Dir3::Z, orient_rot * Dir3::NEG_Z];
            find_best_dir(camera_gt.forward(), candidates.to_vec())
        }
    };
    

    current_transform.rotate_axis(axis, delta);

    let parent_affine = parent_gt.affine().inverse();
    let new_affine = parent_affine * current_transform.compute_affine();
    let new_t = Transform::from_matrix(
        Mat4::from_mat3_translation(new_affine.matrix3.into(), new_affine.translation.into())
    );

        
    if let Ok((mut t, _)) = t_ps.p0().get_mut(selected_entity.0) {
        *t = new_t;
    }
        
    
}
