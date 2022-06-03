use crate::assets::config::structs::CwConfig;
use crate::assets::textures::TerrainTextures;

use super::tile::builder::WorldBuilder;
use super::tile::position::TilePosition;
use super::tile::visibility::{get_screen_rect, get_visible_tiles};
use super::tile::WorldTileDrawInstrucion;
use bevy::prelude::Transform;
use bevy::prelude::*;
use bevy::render::camera::Camera2d;

#[derive(Component, Debug)]
pub struct DrawableTerrainMaterial(pub TilePosition);

pub fn spawn_terrain_instruction(
    mut commands: Commands,
    camera_query: Query<(&Transform, &OrthographicProjection), With<Camera2d>>,
    config: Res<CwConfig>,
    tile_query: Query<(Entity, &DrawableTerrainMaterial)>,
    mut tile_builder: ResMut<WorldBuilder>,
) {
    let (camera_transform, camera_projection) = camera_query.single();

    let screen_rect = get_screen_rect(
        camera_transform,
        camera_projection,
        camera_projection.scale * 1.2,
    );

    for tile_to_create in
        get_visible_tiles(screen_rect, config.world.tile_size, tile_builder.tile_scale)
    {
        if let Some(_) = tile_query.iter().position(|(_, p)| p.0 == tile_to_create) {
            continue;
        }

        let new_tile = tile_builder.create(tile_to_create);

        // Spawn
        commands.spawn().insert(new_tile);
    }
}

pub fn spawn_terrain_from_instruction(
    mut commands: Commands,
    terrain_textures: Res<TerrainTextures>,
    tile_instructions_query: Query<
        (Entity, &WorldTileDrawInstrucion),
        Added<WorldTileDrawInstrucion>,
    >,
) {
    for (instruction_entity, tile_to_spawn) in tile_instructions_query.iter() {
        // This is the BaseTerrain converted to a number so that we can use it in the arrays below
        let bt_index = tile_to_spawn.tile.terrain.clone().base as usize;

        // TODO: Make all these terrains be 'children' so that they can be despawned together.
        commands.entity(instruction_entity).despawn();

        // Draw Base Terrain
        commands
            .spawn_bundle(SpriteBundle {
                texture: terrain_textures.base_terrains[bt_index].base.clone(),
                transform: tile_to_spawn.transform,
                ..Default::default()
            })
            .insert(DrawableTerrainMaterial(tile_to_spawn.tile.position.clone()));

        // Draw Decoration
        if let Some(decoration_index) = tile_to_spawn.tile.terrain.decoration {
            commands
                .spawn_bundle(SpriteBundle {
                    texture: terrain_textures.base_terrains[bt_index].decorations[decoration_index]
                        .clone(),
                    transform: Transform {
                        translation: Vec3::new(
                            tile_to_spawn.transform.translation.x,
                            tile_to_spawn.transform.translation.y,
                            tile_to_spawn.transform.translation.z + 0.00001,
                        ),
                        scale: tile_to_spawn.transform.scale,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(DrawableTerrainMaterial(tile_to_spawn.tile.position.clone()));
        }

        // Draw Borders
        tile_to_spawn
            .tile
            .borders
            .clone()
            .into_iter()
            .enumerate()
            .for_each(|(terrain_order, border_instruction)| {
                let border_layer_base_z = terrain_order as f32 / 1000.0;
                for border_texture_index in border_instruction.get_texture_indexes() {
                    let border_terrain_transform = Transform {
                        translation: Vec3::new(
                            tile_to_spawn.transform.translation.x,
                            tile_to_spawn.transform.translation.y,
                            tile_to_spawn.transform.translation.z + border_layer_base_z + 0.001,
                        ),
                        scale: tile_to_spawn.transform.scale,
                        ..Default::default()
                    };

                    let bt_index_from_border: usize = border_instruction.terrain.clone() as usize;

                    commands
                        .spawn_bundle(SpriteBundle {
                            texture: terrain_textures.base_terrains[bt_index_from_border].borders
                                [border_texture_index]
                                .clone(),
                            transform: border_terrain_transform,
                            ..Default::default()
                        })
                        .insert(DrawableTerrainMaterial(tile_to_spawn.tile.position.clone()));
                }
            });
    }
}
