use benimator::{PlaySpeedMultiplier, SpriteSheetAnimation};
use bevy::{
    prelude::{Assets, Commands, Entity, Handle, Query, Res},
    sprite::{TextureAtlas, TextureAtlasSprite},
};
use bevy_ase::asset::Animation;

use crate::assets::aseprite::GuriAssets;

use super::{
    current_tile::MoveSpeed,
    structs::{Hero, HeroAction, HeroFacing},
};

// TODO: Animation speed has to respect "MoveSpeed"
// TODO: Walk Cycle does not happen on move instruction.
pub fn animate_hero(
    mut commands: Commands,
    guri_assets: Res<GuriAssets>,
    animations: Res<Assets<Animation>>,
    mut hero_query: Query<(
        Entity,
        &Hero,
        &mut Handle<TextureAtlas>,
        &mut Handle<SpriteSheetAnimation>,
        &mut TextureAtlasSprite,
        &MoveSpeed,
    )>,
) {
    let (hero_entity, hero, mut hero_ta, mut hero_sa, mut hero_tap, hero_movespeed) =
        hero_query.single_mut();

    let (hero_current_animation, hero_sprite_animation_sheet) = match hero.action {
        HeroAction::Idling => {
            commands.entity(hero_entity).remove::<PlaySpeedMultiplier>();
            (
                animations.get(guri_assets.idle_anim.0.clone()).unwrap(),
                guri_assets.idle_anim.1.clone(),
            )
        }
        HeroAction::Walking => {
            commands
                .entity(hero_entity)
                .insert(PlaySpeedMultiplier::new(hero_movespeed.0 as f64 * 0.95));
            (
                animations
                    .get(guri_assets.walk_cycle_anim.0.clone())
                    .unwrap(),
                guri_assets.walk_cycle_anim.1.clone(),
            )
        }
    };

    hero_tap.flip_x = match hero.facing {
        HeroFacing::Left => false,
        HeroFacing::Right => true,
    };

    *hero_ta = hero_current_animation.atlas();
    *hero_sa = hero_sprite_animation_sheet;
}
