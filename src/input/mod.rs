use bevy::prelude::*;

use crate::app::GameState;

use self::{
    camera::input_camera_scale,
    hero::hero_input,
    mouse_input::{global_mouse_position, mouse_left_click_to_hero_move_instruction},
};

pub mod camera;
pub mod hero;
pub mod mouse_input;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                input_camera_scale,
                hero_input,
                global_mouse_position,
                mouse_left_click_to_hero_move_instruction,
            )
                .run_if(in_state(GameState::InGame)),
        );

        #[cfg(not(target_arch = "wasm32"))]
        {
            app.add_systems(Update, exit_on_esc);
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn exit_on_esc(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut exit: MessageWriter<bevy::app::AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        exit.write(bevy::app::AppExit::Success);
    }
}
