// use application::{Application, ApplicationInputs};

// mod application;
// mod game;
// mod util;

use bevy::{prelude::*, window::PresentMode};
use game::{
    hero::{animate_hero, hero_input, spawn_hero},
    setup::setup,
    terrain::{despawn_terrain, frustum_culling::update_world_frustum, spawn_terrain},
    timed_console_stats::console_stats,
    window_title::window_title_resolution,
};

mod game;
mod systems;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Chinese Warriors".to_string(),
            scale_factor_override: Some(1.0),
            present_mode: PresentMode::Fifo,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system_to_stage(StartupStage::PreStartup, setup)
        .add_startup_system(spawn_hero)
        .add_system(animate_hero)
        .add_system(spawn_terrain)
        .add_system(despawn_terrain)
        .add_system(hero_input)
        .add_system(console_stats)
        .add_system(update_world_frustum)
        .add_system(window_title_resolution)
        .run();
}
