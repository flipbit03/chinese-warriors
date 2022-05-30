use bevy::prelude::{App, Plugin};

use self::{animation::animate_hero, spawner::spawn_hero};

pub mod animation;
pub mod spawner;
pub mod structs;

pub struct HeroPlugin;

impl Plugin for HeroPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_hero).add_system(animate_hero);
    }
}
