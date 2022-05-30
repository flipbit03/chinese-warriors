use bevy::prelude::{Commands, Res};

use crate::world::tile::TileBuilder;

use super::config::structs::CwConfig;

pub fn load_game_assets(mut commands: Commands, config: Res<CwConfig>) {
    // Spawn Tile Builder with set game_seed;
    commands.insert_resource(TileBuilder::new_with_seed(
        config.terrain.noise_seed,
        config.terrain.tile_size,
        config.terrain.tile_scale,
    ));
}
