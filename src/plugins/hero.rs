use bevy::prelude::{App, Plugin};

use crate::systems::hero::{animate_hero, hero_input, spawner::spawn_hero};

pub struct HeroPlugin;

impl Plugin for HeroPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_hero)
            .add_system(animate_hero)
            .add_system(hero_input);
    }
}
