use crate::assets::config::structs::CwConfig;

pub struct DespawnAllTerrain;

use super::{
    spawner::DrawableTerrainMaterial,
    tile::{
        position::TilePosition,
        visibility::{get_screen_rect, get_visible_tiles},
    },
};

use bevy::{prelude::*, render::camera::Camera2d};

pub fn despawn_terrain(
    mut commands: Commands,
    camera_query: Query<(&Transform, &OrthographicProjection), With<Camera2d>>,
    tile_query: Query<(Entity, &DrawableTerrainMaterial)>,
    config: Res<CwConfig>,
) {
    let (camera_transform, camera_projection) = camera_query.single();

    let screen_rect = get_screen_rect(
        camera_transform,
        camera_projection,
        camera_projection.scale * 1.3,
    );

    let no_despawn_area: Vec<TilePosition> = get_visible_tiles(
        screen_rect,
        config.terrain.tile_size,
        config.terrain.tile_scale,
    )
    .collect();

    for (tile_entity, tile_drawableterrainmaterial) in tile_query.iter() {
        if let None = no_despawn_area
            .iter()
            .position(|x| x == &tile_drawableterrainmaterial.0)
        {
            commands.entity(tile_entity).despawn_recursive()
        }
    }
}



pub fn despawn_all_terrain(
    mut commands: Commands,
    tile_query: Query<Entity, With<DrawableTerrainMaterial>>,
) {
    for tile_entity in tile_query.iter() {
        commands.entity(tile_entity).despawn_recursive()
    }

    commands.remove_resource::<DespawnAllTerrain>();
}
