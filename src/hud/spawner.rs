use bevy::{
    math::Rect,
    prelude::{
        Color, Commands, Component, Query, Res, TextBundle, Transform, With,
        Without,
    },
    text::{HorizontalAlign, Text, TextAlignment, TextStyle},
    ui::{AlignSelf, PositionType, Style, Val},
};

use crate::{
    assets::{config::structs::CwConfig, fonts::MainFont},
    hero::structs::Hero,
    world::spawner::DrawableTerrainMaterial,
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
    hero_query: Query<&Transform, With<Hero>>,
    tile_query: Query<&DrawableTerrainMaterial>,
) {
    let hero_transform = hero_query.single();

    let mut text = hud_query.single_mut();

    text.sections[0].value = format!(
        "x={} y={} (existing_tiles: {})\n{:?}",
        hero_transform.translation.x,
        hero_transform.translation.y,
        tile_query.iter().count(),
        config
    );
}
