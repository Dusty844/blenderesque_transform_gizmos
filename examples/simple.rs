use bevy::{
    dev_tools::infinite_grid::{InfiniteGrid, InfiniteGridPlugin, InfiniteGridSettings}, input::common_conditions::input_toggle_active, prelude::*, window::{CursorGrabMode, CursorOptions}
};



use bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use blenderesque_transform_gizmos::*;


fn main() {
    App::new()
        .add_plugins((DefaultPlugins, BlenderTransformGizmoPlugin, InfiniteGridPlugin))
        .add_plugins(EguiPlugin::default())
        .add_plugins(WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Tab)))
        .add_systems(Startup, (scene.spawn(), toggle_cursor))
        .run();
}

/// set up a simple 3D scene
fn scene() -> impl SceneList {
    bsn_list! [
        (
            #CircularBase
            Mesh3d(asset_value(Circle::new(4.0)))
            MeshMaterial3d::<StandardMaterial>(asset_value(Color::WHITE))
            Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2))
        ),
        (
            #Cube
            Mesh3d(asset_value(Cuboid::new(1.0, 1.0, 1.0)))
            MeshMaterial3d::<StandardMaterial>(asset_value(Color::srgb_u8(68, 92, 255)))
            Transform::from_xyz(0.0, 0.5, 0.0)
        ),
        (
            #Cylinder
            Mesh3d(asset_value(Cylinder::new(0.5, 1.0)))
            MeshMaterial3d::<StandardMaterial>(asset_value(Color::srgb_u8(124, 144, 255)))
            Transform::from_xyz(1.0, 0.5, 3.0)
            Children [
                #Cube2
                Mesh3d(asset_value(Cuboid::new(1.0, 1.0, 1.0)))
                MeshMaterial3d::<StandardMaterial>(asset_value(Color::srgb_u8(124, 112, 155)))
                Transform::from_xyz(1.0, 2.5, 1.0)
            ]
        ),
        (
            PointLight {
                shadow_maps_enabled: true,
            }
            Transform::from_xyz(4.0, 8.0, 4.0)
        ),
        (
            Camera3d
            template_value(Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y))
        ),
        (
            InfiniteGrid
            InfiniteGridSettings::default()
        )
    ]
}


fn toggle_cursor(mut cursor_options: Single<&mut CursorOptions>) {
    cursor_options.grab_mode = CursorGrabMode::Confined;
}
