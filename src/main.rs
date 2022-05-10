// use application::{Application, ApplicationInputs};

// mod application;
// mod game;
// mod util;

use bevy::prelude::*;
use game::{setup::setup, hero::{spawn_hero, animate_hero, hero_input}};

mod game;


fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_startup_system_to_stage(
        StartupStage::PreStartup,
        setup)
        .add_startup_system(spawn_hero)
        .add_system(animate_hero)
        .add_system(hero_input)
        .run();
}
