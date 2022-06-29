use bevy::{
    hierarchy::BuildChildren,
    math::{Vec2, Vec3},
    prelude::{Commands, Entity, Handle, Image, Res, Transform},
    sprite::{Sprite, SpriteBundle},
};

use crate::{
    assets::textures::TerrainTextures,
    world::tile::{
        chunk::CHUNK_SIDE_SIZE, position::ChunkPosition, WorldTileDrawInstrucion,
    },
};

pub fn draw_debug_chunk_grid(
    commands: &mut Commands,
    chunk_pos: &ChunkPosition,
    debug_texture: &Handle<Image>,
) -> Entity {
    let chunk_world_x = (chunk_pos.x * 64 * CHUNK_SIDE_SIZE) as f32;
    let chunk_world_y = (chunk_pos.y * 64 * CHUNK_SIDE_SIZE) as f32;

    commands
        .spawn_bundle(SpriteBundle {
            texture: debug_texture.clone(),
            sprite: Sprite {
                custom_size: Some(Vec2::new(
                    (64 * CHUNK_SIDE_SIZE - 2) as f32,
                    (64 * CHUNK_SIDE_SIZE - 2) as f32,
                )),
                ..Default::default()
            },
            transform: Transform::from_xyz(chunk_world_x, chunk_world_y, 0.9),
            ..Default::default()
        })
        .id()
}

pub fn draw_single_tile(
    commands: &mut Commands,
    // child_builder: &mut ChildBuilder,
    tile_to_spawn: &WorldTileDrawInstrucion,
    terrain_textures: &Res<TerrainTextures>,
) -> Entity {
    let base_terrain_index = tile_to_spawn.tile.terrain.base.clone() as usize;

    // Draw the whole tile and get its ID
    let new_tile_id = commands
        .spawn()
        //////////////////
        // Insert TilePosition (Our marker)
        //////////////////
        .insert(tile_to_spawn.tile.position.clone())
        //////////////////
        // Insert Base Terrain
        //////////////////
        .insert_bundle(SpriteBundle {
            texture: terrain_textures.base_terrains[base_terrain_index]
                .base
                .clone(),
            transform: tile_to_spawn.transform,
            sprite: tile_to_spawn.tile.terrain.terrain_sprite_color(),
            ..Default::default()
        })
        //////////////////
        // Insert Decoration
        //////////////////
        .with_children(|p| {
            if let Some(decoration_index) = tile_to_spawn.tile.decoration {
                p.spawn_bundle(SpriteBundle {
                    texture: terrain_textures.base_terrains[base_terrain_index]
                        .decorations[decoration_index]
                        .clone(),
                    transform: Transform {
                        translation: Vec3::new(0.0, 0.0, 0.00001),
                        ..Default::default()
                    },
                    sprite: tile_to_spawn.tile.terrain.decoration_sprite_color(),
                    ..Default::default()
                });
            }
        })
        //////////////////
        // Insert Borders
        //////////////////
        .with_children(|p| {
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
                                0.0,
                                0.0,
                                border_layer_base_z + 0.0001,
                            ),
                            ..Default::default()
                        };

                        let border_sprite_color =
                            border_instruction.terrain.terrain_sprite_color();

                        let bt_index_from_border: usize =
                            border_instruction.terrain.base.clone() as usize;

                        p.spawn_bundle(SpriteBundle {
                            texture: terrain_textures.base_terrains[bt_index_from_border]
                                .borders[border_texture_index]
                                .clone(),
                            transform: border_terrain_transform,
                            sprite: border_sprite_color,
                            ..Default::default()
                        });
                    }
                });
        })
        .with_children(|p| {
            if tile_to_spawn.debug_grid {
                p.spawn_bundle(SpriteBundle {
                    texture: terrain_textures.debug_grid.clone(),
                    transform: Transform {
                        translation: Vec3::new(0.0, 0.0, 0.01),
                        ..Default::default()
                    },
                    ..Default::default()
                });
            }
        })
        .id();

    new_tile_id
}
