use benimator::Play;
use bevy::{math::Vec3, prelude::*, sprite::SpriteSheetBundle};
use bevy_ase::asset::Animation;

use crate::{
    assets::{aseprite::GuriAssets, config::structs::CwConfig},
    hero::current_tile::MoveSpeed,
};

use super::structs::Hero;

pub fn spawn_hero(
    mut commands: Commands,
    config: Res<CwConfig>,
    guri_assets: Res<GuriAssets>,
    animations: Res<Assets<Animation>>,
) {
    info!("Spawning hero...");

    let animation = animations.get(guri_assets.idle_anim.0.clone()).unwrap();

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: animation.atlas(),
            transform: Transform {
                translation: Vec3::new(
                    config.hero.spawn_point.x,
                    config.hero.spawn_point.y,
                    1.0,
                ),
                ..Transform::from_scale(Vec3::splat(1.0))
            },
            ..Default::default()
        })
        .insert(guri_assets.idle_anim.1.clone())
        .insert(Play)
        .insert(Hero::default())
        .insert(MoveSpeed(1.0));
}
