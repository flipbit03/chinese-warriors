use bevy::prelude::*;

use crate::{
    assets::{config::structs::CwConfig, fonts::MainFont},
    hero::structs::Hero,
    world::tile::{chunk::WorldChunk, WorldTile},
};

#[derive(Component)]
#[allow(dead_code)]
pub struct HudText(u32);

pub fn spawn_hud_text(mut commands: Commands, main_font: Res<MainFont>) {
    commands.spawn((
        Text::new(""),
        TextFont {
            font: main_font.handle.clone(),
            font_size: 24.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(5.0),
            left: Val::Px(15.0),
            ..default()
        },
        HudText(1),
    ));
}

pub fn update_hud_text(
    mut hud_query: Query<&mut Text, (With<HudText>, Without<Hero>)>,
    config: Res<CwConfig>,
    hero_query: Query<(&Transform, Option<&WorldTile>), With<Hero>>,
    world_chunks_query: Query<&WorldChunk>,
    all_entities_query: Query<Entity>,
) {
    let Ok((hero_transform, hero_tile)) = hero_query.single() else {
        return;
    };
    let Ok(mut text) = hud_query.single_mut() else {
        return;
    };

    let tile_str = match hero_tile {
        Some(t) => {
            format!(
                "tilepos  : x={:08.1} y={:08.1}\nbiome={:?}\nterrain={:?} (move_speed_mult={:01.1})",
                t.position.x,
                t.position.y,
                t.biome_name,
                t.terrain.name,
                t.terrain.move_speed_multiplier
            )
        }
        None => "No WorldTile in Hero".to_string(),
    };

    **text = format!(
        "world_pos: x={:08.1} y={:08.1}\n{}\nchunk_count={:4}\nall_entities_count={:4}\nhero_move_speed={:?}",
        hero_transform.translation.x,
        hero_transform.translation.y,
        tile_str,
        world_chunks_query.iter().count(),
        all_entities_query.iter().count(),
        config.hero.move_speed
    );
}
