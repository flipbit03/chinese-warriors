use bevy::{
    math::Rect,
    prelude::{
        Color, Commands, Component, Entity, Query, Res, TextBundle, Transform, With,
        Without,
    },
    text::{HorizontalAlign, Text, TextAlignment, TextStyle},
    ui::{AlignSelf, PositionType, Style, Val},
};

use crate::{
    assets::{config::structs::CwConfig, fonts::MainFont},
    hero::structs::Hero,
    world::tile::{position::TilePosition, WorldTile},
};

#[derive(Component)]
pub struct HudText(u32);

pub fn spawn_hud_text(mut commands: Commands, main_font: Res<MainFont>) {
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(5.0),
                    left: Val::Px(15.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            // Use the `Text::with_section` constructor
            text: Text::with_section(
                // Accepts a `String` or any type that converts into a `String`, such as `&str`
                "",
                TextStyle {
                    font: main_font.handle.clone(),
                    font_size: 24.0,
                    color: Color::WHITE,
                },
                // Note: You can use `Default::default()` in place of the `TextAlignment`
                TextAlignment {
                    horizontal: HorizontalAlign::Left,
                    ..Default::default()
                },
            ),
            ..Default::default()
        })
        .insert(HudText(1));
}

pub fn update_hud_text(
    mut hud_query: Query<&mut Text, (With<HudText>, Without<Hero>)>,
    config: Res<CwConfig>,
    hero_query: Query<(&Transform, Option<&WorldTile>), With<Hero>>,
    tile_query: Query<&TilePosition>,
    all_entities_query: Query<Entity>,
) {
    let (hero_transform, hero_tile) = hero_query.single();

    let mut text = hud_query.single_mut();

    let tile_str = match hero_tile {
        Some(t) => {
            format!(
                "tilepos  : x={:08.1} y={:08.1}\nbiome={:?}\nterrain={:?} (move_speed_mult={:01.1})",
                t.position.x,
                t.position.y,
                t.worldterrain.biome_name,
                t.worldterrain.terrain.name,
                t.worldterrain.terrain.move_speed_multiplier
            )
        }
        None => "No WorldTile in Hero".to_string(),
    };

    text.sections[0].value = format!(
        "world_pos: x={:08.1} y={:08.1}\n{}\ntile_position_count={:4}\nall_entities_count={:4}\nhero_move_speed={:?}",
        hero_transform.translation.x,
        hero_transform.translation.y,
        tile_str,
        tile_query.iter().count(),
        all_entities_query.iter().count(),
        config.hero.move_speed
    );
}
