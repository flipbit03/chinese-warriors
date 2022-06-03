use bevy::prelude::{App, Plugin};
use iyes_loopless::prelude::ConditionSet;

use crate::app::GameState;

use self::{camera::input_camera_scale, hero::hero_input, mouse_input::mouse_click_system};

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
                .with_system(mouse_click_system)
                .into(),
        );
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}

// impl Plugin for HudPlugin {
//     fn build(&self, app: &mut App) {
//         app
//         .add_enter_system(GameState::InGame, spawn_hud_text)
//         .add_system_set(
//             ConditionSet::new()
//                 .run_in_state(GameState::InGame)
//                 .with_system(update_hud_text)
//                 .with_system(update_window_title)
//                 .into()
//         );
//     }
// }
