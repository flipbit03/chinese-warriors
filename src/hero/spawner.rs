use bevy::{math::Vec3, prelude::*, sprite::Sprite};

use crate::{
    assets::aseprite::GuriAssets,
    hero::current_tile::MoveSpeed,
    shadow::structs::CastsShadow,
};

use super::structs::Hero;

pub fn spawn_hero(
    mut commands: Commands,
    config: Res<crate::assets::config::structs::CwConfig>,
    guri_assets: Res<GuriAssets>,
) {
    info!("Spawning hero...");

    commands.spawn((
        Sprite {
            image: guri_assets.texture.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: guri_assets.layout.clone(),
                index: guri_assets.idle_first,
            }),
            ..default()
        },
        Transform {
            translation: Vec3::new(
                config.hero.spawn_point.x,
                config.hero.spawn_point.y,
                1.0,
            ),
            ..Transform::from_scale(Vec3::splat(1.0))
        },
        Hero::default(),
        MoveSpeed(1.0),
        CastsShadow::default(),
        AnimationTimer(Timer::from_seconds(0.15, TimerMode::Repeating)),
    ));
}

/// Simple sprite sheet animation timer
#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);
