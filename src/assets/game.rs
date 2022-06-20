use bevy::prelude::{Commands, Res};

use crate::world::tile::builder::WorldBuilder;

use super::config::structs::CwConfig;

pub fn create_initial_tilebuilder(mut commands: Commands, config: Res<CwConfig>) {
    generate_tile_builder_resource_from_config(&mut commands, config.as_ref())
}

pub fn generate_tile_builder_resource_from_config(
    commands: &mut Commands,
    config: &CwConfig,
) {
    commands.insert_resource(WorldBuilder::new_from_config(&config.world));
}
