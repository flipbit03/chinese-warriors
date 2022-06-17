use crate::assets::config::structs::CwConfig;
use crate::world::tile::builder::WorldBuilder;
use crate::world::tile::position::TilePosition;
use crate::world::tile::visibility::{get_screen_rect, get_visible_tiles};

use bevy::prelude::Transform;
use bevy::prelude::*;
use bevy::render::camera::Camera2d;

pub fn generate_terrain_instruction(
    mut commands: Commands,
    camera_query: Query<(&Transform, &OrthographicProjection), With<Camera2d>>,
    config: Res<CwConfig>,
    tile_query: Query<(Entity, &TilePosition)>,
    world_builder: Res<WorldBuilder>,
) {
    let (camera_transform, camera_projection) = camera_query.single();

    let screen_rect = get_screen_rect(
        camera_transform,
        camera_projection,
        camera_projection.scale * 1.18,
    );

    for tile_to_create in get_visible_tiles(
        screen_rect,
        config.world.tile_size,
        world_builder.tile_scale,
    ) {
        if let Some(_) = tile_query.iter().position(|(_, p)| *p == tile_to_create) {
            continue;
        }

        let new_tile = world_builder.create(tile_to_create);

        // Spawn
        commands.spawn().insert(new_tile);
    }
}
