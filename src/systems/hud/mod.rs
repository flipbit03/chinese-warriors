use super::{hero::types::Hero, world::tile::TileBuilder};
use bevy::prelude::*;
pub mod spawner;
pub mod window_title;

#[derive(Component)]
pub struct HudText;

pub fn update_hud(
    mut hud_query: Query<&mut Text, (With<HudText>, Without<Hero>)>,
    tile_builder: Res<TileBuilder>,
    hero_query: Query<&Transform, With<Hero>>,
) {
    let hero_transform = hero_query.single();

    let mut text = hud_query.single_mut();

    text.sections[0].value = format!(
        "x={} y={} (existing_tiles: {})",
        hero_transform.translation.x,
        hero_transform.translation.y,
        tile_builder.storage.len()
    );
}
