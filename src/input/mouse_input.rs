use bevy::prelude::*;

#[derive(Resource)]
pub struct HeroMoveToInstruction(pub Vec2);

#[derive(Resource)]
pub struct MouseGlobalTranslation(pub Vec2);

pub fn global_mouse_position(
    mut commands: Commands,
    window_query: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
) {
    let Ok((camera, camera_transform)) = camera_query.single() else {
        return;
    };
    let Ok(window) = window_query.single() else {
        return;
    };

    if let Some(cursor_position) = window.cursor_position() {
        if let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_position) {
            commands.insert_resource(MouseGlobalTranslation(world_pos));
        }
    }
}

pub fn mouse_left_click_to_hero_move_instruction(
    mut commands: Commands,
    mouse_global: Option<Res<MouseGlobalTranslation>>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
) {
    if let Some(p) = mouse_global {
        if mouse_button_input.just_pressed(MouseButton::Left) {
            commands.insert_resource(HeroMoveToInstruction(Vec2::new(
                p.0.x.round(),
                p.0.y.round(),
            )));
        }
    }
}
