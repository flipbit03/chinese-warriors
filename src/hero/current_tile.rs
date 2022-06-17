use bevy::prelude::{Commands, Component, Entity, Query, Res, Transform, With};

use crate::{
    assets::config::structs::CwConfig,
    world::tile::{builder::WorldBuilder, position::TilePosition},
};

use super::structs::Hero;

#[derive(Component)]
pub struct MoveSpeed(pub f32);

pub fn hero_current_tile_and_movespeed(
    mut commands: Commands,
    config: Res<CwConfig>,
    hero_query: Query<(Entity, &Transform), With<Hero>>,
    world_builder: Res<WorldBuilder>,
) {
    let (hero_entity, hero_transform) = hero_query.single();

    let hero_x_tilepos_float =
        hero_transform.translation.x / config.world.tile_size.x * config.world.tile_scale;
    let hero_y_tilepos_float =
        (hero_transform.translation.y - 16.0) / config.world.tile_size.y * config.world.tile_scale;

    let hero_pos_tile = TilePosition {
        x: hero_x_tilepos_float.round() as i32,
        y: hero_y_tilepos_float.round() as i32,
    };

    let instruction = world_builder.create(hero_pos_tile.clone());

    let hero_movespeed = MoveSpeed(
        (config.hero.move_speed * instruction.tile.worldterrain.terrain.move_speed_multiplier)
            .max(1.0),
    );

    commands
        .entity(hero_entity)
        .insert(instruction.tile.clone())
        .insert(hero_movespeed);
}
