pub mod terrain;

use bevy::prelude::*;

use crate::app::GameState;

use self::terrain::collision_with_non_walkable_terrain;

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            collision_with_non_walkable_terrain.run_if(in_state(GameState::InGame)),
        );
    }
}
