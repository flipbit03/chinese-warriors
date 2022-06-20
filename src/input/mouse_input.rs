use bevy::{prelude::*, render::camera::Camera2d};

#[derive(Component)]
pub struct HeroMoveToInstruction(pub Vec2);

pub struct MouseGlobalTranslation(pub Vec2);

// This system prints messages when you press or release the left mouse button:
pub fn global_mouse_position(
    mut commands: Commands,
    windows: Res<Windows>,
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
) {
    let (camera, camera_transform) = camera_query.single();
    let window = windows.get_primary().unwrap();

    if let Some(mouse_position) = window.cursor_position() {
        let window_size = Vec2::new(window.width(), window.height());

        // noooo idea how this works
        let ndc = (mouse_position / window_size) * 2.0 - Vec2::ONE;
        let ndc_to_world =
            camera_transform.compute_matrix() * camera.projection_matrix.inverse();
        let global_mouse_position =
            ndc_to_world.project_point3(ndc.extend(-1.0)).truncate();
        commands.insert_resource(MouseGlobalTranslation(global_mouse_position));
    }
}

// This system prints messages when you press or release the left mouse button:
pub fn mouse_left_click_to_hero_move_instruction(
    mut commands: Commands,
    mouse_global: Option<Res<MouseGlobalTranslation>>,
    mouse_button_input: Res<Input<MouseButton>>,
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
