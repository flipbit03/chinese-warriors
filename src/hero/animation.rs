use benimator::SpriteSheetAnimation;
use bevy::{
    prelude::{Assets, Handle, Query, Res},
    sprite::{TextureAtlas, TextureAtlasSprite},
};
use bevy_ase::asset::Animation;

use crate::assets::aseprite::GuriAssets;

use super::structs::{Hero, HeroAction, HeroFacing};

// TODO: Animation speed has to respect "MoveSpeed"
// TODO: Walk Cycle does not happen on move instruction.
pub fn animate_hero(
    guri_assets: Res<GuriAssets>,
    animations: Res<Assets<Animation>>,
    mut hero_query: Query<(
        &Hero,
        &mut Handle<TextureAtlas>,
        &mut Handle<SpriteSheetAnimation>,
        &mut TextureAtlasSprite,
    )>,
) {
    let (hero, mut hero_ta, mut hero_sa, mut hero_tap) = hero_query.single_mut();

    let (hero_current_animation, hero_sprite_animation_sheet) = match hero.action {
        HeroAction::Idling => (
            animations.get(guri_assets.idle_anim.0.clone()).unwrap(),
            guri_assets.idle_anim.1.clone(),
        ),
        HeroAction::Walking => (
            animations
                .get(guri_assets.walk_cycle_anim.0.clone())
                .unwrap(),
            guri_assets.walk_cycle_anim.1.clone(),
        ),
    };

    hero_tap.flip_x = match hero.facing {
        HeroFacing::Left => false,
        HeroFacing::Right => true,
    };

    *hero_ta = hero_current_animation.atlas();
    *hero_sa = hero_sprite_animation_sheet;
}
