use bevy::{
    prelude::{App, Msaa},
    window::{PresentMode, WindowDescriptor},
    DefaultPlugins,
};
use chinese_warriors::app::GameState;
use iyes_loopless::prelude::AppLooplessStateExt;

pub fn main() {
    App::new()
        .insert_resource(Msaa { samples: 1 })
        .insert_resource(WindowDescriptor {
            title: "Chinese Warriors".to_string(),
            width: 1280.0,
            height: 720.0,
            scale_factor_override: Some(1.0),
            present_mode: PresentMode::Fifo,
            ..Default::default()
        })
        .add_system(chinese_warriors::helpers::set_texture_filters_to_nearest)
        .add_plugins(DefaultPlugins)
        .add_loopless_state(GameState::Loading)
        .add_plugin(chinese_warriors::assets::AssetsPlugin)
        .add_plugin(chinese_warriors::camera::CameraPlugin)
        .add_plugin(chinese_warriors::hero::HeroPlugin)
        .add_plugin(chinese_warriors::hud::HudPlugin)
        .add_plugin(chinese_warriors::console::ConsolePlugin)
        .add_plugin(chinese_warriors::world::WorldPlugin)
        .add_plugin(chinese_warriors::input::InputPlugin)
        .run();
}
