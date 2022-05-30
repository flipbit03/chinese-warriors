use bevy::prelude::{App, Plugin};

use self::{camera::input_camera_scale, hero::hero_input};

pub mod camera;
pub mod hero;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(input_camera_scale).add_system(hero_input);
    }
}
