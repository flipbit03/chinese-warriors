use bevy::prelude::{Commands, Res};

use crate::world::tile::TileBuilder;

use super::config::structs::CwConfig;

pub fn create_initial_tilebuilder(mut commands: Commands, config: Res<CwConfig>) {
    generate_tile_builder_resource_from_config(&mut commands, &config)
}

pub fn generate_tile_builder_resource_from_config(commands: &mut Commands, config: &CwConfig) {
    println!("TileBuilder created with config=({:?})", &config);
    commands.insert_resource(TileBuilder::new_with_seed(
        config.terrain.noise_seed,
        config.terrain.tile_size,
        config.terrain.tile_scale,
    ));
}
