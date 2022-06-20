use bevy::{
    prelude::{App, Msaa},
    window::{PresentMode, WindowDescriptor},
    DefaultPlugins,
};
use bevy_ase::loader::AseLoaderDefaultPlugin;
use chinese_warriors::{app::GameState, assets::config::structs::CwCliConfig};
use clap::Parser;
use iyes_loopless::prelude::AppLooplessStateExt;

pub fn main() {
    let app = &mut App::new();

    // CwCliConfig::;
    let args = CwCliConfig::parse();

    #[cfg(target_arch = "wasm32")]
    {
        app.add_plugin(bevy_web_resizer::Plugin);
    }

    app.insert_resource(Msaa { samples: 1 })
        .insert_resource(WindowDescriptor {
            title: "Chinese Warriors".to_string(),
            scale_factor_override: Some(1.0),
            present_mode: PresentMode::Fifo,
            #[cfg(target_arch = "wasm32")]
            canvas: Some("#gamescreen".to_string()),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .insert_resource(args)
        .add_system(chinese_warriors::helpers::set_texture_filters_to_nearest)
        .add_plugin(AseLoaderDefaultPlugin)
        .add_plugin(benimator::AnimationPlugin::default())
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
