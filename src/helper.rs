use bevy::prelude::*;

use crate::{AxisState, BlenderTransformSettings, OrientationState};

pub(crate) fn find_best_dir(
    camera: Dir3,
    possible_normals: Vec<Dir3>,
) -> Dir3{
    let mut current = possible_normals[0];
    let mut dot = camera.dot(possible_normals[0].as_vec3());

    for i in 1..possible_normals.len() {
        let new_dot = camera.dot(possible_normals[i].as_vec3());
        if new_dot > dot {
            current = possible_normals[i];
            dot = new_dot;
        }
    }
    return current;
}

pub(crate) fn orientation_rotation(
    orientation: &OrientationState,
    parent_gt: &GlobalTransform,
    camera_gt: &GlobalTransform,
) -> Quat {
    match orientation {
        OrientationState::Local => parent_gt.rotation(),
        OrientationState::Global => Quat::IDENTITY,
        OrientationState::View => camera_gt.rotation(),
    }
}


pub(crate) fn cycle_axis_states(
    first_state: AxisState,
    axis_state: &Res<State<AxisState>>,
    mut next_axis_state: Mut<NextState<AxisState>>,
    orientation_state: &Res<State<OrientationState>>,
    mut next_orientation_state: Mut<NextState<OrientationState>>,
    settings: &Res<BlenderTransformSettings>,
    
){
    if *axis_state.get() == first_state {
        if *orientation_state.get() == settings.default_orientation {
            if settings.default_orientation == OrientationState::Global {
                next_orientation_state.set(OrientationState::Local);
            } else {
                next_orientation_state.set(OrientationState::Global);
            }
        } else {
            next_axis_state.set(AxisState::Camera);
            next_orientation_state.set(settings.default_orientation);
        }

        
    } else {
        next_axis_state.set(first_state);
        next_orientation_state.set(settings.default_orientation);
    }
}
