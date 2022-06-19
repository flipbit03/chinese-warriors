use bevy::{
    prelude::{Assets, Commands, Entity, Query, Res},
    sprite::TextureAtlasSprite,
};
use bevy_ase::asset::Animation;

use crate::assets::aseprite::GuriAssets;

use super::structs::{Hero, HeroAction, HeroFacing};

// TODO: Animation speed has to respect "MoveSpeed"
// TODO: Walk Cycle does not happen on move instruction.
pub fn animate_hero(
    mut commands: Commands,
    guri_assets: Res<GuriAssets>,
    animations: Res<Assets<Animation>>,
    mut hero_query: Query<(Entity, &Hero, &mut TextureAtlasSprite)>,
) {
    let (hero_entity, hero, mut hero_tap) = hero_query.single_mut();

    let (hero_animation, hero_spriteanimationsheet) = match hero.action {
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

    let mut hero_entity_commands = commands.entity(hero_entity);

    hero_tap.flip_x = match hero.facing {
        HeroFacing::Left => false,
        HeroFacing::Right => true,
    };

    hero_entity_commands
        .insert(hero_animation.atlas())
        .insert(hero_spriteanimationsheet);
}
