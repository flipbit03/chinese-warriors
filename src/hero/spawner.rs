use bevy::{math::Vec3, prelude::*};
use bevy_aseprite_ultra::prelude::*;

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
        AseAnimation {
            aseprite: guri_assets.aseprite.clone(),
            animation: Animation::tag("Idle").with_repeat(AnimationRepeat::Loop),
        },
        Sprite::default(),
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
    ));
}
