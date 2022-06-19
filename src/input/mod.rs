use bevy::{
    input::system::exit_on_esc_system,
    prelude::{App, Plugin},
};
use iyes_loopless::prelude::ConditionSet;

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
        app.add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::InGame)
                .with_system(input_camera_scale)
                .with_system(hero_input)
                .with_system(global_mouse_position)
                .with_system(mouse_left_click_to_hero_move_instruction)
                .with_system(exit_on_esc_system)
                .into(),
        );
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}
