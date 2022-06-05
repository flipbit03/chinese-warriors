pub mod terrain;

use bevy::prelude::{App, Plugin};
use iyes_loopless::prelude::ConditionSet;

use crate::app::GameState;

use self::terrain::collision_with_non_walkable_terrain;

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::InGame)
                .with_system(collision_with_non_walkable_terrain)
                //.with_system(spawn_terrain_from_instruction)
                .into(),
        );
    }
}
