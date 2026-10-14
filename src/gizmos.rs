use bevy::prelude::*;

use crate::{AxisState, OrientationState, SelectedEntity};


#[derive(Default, Reflect, GizmoConfigGroup)]
pub(crate) struct AxesGizmos;


pub(crate) fn draw_gizmo_axes(
    state: Res<State<AxisState>>,
    selected_entity: Res<SelectedEntity>,
    parent_q: Query<&ChildOf>,
    t_q: Query<&GlobalTransform>,
    mut gizmos: Gizmos<AxesGizmos>,
    orientation_state: Res<State<OrientationState>>,
    cam: Single<(&Camera, &GlobalTransform)>
){
    let pos = selected_entity.start_transform.translation;

    let x_axis_color = Color::oklcha(0.5232, 0.1404, 13.84, 1.0);   // red
    let y_axis_color = Color::oklcha(0.5050, 0.1315, 145.0, 1.0);  // green
    let z_axis_color = Color::oklcha(0.4847, 0.1249, 253.08, 1.0);  // blue

    let mut x_dir = Vec3::X;
    let mut y_dir = Vec3::Y;
    let mut z_dir = Vec3::Z;

    match orientation_state.get() {
        OrientationState::Global => {}
        OrientationState::Local => {
            if let Ok(parent) = parent_q.get(selected_entity.entity) {
                if let Ok(p_t) = t_q.get(parent.0) {
                    x_dir = p_t.rotation() * x_dir;
                    y_dir = p_t.rotation() * y_dir;
                    z_dir = p_t.rotation() * z_dir;
                    
                }
            }
        }
        OrientationState::View => {
            x_dir = cam.1.rotation() * x_dir;
            y_dir = cam.1.rotation() * y_dir;
            z_dir = cam.1.rotation() * z_dir;

        }
    }


    match state.get() {
        AxisState::X => {
            gizmos.line(pos + x_dir * 1000.0, pos - x_dir * 1000.0,
                x_axis_color);
        },
        AxisState::Y => {
            gizmos.line(pos + y_dir * 1000.0, pos - y_dir * 1000.0,
                y_axis_color);
        },
        AxisState::Z => {
            gizmos.line(pos + z_dir * 1000.0, pos - z_dir * 1000.0,
                z_axis_color);
        },
        AxisState::PlaneYZ => {
            gizmos.line(pos + y_dir * 1000.0, pos - y_dir * 1000.0,
                y_axis_color);
            gizmos.line(pos + z_dir * 1000.0, pos - z_dir * 1000.0,
                z_axis_color);
        },
        AxisState::PlaneXZ => {
            gizmos.line(pos + x_dir * 1000.0, pos - x_dir * 1000.0,
                x_axis_color);
            gizmos.line(pos + z_dir * 1000.0, pos - z_dir * 1000.0,
                z_axis_color);
        },
        AxisState::PlaneXY => {
            gizmos.line(pos + x_dir * 1000.0, pos - x_dir * 1000.0,
                x_axis_color);
            gizmos.line(pos + y_dir * 1000.0, pos - y_dir * 1000.0,
                y_axis_color);
        },

        _ => {}
        
    }
    
}

pub(crate) fn set_gizmo_settings(
    mut config_store: ResMut<GizmoConfigStore>,
){
    config_store.config_mut::<AxesGizmos>().0.depth_bias = -1.;
}
