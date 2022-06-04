pub mod despawner;
pub mod spawner;
pub mod tile;

use bevy::prelude::{App, Plugin};
use iyes_loopless::prelude::ConditionSet;

use crate::app::GameState;

use self::{spawner::{instruction::generate_terrain_instruction, drawer::draw_terrain_from_instruction}, despawner::{despawn_far_terrain, despawn_all_terrain, DespawnAllTerrain}};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::InGame)
                .with_system(generate_terrain_instruction)
                .with_system(draw_terrain_from_instruction)
                //.with_system(spawn_terrain_from_instruction)
                .with_system(despawn_far_terrain)
                .into(),
        )
        .add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::InGame)
                .run_if_resource_exists::<DespawnAllTerrain>()
                .with_system(despawn_all_terrain)
                .into(),
        );
    }
}
