use bevy::{
    prelude::App,
    window::{PresentMode, WindowDescriptor},
    DefaultPlugins,
};

pub fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Chinese Warriors".to_string(),
            width: 1280.0,
            height: 720.0,
            scale_factor_override: Some(1.0),
            present_mode: PresentMode::Fifo,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(chinese_warriors::camera::CameraPlugin)
        .add_plugin(chinese_warriors::assets::AssetsPlugin)
        .add_plugin(chinese_warriors::hero::HeroPlugin)
        .add_plugin(chinese_warriors::hud::HudPlugin)
        .add_plugin(chinese_warriors::console::ConsolePlugin)
        .add_plugin(chinese_warriors::world::WorldPlugin)
        .add_plugin(chinese_warriors::input::InputPlugin)
        .add_system(chinese_warriors::helpers::set_texture_filters_to_nearest)
        .run();
}
