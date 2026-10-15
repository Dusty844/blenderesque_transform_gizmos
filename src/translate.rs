use bevy::{camera::Camera, ecs::{hierarchy::ChildOf, system::{ParamSet, Query, Res, ResMut, Single}}, input::{ButtonInput, keyboard::KeyCode, mouse::AccumulatedMouseMotion}, math::{Dir3, Mat4, Quat, Vec3, primitives::InfinitePlane3d}, state::state::State, transform::{components::{GlobalTransform, Transform}, helper::TransformHelper}};

use crate::{AxisState, BlenderTransformKeybinds, BlenderTransformSettings, OrientationState, SelectedEntity, helper::{find_best_dir, orientation_rotation}};




pub(crate) fn translation(
    mouse_motion: Res<AccumulatedMouseMotion>,
    mut selected_entity: ResMut<SelectedEntity>,
    parent_q: Query<&ChildOf>,
    mut t_ps: ParamSet<(
        Query<(&mut Transform, Option<&ChildOf>)>,
        TransformHelper,
    )>,
    camera_s: Single<(&Camera, &GlobalTransform)>,
    axis_state: Res<State<AxisState>>,
    orientation_state: Res<State<OrientationState>>,
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
    let current_world_pos = current_transform.translation;

    
    let (camera, camera_gt) = *camera_s;

    selected_entity.accum_delta += mouse_motion.delta;
        
    let Ok(ray) = camera.viewport_to_world(camera_gt, selected_entity.start_vp + selected_entity.accum_delta) else {
        return;
    };

    let cam_normal = camera_gt.forward();
    let orient_rot = orientation_rotation(orientation_state.get(), &parent_gt, camera_gt);

    let normal = match axis_state.get() {
        
        AxisState::PlaneYZ => orient_rot * Dir3::X,
        AxisState::PlaneXZ => orient_rot * Dir3::Y,
        AxisState::PlaneXY => orient_rot * Dir3::Z,

        // Axis modes: rotate the perpendicular candidate set, then pick the best
        AxisState::X => {
            let candidates = [
                orient_rot * Dir3::Y,
                orient_rot * Dir3::NEG_Y,
                orient_rot * Dir3::Z,
                orient_rot * Dir3::NEG_Z,
            ];
            find_best_dir(cam_normal, candidates.to_vec())
        }
        AxisState::Y => {
            let candidates = [
                orient_rot * Dir3::X,
                orient_rot * Dir3::NEG_X,
                orient_rot * Dir3::Z,
                orient_rot * Dir3::NEG_Z,
            ];
            find_best_dir(cam_normal, candidates.to_vec())
        }
        AxisState::Z => {
            let candidates = [
                orient_rot * Dir3::X,
                orient_rot * Dir3::NEG_X,
                orient_rot * Dir3::Y,
                orient_rot * Dir3::NEG_Y,
            ];
            find_best_dir(cam_normal, candidates.to_vec())
        }

        _ => cam_normal,
    };

    let plane = InfinitePlane3d::new(normal);

    let Some(position) = ray.plane_intersection_point(current_world_pos, plane) else { return; };

    let mut offset = position - current_world_pos;

    let p_r = match orientation_state.get() {
        OrientationState::Local => parent_gt.rotation(),
        OrientationState::Global => Quat::IDENTITY,
        OrientationState::View => camera_gt.rotation(),
    };

    match axis_state.get() {
        AxisState::X => {
            let local = p_r.inverse() * offset;
            offset = p_r * Vec3::new(local.x, 0.0, 0.0);
        }
        AxisState::Y => {
            let local = p_r.inverse() * offset;
            offset = p_r * Vec3::new(0.0, local.y, 0.0);
        }
        AxisState::Z => {
            let local = p_r.inverse() * offset;
            offset = p_r * Vec3::new(0.0, 0.0, local.z);
        }
        _ => {}
    }
    current_transform.translation += offset;

    if keyboard_input.pressed(keybinds.grid_snap){
        let local_space = p_r.inverse() * (current_transform.translation - selected_entity.start_transform.translation);

        let stepped = (local_space / settings.translation_grid_snap).round() * settings.translation_grid_snap;

        current_transform.translation = p_r * (stepped + selected_entity.start_transform.translation);
    }

    let parent_affine = parent_gt.affine().inverse();
    let new_affine = parent_affine * current_transform.compute_affine();
    let new_t = Transform::from_matrix(
        Mat4::from_mat3_translation(new_affine.matrix3.into(), new_affine.translation.into())
    );

        
    if let Ok((mut t, _)) = t_ps.p0().get_mut(selected_entity.entity) {
        *t = new_t;
    }
                
}

