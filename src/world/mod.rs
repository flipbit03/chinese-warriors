pub mod despawner;
pub mod spawner;
pub mod tile;

use bevy::prelude::{App, Plugin};

use self::{
    despawner::despawn_terrain,
    spawner::{spawn_terrain_from_instruction, spawn_terrain_instruction},
};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_terrain_instruction)
            .add_system(spawn_terrain_from_instruction)
            .add_system(despawn_terrain);
    }
}
