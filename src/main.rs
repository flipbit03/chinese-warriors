use bevy::{prelude::*, window::PresentMode};

use plugins::{console::ConsolePlugin, hero::HeroPlugin, hud::HudPlugin, startup::StartupPlugin};
use systems::{helpers::set_texture_filters_to_nearest, input::InputPlugin, world::WorldPlugin};

//mod game;
mod plugins;
mod systems;

fn main() {
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
        .add_plugin(StartupPlugin)
        .add_plugin(HeroPlugin)
        .add_plugin(HudPlugin)
        .add_plugin(ConsolePlugin)
        .add_plugin(WorldPlugin)
        .add_plugin(InputPlugin)
        .add_system(set_texture_filters_to_nearest)
        .run();
}
