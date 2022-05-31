use bevy::{
    core::Time,
    prelude::{Assets, Query, Res, With},
    sprite::{TextureAtlas, TextureAtlasSprite},
};

use crate::assets::textures::GuriTextureAtlas;

use super::structs::{Hero, HeroFacing, HeroWalkCycleTimer};

pub fn animate_hero(
    time: Res<Time>,
    guri_atlas: Res<GuriTextureAtlas>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(&mut HeroWalkCycleTimer, &mut TextureAtlasSprite, &Hero), With<Hero>>,
) {
    let (mut timer, mut sprite, hero) = query.single_mut();
    timer.tick(time.delta());
    if timer.just_finished() {
        let texture_atlas = texture_atlases
            .get(guri_atlas.texture_handle.clone())
            .unwrap();
        match hero.facing {
            HeroFacing::Right => {
                sprite.flip_x = true;
            }
            HeroFacing::Left => {
                sprite.flip_x = false;
            }
        };

        if let true = hero.walking {
            sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
        };
    };
}
