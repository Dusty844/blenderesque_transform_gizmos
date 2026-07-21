use bevy::prelude::*;
use bevy_mod_outline::{OutlinePlugin, OutlineVolume};

use crate::{gizmos::{AxesGizmos, draw_gizmo_axes, set_gizmo_settings}, helper::cycle_axis_states, rotate::rotation, trackball::trackball_rotation, translate::translation};

mod translate;
mod trackball;
mod rotate;
pub(crate) mod helper;
pub mod gizmos;

pub struct BlenderTransformGizmoPlugin;

impl Plugin for BlenderTransformGizmoPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((MeshPickingPlugin, OutlinePlugin::JUMP_FLOOD));
        
        app.insert_resource(ClickList::default());
        app.insert_resource(SelectedEntity::default());
        app.insert_resource(BlenderTransformSettings::default());
        app.insert_resource(BlenderTransformKeybinds::default());

        app.init_state::<BlenderTransformGizmoState>();
        app.init_state::<AxisState>();
        app.init_state::<OrientationState>();
        
        app.add_systems(Update, switch_main_state);

        app.add_systems(Update,
            switch_axis_state.run_if(
                in_state(BlenderTransformGizmoState::None).nor_eager(in_state(BlenderTransformGizmoState::Trackball))
        ));

        app.add_systems(OnEnter(BlenderTransformGizmoState::Translate), reset_selected);
        app.add_systems(OnEnter(BlenderTransformGizmoState::Rotate), reset_selected);
        app.add_systems(OnEnter(BlenderTransformGizmoState::Trackball), reset_selected);
        app.add_systems(OnEnter(BlenderTransformGizmoState::Scale), reset_selected);

        app.add_systems(OnExit(BlenderTransformGizmoState::Translate), reset_transform);
        app.add_systems(OnExit(BlenderTransformGizmoState::Rotate), reset_transform);
        app.add_systems(OnExit(BlenderTransformGizmoState::Trackball), reset_transform);
        app.add_systems(OnExit(BlenderTransformGizmoState::Scale), reset_transform);

        app.add_systems(OnExit(AxisState::Camera), reset_transform);
        app.add_systems(OnExit(AxisState::X), reset_transform);
        app.add_systems(OnExit(AxisState::Y), reset_transform);
        app.add_systems(OnExit(AxisState::Z), reset_transform);
        app.add_systems(OnExit(AxisState::PlaneYZ), reset_transform);
        app.add_systems(OnExit(AxisState::PlaneXZ), reset_transform);
        app.add_systems(OnExit(AxisState::PlaneXY), reset_transform);

        app.add_systems(OnExit(OrientationState::Global), reset_transform);
        app.add_systems(OnExit(OrientationState::Local), reset_transform);
        app.add_systems(OnExit(OrientationState::View), reset_transform);


        app.add_systems(Update, translation.run_if(
            in_state(BlenderTransformGizmoState::Translate)
        ));

        app.add_systems(Update, trackball_rotation.run_if(
            in_state(BlenderTransformGizmoState::Trackball)
        ));

        app.add_systems(Update, rotation.run_if(
            in_state(BlenderTransformGizmoState::Rotate)
        ));

        app.add_systems(Last, handle_clicklist.run_if(
            in_state(BlenderTransformGizmoState::None))
        );

        app.add_systems(Update, draw_gizmo_axes); // not sure if it's best here
        app.add_systems(Startup, set_gizmo_settings);

        app.init_gizmo_group::<AxesGizmos>();
        app.add_observer(selectable);
        

    }
}


#[derive(Resource, Clone, Copy, Reflect)]
#[reflect(Resource)]
pub struct SelectedEntity(pub Entity, pub Transform, pub Vec2);

impl Default for SelectedEntity{
    fn default() -> Self {
        Self(Entity::PLACEHOLDER, Transform::default(), Vec2::ZERO)
    }
}

#[derive(Resource, Clone, Copy, Reflect)]
#[reflect(Resource)]
pub struct BlenderTransformKeybinds{
    pub translation_key: KeyCode,
    pub rotation_key: KeyCode,
    pub scale_key: KeyCode,
    pub x_axis_key: KeyCode,
    pub y_axis_key: KeyCode,
    pub z_axis_key: KeyCode,
    pub invert_key: KeyCode,
    pub grid_snap_toggle: KeyCode,
    pub exit_key: KeyCode,
    pub confirm_key: KeyCode,
}

impl Default for BlenderTransformKeybinds{
    fn default() -> Self {
        Self{
            translation_key: KeyCode::KeyG,
            rotation_key: KeyCode::KeyR,
            scale_key: KeyCode::KeyS,
            x_axis_key: KeyCode::KeyX,
            y_axis_key: KeyCode::KeyY,
            z_axis_key: KeyCode::KeyZ,
            invert_key: KeyCode::ShiftLeft,
            grid_snap_toggle: KeyCode::ControlLeft,
            exit_key: KeyCode::Escape,
            confirm_key: KeyCode::Enter,
        }
    }
}

#[derive(Resource, Clone, Copy, Reflect)]
#[reflect(Resource)]
pub struct BlenderTransformSettings{
    pub default_orientation: OrientationState,
}

impl Default for BlenderTransformSettings {
    fn default() -> Self {
        Self{
            default_orientation: OrientationState::Global,
        }
    }
}


#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States, Reflect)]
pub enum BlenderTransformGizmoState{
    #[default]
    None,  
    Translate,
    Rotate,
    Scale,
    Trackball,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States, Reflect)]
pub enum AxisState{
    #[default]
    Camera,
    X,
    Y,
    Z,
    PlaneYZ,
    PlaneXZ,
    PlaneXY,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States, Reflect)]
pub enum OrientationState{
    #[default]
    Global,
    Local,
    View,
}

#[derive(Resource, Clone, Reflect, Default)]
struct ClickList(
    Vec<Entity>,
);


fn selectable(
    click: On<Pointer<Click>>,
    mut clicklist: ResMut<ClickList>)
{
    clicklist.0.push(click.entity);
}

fn handle_clicklist(
    mut clicklist: ResMut<ClickList>,
    mut selected_entity: ResMut<SelectedEntity>,
    t_q: Query<&GlobalTransform>,
    mut next_transform_state: ResMut<NextState<BlenderTransformGizmoState>>,
    window_q: Single<Entity, With<Window>>,
    mut commands: Commands,
) {
    let mut current = Entity::PLACEHOLDER;
    let mut transform = Transform::default();
    if clicklist.0.len() != 0 {
        for clicked_entity in &clicklist.0 {
            println!("clicked entity {}", clicked_entity);
            if let Ok(t) = t_q.get(*clicked_entity) {
                transform = t.compute_transform();
                current = *clicked_entity;
                break; 
            }
            if window_q.index() == clicked_entity.index() {
                if current == Entity::PLACEHOLDER {
                    current = Entity::PLACEHOLDER;
                } else {
                    continue;
                } 
            }
        }
        if selected_entity.0 != Entity::PLACEHOLDER {
            commands.entity(selected_entity.0).try_remove::<OutlineVolume>();
        }
        selected_entity.0 = current;
        commands.entity(selected_entity.0).try_insert(OutlineVolume{
            visible: true,
            width: 2.0,
            colour: Color::srgb(1.0, 1.0, 0.0),
        });
        selected_entity.1 = transform;
        next_transform_state.set(BlenderTransformGizmoState::None);
        clicklist.0.clear();

        
    }
    
}





fn switch_main_state(
    main_state: Res<State<BlenderTransformGizmoState>>,
    mut next_transform_state: ResMut<NextState<BlenderTransformGizmoState>>,
    mut next_axis_state: ResMut<NextState<AxisState>>,
    mut selected_entity: ResMut<SelectedEntity>,
    mut next_orientation_state: ResMut<NextState<OrientationState>>,
    t_q: Query<&GlobalTransform>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    keybinds: Res<BlenderTransformKeybinds>,
    settings: Res<BlenderTransformSettings>,
    window: Single<&Window>,
    camera_s: Single<(&Camera, &GlobalTransform)>,

){
    type MainState = BlenderTransformGizmoState;
    if keyboard_input.just_pressed(keybinds.translation_key)  && *main_state.get() != MainState::Translate {
        println!("started translation");
        next_transform_state.set(MainState::Translate);
        next_axis_state.set(AxisState::Camera);
        next_orientation_state.set(settings.default_orientation);
        let Ok(obj) = camera_s.0.world_to_viewport(camera_s.1, selected_entity.1.translation) else { return; };

        let Some(cursor) = window.cursor_position() else { return; };
        selected_entity.2 = obj - cursor;
        

    }
    if keyboard_input.just_pressed(keybinds.rotation_key) && *main_state.get() != MainState::Trackball {
        if *main_state.get() == BlenderTransformGizmoState::Rotate {
            next_transform_state.set(MainState::Trackball);
            next_axis_state.set(AxisState::Camera);
            next_orientation_state.set(settings.default_orientation);

            println!("started trackball");
        } else {
            println!("started rotation");
            next_transform_state.set(MainState::Rotate);
            next_axis_state.set(AxisState::Camera);
            next_orientation_state.set(settings.default_orientation);


        }
    }
    if keyboard_input.just_pressed(keybinds.scale_key) && *main_state.get() != MainState::Scale {
        next_transform_state.set(MainState::Scale);
        next_axis_state.set(AxisState::Camera);
        next_orientation_state.set(settings.default_orientation);
        println!("started scale");
    }
    if keyboard_input.just_pressed(keybinds.exit_key) {
        next_transform_state.set(MainState::None);
        next_axis_state.set(AxisState::Camera);
        next_orientation_state.set(settings.default_orientation);
        println!("cancelled");
    }
    if (keyboard_input.just_pressed(keybinds.confirm_key)  ||
        mouse_button_input.just_pressed(MouseButton::Left))
        && *main_state.get() != MainState::None {
        if let Ok(t) = t_q.get(selected_entity.0) {
            selected_entity.1 = t.compute_transform();
            next_transform_state.set(MainState::None);
            next_axis_state.set(AxisState::Camera);
            next_orientation_state.set(settings.default_orientation);
            println!("confirmed");
        }
    }
    
}

fn switch_axis_state(
    axis_state: Res<State<AxisState>>,
    orientation_state: Res<State<OrientationState>>,
    mut next_axis_state: ResMut<NextState<AxisState>>,
    mut next_orientation_state: ResMut<NextState<OrientationState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    keybinds: Res<BlenderTransformKeybinds>,
    settings: Res<BlenderTransformSettings>,
){
    //slightly less verbose, i might even want to move the keyboard stuff there too.
    if keyboard_input.just_pressed(keybinds.x_axis_key){
        if keyboard_input.pressed(keybinds.invert_key){
            cycle_axis_states(
                AxisState::PlaneYZ,
                &axis_state,
                next_axis_state.reborrow(),
                &orientation_state,
                next_orientation_state.reborrow(),
                &settings
            );
        } else {
            cycle_axis_states(
                AxisState::X,
                &axis_state,
                next_axis_state.reborrow(),
                &orientation_state,
                next_orientation_state.reborrow(),
                &settings
            );
        }
    }
    if keyboard_input.just_pressed(keybinds.y_axis_key){
        if keyboard_input.pressed(keybinds.invert_key){
            cycle_axis_states(
                AxisState::PlaneXZ,
                &axis_state,
                next_axis_state.reborrow(),
                &orientation_state,
                next_orientation_state.reborrow(),
                &settings
            );
        } else {
            cycle_axis_states(
                AxisState::Y,
                &axis_state,
                next_axis_state.reborrow(),
                &orientation_state,
                next_orientation_state.reborrow(),
                &settings
            );
        }
    }
    if keyboard_input.just_pressed(keybinds.z_axis_key){
        if keyboard_input.pressed(keybinds.invert_key){
            cycle_axis_states(
                AxisState::PlaneXY,
                &axis_state,
                next_axis_state.reborrow(),
                &orientation_state,
                next_orientation_state.reborrow(),
                &settings
            );
        } else {
            cycle_axis_states(
                AxisState::Z,
                &axis_state,
                next_axis_state.reborrow(),
                &orientation_state,
                next_orientation_state.reborrow(),
                &settings
            );
        }
    }
}

fn reset_selected(
    mut selected_entity: ResMut<SelectedEntity>,
    t_q: TransformHelper,
) {
    if let Ok(t) = t_q.compute_global_transform(selected_entity.0) {
        selected_entity.1 = t.compute_transform();
    }
    
}

fn reset_transform(
    selected_entity: Res<SelectedEntity>,
    mut t_ps: ParamSet<(
        Query<(&mut Transform, Option<&ChildOf>)>,
        TransformHelper,
    )>,
    parent_q: Query<&ChildOf>,
    
) {
    if let Ok(parent) = parent_q.get(selected_entity.0) {
        
        let parent_gt = t_ps.p1().compute_global_transform(parent.0).unwrap_or_default();
        let parent_affine = parent_gt.affine().inverse();
        let new_affine = parent_affine * selected_entity.1.compute_affine();
        let new_t = Transform::from_matrix(
            Mat4::from_mat3_translation(new_affine.matrix3.into(), new_affine.translation.into())
        );

        
        if let Ok((mut t, _)) = t_ps.p0().get_mut(selected_entity.0) {
            *t = new_t;
        }
    } else {
        if let Ok((mut t, _)) = t_ps.p0().get_mut(selected_entity.0) {
            *t = selected_entity.1;
            
        }
    }
}
