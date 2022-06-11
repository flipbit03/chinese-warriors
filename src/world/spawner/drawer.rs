use crate::assets::textures::TerrainTextures;
use crate::world::tile::WorldTileDrawInstrucion;
use bevy::prelude::Transform;
use bevy::prelude::*;

pub fn draw_terrain_from_instruction(
    mut commands: Commands,
    terrain_textures: Res<TerrainTextures>,
    tile_instructions_query: Query<
        (Entity, &WorldTileDrawInstrucion),
        Added<WorldTileDrawInstrucion>,
    >,
) {
    for (instruction_entity, tile_to_spawn) in tile_instructions_query.iter() {
        // This is the BaseTerrain converted to a number so that we can use it in the arrays below
        let base_terrain_index = tile_to_spawn.tile.worldterrain.terrain.base.clone() as usize;

        commands
            .entity(instruction_entity)
            //////////////////
            // Remove Draw Instruction
            //////////////////
            .remove::<WorldTileDrawInstrucion>()
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
                sprite: tile_to_spawn.sprite.clone(),
                ..Default::default()
            })
            //////////////////
            // Insert Decoration
            //////////////////
            .with_children(|p| {
                if let Some(decoration_index) = tile_to_spawn.tile.worldterrain.decoration {
                    p.spawn_bundle(SpriteBundle {
                        texture: terrain_textures.base_terrains[base_terrain_index].decorations
                            [decoration_index]
                            .clone(),
                        transform: Transform {
                            translation: Vec3::new(0.0, 0.0, 0.00001),
                            ..Default::default()
                        },
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
                                translation: Vec3::new(0.0, 0.0, border_layer_base_z + 0.0001),
                                ..Default::default()
                            };

                            let border_sprite_color = border_instruction.terrain.sprite_color();

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
            });
    }
}
