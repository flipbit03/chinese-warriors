// use application::{Application, ApplicationInputs};

// mod application;
// mod game;
// mod util;

use bevy::prelude::*;
use game::{
    fpscounter::timed_fps_counter,
    hero::{animate_hero, hero_input, spawn_hero},
    setup::setup,
    terrain::{frustum_culling::update_world_frustum, spawn_terrain},
};

mod game;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system_to_stage(StartupStage::PreStartup, setup)
        .add_startup_system(spawn_hero)
        .add_system(animate_hero)
        .add_system(spawn_terrain)
        .add_system(hero_input)
        .add_system(timed_fps_counter)
        .add_system(update_world_frustum)
        .run();
}
