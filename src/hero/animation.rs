use bevy::prelude::*;

use crate::assets::aseprite::GuriAssets;

use super::{
    current_tile::MoveSpeed,
    spawner::AnimationTimer,
    structs::{Hero, HeroAction, HeroFacing},
};

pub fn animate_hero(
    time: Res<Time>,
    guri_assets: Res<GuriAssets>,
    mut hero_query: Query<(
        &Hero,
        &mut Sprite,
        &mut AnimationTimer,
        &MoveSpeed,
    )>,
) {
    let Ok((hero, mut sprite, mut timer, hero_movespeed)) = hero_query.single_mut() else {
        return;
    };

    // Determine animation range based on action
    let (first, last) = match hero.action {
        HeroAction::Idling => (guri_assets.idle_first, guri_assets.idle_last),
        HeroAction::Walking => (guri_assets.walk_first, guri_assets.walk_last),
    };

    // Adjust animation speed based on move speed when walking
    let base_duration = match hero.action {
        HeroAction::Idling => 0.3,
        HeroAction::Walking => (0.15 / (hero_movespeed.0 as f64 * 0.95)).max(0.05),
    };
    timer.set_duration(std::time::Duration::from_secs_f64(base_duration));

    timer.tick(time.delta());

    // Flip sprite based on facing
    sprite.flip_x = match hero.facing {
        HeroFacing::Left => false,
        HeroFacing::Right => true,
    };

    if timer.just_finished() {
        if let Some(atlas) = &mut sprite.texture_atlas {
            atlas.index = if atlas.index >= last || atlas.index < first {
                first
            } else {
                atlas.index + 1
            };
        }
    }
}
