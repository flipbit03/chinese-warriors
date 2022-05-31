use bevy::{
    core::Timer,
    math::Vec3,
    prelude::{Commands, Res, Transform},
    sprite::SpriteSheetBundle,
};

use crate::assets::textures::GuriTextureAtlas;

use super::structs::{Hero, HeroWalkCycleTimer};

pub fn spawn_hero(mut commands: Commands, guri_atlas: Res<GuriTextureAtlas>) {
    println!("Spawning hero...");
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: guri_atlas.texture_handle.clone(),
            transform: Transform {
                translation: Vec3::new(14711.0, 12905.0, 1.0),
                ..Transform::from_scale(Vec3::splat(1.0))
            },
            ..Default::default()
        })
        .insert(Hero::default())
        .insert(HeroWalkCycleTimer(Timer::from_seconds(0.07, true)));
}
