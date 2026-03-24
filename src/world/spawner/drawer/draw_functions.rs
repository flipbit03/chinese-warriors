use bevy::{
    math::Vec2,
    prelude::{Commands, Entity, Handle, Image, Transform},
    sprite::Sprite,
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
        .spawn((
            Sprite {
                image: debug_texture.clone(),
                custom_size: Some(Vec2::new(
                    (64 * CHUNK_SIDE_SIZE - 2) as f32,
                    (64 * CHUNK_SIDE_SIZE - 2) as f32,
                )),
                ..Default::default()
            },
            Transform::from_xyz(chunk_world_x, chunk_world_y, 0.9),
        ))
        .id()
}

pub fn draw_single_tile(
    commands: &mut Commands,
    tile_to_spawn: &WorldTileDrawInstrucion,
    terrain_textures: &TerrainTextures,
) -> Entity {
    let base_terrain_index = tile_to_spawn.tile.terrain.base.clone() as usize;

    let new_tile_id = commands
        .spawn((
            Sprite {
                image: terrain_textures.base_terrains[base_terrain_index]
                    .base
                    .clone(),
                color: tile_to_spawn.tile.terrain.terrain_sprite_color(),
                ..Default::default()
            },
            tile_to_spawn.transform,
            tile_to_spawn.tile.position,
        ))
        .with_children(|p| {
            // Decoration
            if let Some(decoration_index) = tile_to_spawn.tile.decoration {
                p.spawn((
                    Sprite {
                        image: terrain_textures.base_terrains[base_terrain_index]
                            .decorations[decoration_index]
                            .clone(),
                        color: tile_to_spawn.tile.terrain.decoration_sprite_color(),
                        ..Default::default()
                    },
                    Transform::from_xyz(0.0, 0.0, 0.00001),
                ));
            }

            // Borders
            for (terrain_order, border_instruction) in
                tile_to_spawn.tile.borders.iter().enumerate()
            {
                let border_layer_base_z = terrain_order as f32 / 1000.0;
                for border_texture_index in border_instruction.get_texture_indexes() {
                    let bt_index_from_border: usize =
                        border_instruction.terrain.base.clone() as usize;

                    p.spawn((
                        Sprite {
                            image: terrain_textures.base_terrains[bt_index_from_border]
                                .borders[border_texture_index]
                                .clone(),
                            color: border_instruction.terrain.terrain_sprite_color(),
                            ..Default::default()
                        },
                        Transform::from_xyz(0.0, 0.0, border_layer_base_z + 0.0001),
                    ));
                }
            }

            // Debug grid
            if tile_to_spawn.debug_grid {
                p.spawn((
                    Sprite {
                        image: terrain_textures.debug_grid.clone(),
                        ..Default::default()
                    },
                    Transform::from_xyz(0.0, 0.0, 0.01),
                ));
            }
        })
        .id();

    new_tile_id
}
