use bevy::{prelude::*, window::PresentMode};
use bevy_aseprite_ultra::prelude::AsepriteUltraPlugin;
use chinese_warriors::assets::config::structs::CwCliConfig;

pub fn main() {
    let mut app = App::new();

    // Parse CLI args (skip on WASM)
    #[cfg(not(target_arch = "wasm32"))]
    {
        use clap::Parser;
        let args = CwCliConfig::parse();
        app.insert_resource(args);
    }

    #[cfg(target_arch = "wasm32")]
    {
        app.insert_resource(CwCliConfig {
            config_file: "config/world.config.ron".to_string(),
        });
    }

    app.add_plugins(
        DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Chinese Warriors".to_string(),
                    present_mode: PresentMode::AutoVsync,
                    #[cfg(target_arch = "wasm32")]
                    canvas: Some("#gamescreen".to_string()),
                    #[cfg(target_arch = "wasm32")]
                    fit_canvas_to_parent: true,
                    ..default()
                }),
                ..default()
            }),
    )
    .init_state::<chinese_warriors::app::GameState>()
    .add_plugins(AsepriteUltraPlugin)
    .add_plugins((
        chinese_warriors::assets::AssetsPlugin,
        chinese_warriors::camera::CameraPlugin,
        chinese_warriors::hero::HeroPlugin,
        chinese_warriors::hud::HudPlugin,
        chinese_warriors::console::ConsolePlugin,
        chinese_warriors::world::WorldPlugin,
        chinese_warriors::input::InputPlugin,
        chinese_warriors::shadow::ShadowsPlugin,
    ))
    .run();
}
