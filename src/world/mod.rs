pub mod despawner;
pub mod spawner;
pub mod tile;

use bevy::prelude::{App, Plugin};
use iyes_loopless::prelude::ConditionSet;

use crate::app::GameState;

use self::{
    despawner::despawn_terrain,
    spawner::{spawn_terrain_from_instruction, spawn_terrain_instruction},
};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::InGame)
                .with_system(spawn_terrain_instruction)
                .with_system(spawn_terrain_from_instruction)
                .with_system(despawn_terrain)
                .into(),
        );
    }
}
