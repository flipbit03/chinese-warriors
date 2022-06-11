use bevy::{
    core::Timer,
    math::Vec3,
    prelude::{Commands, Res, Transform},
    sprite::SpriteSheetBundle,
};

use crate::assets::{config::structs::CwConfig, textures::GuriTextureAtlas};

use super::structs::{Hero, HeroWalkCycleTimer};

pub fn spawn_hero(
    mut commands: Commands,
    config: Res<CwConfig>,
    guri_atlas: Res<GuriTextureAtlas>,
) {
    println!("Spawning hero...");
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: guri_atlas.texture_handle.clone(),
            transform: Transform {
                translation: Vec3::new(config.hero.spawn_point.x, config.hero.spawn_point.y, 1.0),
                ..Transform::from_scale(Vec3::splat(1.0))
            },
            ..Default::default()
        })
        .insert(Hero::default())
        .insert(HeroWalkCycleTimer(Timer::from_seconds(0.07, true)));
}
